---
trigger: always_on
globs: ["**/*"]
---

# Perfil de Adaptación del Proyecto: `MedSys-FHIR` (Reverificación — Tesis UNACH 2026)

## 1. Identidad y Contexto del Proyecto
- **Nombre del Proyecto:** `MedSys-FHIR` — Middleware en Rust basado en reglas declarativas YAML v1.2 para la interoperabilidad de bases de datos clínicas relacionales (PostgreSQL 16) con el estándar HL7 FHIR R4 (`v4.0.1`).
- **Autores:** Carlos Eduardo Iglesias de la Cruz y Alexis Andrey Gálvez Roblero (UNACH, 2026).
- **Arquetipo de Proyecto:** `backend-api` + `research-thesis` + `qa-audit`.
- **Identidad experimental vigente:** reverificación local desde `2933964` sobre `18b434a`; la etiqueta histórica `v1.2.0-eval` no está acreditada. Las correcciones requieren nuevo ejecutable identificado por hash y manifiesto; ningún cierre histórico acredita CA-1–CA-5 actuales.

## 2. Subagentes Activos
- [x] `implementer` (Ajustes en scripts de telemetría `k6`, enrutamiento dinámico HTTP `S0`–`S3` y orquestador de experimentos `scripts/run_all_cap4_experiments.py`)
- [x] `tester-qa` (Ejecución de `cargo test --workspace`, `oracle_verifier.py`, `run_fault_corpus.py`, `k6` y `docker stats`)
- [x] `reviewer-auditor` (Auditoría de consistencia numérica entre código, JSONs exportados y tablas del Capítulo IV)
- [x] `documenter-integrator` (Generación de `auditoria/RESULTADOS_CAPITULO_IV.md` y `output/resultados_cap4.json`)

## 3. Comandos Oficiales de Verificación
- **Compilación y Tipos:** `cargo check --workspace`
- **Linter Estricto:** `cargo clippy --workspace --all-targets -- -D warnings`
- **Suite de Pruebas Rust:** `cargo test --workspace`
- **Oráculo Diferencial (16,250 recursos + 20 mutaciones):** `python scripts/oracle_verifier.py --self-test && python scripts/oracle_verifier.py --run-mutations`
- **Corpus de 100 Fallas (`T1`–`T5`):** `python scripts/run_fault_corpus.py`
- **Pipeline Maestro del Capítulo IV:** `python scripts/run_all_cap4_experiments.py`

## 4. Invariantes de Dominio Clínico y Metodológico (Capítulos I, II, III y IV)
1. **Cero Invención de Datos (`Fail-Closed` + `optional: true`):** Prohibido el uso de `fallback_value` o `use_data_absent_reason`. Los campos opcionales nulos (`segundo_nombre` 15 %, `telefono_contacto` 10 %, `frecuencia_respiratoria` 10 % = 250 filas, `tipo_diagnostico` 10 % = 300 filas) se omiten limpiamente; cualquier campo obligatorio nulo o valor corrupto/fuera de catálogo/FK rota/CURP inválida se rechaza con **HTTP `422 Unprocessable Entity`** y `OperationOutcome` (`issue.code = "processing"`).
2. **Censo Exacto de Recursos FHIR R4:** `1,000 Patient + 2,500 Encounter + 9,750 Observation (2,500 BP + 2,500 Temp + 2,500 HR + 2,250 Resp) + 3,000 Condition = 16,250 recursos FHIR R4 válidos` por arquetipo (`S0`, `S1`, `S2`, `S3`).
3. **Contabilidad Visible y Separada de Respuestas `4xx` en `k6` (Sección 3.6.2 del Capítulo III):**
   - Durante las pruebas con `k6`, cuando el barrido aleatorio solicita `GET /fhir/r4/Observation/resp-{id}` sobre una de las 250 filas donde `frecuencia_respiratoria IS NULL` (10 %), el servidor devuelve intencionalmente `HTTP 404 Not Found` con `OperationOutcome` (`code = "not-found"`), y ante registros anómalos devuelve `HTTP 422` (`code = "processing"`).
   - **No se deben ocultar esas respuestas `4xx`:** `load_test.js` y `resilience_and_errors_test.js` deben registrarlas en **contadores diagnósticos visibles dedicados** (`expected_404_optional_null_total`, `expected_422_fail_closed_total` y `unexpected_server_5xx_total`), comprobando que el **100 % de las respuestas `4xx` provienen de datos ausentes (`NULL`) o anomalías inyectadas de origen y el 0 % de fallas del programa**, y simultáneamente declarar `http.setResponseCallback(http.expectedStatuses(200, 404))` (en `load_test.js`) y `http.setResponseCallback(http.expectedStatuses(200, 400, 404, 422))` (en `resilience_and_errors_test.js`) para que no se sumen falsamente como caídas del servidor dentro de `http_req_failed`.
