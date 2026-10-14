# ESTADO ACTIVO DEL PROYECTO (`STATE.md`) — MedSys-FHIR (`v1.2.0-eval`)

## Metadatos de Control
- **Última Actualización:** 2026-10-14T11:35:00-06:00
- **Versión Objetivo:** `1.2.0-eval` (Alineación total con Capítulos I, II y III reestructurados — Tesis UNACH 2026)
- **Sprint Activo:** **Sprint 6 — Motor Declarativo YAML v1.2** (`.agents/backlog/sprint_6_dsl_v1_2_sql_dinamico.md`).
- **Estado General:** 22 / 36 tareas completadas (61.1% — Sprints 1 al 5 completados; Tareas 6.1 y 6.2 completadas).
- **Tarea en Curso:** Ninguna (Listo para iniciar **Tarea 6.3**).
- **Última Tarea Cerrada:** **Tarea 6.2** (Validador *Allowlist* (*Fail-Fast*) y Constructor SQL Dinámico `DynamicSqlBuilder` en `medsys-db`). Validado: `cargo check`, `cargo clippy --workspace --all-targets -- -D warnings` (0 warnings) y `cargo test --workspace` (79 tests PASS, 100 %).
- **Siguiente Tarea Inmediata:** **Tarea 6.3** (Transacciones `REPEATABLE READ READ ONLY` y Verificación Activa de Llaves Foráneas (`T4`), ver `.agents/backlog/sprint_6_dsl_v1_2_sql_dinamico.md`).
- **Estado Actual del Build:**
  - `medsys-core`: 33/33 tests PASS.
  - `medsys-db`: 25/25 tests PASS (6 tests de allowlist, sintaxis SQL, inyección y DynamicSqlBuilder).
  - `medsys-server` y tests E2E/integración: 21/21 tests PASS.
  - Workspace total: 79/79 tests PASS (100 %).

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
- **Siguiente paso:** Tarea 6.3 (Transacciones `REPEATABLE READ READ ONLY` y Verificación Activa de Llaves Foráneas `T4`).
