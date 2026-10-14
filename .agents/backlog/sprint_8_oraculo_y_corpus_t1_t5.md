# SPRINT 8: Oráculo Diferencial Independiente en Python y Corpus de Inyección de Fallas Clínicas (`T1`–`T5`)

## Objetivo del Sprint
Implementar los dos instrumentos de verificación experimental prometidos en el Capítulo III (`§3.1.2`, `§3.2.3` y `§3.6.3`) para evaluar los Objetivos Específicos 2 y 3:
1. El **Oráculo Diferencial Independiente en Python (`scripts/oracle_verifier.py`)** con soporte de pruebas de mutación (*mutation testing*) que verifica la exactitud de mapeo sobre los 16,250 recursos sin compartir ni una sola línea de código con Rust.
2. El **Corpus de Inyección de Fallas Clínicas (`tests/fault_corpus/` + `scripts/run_fault_corpus.py`)** con **100 casos de prueba corruptos** distribuidos en las 5 categorías de la tesis (`T1` a `T5`, 20 casos por categoría), certificando 100 % de rechazo HTTP 422 con `OperationOutcome` (`code = "processing"`) y 0 % de falsos rechazos sobre los 9,000 registros válidos.

---

## Lista de Tareas Atómicas

- [x] **Tarea 8.1: Implementación del Oráculo Diferencial Independiente (`scripts/oracle_verifier.py`)**
  - **Archivos a crear:**
    - `scripts/oracle_verifier.py`
  - **Instrucciones exactas:**
    1. Crear `scripts/oracle_verifier.py` en Python 3 estándar (usando `urllib.request`, `json`, `argparse` y lectura de los datos semilla/SQL o conexión PostgreSQL).
    2. El oráculo debe reconstruir de manera completamente independiente los **16,250 recursos FHIR R4 esperados** para la semilla `20260930`:
       - `1,000 Patient`: verificando `id`, `identifier[0].value` (CURP), `identifier[0].system` (`urn:oid:2.16.840.1.113883.4.629`), `name[0].given`, `name[0].family`, `birthDate`, `gender` (`M->male`, `F->female`, `I->other`) y presencia/ausencia de `telecom[0].value`.
       - `2,500 Encounter`: verificando `status`, `class.code == "AMB"`, `subject.reference`, `period.start`, `period.end` y cédula profesional SEP.
       - `9,750 Observation`: verificando para cada signo vital los 3 recursos obligatorios (`bp-{id}` con LOINC `85354-9`/`8480-6`/`8462-4`, `temp-{id}` con `8310-5`, `hr-{id}` con `8867-4`) y el cuarto recurso `resp-{id}` (`9279-1`) exactamente en las 2,250 filas donde `frecuencia_respiratoria IS NOT NULL` (y verificando que en las 250 filas donde es `NULL`, el endpoint `/fhir/r4/Observation/resp-{id}` devuelve HTTP 404 `OperationOutcome` y no aparece en el Bundle).
       - `3,000 Condition`: verificando `clinicalStatus == "active"`, `code.coding[0].code` (CIE-10), `recordedDate` y que `verificationStatus` sea `"confirmed"`/`"provisional"` en las 2,700 filas con valor y esté omitido (`None`) en las 300 filas donde `tipo_diagnostico IS NULL`.
    3. Soportar ejecución tanto contra el servidor HTTP en vivo (`--base-url http://localhost:3000`) como contra un directorio de JSONs exportados (`--input-dir output`) o en modo de autoverificación local (`--self-test`).
    4. Exportar el reporte detallado en JSON/Markdown (`output/oracle_report.json`).
  - **Criterio de Aceptación:** `python3 scripts/oracle_verifier.py --self-test` termina con código de salida `0` y reporta `16,250 / 16,250` recursos verificados con `100.0%` de exactitud.

- [x] **Tarea 8.2: Modo de Pruebas de Mutación (*Mutation Testing*) en `scripts/oracle_verifier.py`**
  - **Archivos a modificar:**
    - `scripts/oracle_verifier.py`
  - **Instrucciones exactas:**
    - Añadir la bandera `--run-mutations` a `scripts/oracle_verifier.py` (descrita en Cap. III §3.6.3):
      - El script toma 20 recursos FHIR válidos representativos y les inyecta **20 mutaciones artificiales** (ej. cambiar LOINC `8480-6` por `8462-4`, alterar un dígito de la CURP, cambiar unidad `Cel` por `F`, invertir `male`/`female`, dejar `verificationStatus: "provisional"` cuando `tipo_diagnostico` era `NULL`, alterar `Patient/1` por `Patient/2`, etc.).
      - Verifica que el comparador del oráculo detecta y rechaza las **20 de 20 mutaciones (sensibilidad del 100 %)**.
  - **Criterio de Aceptación:** `python3 scripts/oracle_verifier.py --run-mutations` detecta `20/20` mutaciones (`100.0%` sensibilidad) y retorna código de salida `0`.

- [ ] **Tarea 8.3: Construcción del Corpus de 100 Fallas Clínicas (`T1`–`T5`) en `tests/fault_corpus/`**
  - **Archivos a crear:**
    - `tests/fault_corpus/fault_corpus_100.json`
    - `tests/fault_corpus/fault_corpus_100.sql`
    - `tests/fault_corpus/README.md`
  - **Instrucciones exactas:**
    - Construir el corpus determinista de **100 casos de prueba anómalos** divididos exactamente en las 5 categorías definidas en el Capítulo III (§3.2.3 y Tabla 1):
      - **Categoría `T1` (20 casos — Nulos en campos obligatorios):** ej. `primer_nombre` vacío, `apellido_paterno` vacío, `curp` vacía, `sexo_biologico` nulo, `estado_consulta` vacío, `codigo_cie10` vacío, etc.
      - **Categoría `T2` (20 casos — Formatos, tipos y fechas inválidas):** ej. `fecha_nacimiento` imposible `"2026-02-30"`, `fecha_diagnostico` corrupta `"FECHA_INVALIDA"`, `presion_arterial_txt` `"120/XX"`, `frecuencia_respiratoria` negativa `-5`, `presion_sistolica` no numérica en `evaluator`, etc.
      - **Categoría `T3` (20 casos — Valores fuera de diccionario/catálogo):** ej. `sexo_biologico = "X"`, `estado_consulta = "PENDIENTE_REVISION"`, `tipo_diagnostico = "DESCARTADO_MANUAL"`, etc.
      - **Categoría `T4` (20 casos — Llaves foráneas padres rotas / registros huérfanos):** ej. consultas con `id_paciente = 99999`, signos vitales con `id_consulta = 88888` o `id_paciente = 77777`, diagnósticos con `id_consulta = 66666`.
      - **Categoría `T5` (20 casos — Sintaxis de identificadores y regex CURP inválida):** ej. CURP de 17 caracteres, CURP con caracteres especiales, CURP con entidad federativa inexistente `"XX"`, CURP con mes `13`, CURP con sexo `"Z"`, IDs de ruta malformados.
  - **Criterio de Aceptación:** El archivo `tests/fault_corpus/fault_corpus_100.json` contiene exactamente 100 casos etiquetados (`T1-01`..`T1-20`, `T2-01`..`T2-20`, `T3-01`..`T3-20`, `T4-01`..`T4-20`, `T5-01`..`T5-20`).

- [ ] **Tarea 8.4: Corredor Automatizado del Corpus de Fallas (`scripts/run_fault_corpus.py`) y Suite Rust de Integración `T1`–`T5`**
  - **Archivos a crear/modificar:**
    - `scripts/run_fault_corpus.py`
    - `crates/medsys-core/tests/fault_corpus_integration.rs` (nueva suite de integración en Rust que lee `tests/fault_corpus/fault_corpus_100.json`)
    - `tests/k6/resilience_and_errors_test.js`
  - **Instrucciones exactas:**
    1. Crear `crates/medsys-core/tests/fault_corpus_integration.rs`:
       - Cargar `tests/fault_corpus/fault_corpus_100.json` y ejecutar los 100 casos contra el motor de transformación y evaluación de `medsys-core` (y mapeo a `ServerError` en `medsys-server`), verificando que **100 de 100 casos retornan error de dominio mapeado a HTTP 422 `OperationOutcome` con `issue[0].code == "processing"`**.
    2. Crear `scripts/run_fault_corpus.py` para ejecutar los 100 casos y generar la tabla resumen por categoría (`T1` a `T5`: 20/20 rechazados = 100 %, falsos rechazos = 0 %) lista para incluirse en el Capítulo IV de la tesis.
    3. Actualizar `tests/k6/resilience_and_errors_test.js` para que verifique `"code": "invalid"` en HTTP 400 (en lugar del antiguo `"code": "value"`), `"code": "not-found"` en HTTP 404 y `"code": "processing"` en HTTP 422.
  - **Criterio de Aceptación:** `cargo test --workspace` ejecuta los 100 casos del corpus en `fault_corpus_integration.rs` con 100 % PASS y `python3 scripts/run_fault_corpus.py` retorna código `0`.
