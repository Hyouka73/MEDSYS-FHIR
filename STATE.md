# ESTADO ACTIVO DEL PROYECTO (`STATE.md`) — MedSys-FHIR (`v1.2.0-eval`)

## Metadatos de Control
- **Última Actualización:** 2026-10-14T13:06:00-06:00
- **Versión Objetivo:** `1.2.0-eval` (Alineación total con Capítulos I, II y III reestructurados — Tesis UNACH 2026)
- **Sprint Activo:** **Sprint 7 — Banco Multi-Esquema (`S0`–`S3`), Reglas YAML por Arquetipo y Generador Determinista (`SEED=20260930`)** (`.agents/backlog/sprint_7_arquetipos_s1_s3_semilla.md`).
- **Estado General:** 27 / 36 tareas completadas (75.0% — Sprints 1 al 6 completados al 100%; Sprint 7 en progreso: 3/4 tareas).
- **Tarea en Curso:** Ninguna (Listo para iniciar **Tarea 7.4**).
- **Última Tarea Cerrada:** **Tarea 7.3** (Refactorización determinista de `scripts/generate_data.py` con `--seed 20260930 --schema {s0,s1,s2,s3,all}`, validación de 10% exacto de NULL en frecuencia_respiratoria = 250 nulos $\rightarrow$ 9,750 Observaciones y 16,250 recursos FHIR totales, 10% exacto de NULL en tipo_diagnostico = 300 nulos, y hashing SHA-256 reproducible en ejecuciones consecutivas).
- **Siguiente Tarea Inmediata:** **Tarea 7.4** (Pruebas Automatizadas de Equivalencia Semántica entre Arquetipos `S0` vs `S1` vs `S2` vs `S3`, ver `.agents/backlog/sprint_7_arquetipos_s1_s3_semilla.md`).
- **Estado Actual del Build:**
  - `medsys-core`: 34/34 tests PASS.
  - `medsys-db`: 30/30 tests PASS.
  - `medsys-server` y tests E2E/integración: 24/24 tests PASS (10 E2E, 9 integración, 5 persistencia).
  - Workspace total: 88/88 tests PASS (100 %). Clippy: 0 warnings.

---

## 1. Resumen de Brechas Detectadas que Resuelven los Sprints 5 al 9
1. **Sprint 5 (Saneamiento Base `S0`):**
   - Elimina `fallback_value: "provisional"` y `use_data_absent_reason` de `mapping_rules.yaml` y de `medsys-core` (cumpliendo la política *Fail-Closed* D-006/D-029).
   - Completa los 4 bloques de `Observation` en `mapping_rules.yaml` (`85354-9` PA, `8310-5` Temp, `8867-4` FC y `9279-1` FR con `optional: true`) y restituye `telefono_contacto` (`optional: true`) en `Patient.telecom[0]`.
   - Implementa `transform_observation_respiratory_rate`, validación regex de 18 caracteres de RENAPO para `curp` (`T5`), exposición de `resp-{id}` / `{id}-resp` en `medsys-server`, y homologa los códigos `IssueType` de `OperationOutcome` (`400 -> invalid`, `404 -> not-found`, `422 -> processing`, `500/503 -> exception/transient`).
2. **Sprint 6 (Motor DSL YAML v1.2 y SQL Dinámico):**
   - Añade soporte declarativo para `lookup`, `parse_datetime`, `split_part`, `unit_conversion`, `distinct_on`, `parent_checks` y zona horaria `America/Mexico_City` (`chrono-tz`).
   - Implementa el validador de lista blanca (*allowlist*) al arranque (*Fail-Fast*), el constructor `DynamicSqlBuilder`, transacciones `REPEATABLE READ READ ONLY` y verificación activa de existencia de llaves foráneas padres (`T4`).
3. **Sprint 7 (Banco Multi-Esquema `S0`–`S3` y Semilla `SEED=20260930`):**
   - Crea los 4 esquemas DDL (`schema_s0_base.sql`, `schema_s1_flat.sql`, `schema_s2_lookup.sql`, `schema_s3_legacy.sql`) con el rol de solo lectura `medsys_ro` y los 4 archivos `config/mapping_rules_s{0..3}.yaml`.
   - Actualiza `scripts/generate_data.py` (`SEED=20260930`) para inyectar exactamente 10 % de `NULL` en `frecuencia_respiratoria` (250 nulos $\rightarrow$ **9,750 recursos `Observation`** y **16,250 recursos FHIR R4 en total**) y 10 % de `NULL` en `tipo_diagnostico`.
4. **Sprint 8 (Oráculo Diferencial en Python y Corpus `T1`–`T5`):**
   - Crea `scripts/oracle_verifier.py` (con modo `--run-mutations` para 20 mutaciones) y el corpus de 100 fallas clínicas `tests/fault_corpus/fault_corpus_100.json` (`T1` a `T5`, 20 casos por categoría) junto con `scripts/run_fault_corpus.py`.
5. **Sprint 9 (Docker `cgroups v2`, Censo HL7 16,250 JSON, `k6` 50 VUs y Tag `v1.2.0-eval`):**
   - Homologa límites `cgroups v2` en `docker-compose.yml` y `docker/docker-compose.yml` (`medsys-server`: 1 vCPU / 256 MB; `postgres`: 1 vCPU / 512 MB), repara la sintaxis de los scripts PowerShell de `k6`, actualiza `export_fhir_samples.py` para los 16,250 recursos y congela `v1.2.0-eval`.

---

## 2. Instrucción de Arranque para el Agente
Al recibir del usuario la orden `"Comienza"`, `"Continúa"` o `"Haz el Sprint 5"`, abre inmediatamente `.agents/backlog/sprint_5_alineacion_base_s0.md` y ejecuta la **Tarea 5.1**.

---

## 3. Registro de Cierre del Sprint 5
**Modificados:** `mapping_rules.yaml` (v1.2.0, 7 bloques, sin fallback/absent-reason, `telefono_contacto` opcional, `heart_rate` y `respiratory_rate` opcional); `medsys-core`: `model/mapping.rs` (removidos `fallback_value`/`use_data_absent_reason`), `engine/evaluator.rs` (política Fail-Closed, eliminados helpers `*_with_fallback`), `engine/transform.rs` (`is_valid_curp` T5, telecom opcional, sexo obligatorio, Condition `verificationStatus` opcional/T3, `transform_observation_respiratory_rate`), `engine/mod.rs` (tests), `lib.rs`; `medsys-server`: `handlers/observation.rs` (prefijo/sufijo bp/temp/hr/resp/rr, 404 si resp NULL, lista 1:4), `handlers/legacy.rs` (4 Observations), `error.rs` (invalid/not-found/processing/transient, 503 en BD); tests `e2e_interoperability.rs` y `server_integration.rs`.
**CURPs sintéticas corregidas a formato RENAPO válido** (antes tenían 17 caracteres): `ROMA900101HCSMNL09` y `LOPE950512MCSPRN07` en `schema_legado_simulado_nom004.sql`, `scripts/generate_data.py` (fallback de unicidad ahora termina en dígito), `dashboard/src/services/mockData.js` y tests de `medsys-db`.
**Pendiente:** regenerar datos con `generate_data.py` y verificar CURPs en BD viva (Sprint 7).

---

## 4. Registro de Avance Sprint 6: Tarea 6.1 Cerrada
- **Archivos Modificados:**
  - `Cargo.toml` y `crates/medsys-core/Cargo.toml`: incorporado `chrono-tz = "0.9"`.
  - `crates/medsys-core/src/model/mapping.rs`: definidos `timezone_default: "America/Mexico_City"`, `distinct_on`, `parent_checks` (`ParentCheckRule`), `filter_not_null`, `combine_with`, `combine_format`, `lookup` (`LookupRule`), `parse_datetime`, `split_part` (`SplitPartRule`), `unit_conversion`.
  - `crates/medsys-core/src/engine/evaluator.rs`: funciones `apply_unit_conversion` (soporta `"lb_to_kg"` con factor `0.45359237` a 2 decimales y `"f_to_c"`), `parse_datetime_with_formats` (formatos `|` y resolución `chrono_tz::America::Mexico_City`), soporte completo en `evaluate_field_mapping` con fail-closed ante errores `T2`, y suite de 4 tests unitarios dedicados.
  - `crates/medsys-core/src/model/mod.rs` y `crates/medsys-core/src/lib.rs`: re-exportación canónica de los nuevos tipos y utilidades.
- **Validación Automatizada:**
  - `cargo check --workspace`: PASS.
  - `cargo clippy --workspace --all-targets -- -D warnings`: 0 warnings.
  - `cargo test --workspace`: 73/73 tests PASS (33 en `medsys-core`, 19 en `medsys-db`, 3 de persistencia, 10 E2E interoperabilidad, 8 integración de servidor).
- **Siguiente paso:** Tarea 6.2 completada.

---

## 5. Registro de Avance Sprint 6: Tarea 6.2 Cerrada
- **Archivos Creados/Modificados:**
  - `crates/medsys-db/src/query_builder.rs` (nuevo): implementación de `ALLOWED_TABLES`, `ALLOWED_COLUMNS`, validador de sintaxis regex `is_valid_sql_identifier` (`^[a-z_][a-z0-9_]{0,62}$`), validadores `validate_table_identifier`, `validate_column_identifier`, validador integral `validate_rules_identifiers_allowlist` (*Fail-Fast*) y constructor `DynamicSqlBuilder` (`SELECT ... FROM t0`, `LEFT JOIN` para campos con `lookup`, `DISTINCT ON` para sábanas desnormalizadas `S1`, filtros estrictamente parametrizados con `$1`, `$2`).
  - `crates/medsys-db/src/lib.rs`: exposición pública del módulo `query_builder` y sus utilidades.
- **Validación Automatizada:**
  - `cargo check --workspace`: PASS.
  - `cargo clippy --workspace --all-targets -- -D warnings`: 0 warnings.
  - `cargo test --workspace`: 79/79 tests PASS (33 en `medsys-core`, 25 en `medsys-db`, 3 de persistencia, 10 E2E interoperabilidad, 8 integración de servidor).
- **Siguiente paso:** Tarea 6.3 completada.

---

## 6. Registro de Avance Sprint 6: Tarea 6.3 Cerrada
- **Archivos Modificados:**
  - `crates/medsys-db/src/repository/mod.rs`: implementación de `begin_readonly_tx` (`SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY`) y verificadores de integridad referencial activa `verify_patient_exists(id_paciente)` y `verify_encounter_and_patient_exist(id_consulta, id_paciente)` retornando `MedSysError::ProcessingError("T4 Violación de integridad referencial: ...")`.
  - `crates/medsys-db/src/repository/{pacientes,consultas,signos_vitales,diagnosticos}.rs`: integración de transacciones `REPEATABLE READ READ ONLY`.
  - `crates/medsys-server/src/handlers/encounter.rs`: verificación de existencia del paciente padre antes de transformación FHIR.
  - `crates/medsys-server/src/handlers/observation.rs`: verificación de existencia de la consulta y paciente padre antes de transformación FHIR.
  - `crates/medsys-server/src/handlers/condition.rs`: verificación de existencia de la consulta y paciente padre antes de transformación FHIR.
  - `crates/medsys-server/tests/server_integration.rs`: prueba unitaria de integración validando que errores T4 de integridad referencial mapean exactamente a HTTP 422 con recurso canónico `OperationOutcome` e `issue[0].code = "processing"`.
- **Validación Automatizada:**
  - `cargo check --workspace`: PASS.
  - `cargo clippy --workspace --all-targets -- -D warnings`: 0 warnings.
  - `cargo test --workspace`: 80/80 tests PASS (33 en `medsys-core`, 25 en `medsys-db`, 3 de persistencia, 10 E2E interoperabilidad, 9 integración de servidor).
- **Siguiente paso:** Tarea 6.3 completada.

---

## 7. Registro de Cierre del Sprint 6: Motor Declarativo YAML v1.2 y SQL Dinámico Seguro
- **Archivos Creados/Modificados:**
  - `crates/medsys-db/src/dynamic_repo.rs` (nuevo): implementación de `DynamicFhirRepository` capaz de ejecutar consultas dinámicas parametrizadas con `DynamicSqlBuilder`, extracción universal `pg_row_to_json_map` sobre PostgreSQL, evaluación FHIR directa con `evaluate_resource_mapping`, verificador dinámico de llaves foráneas `parent_checks` (`T4`), y conversiones de filas multi-esquema a entidades tipadas `Legacy*` (incluyendo separación de presión arterial `120/80`, conversiones de unidades y catálogos `S2`/`S3`).
  - `crates/medsys-db/src/lib.rs`: exportación pública de `dynamic_repo`, `DynamicFhirRepository`, `pg_row_to_json_map`, y funciones `row_to_legacy_*`.
  - `crates/medsys-db/src/repository/mod.rs`: incorporación del repositorio dinámico en `MedsysRepositories::dynamic`.
  - `crates/medsys-db/src/query_builder.rs`: helpers estáticos y soporte de consultas filtradas por clave foránea paginadas.
  - `crates/medsys-server/src/main.rs`: invocación preventiva de `validate_rules_identifiers_allowlist(&mapping_rules)` inmediatamente tras parsear YAML para garantizar rechazo *Fail-Fast* ante identificadores no autorizados.
- **Validación Automatizada del Cierre de Sprint 6:**
  - `cargo check --workspace`: PASS.
  - `cargo clippy --workspace --all-targets -- -D warnings`: 0 warnings.
  - `cargo test --workspace`: 84/84 tests PASS (33 en `medsys-core`, 29 en `medsys-db`, 3 de persistencia, 10 E2E interoperabilidad, 9 integración de servidor).
- **Siguiente paso:** Sprint 7 — Tarea 7.1 completada.

---

## 8. Registro de Avance Sprint 7: Tarea 7.1 Cerrada
- **Archivos Creados/Modificados:**
  - `schema_legado_simulado_nom004.sql`: creación e inclusión del rol de solo lectura `medsys_ro` con permisos exclusivos `SELECT`, concesión de `USAGE ON SCHEMA public`, y adición de `frecuencia_respiratoria` en la fila 1 (18) y fila 2 (`NULL`).
  - `docker/sql/schema_s0_base.sql` (nuevo): DDL completo del arquetipo `S0` (4 tablas base normalizadas NOM-004 sin `DEFAULT 'CONFIRMADO'` en `tipo_diagnostico`, roles `medsys_user`, `postgres` y `medsys_ro` con solo lectura).
  - `docker/sql/schema_s1_flat.sql` (nuevo): DDL completo del arquetipo `S1` (sábana plana desnormalizada `tbl_sabana_consulta_externa` concentrando paciente, consulta y signos vitales, tabla `tbl_sabana_diagnosticos`, e índices para `DISTINCT ON`).
  - `docker/sql/schema_s2_lookup.sql` (nuevo): DDL completo del arquetipo `S2` (esquema normalizado con tablas de catálogo externo `cat_sexo_biologico`, `cat_estado_consulta`, `cat_cie10`, `cat_tipo_diagnostico`, claves foráneas enteras en `tbl_*_s2` y rol `medsys_ro`).
  - `docker/sql/schema_s3_legacy.sql` (nuevo): DDL completo del arquetipo `S3` (esquema legado con deuda técnica: columnas abreviadas `pac_*`, `con_*`, `sig_*`, `diag_*`, fechas y presión arterial en texto `"120/80"`, peso en libras `peso_libras`, sin restricciones `FOREIGN KEY` en DDL para forzar validación activa `parent_checks` a nivel aplicación, y rol `medsys_ro`).
  - `crates/medsys-db/tests/persistence_integration.rs`: adición del test contractual exhaustivo `test_multi_schema_ddl_archetypes_and_readonly_role_integrity` que valida la presencia del rol `medsys_ro`, concesión de `SELECT` en los 5 esquemas, y reglas estructurales específicas por arquetipo.
- **Validación Automatizada:**
  - `cargo check --workspace`: PASS.
  - `cargo clippy --workspace --all-targets -- -D warnings`: 0 warnings.
  - `cargo test --workspace`: 85/85 tests PASS (33 en `medsys-core`, 29 en `medsys-db`, 4 de persistencia, 10 E2E interoperabilidad, 9 integración de servidor).
- **Siguiente paso:** Sprint 7 — Tarea 7.2 completada.

---

## 9. Registro de Avance Sprint 7: Tarea 7.2 Cerrada
- **Archivos Creados/Modificados:**
  - `config/mapping_rules_s0.yaml` (nuevo): copia canónica idéntica a `mapping_rules.yaml` v1.2.0 para el arquetipo base normalizado NOM-004.
  - `config/mapping_rules_s1.yaml` (nuevo): reglas para el arquetipo sábana plana desnormalizada (`tbl_sabana_consulta_externa`) usando `distinct_on: "id_paciente"` para `Patient`.
  - `config/mapping_rules_s2.yaml` (nuevo): reglas para el arquetipo normalizado con catálogos externos, incorporando directivas `lookup` hacia `cat_sexo_biologico` (con `codigo_fhir`), `cat_estado_consulta` (con `codigo_fhir`), `cat_cie10` (con `clave`) y `cat_tipo_diagnostico` (con `clave`).
  - `config/mapping_rules_s3.yaml` (nuevo): reglas para el arquetipo legado con deuda técnica, incorporando `parse_datetime: "%d/%m/%Y|%Y-%m-%d"`, separación de presión arterial sistólica y diastólica con `split_part` (delimitador `"/"`, índices 0 y 1), conversión de unidades `unit_conversion: "lb_to_kg"` para `peso_libras`, y verificación activa de llaves foráneas `parent_checks` en `Encounter`, `Observation` y `Condition`.
  - `crates/medsys-core/src/engine/mod.rs`: adición del test unitario `test_parse_archetype_mapping_rules_s0_to_s3` verificando que los 4 archivos YAML deserializan sin errores y cumplen con las estructuras esperadas.
  - `crates/medsys-db/src/query_builder.rs`: adición del test unitario `test_archetypes_s0_to_s3_allowlist_validation` asegurando que el 100 % de tablas y columnas declaradas en los 4 arquetipos satisfacen la lista blanca estricta (*Fail-Fast*).
  - `crates/medsys-db/tests/persistence_integration.rs`: adición del test de integración `test_archetype_mapping_rules_s0_to_s3_contract_and_allowlist` vinculando las reglas con el subsistema de persistencia.
- **Validación Automatizada:**
  - `cargo check --workspace`: PASS.
  - `cargo clippy --workspace --all-targets -- -D warnings`: 0 warnings.
  - `cargo test --workspace`: 88/88 tests PASS (34 en `medsys-core`, 30 en `medsys-db`, 5 de persistencia, 10 E2E interoperabilidad, 9 integración de servidor).
- **Siguiente paso:** Sprint 7 — Tarea 7.3 completada.

---

## 10. Registro de Avance Sprint 7: Tarea 7.3 Cerrada
- **Archivos Creados/Modificados:**
  - `scripts/generate_data.py`:
    - Refactorización determinista con semilla fija `SEED = 20260930` usando generador pseudoaleatorio aislado `random.Random(seed)`.
    - Cumplimiento estricto del censo documentado en Capítulo III (§3.2.3):
      - 1,000 Pacientes (IDs 1..1000): 100 % con CURP válida verificada con regex de RENAPO, exactamente 150 con `segundo_nombre = NULL` (15.0 %, 1 en DDL + 149 en bulk) y exactamente 100 con `telefono_contacto = NULL` (10.0 %).
      - 2,500 Consultas (IDs 1..2500): vinculadas a pacientes existentes `1..1000`.
      - 2,500 Signos Vitales (IDs 1..2500): 100 % con presión sistólica, diastólica, frecuencia cardíaca y temperatura presentes; exactamente 250 con `frecuencia_respiratoria = NULL` (10.0 %, 1 en DDL con `id_signo = 2` y 249 en bulk) y 2,250 con FR entre 14 y 22 `/min` (`id_signo = 1` con 18 y 2,249 en bulk). Producción matemática exacta: $2,500\text{ (PA)} + 2,500\text{ (Temp)} + 2,500\text{ (FC)} + 2,250\text{ (FR)} = \mathbf{9,750}$ recursos `Observation`.
      - 3,000 Diagnósticos (IDs 1..3000): exactamente 300 con `tipo_diagnostico = NULL` (10.0 %, omisión limpia de `verificationStatus` sin `fallback_value`) y 2,700 con valor ('CONFIRMADO' / 'PRESUNTIVO').
      - Total recursos FHIR R4: $\mathbf{16,250}$ ($1,000\text{ Patient} + 2,500\text{ Encounter} + 9,750\text{ Observation} + 3,000\text{ Condition}$).
    - Soporte CLI para argumento `--schema {s0,s1,s2,s3,all}` y `--output` generando a partir de las mismas entidades en memoria:
      - `docker/sql/02_bulk_data.sql` (`S0` base normalizado NOM-004)
      - `docker/sql/bulk_s1.sql` (`S1` sábana plana desnormalizada `tbl_sabana_consulta_externa`)
      - `docker/sql/bulk_s2.sql` (`S2` catálogos externos con `cat_cie10` IDs 4..20 e IDs foráneos)
      - `docker/sql/bulk_s3.sql` (`S3` esquema legado con fechas en texto, TA concatenada y peso en libras)
  - `docker/sql/02_bulk_data.sql` (regenerado determinísticamente con hash SHA-256 `d3bb843f910a58074296a1c80ba25ec0ccd84204abfacf3d9112b0a5331bd324`).
  - `docker/sql/bulk_s1.sql`, `docker/sql/bulk_s2.sql`, `docker/sql/bulk_s3.sql` (generados determinísticamente para `--schema all`).
- **Validación Automatizada:**
  - `python scripts/generate_data.py --seed 20260930`: PASS (ejecuciones consecutivas producen idéntico SHA-256).
  - Conteo de nulos en BD: 250 nulos en `frecuencia_respiratoria` (10.0 %) y 300 nulos en `tipo_diagnostico` (10.0 %).
  - `cargo check --workspace`: PASS.
  - `cargo clippy --workspace --all-targets -- -D warnings`: 0 warnings.
  - `cargo test --workspace`: 88/88 tests PASS (100 %).
- **Siguiente paso:** Sprint 7 — Tarea 7.4 (Pruebas Automatizadas de Equivalencia Semántica entre Arquetipos `S0` vs `S1` vs `S2` vs `S3`, ver `.agents/backlog/sprint_7_arquetipos_s1_s3_semilla.md`).




