use chrono::{NaiveDate, NaiveDateTime, TimeZone};
use chrono_tz::America::Mexico_City;
use serde_json::{json, Map, Value};
use std::collections::BTreeMap;

use crate::error::{MedSysError, Result};
use crate::model::mapping::{FieldMapping, ResourceMapping};

/// Segmento individual de una ruta FHIR / JSON (e.g. `identifier[0].value`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PathSegment {
    Key(String),
    Index(usize),
}

/// Parsea una ruta dot/bracket notation como `identifier[0].value` o `name[0].given[1]`.
pub fn parse_path_segments(path: &str) -> Vec<PathSegment> {
    let mut segments = Vec::new();
    let mut current = String::new();
    let mut chars = path.chars().peekable();

    while let Some(c) = chars.next() {
        match c {
            '.' => {
                if !current.is_empty() {
                    segments.push(PathSegment::Key(current.clone()));
                    current.clear();
                }
            }
            '[' => {
                if !current.is_empty() {
                    segments.push(PathSegment::Key(current.clone()));
                    current.clear();
                }
                let mut idx_str = String::new();
                while let Some(&next_c) = chars.peek() {
                    if next_c == ']' {
                        chars.next();
                        break;
                    }
                    idx_str.push(chars.next().unwrap());
                }
                if let Ok(idx) = idx_str.parse::<usize>() {
                    segments.push(PathSegment::Index(idx));
                }
            }
            ']' => {
                // Delimitador de cierre procesado en '['
            }
            other => {
                current.push(other);
            }
        }
    }
    if !current.is_empty() {
        segments.push(PathSegment::Key(current));
    }
    segments
}

/// Inyecta un valor en una estructura JSON en la ruta indicada, creando objetos y arrays automáticamente.
pub fn set_json_path(root: &mut Value, path: &str, new_value: Value) -> Result<()> {
    let segments = parse_path_segments(path);
    if segments.is_empty() {
        return Err(MedSysError::TransformationError(format!(
            "Ruta JSON de destino inválida o vacía: '{path}'"
        )));
    }

    if !root.is_object() {
        *root = Value::Object(Map::new());
    }

    let mut current = root;
    for i in 0..segments.len() - 1 {
        let seg = &segments[i];
        let next_seg = &segments[i + 1];

        match seg {
            PathSegment::Key(key) => {
                if !current.is_object() {
                    *current = Value::Object(Map::new());
                }
                let obj = current.as_object_mut().unwrap();
                if !obj.contains_key(key) {
                    let placeholder = match next_seg {
                        PathSegment::Index(_) => Value::Array(Vec::new()),
                        PathSegment::Key(_) => Value::Object(Map::new()),
                    };
                    obj.insert(key.clone(), placeholder);
                }
                current = obj.get_mut(key).unwrap();
            }
            PathSegment::Index(idx) => {
                if !current.is_array() {
                    *current = Value::Array(Vec::new());
                }
                let arr = current.as_array_mut().unwrap();
                while arr.len() <= *idx {
                    let placeholder = match next_seg {
                        PathSegment::Index(_) => Value::Array(Vec::new()),
                        PathSegment::Key(_) => Value::Object(Map::new()),
                    };
                    arr.push(placeholder);
                }
                current = &mut arr[*idx];
            }
        }
    }

    match segments.last().unwrap() {
        PathSegment::Key(key) => {
            if !current.is_object() {
                *current = Value::Object(Map::new());
            }
            current
                .as_object_mut()
                .unwrap()
                .insert(key.clone(), new_value);
        }
        PathSegment::Index(idx) => {
            if !current.is_array() {
                *current = Value::Array(Vec::new());
            }
            let arr = current.as_array_mut().unwrap();
            while arr.len() <= *idx {
                arr.push(Value::Null);
            }
            arr[*idx] = new_value;
        }
    }

    Ok(())
}

/// Parsea una fecha hacia el formato canónico ISO 8601 (`YYYY-MM-DD`).
pub fn parse_date_iso8601(raw: &str) -> Result<String> {
    let trimmed = raw.trim();

    let parsed_date = NaiveDate::parse_from_str(trimmed, "%Y-%m-%d")
        .or_else(|_| NaiveDate::parse_from_str(trimmed, "%Y/%m/%d"))
        .or_else(|_| NaiveDate::parse_from_str(trimmed, "%d/%m/%Y"))
        .or_else(|_| NaiveDate::parse_from_str(trimmed, "%d-%m-%Y"))
        .or_else(|_| NaiveDate::parse_from_str(trimmed, "%Y%m%d"))
        .or_else(|_| {
            NaiveDateTime::parse_from_str(trimmed, "%Y-%m-%d %H:%M:%S").map(|dt| dt.date())
        })
        .or_else(|_| {
            NaiveDateTime::parse_from_str(trimmed, "%Y-%m-%dT%H:%M:%S").map(|dt| dt.date())
        });

    match parsed_date {
        Ok(date) => Ok(date.format("%Y-%m-%d").to_string()),
        Err(err) => Err(MedSysError::TransformationError(format!(
            "Fallo al parsear fecha '{raw}' con transformación date_iso8601: {err}"
        ))),
    }
}

/// Parsea fecha y hora hacia el formato canónico ISO 8601 (`YYYY-MM-DDTHH:MM:SS`).
pub fn parse_datetime_iso8601(raw: &str) -> Result<String> {
    let trimmed = raw.trim();

    if let Ok(dt_fixed) = chrono::DateTime::parse_from_rfc3339(trimmed) {
        return Ok(dt_fixed.format("%Y-%m-%dT%H:%M:%S%:z").to_string());
    }

    let parsed_dt = NaiveDateTime::parse_from_str(trimmed, "%Y-%m-%d %H:%M:%S")
        .or_else(|_| NaiveDateTime::parse_from_str(trimmed, "%Y-%m-%dT%H:%M:%S"))
        .or_else(|_| NaiveDateTime::parse_from_str(trimmed, "%Y-%m-%dT%H:%M:%SZ"))
        .or_else(|_| NaiveDateTime::parse_from_str(trimmed, "%Y/%m/%d %H:%M:%S"))
        .or_else(|_| NaiveDateTime::parse_from_str(trimmed, "%d/%m/%Y %H:%M:%S"))
        .or_else(|_| {
            NaiveDate::parse_from_str(trimmed, "%Y-%m-%d").map(|d| d.and_hms_opt(0, 0, 0).unwrap())
        });

    match parsed_dt {
        Ok(dt) => {
            let tz = Mexico_City;
            let dt_with_tz = match tz.from_local_datetime(&dt) {
                chrono::LocalResult::Single(dt_tz) => dt_tz,
                chrono::LocalResult::Ambiguous(dt1, _) => dt1,
                chrono::LocalResult::None => dt.and_utc().with_timezone(&tz),
            };
            Ok(dt_with_tz.format("%Y-%m-%dT%H:%M:%S%:z").to_string())
        }
        Err(err) => Err(MedSysError::TransformationError(format!(
            "Fallo al parsear fecha y hora '{raw}' con transformación datetime_iso8601: {err}"
        ))),
    }
}

/// Aplica una directiva `transform` (e.g. `date_iso8601`, `datetime_iso8601`, `reference:Patient/{value}`).
pub fn apply_transform(transform_name: &str, raw_value: &str) -> Result<String> {
    match transform_name {
        "date_iso8601" => parse_date_iso8601(raw_value),
        "datetime_iso8601" => parse_datetime_iso8601(raw_value),
        custom if custom.starts_with("reference:") => {
            let pattern = &custom["reference:".len()..];
            let trimmed = raw_value.trim();
            if trimmed.is_empty() {
                Err(MedSysError::TransformationError(format!(
                    "Valor vacío al construir referencia FHIR con patrón '{pattern}'"
                )))
            } else {
                Ok(pattern.replace("{value}", trimmed))
            }
        }
        unknown => Err(MedSysError::TransformationError(format!(
            "Transformación no soportada: '{unknown}'"
        ))),
    }
}

/// Aplica la búsqueda en diccionario catalogo normativo.
pub fn apply_dictionary(
    dictionary: &BTreeMap<String, String>,
    raw_value: &str,
    target_path: &str,
) -> Result<String> {
    let trimmed = raw_value.trim();
    if let Some(val) = dictionary.get(trimmed) {
        Ok(val.clone())
    } else {
        Err(MedSysError::TransformationError(format!(
            "Valor '{trimmed}' no encontrado en diccionario de mapeo para '{target_path}'"
        )))
    }
}

/// Aplica conversión de unidades físicas soportando "lb_to_kg" y "f_to_c".
pub fn apply_unit_conversion(conversion: &str, raw_value: &str) -> Result<String> {
    let trimmed = raw_value.trim();
    let val_f64: f64 = trimmed.parse().map_err(|_| {
        MedSysError::ProcessingError(format!(
            "T2 Error de formato en unit_conversion '{conversion}': no se pudo parsear como numérico '{trimmed}'"
        ))
    })?;

    match conversion {
        "lb_to_kg" => {
            // Factor oficial de conversión: 1 lb = 0.45359237 kg, redondeado a 2 decimales
            let kg = val_f64 * 0.45359237;
            let rounded = (kg * 100.0).round() / 100.0;
            Ok(format!("{:.2}", rounded))
        }
        "f_to_c" => {
            // (°F - 32) * 5/9, redondeado a 2 decimales
            let c = (val_f64 - 32.0) * 5.0 / 9.0;
            let rounded = (c * 100.0).round() / 100.0;
            Ok(format!("{:.2}", rounded))
        }
        other => Err(MedSysError::InvalidRule(format!(
            "Conversión de unidades no soportada: '{other}'"
        ))),
    }
}

/// Parsea una cadena de fecha/hora probando una lista de formatos separados por '|'
/// y resolviendo la zona horaria (por defecto "America/Mexico_City" via `chrono_tz`)
/// cuando el timestamp no incluya offset explícito.
pub fn parse_datetime_with_formats(
    raw: &str,
    formats_spec: &str,
    default_tz: Option<&str>,
) -> Result<String> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(MedSysError::ProcessingError(
            "T2 Error de formato en parse_datetime: valor de fecha vacío".to_string(),
        ));
    }

    let tz_name = default_tz.unwrap_or("America/Mexico_City");
    let tz: chrono_tz::Tz = tz_name.parse().unwrap_or(Mexico_City);

    let formats = formats_spec
        .split('|')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty());

    for fmt in formats {
        // 1. Si el formato incluye zona horaria / offset explícito
        if let Ok(dt_fixed) = chrono::DateTime::parse_from_str(trimmed, fmt) {
            return Ok(dt_fixed.format("%Y-%m-%dT%H:%M:%S%:z").to_string());
        }

        // 2. Si el formato parsea fecha y hora completa
        if let Ok(naive_dt) = NaiveDateTime::parse_from_str(trimmed, fmt) {
            let dt_with_tz = match tz.from_local_datetime(&naive_dt) {
                chrono::LocalResult::Single(dt) => dt,
                chrono::LocalResult::Ambiguous(dt1, _) => dt1,
                chrono::LocalResult::None => naive_dt.and_utc().with_timezone(&tz),
            };
            return Ok(dt_with_tz.format("%Y-%m-%dT%H:%M:%S%:z").to_string());
        }

        // 3. Formato fecha y hora sin segundos (ej. "%d/%m/%Y %H:%M")
        let mut parsed = chrono::format::Parsed::new();
        if chrono::format::parse(
            &mut parsed,
            trimmed,
            chrono::format::StrftimeItems::new(fmt),
        )
        .is_ok()
        {
            if let Ok(naive_date) = parsed.to_naive_date() {
                if let Ok(naive_time) = parsed.to_naive_time() {
                    let naive_dt = NaiveDateTime::new(naive_date, naive_time);
                    let dt_with_tz = match tz.from_local_datetime(&naive_dt) {
                        chrono::LocalResult::Single(dt) => dt,
                        chrono::LocalResult::Ambiguous(dt1, _) => dt1,
                        chrono::LocalResult::None => naive_dt.and_utc().with_timezone(&tz),
                    };
                    return Ok(dt_with_tz.format("%Y-%m-%dT%H:%M:%S%:z").to_string());
                } else if !fmt.contains("%H") && !fmt.contains("%M") && !fmt.contains("%S") {
                    return Ok(naive_date.format("%Y-%m-%d").to_string());
                } else {
                    let naive_dt = naive_date.and_hms_opt(0, 0, 0).unwrap();
                    let dt_with_tz = match tz.from_local_datetime(&naive_dt) {
                        chrono::LocalResult::Single(dt) => dt,
                        chrono::LocalResult::Ambiguous(dt1, _) => dt1,
                        chrono::LocalResult::None => naive_dt.and_utc().with_timezone(&tz),
                    };
                    return Ok(dt_with_tz.format("%Y-%m-%dT%H:%M:%S%:z").to_string());
                }
            }
        }

        // 4. Si el formato es solo fecha (ej. "%d/%m/%Y" o "%Y-%m-%d")
        if let Ok(naive_d) = NaiveDate::parse_from_str(trimmed, fmt) {
            if !fmt.contains("%H") && !fmt.contains("%M") && !fmt.contains("%S") {
                return Ok(naive_d.format("%Y-%m-%d").to_string());
            } else if let Some(naive_dt) = naive_d.and_hms_opt(0, 0, 0) {
                let dt_with_tz = match tz.from_local_datetime(&naive_dt) {
                    chrono::LocalResult::Single(dt) => dt,
                    chrono::LocalResult::Ambiguous(dt1, _) => dt1,
                    chrono::LocalResult::None => naive_dt.and_utc().with_timezone(&tz),
                };
                return Ok(dt_with_tz.format("%Y-%m-%dT%H:%M:%S%:z").to_string());
            }
        }
    }

    Err(MedSysError::ProcessingError(format!(
        "T2 Error de formato en parse_datetime: el valor '{trimmed}' no coincide con ningún formato de '{formats_spec}'"
    )))
}

/// Evalúa una regla individual `FieldMapping` contra una fila de datos origen y escribe el resultado en `target_json`.
///
/// Política de Fidelidad Clínica (*Fail-Closed*): el motor nunca inventa ni sustituye datos.
/// - Columna `optional: true` con `NULL`/cadena vacía/ausente: se omite la propiedad (`Ok(())`).
/// - Columna obligatoria con `NULL`/vacío: `ProcessingError` (HTTP 422).
/// - Cualquier valor presente pero corrupto (fuera de diccionario, transformación fallida o no numérico),
///   incluso en campos opcionales: `ProcessingError` (HTTP 422).
pub fn evaluate_field_mapping(
    rule: &FieldMapping,
    resource_type: &str,
    row: &Map<String, Value>,
    target_json: &mut Value,
) -> Result<()> {
    // 1. Extracción del valor de origen o constante con soporte para combine_with / combine_format
    let extracted: Option<String> = if let Some(ref col) = rule.source_column {
        let primary_val = match row.get(col) {
            Some(Value::String(s)) if !s.trim().is_empty() => Some(s.clone()),
            Some(Value::Number(n)) => Some(n.to_string()),
            Some(Value::Bool(b)) => Some(b.to_string()),
            Some(Value::Null) | None | Some(Value::String(_)) => None,
            Some(other) => Some(other.to_string()),
        };

        match primary_val {
            Some(val) => {
                if let Some(ref combine_col) = rule.combine_with {
                    let combine_val = match row.get(combine_col) {
                        Some(Value::String(s)) if !s.trim().is_empty() => {
                            Some(s.trim().to_string())
                        }
                        Some(Value::Number(n)) => Some(n.to_string()),
                        Some(Value::Bool(b)) => Some(b.to_string()),
                        _ => None,
                    };
                    if let Some(c_val) = combine_val {
                        if let Some(ref fmt) = rule.combine_format {
                            let formatted = fmt
                                .replace(&format!("{{{col}}}"), val.trim())
                                .replace(&format!("{{{combine_col}}}"), &c_val);
                            Some(formatted)
                        } else {
                            Some(format!("{} {}", val.trim(), c_val))
                        }
                    } else if let Some(ref fmt) = rule.combine_format {
                        let formatted = fmt
                            .replace(&format!("{{{col}}}"), val.trim())
                            .replace(&format!("{{{combine_col}}}"), "")
                            .trim()
                            .to_string();
                        Some(formatted)
                    } else {
                        Some(val)
                    }
                } else {
                    Some(val)
                }
            }
            None => {
                if rule.is_optional() {
                    None
                } else if let Some(ref cv) = rule.constant_value {
                    Some(cv.clone())
                } else {
                    return Err(MedSysError::ProcessingError(format!(
                        "Data corruption: Campo crítico obligatorio '{col}' ausente o nulo para el recurso '{resource_type}'"
                    )));
                }
            }
        }
    } else if let Some(ref cv) = rule.constant_value {
        Some(cv.clone())
    } else {
        return Err(MedSysError::InvalidRule(format!(
            "Regla en '{}' debe definir 'source_column' o 'constant_value'",
            rule.target_path
        )));
    };

    // Omisión controlada: opcional sin dato => no se inyecta nada
    let raw_val = match extracted {
        Some(v) => v,
        None => return Ok(()),
    };

    // 2. Ejecución de split_part si está configurado
    let after_split = if let Some(ref split) = rule.split_part {
        let parts: Vec<&str> = raw_val.split(&split.delimiter).collect();
        if let Some(part) = parts.get(split.index) {
            let trimmed_part = part.trim();
            if trimmed_part.is_empty() {
                return Err(MedSysError::ProcessingError(format!(
                    "T2 Error de formato en split_part: fragmento vacío en índice {} de '{raw_val}' con delimitador '{}'",
                    split.index, split.delimiter
                )));
            }
            trimmed_part.to_string()
        } else {
            return Err(MedSysError::ProcessingError(format!(
                "T2 Error de formato en split_part: índice {} no encontrado en '{raw_val}' con delimitador '{}'",
                split.index, split.delimiter
            )));
        }
    } else {
        raw_val
    };

    // Validación temprana para split_part sobre valueQuantity
    if rule.split_part.is_some()
        && rule.target_path.contains("valueQuantity")
        && after_split.parse::<f64>().is_err()
    {
        return Err(MedSysError::ProcessingError(format!(
            "T2 Error de formato en split_part: fragmento '{after_split}' no es numérico para '{}'",
            rule.target_path
        )));
    }

    // 3. Ejecución de unit_conversion si está configurado
    let after_conversion = if let Some(ref conv) = rule.unit_conversion {
        apply_unit_conversion(conv, &after_split)?
    } else {
        after_split
    };

    // 4. Ejecución de parse_datetime si está configurado
    let after_datetime = if let Some(ref formats) = rule.parse_datetime {
        parse_datetime_with_formats(&after_conversion, formats, Some("America/Mexico_City"))?
    } else {
        after_conversion
    };

    // 5. Aplicación de diccionario si existe (Fail-Closed ante valores fuera de catálogo)
    let processed_val = if let Some(ref dict) = rule.dictionary {
        apply_dictionary(dict, &after_datetime, &rule.target_path).map_err(|e| {
            MedSysError::ProcessingError(format!(
                "Data corruption: Fallo de casteo en diccionario para campo '{}': {}",
                rule.target_path, e
            ))
        })?
    } else {
        after_datetime
    };

    // 6. Aplicación de transformación si existe (Fail-Closed ante datos malformados)
    let final_val = if let Some(ref trans) = rule.transform {
        apply_transform(trans, &processed_val).map_err(|e| {
            MedSysError::ProcessingError(format!(
                "Data corruption: Fallo de casteo en transformación '{}' para campo '{}': {}",
                trans, rule.target_path, e
            ))
        })?
    } else {
        processed_val
    };

    // 7. Inyección en target_path
    // Determinar si el valor debe ser numérico según target_path
    let json_val = if rule.target_path.ends_with(".value")
        && rule.target_path.contains("valueQuantity")
    {
        if let Ok(int_val) = final_val.parse::<i64>() {
            json!(int_val)
        } else if let Ok(num) = final_val.parse::<f64>() {
            json!(num)
        } else if rule.split_part.is_some() {
            return Err(MedSysError::ProcessingError(format!(
                    "T2 Error de formato en split_part: No se pudo parsear como numérico el campo '{}' con valor '{final_val}'",
                    rule.target_path
                )));
        } else {
            return Err(MedSysError::ProcessingError(format!(
                "Data corruption: No se pudo parsear como numérico el campo '{}'",
                rule.target_path
            )));
        }
    } else {
        json!(final_val)
    };

    set_json_path(target_json, &rule.target_path, json_val)?;

    // 8. Inyecciones auxiliares declarativas (system, display, use, unit, code)
    if let Some(ref sys) = rule.system {
        if rule.target_path.ends_with(".value") {
            let sys_path = format!("{}.system", &rule.target_path[..rule.target_path.len() - 6]);
            let _ = set_json_path(target_json, &sys_path, json!(sys));
        } else if rule.target_path.ends_with(".code") {
            let sys_path = format!("{}.system", &rule.target_path[..rule.target_path.len() - 5]);
            let _ = set_json_path(target_json, &sys_path, json!(sys));
        }
    }

    if let Some(ref disp) = rule.display {
        if rule.target_path.ends_with(".code") {
            let disp_path = format!(
                "{}.display",
                &rule.target_path[..rule.target_path.len() - 5]
            );
            let _ = set_json_path(target_json, &disp_path, json!(disp));
        }
    }

    if let Some(u) = rule.effective_use() {
        if rule.target_path.ends_with(".value") {
            let use_path = format!("{}.use", &rule.target_path[..rule.target_path.len() - 6]);
            let _ = set_json_path(target_json, &use_path, json!(u));
        }
    }

    if let Some(ref unit) = rule.unit {
        if rule.target_path.ends_with(".value") {
            let unit_path = format!("{}.unit", &rule.target_path[..rule.target_path.len() - 6]);
            let _ = set_json_path(target_json, &unit_path, json!(unit));
        }
    }

    if let Some(ref code) = rule.code {
        if rule.target_path.ends_with(".value") {
            let code_path = format!("{}.code", &rule.target_path[..rule.target_path.len() - 6]);
            let _ = set_json_path(target_json, &code_path, json!(code));
        }
    }

    Ok(())
}

/// Ejecuta todas las directivas de un `ResourceMapping` sobre una fila de datos origen y emite un `serde_json::Value`.
pub fn evaluate_resource_mapping(
    mapping: &ResourceMapping,
    row: &Map<String, Value>,
) -> Result<Value> {
    let mut target = json!({
        "resourceType": mapping.resource_type
    });

    for rule in &mapping.mappings {
        evaluate_field_mapping(rule, &mapping.resource_type, row, &mut target)?;
    }

    Ok(target)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::mapping::SplitPartRule;
    use serde_json::json;

    #[test]
    fn test_evaluator_combine_with() {
        let rule = FieldMapping {
            source_column: Some("apellido_paterno".to_string()),
            combine_with: Some("apellido_materno".to_string()),
            combine_format: Some("{apellido_paterno} {apellido_materno}".to_string()),
            target_path: "name[0].family".to_string(),
            ..Default::default()
        };

        // Caso 1: Ambos apellidos presentes
        let mut row = Map::new();
        row.insert("apellido_paterno".to_string(), json!("HERNANDEZ"));
        row.insert("apellido_materno".to_string(), json!("GARCIA"));
        let mut target = json!({});
        evaluate_field_mapping(&rule, "Patient", &row, &mut target).unwrap();
        assert_eq!(target["name"][0]["family"], "HERNANDEZ GARCIA");

        // Caso 2: Apellido materno ausente o vacío (se conserva apellido paterno)
        let mut row_single = Map::new();
        row_single.insert("apellido_paterno".to_string(), json!("HERNANDEZ"));
        let mut target_single = json!({});
        evaluate_field_mapping(&rule, "Patient", &row_single, &mut target_single).unwrap();
        assert_eq!(target_single["name"][0]["family"], "HERNANDEZ");
    }

    #[test]
    fn test_evaluator_split_part_blood_pressure() {
        let rule_sys = FieldMapping {
            source_column: Some("presion_arterial_txt".to_string()),
            split_part: Some(SplitPartRule {
                delimiter: "/".to_string(),
                index: 0,
            }),
            target_path: "component[0].valueQuantity.value".to_string(),
            unit: Some("mmHg".to_string()),
            ..Default::default()
        };

        let rule_dia = FieldMapping {
            source_column: Some("presion_arterial_txt".to_string()),
            split_part: Some(SplitPartRule {
                delimiter: "/".to_string(),
                index: 1,
            }),
            target_path: "component[1].valueQuantity.value".to_string(),
            unit: Some("mmHg".to_string()),
            ..Default::default()
        };

        let mut row = Map::new();
        row.insert("presion_arterial_txt".to_string(), json!("130/85"));
        let mut target = json!({});
        evaluate_field_mapping(&rule_sys, "Observation", &row, &mut target).unwrap();
        evaluate_field_mapping(&rule_dia, "Observation", &row, &mut target).unwrap();
        assert_eq!(target["component"][0]["valueQuantity"]["value"], 130);
        assert_eq!(target["component"][1]["valueQuantity"]["value"], 85);

        // Caso de error: índice fuera de rango => T2 Error
        let rule_oob = FieldMapping {
            source_column: Some("presion_arterial_txt".to_string()),
            split_part: Some(SplitPartRule {
                delimiter: "/".to_string(),
                index: 2,
            }),
            target_path: "component[2].valueQuantity.value".to_string(),
            ..Default::default()
        };
        let res_oob = evaluate_field_mapping(&rule_oob, "Observation", &row, &mut target);
        match res_oob {
            Err(MedSysError::ProcessingError(msg)) => {
                assert!(msg.contains("T2 Error de formato en split_part"));
            }
            other => panic!("Esperaba ProcessingError T2 pero obtuvo: {:?}", other),
        }

        // Caso de error: fragmento no numérico en valueQuantity => T2 Error
        let mut row_corrupt = Map::new();
        row_corrupt.insert("presion_arterial_txt".to_string(), json!("130/INVALID"));
        let res_corrupt =
            evaluate_field_mapping(&rule_dia, "Observation", &row_corrupt, &mut target);
        match res_corrupt {
            Err(MedSysError::ProcessingError(msg)) => {
                assert!(msg.contains("T2 Error de formato en split_part"));
            }
            other => panic!("Esperaba ProcessingError T2 pero obtuvo: {:?}", other),
        }
    }

    #[test]
    fn test_evaluator_parse_datetime_custom_formats() {
        let rule = FieldMapping {
            source_column: Some("fech_ini_txt".to_string()),
            parse_datetime: Some("%d/%m/%Y %H:%M|%Y-%m-%d %H:%M:%S".to_string()),
            target_path: "effectiveDateTime".to_string(),
            ..Default::default()
        };

        let mut row = Map::new();
        row.insert("fech_ini_txt".to_string(), json!("18/09/2026 09:15"));
        let mut target = json!({});
        evaluate_field_mapping(&rule, "Encounter", &row, &mut target).unwrap();
        assert_eq!(target["effectiveDateTime"], "2026-09-18T09:15:00-06:00");

        // Caso de fecha pura sin hora
        let rule_date = FieldMapping {
            source_column: Some("fech_nac_txt".to_string()),
            parse_datetime: Some("%d/%m/%Y|%Y-%m-%d".to_string()),
            target_path: "birthDate".to_string(),
            ..Default::default()
        };
        let mut row_date = Map::new();
        row_date.insert("fech_nac_txt".to_string(), json!("01/01/1990"));
        let mut target_date = json!({});
        evaluate_field_mapping(&rule_date, "Patient", &row_date, &mut target_date).unwrap();
        assert_eq!(target_date["birthDate"], "1990-01-01");

        // Caso corrupto que no coincide con los formatos => T2 Error
        let mut row_corrupt = Map::new();
        row_corrupt.insert("fech_ini_txt".to_string(), json!("FECHA_CORRUPTA"));
        let res = evaluate_field_mapping(&rule, "Encounter", &row_corrupt, &mut target);
        match res {
            Err(MedSysError::ProcessingError(msg)) => {
                assert!(msg.contains("T2 Error de formato en parse_datetime"));
            }
            other => panic!("Esperaba ProcessingError T2 pero obtuvo: {:?}", other),
        }
    }

    #[test]
    fn test_evaluator_unit_conversion_lb_to_kg() {
        let rule_weight = FieldMapping {
            source_column: Some("peso_libras".to_string()),
            unit_conversion: Some("lb_to_kg".to_string()),
            target_path: "valueQuantity.value".to_string(),
            unit: Some("kg".to_string()),
            ..Default::default()
        };

        let mut row = Map::new();
        row.insert("peso_libras".to_string(), json!("154.32"));
        let mut target = json!({});
        evaluate_field_mapping(&rule_weight, "Observation", &row, &mut target).unwrap();
        // 154.32 * 0.45359237 = 70.00
        assert_eq!(target["valueQuantity"]["value"], 70.0);
        assert_eq!(target["valueQuantity"]["unit"], "kg");

        // Conversión f_to_c
        let rule_temp = FieldMapping {
            source_column: Some("temp_f".to_string()),
            unit_conversion: Some("f_to_c".to_string()),
            target_path: "valueQuantity.value".to_string(),
            unit: Some("Cel".to_string()),
            ..Default::default()
        };
        let mut row_temp = Map::new();
        row_temp.insert("temp_f".to_string(), json!("98.6"));
        let mut target_temp = json!({});
        evaluate_field_mapping(&rule_temp, "Observation", &row_temp, &mut target_temp).unwrap();
        // (98.6 - 32) * 5/9 = 37.00
        assert_eq!(target_temp["valueQuantity"]["value"], 37.0);

        // Caso no numérico => T2 Error
        let mut row_corrupt = Map::new();
        row_corrupt.insert("peso_libras".to_string(), json!("NO_NUMERO"));
        let res = evaluate_field_mapping(&rule_weight, "Observation", &row_corrupt, &mut target);
        match res {
            Err(MedSysError::ProcessingError(msg)) => {
                assert!(msg.contains("T2 Error de formato en unit_conversion"));
            }
            other => panic!("Esperaba ProcessingError T2 pero obtuvo: {:?}", other),
        }
    }
}
