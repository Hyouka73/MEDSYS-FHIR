# SPRINT 10: Contabilidad Visible de Respuestas `4xx` en `k6`, Enrutamiento Dinámico HTTP (`S0`–`S3`) y Generación Automatizada de Resultados del Capítulo IV

## Objetivo del Sprint
Ejecutar de forma 100 % automatizada toda la batería experimental del **Capítulo IV (Resultados y Discusión)** cumpliendo al pie de la letra el nuevo apartado de la **Sección 3.6.2 del Capítulo III (`Tratamiento metodológico y contabilidad visible de respuestas 4xx en las pruebas con k6`)**, y generar el informe consolidado `auditoria/RESULTADOS_CAPITULO_IV.md` y `output/resultados_cap4.json`.

---

## Lista de Tareas Atómicas

- [ ] **Tarea 10.1: Instrumentación de Contadores Visibles de Respuestas `4xx` Esperadas y Separación de Fallos Técnicos (`5xx`) en `k6`**
  - **Archivos a crear/modificar:**
    - `scripts/generate_data.py`
    - `tests/k6/valid_resp_ids.json` y `tests/k6/omitted_resp_ids.json`
    - `tests/k6/load_test.js`
    - `tests/k6/resilience_and_errors_test.js`
    - `tests/k6/smoke_test.js`
  - **Instrucciones exactas:**
    1. En `scripts/generate_data.py`:
       - Exportar determinísticamente (`SEED=20260930`) `tests/k6/valid_resp_ids.json` (los **2,250 IDs** con `frecuencia_respiratoria IS NOT NULL`) y `tests/k6/omitted_resp_ids.json` (los **250 IDs** con `frecuencia_respiratoria IS NULL`).
    2. En `tests/k6/load_test.js`:
       - Declarar al inicio:
         ```javascript
         const OMITTED_RESP_SET = new Set(JSON.parse(open('./omitted_resp_ids.json')));
         http.setResponseCallback(http.expectedStatuses(200, 404));
         ```
       - Añadir **contadores diagnósticos visibles** en `load_test.js`:
         - `export const http200ValidResourcesCounter = new Counter('http_200_valid_resources_total');`
         - `export const expected404OptionalNullCounter = new Counter('expected_404_optional_null_total');`
         - `export const unexpectedErrorCounter = new Counter('unexpected_technical_errors_total');`
         - `export const expected4xxAttributionRate = new Rate('expected_4xx_due_to_missing_data_rate');`
       - Cuando `obsType === 'resp'`:
         - Si `resObs.status === 200` y `!OMITTED_RESP_SET.has(obsId)`: incrementar `http200ValidResourcesCounter.add(1)` y registrar éxito nominal (`true`).
         - Si `resObs.status === 404` y `OMITTED_RESP_SET.has(obsId)` y el cuerpo contiene `"resourceType": "OperationOutcome"` y `"code": "not-found"`: incrementar **`expected404OptionalNullCounter.add(1)`**, registrar **`expected4xxAttributionRate.add(true)`** (demostrando que el 100 % de los 404 observados fueron causados por la ausencia deliberada del dato opcional `frecuencia_respiratoria = NULL` en esa consulta) y registrar éxito de protocolo (`isObsOk = true`).
         - En cualquier otro caso (ej. un 404 en un ID que sí tenía dato, o un 500/503): incrementar `unexpectedErrorCounter.add(1)` y `expected4xxAttributionRate.add(false)`.
       - Así, en el resumen de `k6` y en el Capítulo IV queda **100 % visible cuántos códigos `404` ocurrieron**, se demuestra con `expected_4xx_due_to_missing_data_rate == 100.00%` que **el 100 % de esos errores fueron por falta de datos opcionales en origen**, y `http_req_failed` marca **`0.00 %`** de fallos técnicos del servidor.
    3. En `tests/k6/resilience_and_errors_test.js` y `tests/k6/smoke_test.js`:
       - Declarar `http.setResponseCallback(http.expectedStatuses(200, 400, 404, 422));` y añadir contadores visibles por tipo de rechazo (`expected_400_invalid_param_total`, `expected_404_not_found_total`, `expected_422_fail_closed_total`, `unexpected_5xx_total`).
  - **Criterio de Aceptación:** `python scripts/generate_data.py --seed 20260930` genera `valid_resp_ids.json` (2,250 IDs) y `omitted_resp_ids.json` (250 IDs), y `load_test.js` / `resilience_and_errors_test.js` reportan de forma visible y separada los contadores de `4xx` por falta de datos frente a `0` errores técnicos `5xx`.

- [ ] **Tarea 10.2: Enrutamiento Dinámico Multi-Arquetipo (`S0`–`S3`) en los Manejadores HTTP de `medsys-server` (`CA-4`)**
  - **Archivos a modificar:**
    - `crates/medsys-server/src/handlers/patient.rs`
    - `crates/medsys-server/src/handlers/encounter.rs`
    - `crates/medsys-server/src/handlers/observation.rs`
    - `crates/medsys-server/src/handlers/condition.rs`
    - `crates/medsys-server/tests/e2e_interoperability.rs`
  - **Instrucciones exactas:**
    - En los 4 handlers HTTP, verificar si el `source_table` configurado en `state.mapping_rules` corresponde a `S0` (`tbl_pacientes`, `tbl_consultas`, `tbl_signos_vitales`, `tbl_diagnosticos`) o a los arquetipos `S1`, `S2` o `S3`. Cuando sea `S1`–`S3`, delegar la lectura a `state.repositories.dynamic.find_by_id(&mapping, id)` y convertir con `medsys_db::row_to_legacy_*` para que el mismo binario compilado sirva por HTTP los 4 arquetipos cambiando únicamente `MAPPING_RULES_PATH`.
  - **Criterio de Aceptación:** `cargo test --workspace` y `cargo clippy --workspace --all-targets -- -D warnings` 100 % PASS con 0 warnings.

- [ ] **Tarea 10.3: Creación y Ejecución del Orquestador Maestro de Experimentos del Capítulo IV (`scripts/run_all_cap4_experiments.py`)**
  - **Archivos a crear:**
    - `scripts/run_all_cap4_experiments.py`
  - **Instrucciones exactas:**
    - Implementar y ejecutar `scripts/run_all_cap4_experiments.py` para que corra automáticamente y consolide las métricas de las 6 secciones del Capítulo IV:
      1. **Sección 4.1 (`CA-1` — Conformidad Sintáctica Oficial):** Censo de `16,250` recursos (`1,000 Patient`, `2,500 Encounter`, `9,750 Observation` [2,500 BP, 2,500 Temp, 2,500 HR, 2,250 Resp], `3,000 Condition`), tamaño promedio en bytes/KB por recurso, y conteo de severidades (`0 Fatal`, `0 Error`).
      2. **Sección 4.2 (`CA-2` — Fidelidad Clínica y Oráculo Independiente):** Ejecución de `oracle_verifier.py --self-test` (`16,250/16,250 = 100.0 %`) y `--run-mutations` (`20/20 = 100.0 %`), incluyendo tabla de auditoría de nulos opcionales (`150` `segundo_nombre`, `100` `telefono_contacto`, `250` `frecuencia_respiratoria`, `300` `tipo_diagnostico`).
      3. **Sección 4.3 (`CA-3` — Rechazo Seguro *Fail-Closed* e Inyección de Fallos):** Ejecución de `run_fault_corpus.py` sobre las 100 anomalías (`T1` a `T5`, `20/20` cada una $\rightarrow$ `100/100` HTTP 422 `OperationOutcome` con `code = "processing"`) y `0.0 %` de falsos rechazos sobre los `16,250` recursos válidos.
      4. **Sección 4.4 (`CA-4` y `CA-5` — Adaptación Declarativa Multi-Esquema `S0`–`S3`):** Tabla comparativa de líneas de YAML, directivas empleadas por esquema (`distinct_on`, `lookup`, `parse_datetime`, `split_part`, `unit_conversion`, `parent_checks`), `0` líneas modificadas en Rust y resultado de equivalencia semántica $S_0 \equiv S_1 \equiv S_2 \equiv S_3$.
      5. **Sección 4.5 (`CT` — Caracterización Técnica de Tiempos de Respuesta, Contabilidad Visible de `4xx` y Recursos):** Resultados de `k6` (latencia media, $p_{50}$, $p_{90}$, $p_{95}$, $p_{99}$, RPS, desglose visible de peticiones `200 OK` vs. respuestas `404` esperadas por `frecuencia_respiratoria = NULL` [100 % atribuidas a falta de datos opcionales] vs. `0` errores técnicos `5xx`) y telemetría conjunta de `docker stats` + `VmRSS` para `medsys-server` (`1 vCPU / 256 MB`) y `postgres` (`1 vCPU / 512 MB`).
      6. **Sección 4.6 (Matriz de Decisión de Viabilidad Técnica):** Tabla resumen contrastando `CA-1` a `CA-5` contra la Regla de Decisión de la Tabla 3.1 del Capítulo III.
  - **Criterio de Aceptación:** `python scripts/run_all_cap4_experiments.py` termina con código `0` y genera `auditoria/RESULTADOS_CAPITULO_IV.md` y `output/resultados_cap4.json`.

- [ ] **Tarea 10.4: Verificación Final del Informe `auditoria/RESULTADOS_CAPITULO_IV.md` y Cierre `40/40`**
  - **Archivos a crear/modificar:**
    - `auditoria/RESULTADOS_CAPITULO_IV.md`
    - `output/resultados_cap4.json`
    - `STATE.md`, `BACKLOG.md`, `DECISIONS.md`
  - **Criterio de Aceptación:** `auditoria/RESULTADOS_CAPITULO_IV.md` existe con todas las tablas completas, `cargo test --workspace` pasa al 100 % y `validate_harness.py` reporta `0` errores.
