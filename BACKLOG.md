# BACKLOG PRINCIPAL — MedSys-FHIR (`v1.2.0-eval`)

## Parte I: Cronograma Base Completado (`v1.1.0` — Sprints 1 al 4)

### Sprint 1: Motor de Mapeo Declarativo YAML y Monorepo Workspace (COMPLETADO)
- [x] **Tarea 1.1:** Inicialización del Monorepo Cargo Workspace (`Cargo.toml`) y estructura de crates (`medsys-core`, `medsys-db`, `medsys-server`).
- [x] **Tarea 1.2:** Definición del sistema centralizado de errores `MedSysError` y modelos de mapeo en `medsys-core`.
- [x] **Tarea 1.3:** Parser y deserialización de reglas YAML (`mapping_rules.yaml`).
- [x] **Tarea 1.4:** Pruebas unitarias del motor de mapeo con macro `include_str!` en memoria.

### Sprint 2: Recursos HL7 FHIR R4 Canónicos (`helios-fhir`) (COMPLETADO)
- [x] **Tarea 2.1:** Modelado canónico del recurso `Patient` (CURP oficial y demografía).
- [x] **Tarea 2.2:** Modelado canónico del recurso `Encounter` (clase AMB y médico tratante).
- [x] **Tarea 2.3:** Modelado canónico del recurso `Observation` (códigos LOINC de signos vitales).
- [x] **Tarea 2.4:** Modelado canónico del recurso `Condition` (codificación internacional CIE-10).

### Sprint 3: Persistencia Asíncrona SQLx y Entorno Docker PostgreSQL 16 (COMPLETADO)
- [x] **Tarea 3.1:** Configuración de infraestructura Docker Compose con PostgreSQL 16 y esquema sintético.
- [x] **Tarea 3.2:** Configuración del pool asíncrono SQLx en `medsys-db`.
- [x] **Tarea 3.3:** Implementación de repositorios de lectura parametrizada (`$1`, `$2`).
- [x] **Tarea 3.4:** Pruebas de integración de persistencia sobre datos sintéticos.

### Sprint 4: API REST Axum, `OperationOutcome` y Dashboard React 19 (COMPLETADO)
- [x] **Tarea 4.1:** Enrutamiento HTTP en Axum y endpoints REST FHIR canónicos (`/fhir/r4/`).
- [x] **Tarea 4.2:** Manejador global de excepciones traduciendo a `OperationOutcome`.
- [x] **Tarea 4.3:** Dashboard interactivo en React 19 + Vite + Tailwind CSS.
- [x] **Tarea 4.4:** Pruebas E2E de interoperabilidad, validación con k6, auditoría Clippy y release final.

---

## Parte II: Sprints de Alineación Metodológica y Evaluación Experimental (`v1.2.0-eval` — Capítulos I, II y III)

### Sprint 5: Saneamiento del Esquema Base (`S0`), Fin de `fallback_value`, `Observation` 1:4 y `OperationOutcome` Canónico (COMPLETADO)
*Detalle técnico en: `.agents/backlog/sprint_5_alineacion_base_s0.md`*
- [x] **Tarea 5.1:** Actualización de `mapping_rules.yaml` a `v1.2.0` (7 bloques: `Patient` con `telefono_contacto` opcional, `Encounter`, `Observation` 1:4 incluyendo `heart_rate` `8867-4` y `respiratory_rate` `9279-1` opcional, `Condition` con `tipo_diagnostico` opcional) y eliminación total de `fallback_value` en modelos.
- [x] **Tarea 5.2:** Implementación en `medsys-core` de validación regex oficial de CURP (`T5`), `Patient.telecom` opcional, `Condition.verificationStatus` opcional/Fail-Closed (`T3`) y `transform_observation_respiratory_rate` (`LOINC 9279-1`).
- [x] **Tarea 5.3:** Exposición HTTP de los 4 subtipos de `Observation` (`bp`, `temp`, `hr`, `resp` con soporte de prefijo y sufijo) en `medsys-server` y homologación de códigos `IssueType` de `OperationOutcome` (`400 -> invalid`, `404 -> not-found`, `422 -> processing`, `500/503 -> exception/transient`).
- [x] **Tarea 5.4:** Reparación de las 2 pruebas unitarias rotas en `medsys-core` y ampliación de la suite de pruebas (`cargo test --workspace` 100 % PASS y `cargo clippy -- -D warnings` 0 warnings).

### Sprint 6: Motor Declarativo YAML v1.2, Constructor SQL Dinámico Seguro y Verificación Referencial Activa (`T4`) (COMPLETADO)
*Detalle técnico en: `.agents/backlog/sprint_6_dsl_v1_2_sql_dinamico.md`*
- [x] **Tarea 6.1:** Ampliación del DSL YAML v1.2 en `medsys-core` (`lookup`, `parse_datetime`, `split_part`, `unit_conversion`, `distinct_on`, `parent_checks` y zona horaria `America/Mexico_City` con `chrono-tz`).
- [x] **Tarea 6.2:** Validador de lista blanca (*allowlist*) de identificadores SQL al arranque (*Fail-Fast*) y constructor dinámico `DynamicSqlBuilder` (`SELECT`, `LEFT JOIN`, `DISTINCT ON`) en `medsys-db`.
- [x] **Tarea 6.3:** Ejecución sobre transacciones `REPEATABLE READ READ ONLY` y verificación activa de existencia de llaves foráneas padres (`id_paciente`, `id_consulta`) para rechazar registros huérfanos (`T4`) con HTTP 422 `OperationOutcome`.
- [x] **Tarea 6.4:** Adaptador `DynamicFhirRepository` en `medsys-db` y validación *Fail-Fast* al arranque en `crates/medsys-server/src/main.rs`.

### Sprint 7: Banco Multi-Esquema (`S0`–`S3`), Reglas YAML por Arquetipo y Generador Determinista (`SEED=20260930`)
*Detalle técnico en: `.agents/backlog/sprint_7_arquetipos_s1_s3_semilla.md`*
- [x] **Tarea 7.1:** Creación de los 4 esquemas DDL (`docker/sql/schema_s0_base.sql`, `schema_s1_flat.sql`, `schema_s2_lookup.sql`, `schema_s3_legacy.sql`) y configuración del rol de solo lectura `medsys_ro`.
- [x] **Tarea 7.2:** Creación de los 4 archivos declarativos `config/mapping_rules_s0.yaml`, `mapping_rules_s1.yaml`, `mapping_rules_s2.yaml` y `mapping_rules_s3.yaml`.
- [x] **Tarea 7.3:** Refactorización determinista de `scripts/generate_data.py` (`--seed 20260930 --schema {s0,s1,s2,s3,all}`) con 10 % exacto de `NULL` en `frecuencia_respiratoria` (250 nulos $\rightarrow$ 9,750 `Observation` y 16,250 recursos FHIR totales) y 10 % de `NULL` en `tipo_diagnostico`.
- [ ] **Tarea 7.4:** Pruebas automatizadas de equivalencia semántica en Rust verificando que `S0`, `S1`, `S2` y `S3` generan idénticos recursos FHIR R4 sin recompilar el binario.

### Sprint 8: Oráculo Diferencial Independiente en Python y Corpus de 100 Fallas Clínicas (`T1`–`T5`)
*Detalle técnico en: `.agents/backlog/sprint_8_oraculo_y_corpus_t1_t5.md`*
- [ ] **Tarea 8.1:** Implementación del oráculo diferencial independiente `scripts/oracle_verifier.py` para verificar campo por campo los 16,250 recursos FHIR R4.
- [ ] **Tarea 8.2:** Implementación de `--run-mutations` en `scripts/oracle_verifier.py` (20 mutaciones artificiales con 100 % de sensibilidad de detección).
- [ ] **Tarea 8.3:** Creación del corpus determinista de 100 fallas clínicas `tests/fault_corpus/fault_corpus_100.json` y `.sql` (20 casos por categoría: `T1`, `T2`, `T3`, `T4`, `T5`).
- [ ] **Tarea 8.4:** Implementación de `scripts/run_fault_corpus.py`, suite de integración `crates/medsys-core/tests/fault_corpus_integration.rs` y actualización de `tests/k6/resilience_and_errors_test.js` (100 % de rechazo HTTP 422 `OperationOutcome`).

### Sprint 9: Contenedores con Límites Estrictos, Exportación Censal (16,250 JSON), Benchmark `k6` (50 VUs) y Congelamiento `v1.2.0-eval`
*Detalle técnico en: `.agents/backlog/sprint_9_docker_k6_censo_congelamiento.md`*
- [ ] **Tarea 9.1:** Homologación de límites `cgroups v2` en `docker-compose.yml` y `docker/docker-compose.yml` (`medsys-server`: 1 vCPU / 256 MB RAM; `postgres`: 1 vCPU / 512 MB RAM) y conexión por defecto con `medsys_ro`.
- [ ] **Tarea 9.2:** Actualización de `scripts/export_fhir_samples.py` y `scripts/validate_fhir.ps1` para exportar y validar por lotes el censo completo de 16,250 recursos JSON con `validator_cli.jar -version 4.0.1 -tx n/a`.
- [ ] **Tarea 9.3:** Reparación de sintaxis en `tests/k6/run_benchmark_with_stats.ps1` y `run_all_benchmarks.ps1`, actualización de `load_test.js` (`bp`, `temp`, `hr`, `resp`) y captura conjunta de `docker stats` + `VmRSS`.
- [ ] **Tarea 9.4:** Actualización de versión de crates a `1.2.0`, auditoría final (`cargo fmt`, `cargo clippy`, `cargo test`, oráculo y corpus) y cierre de `STATE.md` / `BACKLOG.md` para `v1.2.0-eval`.
