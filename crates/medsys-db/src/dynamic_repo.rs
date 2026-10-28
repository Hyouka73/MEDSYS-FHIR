//! dynamic_repo: Repositorio dinámico multi-arquetipo para MedSys-FHIR.
//!
//! Permite ejecutar consultas SQL parametrizadas generadas por `DynamicSqlBuilder`
//! sobre esquemas heterogéneos (S0 normalizado, S1 sábana plana, S2 con catálogos externos,
//! S3 esquema legado con deuda técnica) y extraer los resultados hacia `serde_json::Map<String, Value>`.
//! Soporta evaluación directa hacia recursos FHIR R4 con `evaluate_resource_mapping`,
//! conversión tipada a entidades de dominio clínico `Legacy*`, y verificación referencial
//! activa declarativa (`parent_checks` / categoría de falla `T4`).

use chrono::{NaiveDate, NaiveDateTime};
use medsys_core::engine::evaluator::evaluate_resource_mapping;
use medsys_core::error::{MedSysError, Result};
use medsys_core::model::legacy::{
    LegacyConsulta, LegacyDiagnostico, LegacyPaciente, LegacySignoVital,
};
use medsys_core::model::mapping::ResourceMapping;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use serde_json::{Map, Number, Value};
use sqlx::postgres::PgRow;
use sqlx::{Column, PgPool, Row, TypeInfo};
use std::str::FromStr;

use crate::error::map_sqlx_error;
use crate::query_builder::DynamicSqlBuilder;

/// Repositorio dinámico FHIR que opera sobre PostgreSQL mediante especificaciones declarativas YAML.
#[derive(Debug, Clone)]
pub struct DynamicFhirRepository {
    pool: PgPool,
}

impl DynamicFhirRepository {
    /// Crea una nueva instancia del repositorio dinámico con el pool de conexiones provisto.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Retorna una referencia al pool de conexiones subyacente.
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Inicia una transacción asíncrona de solo lectura con nivel de aislamiento `REPEATABLE READ`.
    pub async fn begin_readonly_tx(&self) -> Result<sqlx::Transaction<'_, sqlx::Postgres>> {
        let mut tx = self.pool.begin().await.map_err(map_sqlx_error)?;
        sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
            .execute(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        Ok(tx)
    }

    /// Comprueba los padres dentro de la misma instantánea que la lectura.
    async fn verify_parent_checks_tx(
        mapping: &ResourceMapping,
        row: &Map<String, Value>,
        tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
        cache: &mut std::collections::BTreeMap<(String, String, i64), Value>,
    ) -> Result<()> {
        for rule in &mapping.parent_checks {
            let fk_id = row
                .get(&rule.foreign_key_column)
                .and_then(|v| match v {
                    Value::Number(n) => n.as_i64(),
                    Value::String(s) => s.parse::<i64>().ok(),
                    _ => None,
                })
                .ok_or_else(|| {
                    MedSysError::ProcessingError(format!(
                        "T4 Clave padre ausente o inválida: {}",
                        rule.foreign_key_column
                    ))
                })?;
            crate::query_builder::validate_table_identifier(&rule.parent_table)?;
            crate::query_builder::validate_column_identifier(&rule.parent_pk)?;
            let key = (rule.parent_table.clone(), rule.parent_pk.clone(), fk_id);
            if let std::collections::btree_map::Entry::Vacant(entry) = cache.entry(key.clone()) {
                let query = format!(
                    "SELECT row_to_json(p)::text FROM {} p WHERE {} = $1 LIMIT 1",
                    rule.parent_table, rule.parent_pk
                );
                let parent = sqlx::query_scalar::<_, String>(&query)
                    .bind(fk_id)
                    .fetch_optional(&mut **tx)
                    .await
                    .map_err(map_sqlx_error)?
                    .ok_or_else(|| {
                        MedSysError::ProcessingError(format!(
                            "T4 Registro padre inexistente en {} con {}={fk_id}",
                            rule.parent_table, rule.parent_pk
                        ))
                    })?;
                let parent: Value = serde_json::from_str(&parent)
                    .map_err(|e| MedSysError::DatabaseError(e.to_string()))?;
                entry.insert(parent);
            }
            // La existencia de ambos padres no basta: el encuentro debe pertenecer
            // al mismo paciente declarado por el hijo.
            if matches!(rule.parent_pk.as_str(), "id_consulta" | "con_id") {
                let child_patient = extract_i32(row, &["id_paciente", "pac_id"]);
                let parent_patient = cache
                    .get(&key)
                    .and_then(Value::as_object)
                    .and_then(|p| extract_i32(p, &["id_paciente", "pac_id"]));
                if child_patient.is_none() || child_patient != parent_patient {
                    return Err(MedSysError::ProcessingError(
                        "T4 El encuentro no pertenece al paciente del recurso".into(),
                    ));
                }
            }
        }
        Ok(())
    }

    /// API de comprobación explícita; las lecturas usan la variante con su propia transacción.
    pub async fn verify_parent_checks(
        &self,
        mapping: &ResourceMapping,
        row: &Map<String, Value>,
    ) -> Result<()> {
        let mut tx = self.begin_readonly_tx().await?;
        Self::verify_parent_checks_tx(mapping, row, &mut tx, &mut Default::default()).await?;
        tx.commit().await.map_err(map_sqlx_error)
    }

    /// Lee y comprueba padres en una única transacción REPEATABLE READ READ ONLY.
    pub async fn find_by_id_raw(
        &self,
        mapping: &ResourceMapping,
        id: i32,
    ) -> Result<Option<Map<String, Value>>> {
        let sql = DynamicSqlBuilder::select_by_id(mapping);
        let mut tx = self.begin_readonly_tx().await?;
        let row = sqlx::query(&sql)
            .bind(id)
            .fetch_optional(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        let result = match row {
            Some(row) => {
                let map = pg_row_to_json_map(&row)?;
                Self::verify_parent_checks_tx(mapping, &map, &mut tx, &mut Default::default())
                    .await?;
                Some(map)
            }
            None => None,
        };
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(result)
    }

    pub async fn find_by_id(
        &self,
        mapping: &ResourceMapping,
        id: i32,
    ) -> Result<Map<String, Value>> {
        self.find_by_id_raw(mapping, id).await?.ok_or_else(|| {
            MedSysError::NotFound(format!(
                "Recurso '{}' con identificador {id} no encontrado en la base de datos",
                mapping.resource_type
            ))
        })
    }

    pub async fn find_by_id_fhir(&self, mapping: &ResourceMapping, id: i32) -> Result<Value> {
        let row = self.find_by_id(mapping, id).await?;
        evaluate_resource_mapping(mapping, &row)
    }

    pub async fn find_all_raw(
        &self,
        mapping: &ResourceMapping,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Map<String, Value>>> {
        let sql = DynamicSqlBuilder::select_all_paged(mapping);
        let mut tx = self.begin_readonly_tx().await?;
        let rows = sqlx::query(&sql)
            .bind(limit)
            .bind(offset)
            .fetch_all(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        let mut results = Vec::with_capacity(rows.len());
        let mut cache = Default::default();
        for row in rows {
            let map = pg_row_to_json_map(&row)?;
            Self::verify_parent_checks_tx(mapping, &map, &mut tx, &mut cache).await?;
            results.push(map);
        }
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(results)
    }

    pub async fn find_all_fhir(
        &self,
        mapping: &ResourceMapping,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Value>> {
        self.find_all_raw(mapping, limit, offset)
            .await?
            .iter()
            .map(|r| evaluate_resource_mapping(mapping, r))
            .collect()
    }

    pub async fn find_filtered_raw(
        &self,
        mapping: &ResourceMapping,
        filter_col: &str,
        filter_val: i32,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Map<String, Value>>> {
        let target = match filter_col {
            "id_paciente" => Some("subject.reference"),
            "id_consulta" => Some("encounter.reference"),
            _ => None,
        };
        let column = target
            .and_then(|target| mapping.mappings.iter().find(|m| m.target_path == target))
            .and_then(|m| m.source_column.as_deref())
            .unwrap_or(filter_col);
        crate::query_builder::validate_column_identifier(column)?;
        let sql = DynamicSqlBuilder::select_by_foreign_key_paged(mapping, column);
        let mut tx = self.begin_readonly_tx().await?;
        let rows = sqlx::query(&sql)
            .bind(filter_val)
            .bind(limit)
            .bind(offset)
            .fetch_all(&mut *tx)
            .await
            .map_err(map_sqlx_error)?;
        let mut results = Vec::with_capacity(rows.len());
        let mut cache = Default::default();
        for row in rows {
            let map = pg_row_to_json_map(&row)?;
            Self::verify_parent_checks_tx(mapping, &map, &mut tx, &mut cache).await?;
            results.push(map);
        }
        tx.commit().await.map_err(map_sqlx_error)?;
        Ok(results)
    }

    pub async fn find_filtered_fhir(
        &self,
        mapping: &ResourceMapping,
        filter_col: &str,
        filter_val: i32,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<Value>> {
        self.find_filtered_raw(mapping, filter_col, filter_val, limit, offset)
            .await?
            .iter()
            .map(|r| evaluate_resource_mapping(mapping, r))
            .collect()
    }
}

/// Extrae de forma dinámica todas las columnas de una `PgRow` hacia un `Map<String, Value>`.
pub fn pg_row_to_json_map(row: &PgRow) -> Result<Map<String, Value>> {
    let mut map = Map::new();
    for (i, col) in row.columns().iter().enumerate() {
        let name = col.name().to_string();
        let type_name = col.type_info().name();

        let val: Value = match type_name {
            "INT2" | "SMALLINT" | "SMALLSERIAL" => match row.try_get::<Option<i16>, _>(i) {
                Ok(Some(v)) => Value::Number((v as i64).into()),
                Ok(None) => Value::Null,
                Err(e) => {
                    return Err(MedSysError::DatabaseError(format!(
                        "Error decodificando columna INT2 '{name}': {e}"
                    )))
                }
            },
            "INT4" | "INTEGER" | "SERIAL" => match row.try_get::<Option<i32>, _>(i) {
                Ok(Some(v)) => Value::Number((v as i64).into()),
                Ok(None) => Value::Null,
                Err(e) => {
                    return Err(MedSysError::DatabaseError(format!(
                        "Error decodificando columna INT4 '{name}': {e}"
                    )))
                }
            },
            "INT8" | "BIGINT" | "BIGSERIAL" => match row.try_get::<Option<i64>, _>(i) {
                Ok(Some(v)) => Value::Number(v.into()),
                Ok(None) => Value::Null,
                Err(e) => {
                    return Err(MedSysError::DatabaseError(format!(
                        "Error decodificando columna INT8 '{name}': {e}"
                    )))
                }
            },
            "FLOAT4" | "REAL" => match row.try_get::<Option<f32>, _>(i) {
                Ok(Some(v)) => Number::from_f64(v as f64)
                    .map(Value::Number)
                    .unwrap_or(Value::Null),
                Ok(None) => Value::Null,
                Err(e) => {
                    return Err(MedSysError::DatabaseError(format!(
                        "Error decodificando columna FLOAT4 '{name}': {e}"
                    )))
                }
            },
            "FLOAT8" | "DOUBLE PRECISION" => match row.try_get::<Option<f64>, _>(i) {
                Ok(Some(v)) => Number::from_f64(v)
                    .map(Value::Number)
                    .unwrap_or(Value::Null),
                Ok(None) => Value::Null,
                Err(e) => {
                    return Err(MedSysError::DatabaseError(format!(
                        "Error decodificando columna FLOAT8 '{name}': {e}"
                    )))
                }
            },
            "NUMERIC" | "DECIMAL" => match row.try_get::<Option<Decimal>, _>(i) {
                Ok(Some(v)) => Value::String(v.to_string()),
                Ok(None) => Value::Null,
                Err(e) => {
                    return Err(MedSysError::DatabaseError(format!(
                        "Error decodificando columna NUMERIC '{name}': {e}"
                    )))
                }
            },
            "BOOL" | "BOOLEAN" => match row.try_get::<Option<bool>, _>(i) {
                Ok(Some(v)) => Value::Bool(v),
                Ok(None) => Value::Null,
                Err(e) => {
                    return Err(MedSysError::DatabaseError(format!(
                        "Error decodificando columna BOOL '{name}': {e}"
                    )))
                }
            },
            "DATE" => match row.try_get::<Option<NaiveDate>, _>(i) {
                Ok(Some(d)) => Value::String(d.to_string()),
                Ok(None) => Value::Null,
                Err(e) => {
                    return Err(MedSysError::DatabaseError(format!(
                        "Error decodificando columna DATE '{name}': {e}"
                    )))
                }
            },
            "TIMESTAMP" => match row.try_get::<Option<NaiveDateTime>, _>(i) {
                Ok(Some(dt)) => Value::String(dt.format("%Y-%m-%d %H:%M:%S%.f").to_string()),
                Ok(None) => Value::Null,
                Err(e) => {
                    return Err(MedSysError::DatabaseError(format!(
                        "Error decodificando columna TIMESTAMP '{name}': {e}"
                    )))
                }
            },
            "TIMESTAMPTZ" => match row.try_get::<Option<chrono::DateTime<chrono::Utc>>, _>(i) {
                Ok(Some(dt)) => Value::String(dt.to_rfc3339()),
                Ok(None) => Value::Null,
                Err(e) => {
                    return Err(MedSysError::DatabaseError(format!(
                        "Error decodificando columna TIMESTAMPTZ '{name}': {e}"
                    )))
                }
            },
            _ => match row.try_get::<Option<String>, _>(i) {
                Ok(Some(s)) => Value::String(s),
                Ok(None) => Value::Null,
                Err(e) => return Err(MedSysError::DatabaseError(format!(
                    "Tipo SQL no soportado o error decodificando columna '{name}' ({type_name}): {e}"
                ))),
            },
        };
        map.insert(name, val);
    }
    for (name, source) in &map {
        if let Some(resolved) = name.strip_suffix("__source") {
            if !source.is_null() && map.get(resolved).is_none_or(Value::is_null) {
                return Err(MedSysError::ProcessingError(format!(
                    "T3 Catálogo sin correspondencia para {resolved}"
                )));
            }
        }
    }
    Ok(map)
}

// -----------------------------------------------------------------------------
// Utilidades de Conversión Flexible Multi-Arquetipo (S0, S1, S2, S3)
// -----------------------------------------------------------------------------

fn extract_string(row: &Map<String, Value>, candidates: &[&str]) -> Option<String> {
    for &key in candidates {
        if let Some(val) = row.get(key) {
            match val {
                Value::String(s) if !s.trim().is_empty() => return Some(s.clone()),
                Value::Number(n) => return Some(n.to_string()),
                _ => {}
            }
        }
    }
    None
}

fn extract_opt_string(row: &Map<String, Value>, candidates: &[&str]) -> Option<String> {
    for &key in candidates {
        if let Some(val) = row.get(key) {
            match val {
                Value::String(s) => {
                    let trimmed = s.trim();
                    if trimmed.is_empty() {
                        return None;
                    }
                    return Some(trimmed.to_string());
                }
                Value::Null => return None,
                Value::Number(n) => return Some(n.to_string()),
                _ => {}
            }
        }
    }
    None
}

fn extract_i32(row: &Map<String, Value>, candidates: &[&str]) -> Option<i32> {
    for &key in candidates {
        if let Some(val) = row.get(key) {
            match val {
                Value::Number(n) => {
                    if let Some(v) = n.as_i64() {
                        return i32::try_from(v).ok();
                    }
                }
                Value::String(s) => {
                    if let Ok(v) = s.trim().parse::<i32>() {
                        return Some(v);
                    }
                }
                _ => {}
            }
        }
    }
    None
}

fn extract_opt_i32(row: &Map<String, Value>, candidates: &[&str]) -> Result<Option<i32>> {
    for &key in candidates {
        if let Some(val) = row.get(key) {
            if val.is_null() {
                return Ok(None);
            }
            return extract_i32(row, &[key]).map(Some).ok_or_else(|| {
                MedSysError::ProcessingError(format!("T2 Entero opcional inválido: {key}"))
            });
        }
    }
    Ok(None)
}

fn parse_flexible_date(s: &str) -> Option<NaiveDate> {
    let trimmed = s.trim();
    if let Ok(d) = NaiveDate::parse_from_str(trimmed, "%Y-%m-%d") {
        return Some(d);
    }
    if let Ok(d) = NaiveDate::parse_from_str(trimmed, "%d/%m/%Y") {
        return Some(d);
    }
    if let Ok(d) = NaiveDate::parse_from_str(trimmed, "%Y/%m/%d") {
        return Some(d);
    }
    if let Ok(d) = NaiveDate::parse_from_str(trimmed, "%d-%m-%Y") {
        return Some(d);
    }
    None
}

fn extract_naive_date(row: &Map<String, Value>, candidates: &[&str]) -> Option<NaiveDate> {
    for &key in candidates {
        if let Some(Value::String(s)) = row.get(key) {
            if let Some(d) = parse_flexible_date(s) {
                return Some(d);
            }
        }
    }
    None
}

fn parse_flexible_datetime(s: &str) -> Option<NaiveDateTime> {
    let trimmed = s.trim();
    if let Ok(dt) = NaiveDateTime::parse_from_str(trimmed, "%Y-%m-%d %H:%M:%S%.f") {
        return Some(dt);
    }
    if let Ok(dt) = NaiveDateTime::parse_from_str(trimmed, "%Y-%m-%d %H:%M") {
        return Some(dt);
    }
    if let Ok(dt) = NaiveDateTime::parse_from_str(trimmed, "%d/%m/%Y %H:%M:%S%.f") {
        return Some(dt);
    }
    if let Ok(dt) = NaiveDateTime::parse_from_str(trimmed, "%d/%m/%Y %H:%M") {
        return Some(dt);
    }
    if let Ok(dt) = NaiveDateTime::parse_from_str(trimmed, "%Y-%m-%dT%H:%M:%S%.f") {
        return Some(dt);
    }
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(trimmed) {
        return Some(
            dt.with_timezone(&chrono_tz::America::Mexico_City)
                .naive_local(),
        );
    }
    None
}

/// Canonicaliza instantes a reloj UTC antes del adaptador NaiveDateTime. Los
/// timestamps sin offset se interpretan en la zona declarada. Un offset explícito
/// conserva su instante aun durante una transición DST; no se elimina la zona.
pub fn normalize_row_timezone(
    mut row: Map<String, Value>,
    timezone: &str,
) -> Result<Map<String, Value>> {
    use chrono::TimeZone;
    let tz: chrono_tz::Tz = timezone
        .parse()
        .map_err(|_| MedSysError::InvalidRule(format!("Zona horaria inválida: {timezone}")))?;
    for key in [
        "fecha_hora_inicio",
        "fecha_hora_fin",
        "fecha_registro",
        "fech_ini_txt",
        "fech_fin_txt",
        "fech_reg_txt",
        "fech_sig_txt",
    ] {
        if let Some(Value::String(s)) = row.get(key).cloned() {
            let utc = if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(s.trim()) {
                dt.naive_utc()
            } else if let Some(date) = parse_flexible_date(&s) {
                // La hora cero es sólo un portador interno requerido por Legacy.
                // El contexto obliga al servidor a emitir la fecha sin esa hora.
                row.insert(
                    format!("__date_precision_{key}"),
                    Value::String(date.format("%Y-%m-%d").to_string()),
                );
                date.and_hms_opt(0, 0, 0).expect("valid midnight carrier")
            } else {
                let local = parse_flexible_datetime(&s).ok_or_else(|| {
                    MedSysError::ProcessingError(format!("T2 Fecha/hora inválida: {key}"))
                })?;
                tz.from_local_datetime(&local)
                    .single()
                    .ok_or_else(|| {
                        MedSysError::ProcessingError(format!(
                            "T2 Fecha/hora ambigua o inexistente en {timezone}: {s}"
                        ))
                    })?
                    .naive_utc()
            };
            row.insert(
                key.into(),
                Value::String(utc.format("%Y-%m-%d %H:%M:%S%.f").to_string()),
            );
        }
    }
    Ok(row)
}

fn extract_naive_datetime(row: &Map<String, Value>, candidates: &[&str]) -> Option<NaiveDateTime> {
    for &key in candidates {
        if let Some(Value::String(s)) = row.get(key) {
            if let Some(dt) = parse_flexible_datetime(s) {
                return Some(dt);
            }
        }
    }
    None
}

fn extract_opt_naive_datetime(
    row: &Map<String, Value>,
    candidates: &[&str],
) -> Option<NaiveDateTime> {
    for &key in candidates {
        if let Some(val) = row.get(key) {
            match val {
                Value::Null => return None,
                Value::String(s) => {
                    let trimmed = s.trim();
                    if trimmed.is_empty() {
                        return None;
                    }
                    if let Some(dt) = parse_flexible_datetime(trimmed) {
                        return Some(dt);
                    }
                }
                _ => {}
            }
        }
    }
    None
}

fn extract_decimal(row: &Map<String, Value>, candidates: &[&str]) -> Option<Decimal> {
    for &key in candidates {
        if let Some(val) = row.get(key) {
            match val {
                Value::Number(n) => {
                    if let Some(f) = n.as_f64() {
                        if let Some(d) = Decimal::from_f64(f) {
                            return Some(d);
                        }
                    }
                }
                Value::String(s) => {
                    if let Ok(d) = Decimal::from_str(s.trim()) {
                        return Some(d);
                    }
                }
                _ => {}
            }
        }
    }
    None
}

fn extract_opt_decimal(row: &Map<String, Value>, candidates: &[&str]) -> Option<Decimal> {
    for &key in candidates {
        if let Some(val) = row.get(key) {
            match val {
                Value::Null => return None,
                Value::Number(n) => {
                    if let Some(f) = n.as_f64() {
                        if let Some(d) = Decimal::from_f64(f) {
                            return Some(d);
                        }
                    }
                }
                Value::String(s) => {
                    let trimmed = s.trim();
                    if trimmed.is_empty() {
                        return None;
                    }
                    if let Ok(d) = Decimal::from_str(trimmed) {
                        return Some(d);
                    }
                }
                _ => {}
            }
        }
    }
    None
}

/// Convierte una fila de datos de cualquier arquetipo relacional hacia `LegacyPaciente`.
pub fn row_to_legacy_paciente(row: &Map<String, Value>) -> Result<LegacyPaciente> {
    let id_paciente = extract_i32(row, &["id_paciente", "pac_id", "id"]).ok_or_else(|| {
        MedSysError::MissingField {
            field: "id_paciente".into(),
            resource: "LegacyPaciente".into(),
        }
    })?;

    let curp =
        extract_string(row, &["curp", "pac_curp"]).ok_or_else(|| MedSysError::MissingField {
            field: "curp".into(),
            resource: "LegacyPaciente".into(),
        })?;

    let primer_nombre =
        extract_string(row, &["primer_nombre", "pac_nom1", "nombre"]).ok_or_else(|| {
            MedSysError::MissingField {
                field: "primer_nombre".into(),
                resource: "LegacyPaciente".into(),
            }
        })?;

    let segundo_nombre = extract_opt_string(row, &["segundo_nombre", "pac_nom2"]);
    let apellido_paterno =
        extract_string(row, &["apellido_paterno", "pac_ap_pat"]).ok_or_else(|| {
            MedSysError::MissingField {
                field: "apellido_paterno".into(),
                resource: "LegacyPaciente".into(),
            }
        })?;

    let apellido_materno = extract_opt_string(row, &["apellido_materno", "pac_ap_mat"]);
    let fecha_nacimiento =
        extract_naive_date(row, &["fecha_nacimiento", "fech_nac_txt", "fech_nac"]).ok_or_else(
            || MedSysError::MissingField {
                field: "fecha_nacimiento".into(),
                resource: "LegacyPaciente".into(),
            },
        )?;

    let sexo_biologico = extract_opt_string(
        row,
        &["sexo_biologico", "des_sexo", "sexo", "sexo_cod", "id_sexo"],
    );
    let telefono_contacto =
        extract_opt_string(row, &["telefono_contacto", "tel_contacto", "telefono"]);
    let fecha_registro =
        extract_opt_naive_datetime(row, &["fecha_registro", "fech_reg_txt", "fech_reg"]);

    Ok(LegacyPaciente {
        id_paciente,
        curp,
        primer_nombre,
        segundo_nombre,
        apellido_paterno,
        apellido_materno,
        fecha_nacimiento,
        sexo_biologico,
        telefono_contacto,
        fecha_registro,
    })
}

/// Convierte una fila de datos de cualquier arquetipo relacional hacia `LegacyConsulta`.
pub fn row_to_legacy_consulta(row: &Map<String, Value>) -> Result<LegacyConsulta> {
    let id_consulta = extract_i32(row, &["id_consulta", "con_id", "id"]).ok_or_else(|| {
        MedSysError::MissingField {
            field: "id_consulta".into(),
            resource: "LegacyConsulta".into(),
        }
    })?;

    let id_paciente =
        extract_i32(row, &["id_paciente", "pac_id"]).ok_or_else(|| MedSysError::MissingField {
            field: "id_paciente".into(),
            resource: "LegacyConsulta".into(),
        })?;

    let cedula_medico_tratante = extract_string(
        row,
        &["cedula_medico_tratante", "ced_medico", "cedula_profesional"],
    )
    .ok_or_else(|| MedSysError::MissingField {
        field: "cedula_medico_tratante".into(),
        resource: "LegacyConsulta".into(),
    })?;

    let nombre_medico = extract_string(row, &["nombre_medico", "nom_medico", "medico"])
        .ok_or_else(|| MedSysError::MissingField {
            field: "nombre_medico".into(),
            resource: "LegacyConsulta".into(),
        })?;

    let estado_consulta = extract_string(
        row,
        &[
            "estado_consulta",
            "des_estado_consulta",
            "estado",
            "est_con",
            "est_consulta",
            "id_estado_consulta",
        ],
    )
    .ok_or_else(|| MedSysError::MissingField {
        field: "estado_consulta".into(),
        resource: "LegacyConsulta".into(),
    })?;

    let motivo_consulta = extract_string(row, &["motivo_consulta", "motivo", "mot_con"])
        .ok_or_else(|| MedSysError::MissingField {
            field: "motivo_consulta".into(),
            resource: "LegacyConsulta".into(),
        })?;

    let fecha_hora_inicio = extract_naive_datetime(
        row,
        &[
            "fecha_hora_inicio",
            "fech_ini_txt",
            "fecha_inicio",
            "inicio",
        ],
    )
    .ok_or_else(|| MedSysError::MissingField {
        field: "fecha_hora_inicio".into(),
        resource: "LegacyConsulta".into(),
    })?;

    let fecha_hora_fin =
        extract_naive_datetime(row, &["fecha_hora_fin", "fech_fin_txt", "fecha_fin", "fin"])
            .ok_or_else(|| MedSysError::MissingField {
                field: "fecha_hora_fin".into(),
                resource: "LegacyConsulta".into(),
            })?;

    let unidad_medica = extract_opt_string(row, &["unidad_medica", "unidad", "clinica"]);

    Ok(LegacyConsulta {
        id_consulta,
        id_paciente,
        cedula_medico_tratante,
        nombre_medico,
        estado_consulta,
        motivo_consulta,
        fecha_hora_inicio,
        fecha_hora_fin,
        unidad_medica,
    })
}

/// Convierte una fila de datos de cualquier arquetipo relacional hacia `LegacySignoVital`.
/// Soporta separación automática de presión arterial `"120/80"` (S3) y conversiones de unidades.
pub fn row_to_legacy_signo_vital(row: &Map<String, Value>) -> Result<LegacySignoVital> {
    let id_signo = extract_i32(row, &["id_signo", "sig_id", "id"]).ok_or_else(|| {
        MedSysError::MissingField {
            field: "id_signo".into(),
            resource: "LegacySignoVital".into(),
        }
    })?;

    let id_consulta =
        extract_i32(row, &["id_consulta", "con_id"]).ok_or_else(|| MedSysError::MissingField {
            field: "id_consulta".into(),
            resource: "LegacySignoVital".into(),
        })?;

    let id_paciente =
        extract_i32(row, &["id_paciente", "pac_id"]).ok_or_else(|| MedSysError::MissingField {
            field: "id_paciente".into(),
            resource: "LegacySignoVital".into(),
        })?;

    // Presión arterial: soportar campos directos o fragmentación de "sistolica/diastolica" (S3)
    let (presion_sistolica, presion_diastolica) = if let (Some(sys), Some(dia)) = (
        extract_i32(row, &["presion_sistolica", "pas"]),
        extract_i32(row, &["presion_diastolica", "pad"]),
    ) {
        (sys, dia)
    } else if let Some(bp_txt) = extract_string(row, &["presion_arterial_txt", "ta_txt", "presion"])
    {
        let parts: Vec<&str> = bp_txt.split('/').collect();
        if parts.len() != 2 {
            return Err(MedSysError::ProcessingError(format!(
                "T2 Formato de presión arterial inválido: '{bp_txt}'"
            )));
        }
        let sys = parts[0].trim().parse::<i32>().map_err(|_| {
            MedSysError::ProcessingError(format!(
                "T2 Presión sistólica no numérica: '{}'",
                parts[0]
            ))
        })?;
        let dia = parts[1].trim().parse::<i32>().map_err(|_| {
            MedSysError::ProcessingError(format!(
                "T2 Presión diastólica no numérica: '{}'",
                parts[1]
            ))
        })?;
        (sys, dia)
    } else {
        return Err(MedSysError::MissingField {
            field: "presion_sistolica".into(),
            resource: "LegacySignoVital".into(),
        });
    };

    let frecuencia_cardiaca = extract_i32(row, &["frecuencia_cardiaca", "fc", "pulso"])
        .ok_or_else(|| MedSysError::MissingField {
            field: "frecuencia_cardiaca".into(),
            resource: "LegacySignoVital".into(),
        })?;

    let frecuencia_respiratoria = extract_opt_i32(row, &["frecuencia_respiratoria", "fr"])?;

    // Temperatura: directa o conversión de Fahrenheit a Celsius
    let temperatura_celsius = if let Some(temp) = extract_decimal(
        row,
        &["temperatura_celsius", "temp_celsius", "temp_c", "temp"],
    ) {
        temp
    } else if let Some(temp_f) = extract_decimal(row, &["temperatura_f", "temp_farenheit"]) {
        ((temp_f - Decimal::from(32)) * Decimal::from(5) / Decimal::from(9)).round_dp(1)
    } else {
        return Err(MedSysError::MissingField {
            field: "temperatura_celsius".into(),
            resource: "LegacySignoVital".into(),
        });
    };

    // Peso: directo o conversión de libras a kilogramos (factor 0.45359237)
    let peso_kg = if let Some(peso) = extract_opt_decimal(row, &["peso_kg", "peso"]) {
        Some(peso)
    } else {
        extract_opt_decimal(row, &["peso_libras", "peso_lb"])
            .map(|lb| (lb * Decimal::from_str("0.45359237").expect("fixed decimal")).round_dp(2))
    };

    let talla_cm = extract_opt_i32(row, &["talla_cm", "talla", "estatura"])?;

    let fecha_registro = extract_naive_datetime(
        row,
        &["fecha_registro", "fech_reg_txt", "fech_reg", "fecha"],
    )
    .ok_or_else(|| MedSysError::MissingField {
        field: "fecha_registro".into(),
        resource: "LegacySignoVital".into(),
    })?;

    Ok(LegacySignoVital {
        id_signo,
        id_consulta,
        id_paciente,
        presion_sistolica,
        presion_diastolica,
        frecuencia_cardiaca,
        frecuencia_respiratoria,
        temperatura_celsius,
        peso_kg,
        talla_cm,
        fecha_registro,
    })
}

/// Convierte una fila de datos de cualquier arquetipo relacional hacia `LegacyDiagnostico`.
pub fn row_to_legacy_diagnostico(row: &Map<String, Value>) -> Result<LegacyDiagnostico> {
    let id_diagnostico =
        extract_i32(row, &["id_diagnostico", "diag_id", "id"]).ok_or_else(|| {
            MedSysError::MissingField {
                field: "id_diagnostico".into(),
                resource: "LegacyDiagnostico".into(),
            }
        })?;

    let id_consulta =
        extract_i32(row, &["id_consulta", "con_id"]).ok_or_else(|| MedSysError::MissingField {
            field: "id_consulta".into(),
            resource: "LegacyDiagnostico".into(),
        })?;

    let id_paciente =
        extract_i32(row, &["id_paciente", "pac_id"]).ok_or_else(|| MedSysError::MissingField {
            field: "id_paciente".into(),
            resource: "LegacyDiagnostico".into(),
        })?;

    let codigo_cie10 = extract_string(
        row,
        &["codigo_cie10", "cie10", "diag_cod", "codigo", "id_cie10"],
    )
    .ok_or_else(|| MedSysError::MissingField {
        field: "codigo_cie10".into(),
        resource: "LegacyDiagnostico".into(),
    })?;

    let descripcion_diagnostico = extract_string(
        row,
        &[
            "descripcion_diagnostico",
            "des_diagnostico",
            "descripcion",
            "diag_des",
            "desc_diag",
        ],
    )
    .ok_or_else(|| MedSysError::MissingField {
        field: "descripcion_diagnostico".into(),
        resource: "LegacyDiagnostico".into(),
    })?;

    let tipo_diagnostico = extract_opt_string(
        row,
        &[
            "tipo_diagnostico",
            "des_tipo_diagnostico",
            "tipo_diag",
            "id_tipo_diag",
            "tipo",
        ],
    );

    let fecha_diagnostico = extract_naive_date(
        row,
        &["fecha_diagnostico", "fech_diag_txt", "fech_diag", "fecha"],
    )
    .ok_or_else(|| MedSysError::MissingField {
        field: "fecha_diagnostico".into(),
        resource: "LegacyDiagnostico".into(),
    })?;

    Ok(LegacyDiagnostico {
        id_diagnostico,
        id_consulta,
        id_paciente,
        codigo_cie10,
        descripcion_diagnostico,
        tipo_diagnostico,
        fecha_diagnostico,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_row_to_legacy_paciente_s0() {
        let mut row = Map::new();
        row.insert("id_paciente".into(), json!(1));
        row.insert("curp".into(), json!("ROMA900101HCSMNL09"));
        row.insert("primer_nombre".into(), json!("Alberto"));
        row.insert("segundo_nombre".into(), json!("Manuel"));
        row.insert("apellido_paterno".into(), json!("Ramos"));
        row.insert("apellido_materno".into(), json!("Gómez"));
        row.insert("fecha_nacimiento".into(), json!("1990-01-01"));
        row.insert("sexo_biologico".into(), json!("M"));
        row.insert("telefono_contacto".into(), json!("9611234567"));
        row.insert("fecha_registro".into(), json!("2026-01-01 10:00:00"));

        let pac = row_to_legacy_paciente(&row).expect("Debe mapear paciente S0");
        assert_eq!(pac.id_paciente, 1);
        assert_eq!(pac.curp, "ROMA900101HCSMNL09");
        assert_eq!(pac.primer_nombre, "Alberto");
        assert_eq!(
            pac.fecha_nacimiento,
            NaiveDate::from_ymd_opt(1990, 1, 1).unwrap()
        );
        assert_eq!(pac.sexo_biologico.as_deref(), Some("M"));
    }

    #[test]
    fn test_row_to_legacy_paciente_s3_legacy_names_and_date() {
        let mut row = Map::new();
        row.insert("pac_id".into(), json!(2));
        row.insert("pac_curp".into(), json!("LOPE950512MCSPRN07"));
        row.insert("pac_nom1".into(), json!("Mariana"));
        row.insert("pac_nom2".into(), Value::Null);
        row.insert("pac_ap_pat".into(), json!("López"));
        row.insert("pac_ap_mat".into(), json!("Hernández"));
        row.insert("fech_nac_txt".into(), json!("12/05/1995"));
        row.insert("des_sexo".into(), json!("Femenino"));
        row.insert("tel_contacto".into(), Value::Null);

        let pac = row_to_legacy_paciente(&row).expect("Debe mapear paciente S3");
        assert_eq!(pac.id_paciente, 2);
        assert_eq!(pac.curp, "LOPE950512MCSPRN07");
        assert_eq!(pac.primer_nombre, "Mariana");
        assert_eq!(pac.segundo_nombre, None);
        assert_eq!(
            pac.fecha_nacimiento,
            NaiveDate::from_ymd_opt(1995, 5, 12).unwrap()
        );
        assert_eq!(pac.sexo_biologico.as_deref(), Some("Femenino"));
    }

    #[test]
    fn test_row_to_legacy_signo_vital_s3_split_bp_and_units() {
        let mut row = Map::new();
        row.insert("sig_id".into(), json!(10));
        row.insert("con_id".into(), json!(5));
        row.insert("pac_id".into(), json!(1));
        row.insert("presion_arterial_txt".into(), json!("130/85"));
        row.insert("fc".into(), json!(72));
        row.insert("fr".into(), json!(18));
        row.insert("temperatura_f".into(), json!(98.6));
        row.insert("peso_libras".into(), json!(154.32));
        row.insert("talla".into(), json!(170));
        row.insert("fech_reg_txt".into(), json!("18/09/2026 09:30"));

        let sv = row_to_legacy_signo_vital(&row).expect("Debe mapear signo vital S3");
        assert_eq!(sv.presion_sistolica, 130);
        assert_eq!(sv.presion_diastolica, 85);
        assert_eq!(sv.frecuencia_cardiaca, 72);
        assert_eq!(sv.frecuencia_respiratoria, Some(18));
        assert_eq!(sv.temperatura_celsius, Decimal::from_str("37.0").unwrap());
        assert_eq!(sv.peso_kg, Some(Decimal::from_str("70.00").unwrap()));
    }

    #[test]
    fn test_row_to_legacy_diagnostico_s0_and_s2() {
        let mut row = Map::new();
        row.insert("id_diagnostico".into(), json!(100));
        row.insert("id_consulta".into(), json!(20));
        row.insert("id_paciente".into(), json!(3));
        row.insert("codigo_cie10".into(), json!("E11.9"));
        row.insert(
            "des_diagnostico".into(),
            json!("Diabetes mellitus tipo 2 sin mención de complicación"),
        );
        row.insert("des_tipo_diagnostico".into(), json!("CONFIRMADO"));
        row.insert("fecha_diagnostico".into(), json!("2026-09-18"));

        let diag = row_to_legacy_diagnostico(&row).expect("Debe mapear diagnóstico S2");
        assert_eq!(diag.id_diagnostico, 100);
        assert_eq!(diag.codigo_cie10, "E11.9");
        assert_eq!(
            diag.descripcion_diagnostico,
            "Diabetes mellitus tipo 2 sin mención de complicación"
        );
        assert_eq!(diag.tipo_diagnostico.as_deref(), Some("CONFIRMADO"));
    }
    #[test]
    fn regression_s2_lookup_aliases_and_required_encounter_fields() {
        let mut patient = json!({"id_paciente":1,"curp":"ROMA900101HCSMNL09","primer_nombre":"Ana","apellido_paterno":"Ramos","fecha_nacimiento":"1990-01-01","id_sexo":"female"}).as_object().unwrap().clone();
        assert_eq!(
            row_to_legacy_paciente(&patient)
                .unwrap()
                .sexo_biologico
                .as_deref(),
            Some("female")
        );
        patient.insert("id_paciente".into(), json!(i64::MAX));
        assert!(
            row_to_legacy_paciente(&patient).is_err(),
            "integer overflow must reject"
        );
        let mut encounter = json!({"id_consulta":1,"id_paciente":1,"cedula_medico_tratante":"123","nombre_medico":"Ana","id_estado_consulta":"finished","motivo_consulta":"Control","fecha_hora_inicio":"2026-01-01 10:00:00","fecha_hora_fin":"2026-01-01 11:00:00"}).as_object().unwrap().clone();
        assert_eq!(
            row_to_legacy_consulta(&encounter).unwrap().estado_consulta,
            "finished"
        );
        encounter.remove("id_estado_consulta");
        assert!(
            row_to_legacy_consulta(&encounter).is_err(),
            "missing status must reject"
        );
        encounter.insert("estado_consulta".into(), json!("FINALIZADA"));
        encounter.insert("fecha_hora_fin".into(), json!("invalid"));
        assert!(
            row_to_legacy_consulta(&encounter).is_err(),
            "invalid end must reject"
        );
    }

    #[test]
    fn regression_s3_actual_columns_and_optional_corruption() {
        let mut row = json!({"sig_id":1,"pac_id":1,"con_id":1,"presion_arterial_txt":"120/80","fc":70,"fr":18,"temp_celsius":36.5,"fech_reg_txt":"2026-01-01 10:00:00"}).as_object().unwrap().clone();
        assert_eq!(
            row_to_legacy_signo_vital(&row).unwrap().temperatura_celsius,
            Decimal::from_str("36.5").unwrap()
        );
        row.insert("fr".into(), json!("invalid"));
        assert!(
            row_to_legacy_signo_vital(&row).is_err(),
            "optional corrupt value must not become absent"
        );
        let diag = json!({"diag_id":1,"pac_id":1,"con_id":1,"cie10":"E11.9","desc_diag":"Diabetes","fech_diag_txt":"2026-01-01"}).as_object().unwrap().clone();
        assert_eq!(
            row_to_legacy_diagnostico(&diag)
                .unwrap()
                .descripcion_diagnostico,
            "Diabetes"
        );
    }
    #[test]
    fn regression_timestamps_preserve_instants_precision_and_dst() {
        let naive = serde_json::json!({"fecha_hora_inicio":"2026-01-01 10:00:00.123456"})
            .as_object()
            .unwrap()
            .clone();
        let explicit = serde_json::json!({"fecha_hora_inicio":"2026-01-01T17:00:00.123456+01:00"})
            .as_object()
            .unwrap()
            .clone();
        let first = normalize_row_timezone(naive, "America/Mexico_City").unwrap();
        let second = normalize_row_timezone(explicit, "America/Mexico_City").unwrap();
        assert_eq!(first, second);
        assert_eq!(first["fecha_hora_inicio"], "2026-01-01 16:00:00.123456");
        let ambiguous = serde_json::json!({"fecha_registro":"2021-10-31 01:30:00"})
            .as_object()
            .unwrap()
            .clone();
        assert!(normalize_row_timezone(ambiguous, "America/Mexico_City").is_err());
        let explicit = serde_json::json!({"fecha_registro":"2021-10-31T01:30:00-05:00"})
            .as_object()
            .unwrap()
            .clone();
        assert_eq!(
            normalize_row_timezone(explicit, "America/Mexico_City").unwrap()["fecha_registro"],
            "2021-10-31 06:30:00"
        );
    }
    #[test]
    fn regression_date_only_timestamps_preserve_source_precision() {
        let row = json!({"fech_reg_txt":"01/01/2026"})
            .as_object()
            .unwrap()
            .clone();
        let normalized =
            normalize_row_timezone(row, "America/Mexico_City").expect("YAML admite fecha sin hora");
        assert_eq!(normalized["__date_precision_fech_reg_txt"], "2026-01-01");
    }
}
