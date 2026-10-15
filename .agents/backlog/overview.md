# PANORAMA GENERAL DEL BACKLOG (OVERVIEW) — MedSys-FHIR (`v1.2.0-eval`)

## Resumen Ejecutivo del Cronograma
El desarrollo de **MedSys-FHIR** consta de:
- **Fase Base (`v1.1.0`, Sprints 1 al 4):** 16 tareas completadas (arquitectura base Cargo Workspace, modelos `helios-fhir`, pool SQLx, servidor Axum y Dashboard React 19).
- **Fase de Alineación Metodológica y Evaluación Experimental (`v1.2.0-eval`, Sprints 5 al 9):** 20 tareas técnicas atómicas diseñadas para alinear al 100 % el código fuente con los Capítulos I, II y III de la tesis (eliminación de `fallback_value`, expansión 1:4 de `Observation` a 9,750 instancias y 16,250 recursos totales, motor declarativo YAML v1.2, soporte multi-esquema `S0`–`S3`, oráculo diferencial en Python, corpus de 100 fallas `T1`–`T5` y pipeline reproducible de telemetría `k6` + `docker stats` + `VmRSS`).

| Sprint | Enfoque Principal | Estado | Tareas Hechas | Total |
| :--- | :--- | :---: | :---: | :---: |
| **Sprint 1** | Motor de Mapeo Declarativo YAML y Monorepo Workspace | COMPLETADO | 4 | 4 |
| **Sprint 2** | Recursos HL7 FHIR R4 Canónicos (`helios-fhir`) | COMPLETADO | 4 | 4 |
| **Sprint 3** | Persistencia Asíncrona SQLx y Entorno Docker PostgreSQL 16 | COMPLETADO | 4 | 4 |
| **Sprint 4** | API Axum Facade, `OperationOutcome` y Dashboard React 19 | COMPLETADO | 4 | 4 |
| **Sprint 5** | Saneamiento `S0`: Fin de `fallback_value`, `Observation` 1:4, CURP Regex y `OperationOutcome` | COMPLETADO | 4 | 4 |
| **Sprint 6** | Motor DSL YAML v1.2, SQL Dinámico con *Allowlist* y Verificación Referencial (`T4`) | COMPLETADO | 4 | 4 |
| **Sprint 7** | Banco Multi-Esquema (`S0`–`S3`), Reglas YAML por Arquetipo y Semilla Determinista (`16,250` recursos) | COMPLETADO | 4 | 4 |
| **Sprint 8** | Oráculo Diferencial Independiente en Python y Corpus de 100 Fallas Clínicas (`T1`–`T5`) | COMPLETADO | 4 | 4 |
| **Sprint 9** | Límites Docker `cgroups v2`, Exportación Censal HL7, Benchmark `k6` (50 VUs) y Tag `v1.2.0-eval` | COMPLETADO | 4 | 4 |
| **Sprint 10** | Contabilidad Visible de Respuestas `4xx` en `k6`, Enrutamiento Dinámico (`S0`–`S3`) y Resultados Cap. IV | COMPLETADO | 4 | 4 |

**Progreso Total:** 40 / 40 tareas completadas (100.0%) — **EVALUACIÓN Y EXPERIMENTACIÓN CULMINADA AL 100%**.

---

## Índice de Archivos de Sprint (`.agents/backlog/`)
- [x] `sprint_1_yaml_engine.md` (100%)
- [x] `sprint_2_helios_fhir.md` (100%)
- [x] `sprint_3_sqlx_docker.md` (100%)
- [x] `sprint_4_axum_dashboard.md` (100%)
- [x] `sprint_5_alineacion_base_s0.md` (100%)
- [x] `sprint_6_dsl_v1_2_sql_dinamico.md` (100%)
- [x] `sprint_7_arquetipos_s1_s3_semilla.md` (100%)
- [x] `sprint_8_oraculo_y_corpus_t1_t5.md` (100%)
- [x] `sprint_9_docker_k6_censo_congelamiento.md` (100%)
- [x] `sprint_10_automatizacion_experimentos_cap4.md` (100%)

