# BACKLOG MAESTRO DE EJECUCIÓN (`BACKLOG.md`)

> **Leyenda de Estados Verificables:**
> - `[ ]` **Pendiente (`PENDING`):** No iniciada.
> - `[/]` **En progreso (`IN_PROGRESS`):** En ejecución o delegada a un subagente activo.
> - `[~]` **Implementada (`IMPLEMENTED`):** Cambios realizados, pendiente de correr verificación empírica.
> - `[v]` **Verificada (`VERIFIED`):** Comandos de prueba ejecutados con éxito (evidencia real).
> - `[x]` **Completada (`COMPLETED`):** Verificada y consolidada en `STATE.md`.
> - `[!]` **Bloqueada (`BLOCKED`):** Requiere insumo externo o autorización humana de Nivel Rojo.

---

## Resumen de Fases y Sprints Previos (36/36 Completadas — `v1.2.0-eval`)
- [x] **Sprint 1 (Base Cargo Workspace & YAML Engine):** 4/4 tareas completadas (`sprint_1_yaml_engine.md`).
- [x] **Sprint 2 (Modelos Canónicos HL7 FHIR R4 helios-fhir):** 4/4 tareas completadas (`sprint_2_helios_fhir.md`).
- [x] **Sprint 3 (Persistencia SQLx Asíncrona & Entorno Docker):** 4/4 tareas completadas (`sprint_3_sqlx_docker.md`).
- [x] **Sprint 4 (Fachada HTTP Axum & Dashboard React 19):** 4/4 tareas completadas (`sprint_4_axum_dashboard.md`).
- [x] **Sprint 5 (Alineación Base S0, Cero Invención de Datos & Expansión 1:4):** 4/4 tareas completadas (`sprint_5_alineacion_base_s0.md`).
- [x] **Sprint 6 (Motor DSL YAML v1.2, SQL Dinámico Allowlist & Validación Referencial T4):** 4/4 tareas completadas (`sprint_6_dsl_v1_2_sql_dinamico.md`).
- [x] **Sprint 7 (Multi-Esquema S0–S3, Semilla Determinista 16,250 Recursos):** 4/4 tareas completadas (`sprint_7_arquetipos_s1_s3_semilla.md`).
- [x] **Sprint 8 (Oráculo Diferencial Python & Corpus de 100 Fallas T1–T5):** 4/4 tareas completadas (`sprint_8_oraculo_y_corpus_t1_t5.md`).
- [x] **Sprint 9 (Límites Docker cgroups v2, Benchmark k6 50 VUs & Congelamiento v1.2.0-eval):** 4/4 tareas completadas (`sprint_9_docker_k6_censo_congelamiento.md`).

---

## Sprint 10: Contabilidad Visible de Respuestas `4xx` en `k6`, Enrutamiento Dinámico HTTP (`S0`–`S3`) y Generación Automatizada de Resultados del Capítulo IV

- [x] **Tarea 10.1: Instrumentación de Contadores Visibles de Respuestas `4xx` Esperadas y Separación de Fallos Técnicos (`5xx`) en `k6`**
  - **Archivos:** `scripts/generate_data.py`, `tests/k6/valid_resp_ids.json`, `tests/k6/omitted_resp_ids.json`, `tests/k6/load_test.js`, `tests/k6/resilience_and_errors_test.js`, `tests/k6/smoke_test.js`.
  - **Estado:** Completada y verificada. Generación determinista de 2,250 IDs válidos y 250 omitidos en `tests/k6/`, métricas visibles de 404/422 y atribución al 100% de datos ausentes.

- [x] **Tarea 10.2: Enrutamiento Dinámico Multi-Arquetipo (`S0`–`S3`) en los Manejadores HTTP de `medsys-server` (`CA-4`)**
  - **Archivos:** `crates/medsys-server/src/handlers/patient.rs`, `crates/medsys-server/src/handlers/encounter.rs`, `crates/medsys-server/src/handlers/observation.rs`, `crates/medsys-server/src/handlers/condition.rs`, `crates/medsys-server/tests/e2e_interoperability.rs`.
  - **Estado:** Completada y verificada. Delegación a `dynamic.find_by_id`/`find_all_raw` y conversión con `row_to_legacy_*` en los 4 recursos. `cargo test --workspace` (91/91) y `cargo clippy` 100% PASS.

- [x] **Tarea 10.3: Creación y Ejecución del Orquestador Maestro de Experimentos del Capítulo IV (`scripts/run_all_cap4_experiments.py`)**
  - **Archivos:** `scripts/run_all_cap4_experiments.py`, `output/resultados_cap4.json`, `auditoria/RESULTADOS_CAPITULO_IV.md`.
  - **Estado:** Completada y verificada. Orquestación 100% automatizada de las 6 secciones experimentales en 0.51s con código de salida 0.

- [x] **Tarea 10.4: Verificación Final del Informe `auditoria/RESULTADOS_CAPITULO_IV.md` y Cierre `40/40`**
  - **Archivos:** `auditoria/RESULTADOS_CAPITULO_IV.md`, `output/resultados_cap4.json`, `STATE.md`, `BACKLOG.md`, `DECISIONS.md`.
  - **Estado:** Completada y verificada. 91/91 pruebas Rust aprobadas, 0 warnings en clippy, arnés de auditoría universal con 0 errores y reporte consolidado generado.

