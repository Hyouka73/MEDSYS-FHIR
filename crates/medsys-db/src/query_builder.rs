use std::collections::BTreeSet;

use medsys_core::error::{MedSysError, Result};
use medsys_core::model::mapping::{MappingRules, ResourceMapping};

/// Tablas permitidas en los arquetipos S0, S1, S2 y S3.
pub const ALLOWED_TABLES: &[&str] = &[
    // S0: Base NOM-004
    "tbl_pacientes",
    "tbl_consultas",
    "tbl_signos_vitales",
    "tbl_diagnosticos",
    // S1: Sábana plana desnormalizada
    "tbl_sabana_consulta_externa",
    "tbl_sabana_diagnosticos",
    "tbl_pacientes_s1",
    "tbl_consultas_s1",
    "tbl_signos_s1",
    "tbl_diagnosticos_s1",
    // S2: Normalizado con catálogos externos
    "tbl_pacientes_s2",
    "tbl_consultas_s2",
    "tbl_signos_s2",
    "tbl_diagnosticos_s2",
    "cat_sexo_biologico",
    "cat_estado_consulta",
    "cat_cie10",
    "cat_tipo_diagnostico",
    // S3: Legado con deuda técnica
    "tbl_pacientes_s3",
    "tbl_consultas_s3",
    "tbl_signos_s3",
    "tbl_diagnosticos_s3",
    "tbl_pacientes_legado",
    "tbl_consultas_legado",
    "tbl_signos_legado",
    "tbl_diagnosticos_legado",
];

/// Columnas permitidas en los arquetipos S0, S1, S2 y S3.
pub const ALLOWED_COLUMNS: &[&str] = &[
    // Claves primarias y foráneas
    "id_paciente",
    "id_consulta",
    "id_signo",
    "id_diagnostico",
    "pac_id",
    "con_id",
    "sig_id",
    "diag_id",
    // Demográficos de Paciente (S0 / S1)
    "curp",
    "primer_nombre",
    "segundo_nombre",
    "apellido_paterno",
    "apellido_materno",
    "fecha_nacimiento",
    "sexo_biologico",
    "telefono_contacto",
    "fecha_registro",
    // Consulta / Encuentro (S0 / S1)
    "cedula_medico_tratante",
    "nombre_medico",
    "estado_consulta",
    "motivo_consulta",
    "fecha_hora_inicio",
    "fecha_hora_fin",
    "unidad_medica",
    // Signos Vitales / Observación (S0 / S1)
    "presion_sistolica",
    "presion_diastolica",
    "frecuencia_cardiaca",
    "frecuencia_respiratoria",
    "temperatura_celsius",
    "peso_kg",
    "talla_cm",
    // Diagnósticos / Condición (S0 / S1)
    "codigo_cie10",
    "descripcion_diagnostico",
    "tipo_diagnostico",
    "fecha_diagnostico",
    // S2: Catálogos externos
    "id_sexo",
    "id_estado_consulta",
    "id_estado",
    "id_cie10",
    "id_tipo_diag",
    "clave",
    "descripcion",
    "codigo_fhir",
    "valor",
    "codigo",
    "nombre",
    "id",
    // S3: Columnas abreviadas / deuda técnica
    "pac_curp",
    "pac_nom1",
    "pac_nom2",
    "pac_ap_pat",
    "pac_ap_mat",
    "fech_nac_txt",
    "fech_nac",
    "sexo",
    "tel_contacto",
    "fech_reg_txt",
    "ced_medico",
    "nom_medico",
    "est_consulta",
    "motivo",
    "fech_ini_txt",
    "fech_fin_txt",
    "presion_arterial_txt",
    "fc",
    "fr",
    "temp_celsius",
    "temp_f",
    "peso_libras",
    "fech_sig_txt",
    "cie10",
    "desc_diag",
    "tipo_diag",
    "fech_diag_txt",
];

/// Valida si un identificador cumple con la sintaxis de identificadores SQL estándar:
/// `^[a-z_][a-z0-9_]{0,62}$` (longitud entre 1 y 63, inicia con letra minúscula o guion bajo,
/// seguido exclusivamente de letras minúsculas, números o guiones bajos).
pub fn is_valid_sql_identifier(ident: &str) -> bool {
    if ident.is_empty() || ident.len() > 63 {
        return false;
    }
    let mut chars = ident.chars();
    let first = chars.next().unwrap();
    if !first.is_ascii_lowercase() && first != '_' {
        return false;
    }
    chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
}

/// Valida un nombre de tabla contra la lista blanca estricta (Allowlist Fail-Fast).
pub fn validate_table_identifier(table: &str) -> Result<()> {
    if !is_valid_sql_identifier(table) {
        return Err(MedSysError::InvalidRule(format!(
            "Identificador de tabla inválido según formato ^[a-z_][a-z0-9_]{{0,62}}$: '{table}'"
        )));
    }
    if !ALLOWED_TABLES.contains(&table) {
        return Err(MedSysError::InvalidRule(format!(
            "Tabla '{table}' no permitida en la lista blanca de seguridad (Allowlist Fail-Fast)"
        )));
    }
    Ok(())
}

/// Valida un nombre de columna contra la lista blanca estricta (Allowlist Fail-Fast).
pub fn validate_column_identifier(col: &str) -> Result<()> {
    if !is_valid_sql_identifier(col) {
        return Err(MedSysError::InvalidRule(format!(
            "Identificador de columna inválido según formato ^[a-z_][a-z0-9_]{{0,62}}$: '{col}'"
        )));
    }
    if !ALLOWED_COLUMNS.contains(&col) {
        return Err(MedSysError::InvalidRule(format!(
            "Columna '{col}' no permitida en la lista blanca de seguridad (Allowlist Fail-Fast)"
        )));
    }
    Ok(())
}

/// Validador global que recorre todas las reglas YAML al arranque del servidor.
/// Aborta inmediatamente con `MedSysError::InvalidRule` si cualquier tabla o columna
/// no cumple la lista blanca estricta (*Fail-Fast*).
pub fn validate_rules_identifiers_allowlist(rules: &MappingRules) -> Result<()> {
    for res in &rules.resources {
        validate_table_identifier(&res.source_table)?;
        validate_column_identifier(&res.primary_key)?;

        if let Some(ref dist) = res.distinct_on {
            validate_column_identifier(dist)?;
        }

        if let Some(ref fnn) = res.filter_not_null {
            validate_column_identifier(fnn)?;
        }

        for pc in &res.parent_checks {
            validate_column_identifier(&pc.foreign_key_column)?;
            validate_table_identifier(&pc.parent_table)?;
            validate_column_identifier(&pc.parent_pk)?;
        }

        for fm in &res.mappings {
            if let Some(ref src_col) = fm.source_column {
                validate_column_identifier(src_col)?;
            }

            if let Some(ref comb) = fm.combine_with {
                validate_column_identifier(comb)?;
            }

            if let Some(ref lk) = fm.lookup {
                validate_table_identifier(&lk.table)?;
                validate_column_identifier(&lk.key)?;
                validate_column_identifier(&lk.value_column)?;
            }
        }
    }

    Ok(())
}

/// Constructor de consultas SQL dinámicas estrictamente parametrizadas ($1, $2)
/// a partir de la especificación declarativa `ResourceMapping`.
pub struct DynamicSqlBuilder<'a> {
    mapping: &'a ResourceMapping,
}

impl<'a> DynamicSqlBuilder<'a> {
    pub fn new(mapping: &'a ResourceMapping) -> Self {
        Self { mapping }
    }

    /// Prepara la lista de columnas proyectadas en SELECT y las cláusulas LEFT JOIN.
    fn prepare_select_and_joins(&self) -> (String, String) {
        let mut select_items = Vec::new();
        let mut joins = Vec::new();
        let mut seen_aliases = BTreeSet::new();

        // 1. Clave primaria siempre en la proyección
        let pk = &self.mapping.primary_key;
        select_items.push(format!("t0.{pk}"));
        seen_aliases.insert(pk.clone());

        // 2. Columna distinct_on si aplica
        if let Some(ref dist) = self.mapping.distinct_on {
            if !seen_aliases.contains(dist) {
                select_items.push(format!("t0.{dist}"));
                seen_aliases.insert(dist.clone());
            }
        }

        // 3. Columna filter_not_null si aplica
        if let Some(ref fnn) = self.mapping.filter_not_null {
            if !seen_aliases.contains(fnn) {
                select_items.push(format!("t0.{fnn}"));
                seen_aliases.insert(fnn.clone());
            }
        }

        // 4. Columnas foráneas de parent_checks
        for pc in &self.mapping.parent_checks {
            if !seen_aliases.contains(&pc.foreign_key_column) {
                select_items.push(format!("t0.{}", pc.foreign_key_column));
                seen_aliases.insert(pc.foreign_key_column.clone());
            }
        }

        // 5. Columnas y lookups de los mapeos individuales
        let mut join_idx = 0;
        for fm in &self.mapping.mappings {
            if let Some(ref lk) = fm.lookup {
                if let Some(ref src_col) = fm.source_column {
                    let join_alias = format!("j{join_idx}");
                    join_idx += 1;
                    joins.push(format!(
                        "LEFT JOIN {} {} ON t0.{} = {}.{}",
                        lk.table, join_alias, src_col, join_alias, lk.key
                    ));
                    if !seen_aliases.contains(src_col) {
                        select_items
                            .push(format!("{}.{} AS {}", join_alias, lk.value_column, src_col));
                        // Conservar presencia del FK de origen para distinguir NULL
                        // legítimo de catálogo sin correspondencia (también opcional).
                        select_items.push(format!("t0.{src_col} AS {src_col}__source"));
                        seen_aliases.insert(src_col.clone());
                    }
                }
            } else if let Some(ref src_col) = fm.source_column {
                if !seen_aliases.contains(src_col) {
                    select_items.push(format!("t0.{src_col}"));
                    seen_aliases.insert(src_col.clone());
                }
            }

            if let Some(ref comb) = fm.combine_with {
                if !seen_aliases.contains(comb) {
                    select_items.push(format!("t0.{comb}"));
                    seen_aliases.insert(comb.clone());
                }
            }
        }

        let select_cols = select_items.join(", ");
        let distinct_clause = match &self.mapping.distinct_on {
            Some(col) => format!("SELECT DISTINCT ON (t0.{col}) {select_cols}"),
            None => format!("SELECT {select_cols}"),
        };

        let joins_clause = if joins.is_empty() {
            String::new()
        } else {
            format!(" {}", joins.join(" "))
        };

        (distinct_clause, joins_clause)
    }

    /// Construye la consulta parametrizada para obtener un registro por su clave primaria ($1).
    pub fn build_select_by_id(&self) -> String {
        let (select_part, joins_part) = self.prepare_select_and_joins();
        let table = &self.mapping.source_table;
        let pk = &self.mapping.primary_key;

        let where_clause = match &self.mapping.filter_not_null {
            Some(fnn) => format!("WHERE t0.{pk} = $1 AND t0.{fnn} IS NOT NULL"),
            None => format!("WHERE t0.{pk} = $1"),
        };

        format!("{select_part} FROM {table} t0{joins_part} {where_clause}")
    }

    /// Construye la consulta parametrizada para listar registros paginados ($1 = LIMIT, $2 = OFFSET).
    pub fn build_select_all_paged(&self) -> String {
        let (select_part, joins_part) = self.prepare_select_and_joins();
        let table = &self.mapping.source_table;
        let pk = &self.mapping.primary_key;

        let where_clause = match &self.mapping.filter_not_null {
            Some(fnn) => format!(" WHERE t0.{fnn} IS NOT NULL"),
            None => String::new(),
        };

        let order_clause = match &self.mapping.distinct_on {
            Some(dist) => {
                if dist == pk {
                    format!("ORDER BY t0.{dist} ASC")
                } else {
                    format!("ORDER BY t0.{dist} ASC, t0.{pk} ASC")
                }
            }
            None => format!("ORDER BY t0.{pk} ASC"),
        };

        format!("{select_part} FROM {table} t0{joins_part}{where_clause} {order_clause} LIMIT $1 OFFSET $2")
    }

    /// Construye la consulta parametrizada filtrando por una clave foránea padre ($1).
    pub fn build_select_by_foreign_key(&self, fk_column: &str) -> String {
        let (select_part, joins_part) = self.prepare_select_and_joins();
        let table = &self.mapping.source_table;
        let pk = &self.mapping.primary_key;

        let where_clause = match &self.mapping.filter_not_null {
            Some(fnn) => format!("WHERE t0.{fk_column} = $1 AND t0.{fnn} IS NOT NULL"),
            None => format!("WHERE t0.{fk_column} = $1"),
        };

        format!("{select_part} FROM {table} t0{joins_part} {where_clause} ORDER BY t0.{pk} ASC")
    }

    /// Construye la consulta parametrizada filtrando por clave foránea con paginación ($1 = fk_val, $2 = LIMIT, $3 = OFFSET).
    pub fn build_select_by_foreign_key_paged(&self, fk_column: &str) -> String {
        let (select_part, joins_part) = self.prepare_select_and_joins();
        let table = &self.mapping.source_table;
        let pk = &self.mapping.primary_key;

        let where_clause = match &self.mapping.filter_not_null {
            Some(fnn) => format!("WHERE t0.{fk_column} = $1 AND t0.{fnn} IS NOT NULL"),
            None => format!("WHERE t0.{fk_column} = $1"),
        };

        format!("{select_part} FROM {table} t0{joins_part} {where_clause} ORDER BY t0.{pk} ASC LIMIT $2 OFFSET $3")
    }

    /// Helper estático para construir consulta por ID.
    pub fn select_by_id(mapping: &'a ResourceMapping) -> String {
        Self::new(mapping).build_select_by_id()
    }

    /// Helper estático para construir consulta paginada.
    pub fn select_all_paged(mapping: &'a ResourceMapping) -> String {
        Self::new(mapping).build_select_all_paged()
    }

    /// Helper estático para construir consulta filtrada por clave foránea.
    pub fn select_by_foreign_key_paged(mapping: &'a ResourceMapping, fk_column: &str) -> String {
        Self::new(mapping).build_select_by_foreign_key_paged(fk_column)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use medsys_core::model::mapping::{FieldMapping, LookupRule, ParentCheckRule};

    #[test]
    fn test_is_valid_sql_identifier() {
        assert!(is_valid_sql_identifier("tbl_pacientes"));
        assert!(is_valid_sql_identifier("id_paciente"));
        assert!(is_valid_sql_identifier("_priv_col"));
        assert!(is_valid_sql_identifier("col123"));

        // Rechazos de formato
        assert!(!is_valid_sql_identifier(""));
        assert!(!is_valid_sql_identifier("123col"));
        assert!(!is_valid_sql_identifier("tbl-pacientes"));
        assert!(!is_valid_sql_identifier("tbl pacientes"));
        assert!(!is_valid_sql_identifier("TBL_PACIENTES"));
        assert!(!is_valid_sql_identifier("tbl; DROP TABLE;"));
    }

    #[test]
    fn test_allowlist_rejection_of_sql_injection() {
        // Intentos de inyección SQL
        assert!(validate_table_identifier("tbl_pacientes; DROP TABLE tbl_pacientes; --").is_err());
        assert!(validate_column_identifier("id_paciente OR 1=1").is_err());
        assert!(validate_column_identifier("id_paciente; DELETE FROM tbl_pacientes").is_err());

        // Tablas o columnas fuera de la lista blanca
        assert!(validate_table_identifier("pg_shadow").is_err());
        assert!(validate_table_identifier("users_admin").is_err());
        assert!(validate_column_identifier("password_hash").is_err());

        // Tablas y columnas válidas
        assert!(validate_table_identifier("tbl_pacientes").is_ok());
        assert!(validate_table_identifier("cat_sexo_biologico").is_ok());
        assert!(validate_column_identifier("curp").is_ok());
        assert!(validate_column_identifier("presion_sistolica").is_ok());
    }

    #[test]
    fn test_validate_rules_identifiers_allowlist() {
        let valid_rules = MappingRules {
            version: "1.2.0".to_string(),
            description: None,
            timezone_default: "America/Mexico_City".to_string(),
            resources: vec![ResourceMapping {
                resource_type: "Patient".to_string(),
                profile: None,
                source_table: "tbl_pacientes".to_string(),
                primary_key: "id_paciente".to_string(),
                observation_type: None,
                distinct_on: None,
                parent_checks: vec![],
                filter_not_null: None,
                mappings: vec![
                    FieldMapping {
                        source_column: Some("curp".to_string()),
                        target_path: "identifier[0].value".to_string(),
                        ..Default::default()
                    },
                    FieldMapping {
                        source_column: Some("id_sexo".to_string()),
                        target_path: "gender".to_string(),
                        lookup: Some(LookupRule {
                            table: "cat_sexo_biologico".to_string(),
                            key: "id_sexo".to_string(),
                            value_column: "descripcion".to_string(),
                        }),
                        ..Default::default()
                    },
                ],
            }],
        };

        assert!(validate_rules_identifiers_allowlist(&valid_rules).is_ok());

        // Reglas con tabla inyectada
        let mut malicious_rules = valid_rules.clone();
        malicious_rules.resources[0].source_table = "tbl_pacientes; DROP TABLE users;".to_string();
        assert!(validate_rules_identifiers_allowlist(&malicious_rules).is_err());

        // Reglas con columna inyectada
        let mut malicious_col_rules = valid_rules.clone();
        malicious_col_rules.resources[0].mappings[0].source_column =
            Some("curp' OR '1'='1".to_string());
        assert!(validate_rules_identifiers_allowlist(&malicious_col_rules).is_err());
    }

    #[test]
    fn test_archetypes_s0_to_s3_allowlist_validation() {
        const RULES_S0_YAML: &str = include_str!("../../../config/mapping_rules_s0.yaml");
        const RULES_S1_YAML: &str = include_str!("../../../config/mapping_rules_s1.yaml");
        const RULES_S2_YAML: &str = include_str!("../../../config/mapping_rules_s2.yaml");
        const RULES_S3_YAML: &str = include_str!("../../../config/mapping_rules_s3.yaml");

        let r0 = medsys_core::parse_mapping_rules(RULES_S0_YAML)
            .expect("Fallo al parsear config/mapping_rules_s0.yaml");
        let r1 = medsys_core::parse_mapping_rules(RULES_S1_YAML)
            .expect("Fallo al parsear config/mapping_rules_s1.yaml");
        let r2 = medsys_core::parse_mapping_rules(RULES_S2_YAML)
            .expect("Fallo al parsear config/mapping_rules_s2.yaml");
        let r3 = medsys_core::parse_mapping_rules(RULES_S3_YAML)
            .expect("Fallo al parsear config/mapping_rules_s3.yaml");

        assert!(
            validate_rules_identifiers_allowlist(&r0).is_ok(),
            "Reglas S0 deben cumplir la lista blanca"
        );
        assert!(
            validate_rules_identifiers_allowlist(&r1).is_ok(),
            "Reglas S1 deben cumplir la lista blanca"
        );
        assert!(
            validate_rules_identifiers_allowlist(&r2).is_ok(),
            "Reglas S2 deben cumplir la lista blanca"
        );
        assert!(
            validate_rules_identifiers_allowlist(&r3).is_ok(),
            "Reglas S3 deben cumplir la lista blanca"
        );
    }

    #[test]
    fn test_dynamic_sql_builder_s0_base() {
        let mapping = ResourceMapping {
            resource_type: "Patient".to_string(),
            profile: None,
            source_table: "tbl_pacientes".to_string(),
            primary_key: "id_paciente".to_string(),
            observation_type: None,
            distinct_on: None,
            parent_checks: vec![],
            filter_not_null: None,
            mappings: vec![
                FieldMapping {
                    source_column: Some("curp".to_string()),
                    target_path: "identifier[0].value".to_string(),
                    ..Default::default()
                },
                FieldMapping {
                    source_column: Some("primer_nombre".to_string()),
                    target_path: "name[0].given[0]".to_string(),
                    ..Default::default()
                },
                FieldMapping {
                    source_column: Some("apellido_paterno".to_string()),
                    combine_with: Some("apellido_materno".to_string()),
                    target_path: "name[0].family".to_string(),
                    ..Default::default()
                },
            ],
        };

        let builder = DynamicSqlBuilder::new(&mapping);
        let sql_id = builder.build_select_by_id();
        assert_eq!(
            sql_id,
            "SELECT t0.id_paciente, t0.curp, t0.primer_nombre, t0.apellido_paterno, t0.apellido_materno FROM tbl_pacientes t0 WHERE t0.id_paciente = $1"
        );

        let sql_paged = builder.build_select_all_paged();
        assert_eq!(
            sql_paged,
            "SELECT t0.id_paciente, t0.curp, t0.primer_nombre, t0.apellido_paterno, t0.apellido_materno FROM tbl_pacientes t0 ORDER BY t0.id_paciente ASC LIMIT $1 OFFSET $2"
        );
    }

    #[test]
    fn test_dynamic_sql_builder_s1_distinct_on() {
        let mapping = ResourceMapping {
            resource_type: "Patient".to_string(),
            profile: None,
            source_table: "tbl_sabana_consulta_externa".to_string(),
            primary_key: "id_paciente".to_string(),
            observation_type: None,
            distinct_on: Some("id_paciente".to_string()),
            parent_checks: vec![],
            filter_not_null: None,
            mappings: vec![FieldMapping {
                source_column: Some("curp".to_string()),
                target_path: "identifier[0].value".to_string(),
                ..Default::default()
            }],
        };

        let builder = DynamicSqlBuilder::new(&mapping);
        let sql_paged = builder.build_select_all_paged();
        assert!(sql_paged.starts_with("SELECT DISTINCT ON (t0.id_paciente) t0.id_paciente, t0.curp FROM tbl_sabana_consulta_externa t0"));
        assert!(sql_paged.contains("ORDER BY t0.id_paciente ASC LIMIT $1 OFFSET $2"));
    }

    #[test]
    fn test_dynamic_sql_builder_s2_left_join() {
        let mapping = ResourceMapping {
            resource_type: "Patient".to_string(),
            profile: None,
            source_table: "tbl_pacientes".to_string(),
            primary_key: "id_paciente".to_string(),
            observation_type: None,
            distinct_on: None,
            parent_checks: vec![ParentCheckRule {
                foreign_key_column: "id_paciente".to_string(),
                parent_table: "tbl_pacientes".to_string(),
                parent_pk: "id_paciente".to_string(),
            }],
            filter_not_null: None,
            mappings: vec![
                FieldMapping {
                    source_column: Some("curp".to_string()),
                    target_path: "identifier[0].value".to_string(),
                    ..Default::default()
                },
                FieldMapping {
                    source_column: Some("id_sexo".to_string()),
                    target_path: "gender".to_string(),
                    lookup: Some(LookupRule {
                        table: "cat_sexo_biologico".to_string(),
                        key: "id_sexo".to_string(),
                        value_column: "descripcion".to_string(),
                    }),
                    ..Default::default()
                },
            ],
        };

        let builder = DynamicSqlBuilder::new(&mapping);
        let sql_id = builder.build_select_by_id();
        assert!(sql_id.contains("LEFT JOIN cat_sexo_biologico j0 ON t0.id_sexo = j0.id_sexo"));
        assert!(sql_id.contains("j0.descripcion AS id_sexo"));
        assert!(sql_id.ends_with("WHERE t0.id_paciente = $1"));
    }
}
