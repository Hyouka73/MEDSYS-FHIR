# SPRINT 6: Motor Declarativo YAML v1.2, Constructor SQL Dinámico Seguro y Verificación Referencial Activa (`T4`)

## Objetivo del Sprint
Dotar a `medsys-core` y `medsys-db` de las capacidades declarativas y relacionales prometidas en el Capítulo III (`§3.2.2`, `§3.3.2` y `§3.4`) para que el middleware pueda operar sobre distintos esquemas relacionales (`S0`, `S1`, `S2`, `S3`) cambiando únicamente el archivo YAML sin recompilar el binario en Rust, manteniendo seguridad total contra inyección SQL (*allowlist* al arranque), aislamiento `REPEATABLE READ READ ONLY` y verificación activa de llaves foráneas padres (`T4`).

---

## Lista de Tareas Atómicas

- [x] **Tarea 6.1: Ampliación del DSL Declarativo YAML v1.2 en `medsys-core`**
  - **Archivos a modificar:**
    - `Cargo.toml` y `crates/medsys-core/Cargo.toml` (agregar `chrono-tz = "0.9"`)
    - `crates/medsys-core/src/model/mapping.rs`
    - `crates/medsys-core/src/engine/evaluator.rs`
  - **Instrucciones exactas:**
    1. En `crates/medsys-core/src/model/mapping.rs`:
       - Agregar en `MappingRules`:
         - `#[serde(default = "default_timezone")] pub timezone_default: String` (por defecto `"America/Mexico_City"`).
       - Agregar en `ResourceMapping`:
         - `#[serde(default)] pub distinct_on: Option<String>` (para extraer entidades únicas en vistas o sábanas desnormalizadas como `S1`).
         - `#[serde(default)] pub parent_checks: Vec<ParentCheckRule>` donde `ParentCheckRule` define `{ pub foreign_key_column: String, pub parent_table: String, pub parent_pk: String }` para validar integridad referencial activa (`T4`).
         - `#[serde(default)] pub filter_not_null: Option<String>` (para filtrar filas en sábanas planas donde una columna clave esté presente).
       - Agregar en `FieldMapping`:
         - `#[serde(default)] pub combine_with: Option<String>` y `#[serde(default)] pub combine_format: Option<String>` (formalizando en el struct la directiva que ya existe en `mapping_rules.yaml`).
         - `#[serde(default)] pub lookup: Option<LookupRule>` donde `LookupRule` tiene `{ pub table: String, pub key: String, pub value_column: String }` (para resolver catálogos externos en `S2`).
         - `#[serde(default)] pub parse_datetime: Option<String>` (lista de formatos `chrono` separados por `|`, ej. `"%d/%m/%Y %H:%M|%Y-%m-%d %H:%M:%S"`, para `S3`).
         - `#[serde(default)] pub split_part: Option<SplitPartRule>` donde `SplitPartRule` tiene `{ pub delimiter: String, pub index: usize }` (para separar `"120/80"` en sistólica `index: 0` y diastólica `index: 1` en `S3`).
         - `#[serde(default)] pub unit_conversion: Option<String>` (soportando `"lb_to_kg"` con factor `0.45359237` redondeado a 2 decimales y `"f_to_c"` en `S3`).
    2. En `crates/medsys-core/src/engine/evaluator.rs`:
       - Implementar la ejecución declarativa de `combine_with` / `combine_format`, `split_part`, `parse_datetime`, `unit_conversion` y resolución de zona horaria con `chrono_tz::America::Mexico_City` cuando un timestamp no incluya offset explícito.
       - Si `split_part` no encuentra el índice solicitado o el fragmento resultante no es numérico en un `valueQuantity`, retornar `Err(MedSysError::ProcessingError("T2 Error de formato en split_part...".into()))`.
  - **Criterio de Aceptación:** Pruebas unitarias en `evaluator.rs` verificando `combine_with`, `split_part` sobre `"130/85"`, `parse_datetime` sobre `"18/09/2026 09:15"` y `unit_conversion: "lb_to_kg"`.

- [x] **Tarea 6.2: Validador *Allowlist* (*Fail-Fast*) y Constructor SQL Dinámico en `medsys-db`**
  - **Archivos a crear/modificar:**
    - `crates/medsys-db/src/query_builder.rs` (nuevo módulo)
    - `crates/medsys-db/src/lib.rs`
  - **Instrucciones exactas:**
    1. Crear `crates/medsys-db/src/query_builder.rs`:
       - Definir la lista blanca (*allowlist*) estática de todos los nombres de tablas y columnas permitidos en los esquemas `S0`, `S1`, `S2` y `S3`, además de validar que todo identificador cumpla `^[a-z_][a-z0-9_]{0,62}$`.
       - Implementar `pub fn validate_rules_identifiers_allowlist(rules: &MappingRules) -> Result<()>` que recorra todos los `ResourceMapping`, `FieldMapping`, `lookup` y `parent_checks` al iniciar el servidor. Si una tabla o columna no es un identificador válido de la *allowlist*, abortar el arranque con `MedSysError::InvalidRule` (*Fail-Fast*).
       - Implementar un constructor de consultas parametrizadas `DynamicSqlBuilder` que, a partir de un `ResourceMapping`:
         - Construya `SELECT ... FROM {source_table} t0` incluyendo automáticamente `LEFT JOIN {lookup.table} jN ON t0.{source_column} = jN.{lookup.key}` cuando un campo defina `lookup`.
         - Soporte `SELECT DISTINCT ON (t0.{distinct_on}) ...` cuando `distinct_on` esté presente (esquema `S1`).
         - Utilice exclusivamente placeholders `$1`, `$2` para los valores de filtro (`WHERE t0.{primary_key} = $1` o `LIMIT $1 OFFSET $2`).
  - **Criterio de Aceptación:** Pruebas unitarias en `crates/medsys-db/src/query_builder.rs` probando la generación de SQL para `S0`, `S1` (`DISTINCT ON`), `S2` (`LEFT JOIN`) y el rechazo inmediato de cualquier intento de inyección SQL en nombres de tablas/columnas.

- [x] **Tarea 6.3: Transacciones `REPEATABLE READ READ ONLY` y Verificación Activa de Llaves Foráneas (`T4`)**
  - **Archivos a modificar:**
    - `crates/medsys-db/src/repository/mod.rs`
    - `crates/medsys-db/src/repository/pacientes.rs`
    - `crates/medsys-db/src/repository/consultas.rs`
    - `crates/medsys-db/src/repository/signos_vitales.rs`
    - `crates/medsys-db/src/repository/diagnosticos.rs`
    - `crates/medsys-server/src/handlers/encounter.rs`
    - `crates/medsys-server/src/handlers/observation.rs`
    - `crates/medsys-server/src/handlers/condition.rs`
  - **Instrucciones exactas:**
    1. Implementar en `MedsysRepositories` el helper `pub async fn begin_readonly_tx(&self) -> Result<sqlx::Transaction<'_, sqlx::Postgres>>` que ejecute `BEGIN TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY`.
    2. Implementar en `MedsysRepositories` el método de verificación de integridad referencial activa (`T4` en Cap. III §3.4.2):
       - `pub async fn verify_patient_exists(&self, id_paciente: i32) -> Result<()>`
       - `pub async fn verify_encounter_and_patient_exist(&self, id_consulta: i32, id_paciente: i32) -> Result<()>`
       - Si una consulta (`Encounter`), signo vital (`Observation`) o diagnóstico (`Condition`) apunta a un `id_paciente` o `id_consulta` que no existe en `tbl_pacientes` / `tbl_consultas` (como ocurre en esquemas sin `FOREIGN KEY` activa o en la categoría de falla `T4`), estos métodos deben retornar `Err(MedSysError::ProcessingError(format!("T4 Violación de integridad referencial: registro padre inexistente...")))`, lo que produce automáticamente **HTTP `422 Unprocessable Entity`** con `OperationOutcome` (`code = "processing"`).
    3. Invocar dicha verificación en los handlers `get_encounter`, `get_observation` y `get_condition`.
  - **Criterio de Aceptación:** `cargo test --workspace` verifica que un registro hijo con `id_paciente` o `id_consulta` inexistente es rechazado con HTTP 422 `OperationOutcome`.

- [ ] **Tarea 6.4: Ejecución Dinámica Multi-Arquetipo desde `medsys-db` hacia `medsys-core`**
  - **Archivos a crear/modificar:**
    - `crates/medsys-db/src/dynamic_repo.rs` (nuevo módulo)
    - `crates/medsys-db/src/lib.rs`
    - `crates/medsys-server/src/main.rs`
  - **Instrucciones exactas:**
    1. En `crates/medsys-db/src/dynamic_repo.rs`, implementar `DynamicFhirRepository` capaz de ejecutar la consulta generada por `DynamicSqlBuilder` sobre PostgreSQL, extraer la fila como `serde_json::Map<String, Value>` y transformarla mediante `evaluate_resource_mapping` (y/o convertirla a `LegacyPaciente`, `LegacyConsulta`, `LegacySignoVital`, `LegacyDiagnostico`) para que `medsys-server` soporte de forma transparente tanto el esquema `S0` como los arquetipos `S1`, `S2` y `S3` al cambiar `MAPPING_RULES_PATH`.
    2. En `crates/medsys-server/src/main.rs`, invocar `validate_rules_identifiers_allowlist(&mapping_rules)` inmediatamente después de `parse_mapping_rules(&yaml_content)` para garantizar el arranque *Fail-Fast*.
  - **Criterio de Aceptación:** `cargo check --workspace`, `cargo clippy --workspace --all-targets -- -D warnings` y `cargo test --workspace` 100 % PASS.
