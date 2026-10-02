use chrono::{NaiveDate, NaiveDateTime};
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

/// Construye la estructura JSON canónica de la extensión FHIR data-absent-reason.
pub fn fhir_data_absent_reason_json() -> Value {
    json!({
        "extension": [
            {
                "url": "http://hl7.org/fhir/StructureDefinition/data-absent-reason",
                "valueCode": "error"
            }
        ]
    })
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

/// Helper para compatibilidad hacia atrás en caso de requerir un valor de contingencia opcional.
pub fn parse_date_iso8601_with_fallback(raw: &str, fallback: Option<&str>) -> Result<String> {
    match parse_date_iso8601(raw) {
        Ok(val) => Ok(val),
        Err(err) => {
            if let Some(fb) = fallback {
                Ok(fb.to_string())
            } else {
                Err(err)
            }
        }
    }
}

/// Parsea fecha y hora hacia el formato canónico ISO 8601 (`YYYY-MM-DDTHH:MM:SS`).
pub fn parse_datetime_iso8601(raw: &str) -> Result<String> {
    let trimmed = raw.trim();

    let parsed_dt = NaiveDateTime::parse_from_str(trimmed, "%Y-%m-%d %H:%M:%S")
        .or_else(|_| NaiveDateTime::parse_from_str(trimmed, "%Y-%m-%dT%H:%M:%S"))
        .or_else(|_| NaiveDateTime::parse_from_str(trimmed, "%Y-%m-%dT%H:%M:%SZ"))
        .or_else(|_| NaiveDateTime::parse_from_str(trimmed, "%Y/%m/%d %H:%M:%S"))
        .or_else(|_| NaiveDateTime::parse_from_str(trimmed, "%d/%m/%Y %H:%M:%S"))
        .or_else(|_| {
            NaiveDate::parse_from_str(trimmed, "%Y-%m-%d").map(|d| d.and_hms_opt(0, 0, 0).unwrap())
        });

    match parsed_dt {
        Ok(dt) => Ok(dt.format("%Y-%m-%dT%H:%M:%S").to_string()),
        Err(err) => Err(MedSysError::TransformationError(format!(
            "Fallo al parsear fecha y hora '{raw}' con transformación datetime_iso8601: {err}"
        ))),
    }
}

/// Helper para compatibilidad hacia atrás en caso de requerir un valor de contingencia opcional.
pub fn parse_datetime_iso8601_with_fallback(raw: &str, fallback: Option<&str>) -> Result<String> {
    match parse_datetime_iso8601(raw) {
        Ok(val) => Ok(val),
        Err(err) => {
            if let Some(fb) = fallback {
                Ok(fb.to_string())
            } else {
                Err(err)
            }
        }
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

/// Evalúa una regla individual `FieldMapping` contra una fila de datos origen y escribe el resultado en `target_json`.
/// Si el casteo o extracción falla y `use_data_absent_reason` es true, inyecta la extensión canónica FHIR
/// `data-absent-reason` en el nodo de destino. Si falla en un campo crítico obligatorio, retorna `ProcessingError`.
pub fn evaluate_field_mapping(
    rule: &FieldMapping,
    resource_type: &str,
    row: &Map<String, Value>,
    target_json: &mut Value,
) -> Result<()> {
    // 1. Extracción del valor de origen o constante
    let extracted: Option<String> = if let Some(ref col) = rule.source_column {
        match row.get(col) {
            Some(Value::String(s)) if !s.trim().is_empty() => Some(s.clone()),
            Some(Value::Number(n)) => Some(n.to_string()),
            Some(Value::Bool(b)) => Some(b.to_string()),
            Some(Value::Null) | None | Some(Value::String(_)) => {
                // Extracción fallida (columna ausente, nula o cadena vacía)
                if rule.use_data_absent_reason() {
                    set_json_path(
                        target_json,
                        &rule.target_path,
                        fhir_data_absent_reason_json(),
                    )?;
                    return Ok(());
                } else if rule.is_optional() {
                    None
                } else if let Some(ref cv) = rule.constant_value {
                    Some(cv.clone())
                } else {
                    return Err(MedSysError::ProcessingError(format!(
                        "Data corruption: Campo crítico obligatorio '{col}' ausente o nulo para el recurso '{resource_type}'"
                    )));
                }
            }
            Some(other) => Some(other.to_string()),
        }
    } else if let Some(ref cv) = rule.constant_value {
        Some(cv.clone())
    } else {
        return Err(MedSysError::InvalidRule(format!(
            "Regla en '{}' debe definir 'source_column' o 'constant_value'",
            rule.target_path
        )));
    };

    // Si no se extrajo nada y es opcional, terminamos sin inyectar
    let raw_val = match extracted {
        Some(v) => v,
        None => return Ok(()),
    };

    // 2. Aplicación de diccionario si existe
    let processed_val = if let Some(ref dict) = rule.dictionary {
        match apply_dictionary(dict, &raw_val, &rule.target_path) {
            Ok(v) => v,
            Err(e) => {
                if rule.use_data_absent_reason() {
                    set_json_path(
                        target_json,
                        &rule.target_path,
                        fhir_data_absent_reason_json(),
                    )?;
                    return Ok(());
                } else if rule.is_optional() {
                    return Ok(());
                } else {
                    return Err(MedSysError::ProcessingError(format!(
                        "Data corruption: Fallo de casteo en diccionario para campo crítico '{}': {}",
                        rule.target_path, e
                    )));
                }
            }
        }
    } else {
        raw_val
    };

    // 3. Aplicación de transformación si existe
    let final_val = if let Some(ref trans) = rule.transform {
        match apply_transform(trans, &processed_val) {
            Ok(v) => v,
            Err(e) => {
                if rule.use_data_absent_reason() {
                    set_json_path(
                        target_json,
                        &rule.target_path,
                        fhir_data_absent_reason_json(),
                    )?;
                    return Ok(());
                } else if rule.is_optional() {
                    return Ok(());
                } else {
                    return Err(MedSysError::ProcessingError(format!(
                        "Data corruption: Fallo de casteo en transformación '{}' para campo crítico '{}': {}",
                        trans, rule.target_path, e
                    )));
                }
            }
        }
    } else {
        processed_val
    };

    // 4. Inyección en target_path
    // Determinar si el valor debe ser numérico o booleano según target_path
    let json_val =
        if rule.target_path.ends_with(".value") && rule.target_path.contains("valueQuantity") {
            if let Ok(num) = final_val.parse::<f64>() {
                json!(num)
            } else if rule.use_data_absent_reason() {
                set_json_path(
                    target_json,
                    &rule.target_path,
                    fhir_data_absent_reason_json(),
                )?;
                return Ok(());
            } else if rule.is_optional() {
                return Ok(());
            } else {
                return Err(MedSysError::ProcessingError(format!(
                    "Data corruption: No se pudo parsear como numérico el campo crítico '{}'",
                    rule.target_path
                )));
            }
        } else {
            json!(final_val)
        };

    set_json_path(target_json, &rule.target_path, json_val)?;

    // 5. Inyecciones auxiliares declarativas (system, use, unit, code)
    if let Some(ref sys) = rule.system {
        if rule.target_path.ends_with(".value") {
            let sys_path = format!("{}.system", &rule.target_path[..rule.target_path.len() - 6]);
            let _ = set_json_path(target_json, &sys_path, json!(sys));
        } else if rule.target_path.ends_with(".code") {
            let sys_path = format!("{}.system", &rule.target_path[..rule.target_path.len() - 5]);
            let _ = set_json_path(target_json, &sys_path, json!(sys));
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
