# EVIDENCIA DEL REPOSITORIO — MedSys-FHIR
**Fecha de Auditoría:** 2026-10-02  
**Repositorio:** MedSys-FHIR  
**Modo:** Solo lectura (Inspección empírica directa, sin commits ni modificaciones de código)

---

## 1. Salida literal de: `grep -rn "fallback_value" . --include="*"` (excluyendo `target/` y `.git/`)

```text
./crates/medsys-core/src/engine/evaluator.rs:276:                if let Some(ref fb) = rule.fallback_value {
./crates/medsys-core/src/engine/evaluator.rs:321:                    if let Some(ref fb) = rule.fallback_value {
./crates/medsys-core/src/engine/evaluator.rs:353:                    if let Some(ref fb) = rule.fallback_value {
./crates/medsys-core/src/engine/mod.rs:277:            tipo_diag.fallback_value.as_deref(),
./crates/medsys-core/src/engine/mod.rs:932:            fallback_value: None,
./crates/medsys-core/src/engine/mod.rs:973:            fallback_value: None,
./crates/medsys-core/src/engine/mod.rs:1019:            fallback_value: None,
./crates/medsys-core/src/engine/mod.rs:1057:            fallback_value: None,
./crates/medsys-core/src/engine/mod.rs:1240:    fn test_transform_condition_fallback_value_when_tipo_diagnostico_is_none() {
./crates/medsys-core/src/engine/mod.rs:1255:            .expect("Transformación de Condition con tipo_diagnostico = None debe aplicar fallback_value sin fallar");
./crates/medsys-core/src/engine/mod.rs:1275:            "Debe aplicar fallback_value: 'provisional' ante columna nula"
./crates/medsys-core/src/engine/mod.rs:1296:    fn test_evaluator_condition_declarative_fallback_value_null_column() {
./crates/medsys-core/src/engine/mod.rs:1321:            .expect("Evaluación declarativa debe aplicar fallback_value ante columna con valor SQL NULL");
./crates/medsys-core/src/engine/mod.rs:1338:        // Modificamos las reglas para remover fallback_value de tipo_diagnostico
./crates/medsys-core/src/engine/mod.rs:1350:        tipo_diag_map.fallback_value = None;
./crates/medsys-core/src/engine/mod.rs:1362:        // Sin fallback_value configurado, el motor debe fallar cerrado (Fail-Closed, D-006)
./crates/medsys-core/src/engine/mod.rs:1366:            "Debe emitir ProcessingError si tipo_diagnostico es nulo y no hay fallback_value"
./crates/medsys-core/src/engine/transform.rs:787:    // con soporte de fallback_value ante valores nulos o desconocidos
./crates/medsys-core/src/engine/transform.rs:788:    let fallback = ver_rule.and_then(|r| r.fallback_value.as_deref());
./crates/medsys-core/src/engine/transform.rs:862:                    "Data corruption: Campo obligatorio 'tipo_diagnostico' ausente o nulo sin fallback_value configurado".to_string(),
./crates/medsys-core/src/model/mapping.rs:121:    pub fallback_value: Option<String>,
./mapping_rules.yaml:212:        fallback_value: "provisional"
./tests/k6/resilience_and_errors_test.js:9:// Refactorización: Se elimina el anti-patrón de inventar datos falsos (fallback_value).
```

*Total de ocurrencias encontradas:* 23 ocurrencias.

---

## 2. Lista de archivos que existen y no existen (ruta exacta)

| Elemento solicitado | Ruta exacta en el entorno | Estado | Observación detallada |
| :--- | :--- | :--- | :--- |
| `crates/*` | `crates/medsys-core`<br>`crates/medsys-db`<br>`crates/medsys-server` | **EXISTE** | Monorepo de Rust compuesto por 3 crates activos (`medsys-core`, `medsys-db`, `medsys-server`). |
| `Dockerfile` | `./Dockerfile` | **EXISTE** | Archivo multi-stage en la raíz (adicionalmente existe `docker/Dockerfile.server`). |
| `docker-compose.yml` | `./docker-compose.yml` | **EXISTE** | Archivo de orquestación en la raíz (adicionalmente existe `docker/docker-compose.yml`). |
| `load_test.js` | `./load_test.js`<br>`tests/k6/load_test.js` | **NO EXISTE** (en raíz)<br>**EXISTE** (en subdirectorio) | No existe en `./load_test.js`. La ruta exacta donde se encuentra es `tests/k6/load_test.js`. |
| `resilience_test.js` | `./resilience_test.js`<br>`tests/k6/resilience_and_errors_test.js` | **NO EXISTE** | No existe ningún archivo denominado `resilience_test.js` en el repositorio; el script correspondiente se llama `tests/k6/resilience_and_errors_test.js`. |
| `scripts/generate_data.py` | `scripts/generate_data.py` | **EXISTE** | Script en Python generador de dataset sintético ubicado en la ruta solicitada. |
| `scripts/export_fhir_samples.py` | `scripts/export_fhir_samples.py` | **EXISTE** | Script en Python para extracción de muestras FHIR JSON ubicado en la ruta solicitada. |
| `validator_cli.jar` | `validator_cli.jar` | **NO EXISTE** | No existe en la raíz ni en ningún subdirectorio del repositorio. |
| `dashboard/` | `dashboard/` | **EXISTE** | Directorio del frontend SPA (Vite + React 19) en la raíz del repositorio. |
| `STATE.md` | `STATE.md` | **EXISTE** | Archivo de control de estado del proyecto ubicado en la raíz del repositorio. |
| `backlog` | `./backlog`<br>`BACKLOG.md`<br>`.agents/backlog/` | **NO EXISTE** (como archivo o carpeta sin extensión en raíz)<br>**EXISTE** (`BACKLOG.md` y `.agents/backlog/`) | No existe una entidad nombrada `backlog` en raíz; existen `BACKLOG.md` (archivo markdown en raíz) y `.agents/backlog/` (directorio con archivos de sprint). |
| `.agents/` | `.agents/` | **EXISTE** | Directorio de configuración de agentes y metodologías en la raíz del repositorio. |

---

## 3. Resultado de `cargo test --workspace`

### Resumen General
- **Total de pruebas en el workspace:** **59 pruebas**
- **¿El total es 59?:** **SÍ, el total es exactamente 59** (57 pruebas superadas, 2 falladas).
- **Pruebas superadas (passed):** 57
- **Pruebas falladas (failed):** 2

### Desglose por Suite de Pruebas

1. **`medsys-core`** (`unittests src\lib.rs` / `target\debug\deps\medsys_core-*.exe`):
   - **Total:** 24 pruebas
   - **Pasadas:** 22
   - **Falladas:** 2
   - **Nombres de las pruebas fallidas:**
     * `engine::tests::test_observation_resource_mappings` (Causa: `assertion left == right failed (left: 3, right: 2)`)
     * `engine::tests::test_parse_embedded_specification_yaml` (Causa: `assertion left == right failed (left: 6, right: 5)`)

2. **`medsys-db`** (`unittests src\lib.rs` / `target\debug\deps\medsys_db-*.exe`):
   - **Total:** 19 pruebas
   - **Pasadas:** 19
   - **Falladas:** 0

3. **`persistence_integration`** (`tests\persistence_integration.rs` / `target\debug\deps\persistence_integration-*.exe`):
   - **Total:** 3 pruebas
   - **Pasadas:** 3
   - **Falladas:** 0

4. **`medsys-server` (lib)** (`unittests src\lib.rs` / `target\debug\deps\medsys_server-*.exe`):
   - **Total:** 0 pruebas
   - **Pasadas:** 0
   - **Falladas:** 0

5. **`medsys-server` (bin)** (`unittests src\main.rs` / `target\debug\deps\medsys_server-*.exe`):
   - **Total:** 0 pruebas
   - **Pasadas:** 0
   - **Falladas:** 0

6. **`e2e_interoperability`** (`tests\e2e_interoperability.rs` / `target\debug\deps\e2e_interoperability-*.exe`):
   - **Total:** 8 pruebas
   - **Pasadas:** 8
   - **Falladas:** 0

7. **`server_integration`** (`tests\server_integration.rs` / `target\debug\deps\server_integration-*.exe`):
   - **Total:** 5 pruebas
   - **Pasadas:** 5
   - **Falladas:** 0

8. **Doc-tests (`medsys_core`, `medsys_db`, `medsys_server`)**:
   - **Total:** 0 pruebas

---

## 4. Descripción de qué hace cada archivo o directorio existente del punto 2

- `crates/medsys-core`: Contiene las estructuras de dominio clínico, modelos de reglas YAML y el motor de mapeo declarativo que transforma datos relacionales a recursos HL7 FHIR R4.
- `crates/medsys-db`: Implementa la capa de persistencia relacional asíncrona mediante pool SQLx hacia PostgreSQL 16 y repositorios de lectura parametrizada ($1, $2).
- `crates/medsys-server`: Provee el servidor API HTTP en Axum con endpoints canónicos REST FHIR (`/fhir/r4/`), interoperabilidad, bundles searchset y manejador global de excepciones vía `OperationOutcome`.
- `Dockerfile` (`./Dockerfile`): Define la compilación multi-stage de la aplicación en Rust y la construcción de la imagen de producción ligera sobre `debian:bookworm-slim`.
- `docker-compose.yml` (`./docker-compose.yml`): Orquesta los contenedores de la base de datos PostgreSQL 16 (`medsys_fhir_postgres`) y del middleware Axum (`medsys-server`) bajo límites de 256MB RAM y 1 CPU.
- `tests/k6/load_test.js`: Ejecuta la prueba de carga nominal en k6 con 50 usuarios virtuales sostenidos sobre endpoints de lectura FHIR para evaluar latencias p95 y tasa de éxito.
- `scripts/generate_data.py`: Genera un script SQL (`02_bulk_data.sql`) con datos clínicos sintéticos reproducibles (NOM-004/NOM-024) utilizando una semilla determinista (20260930).
- `scripts/export_fhir_samples.py`: Consulta por lotes y de manera concurrente los endpoints del servidor para exportar muestras completas de recursos FHIR en archivos JSON dentro de `output/`.
- `dashboard/`: Aplicación web SPA (Vite + React 19) para monitoreo visual de interoperabilidad, inspección comparativa de datos relacionales frente a FHIR y consola de eventos.
- `STATE.md`: Bitácora de control de estado del proyecto que reporta el avance de sprints, conformidad regulatoria de privacidad, estado de compilación y decisiones de arquitectura.
- `BACKLOG.md` / `.agents/backlog/`: Define el cronograma canónico de desarrollo estructurado en 4 sprints y 16 tareas técnicas para el middleware MedSys-FHIR.
- `.agents/`: Almacena la definición de roles de agentes de desarrollo, reglas de trabajo, flujos de orquestación y directrices metodológicas del repositorio.

---

## 5. Implementación de `verificationStatus`, `OperationOutcome` y respuesta HTTP 422

### A. `verificationStatus` (EXISTE)
- **`crates/medsys-core/src/engine/transform.rs:768`**: Localiza en la configuración de reglas la asignación cuyo target_path contenga `"verificationStatus"`.
- **`crates/medsys-core/src/engine/transform.rs:786-866`**: Mapea `tipo_diagnostico` a los estados de verificación FHIR (`CONFIRMADO` -> `confirmed`, `PRESUNTIVO` -> `provisional`). En caso de ausencia (`NULL`), cadena vacía o valor no contemplado en diccionarios, evalúa si existe `fallback_value` o si aplica `data_absent_reason`; de lo contrario, rechaza emitiendo `MedSysError::ProcessingError` (Fail-Closed).
- **`crates/medsys-core/src/engine/transform.rs:868-873`**: Construye el `CodeableConcept` normativo con system `"http://terminology.hl7.org/CodeSystem/condition-ver-status"` y código resultante, incorporándolo al atributo `verification_status` de `Condition` (línea 898).
- **`crates/medsys-core/src/engine/mod.rs:282`**: Define el target_path `"verificationStatus.coding[0].code"` dentro de las pruebas unitarias y especificación embebida.
- **`mapping_rules.yaml:207-212`**: Regla declarativa con `target_path: "verificationStatus.coding[0].code"` y `fallback_value: "provisional"`.

### B. `OperationOutcome` (EXISTE)
- **`crates/medsys-core/src/engine/transform.rs:1023-1059`**: Función `pub fn create_operation_outcome(...) -> OperationOutcome` que ensambla la estructura canónica del recurso FHIR con severidad, código de issue, codificación del sistema `http://hl7.org/fhir/issue-type` y diagnóstico técnico en español.
- **`crates/medsys-core/src/engine/mod.rs:66-79`**: Prueba unitaria `test_create_operation_outcome_canonical` que valida la serialización canónica de `OperationOutcome`.
- **`crates/medsys-server/src/error.rs:86-105`**: Implementación de `IntoResponse` para `ServerError`, generando un `OperationOutcome` serializado con cabecera `Content-Type: application/fhir+json; charset=utf-8` para toda falla del servidor.
- **`crates/medsys-server/src/error.rs:120-136`**: Función `not_found_fallback` que captura cualquier URI inexistente y responde con un `OperationOutcome` canónico con severidad `"error"` y código `"not-found"`.

### C. Respuesta HTTP 422 (Unprocessable Entity) (EXISTE)
- **`crates/medsys-server/src/error.rs:39-42`**:
  ```rust
  ServerError::Domain(MedSysError::ValidationError(msg))
  | ServerError::Domain(MedSysError::InvalidRule(msg)) => {
      (StatusCode::UNPROCESSABLE_ENTITY, "invalid", msg.clone())
  }
  ```
  Mapea errores de validación de negocio y reglas inválidas a HTTP 422 con issue code `"invalid"`.
- **`crates/medsys-server/src/error.rs:43-47`**:
  ```rust
  ServerError::Domain(MedSysError::MissingField { field, resource }) => (
      StatusCode::UNPROCESSABLE_ENTITY,
      "required",
      format!("Campo requerido '{field}' faltante para el recurso '{resource}'"),
  )
  ```
  Mapea omisión de campos obligatorios a HTTP 422 con issue code `"required"`.
- **`crates/medsys-server/src/error.rs:48-52`**:
  ```rust
  ServerError::Domain(MedSysError::ProcessingError(msg)) => (
      StatusCode::UNPROCESSABLE_ENTITY,
      "processing",
      format!("Error de procesamiento o corrupción de datos clínicos: {msg}"),
  )
  ```
  Mapea fallas de procesamiento o corrupción de datos a HTTP 422 con issue code `"processing"`.
- **`crates/medsys-server/src/error.rs:108-112`**: Retorna el `StatusCode::UNPROCESSABLE_ENTITY` (422) junto a la cabecera `FHIR_JSON_CONTENT_TYPE` y el cuerpo JSON con el recurso `OperationOutcome`.
- **`crates/medsys-server/tests/server_integration.rs:111, 127, 136`**: Pruebas de integración que comprueban que las respuestas devueltas tienen `StatusCode::UNPROCESSABLE_ENTITY`.
