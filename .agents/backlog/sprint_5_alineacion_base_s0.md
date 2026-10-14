# SPRINT 5: Saneamiento del Esquema Base (`S0`), Fin de `fallback_value`, `Observation` 1:4 y `OperationOutcome` Canónico

## Objetivo del Sprint
Corregir todas las discrepancias detectadas en la auditoría del código actual sobre el esquema base `S0`:
1. Eliminar por completo `fallback_value` y `use_data_absent_reason` del código Rust y de `mapping_rules.yaml` (cumpliendo la política *Fail-Closed* y omisión controlada `optional: true` de los Capítulos I, II y III).
2. Completar las **4 reglas de `Observation`** en `mapping_rules.yaml` (`blood_pressure_panel` `85354-9`, `body_temperature` `8310-5`, `heart_rate` `8867-4` y `respiratory_rate` `9279-1` con `optional: true`) y restituir el mapeo opcional de `telefono_contacto` en `Patient.telecom[0]`.
3. Implementar `transform_observation_respiratory_rate` en `medsys-core`, la validación con expresión regular oficial de la CURP (categoría de falla `T5`) y exponer los 4 subtipos de `Observation` en `medsys-server`.
4. Homologar los códigos `IssueType` de `OperationOutcome` en `crates/medsys-server/src/error.rs` (`400 -> invalid`, `404 -> not-found`, `422 -> processing`, `500/503 -> exception/transient`) y dejar el 100 % de `cargo test --workspace` en verde.

---

## Lista de Tareas Atómicas

- [x] **Tarea 5.1: Actualización de `mapping_rules.yaml` a `v1.2.0` y Purga de `fallback_value` en Modelos**
  - **Archivos a modificar:**
    - `mapping_rules.yaml`
    - `crates/medsys-core/src/model/mapping.rs`
    - `crates/medsys-core/src/model/fhir_helpers.rs`
    - `crates/medsys-core/src/engine/evaluator.rs`
  - **Instrucciones exactas:**
    1. En `mapping_rules.yaml`:
       - Actualizar `version: "1.2.0"`.
       - En `Patient` (`tbl_pacientes`):
         - Mantener `curp` (`identifier[0].value`, `system: "urn:oid:2.16.840.1.113883.4.629"`, `use: "official"`).
         - Mantener `primer_nombre` (`name[0].given[0]`), `segundo_nombre` (`name[0].given[1]`, `optional: true`), `apellido_paterno` (`name[0].family`, `combine_with: "apellido_materno"`, `combine_format: "{apellido_paterno} {apellido_materno}"`), `fecha_nacimiento` (`birthDate`, `transform: "date_iso8601"`).
         - En `sexo_biologico` (`gender`, diccionario `M/F/I`): **eliminar** `use_data_absent_reason: true` (es un campo obligatorio; si es nulo o está fuera de catálogo debe fallar cerrado con HTTP 422).
         - **Agregar** el mapeo de `telefono_contacto` como opcional (según Cap. III Tabla 8):
           ```yaml
           - source_column: "telefono_contacto"
             target_path: "telecom[0].value"
             system: "phone"
             use: "mobile"
             optional: true
           ```
       - En `Observation` (`tbl_signos_vitales`), definir exactamente los **4 bloques** canónicos:
         - **Recurso 3A:** `observation_type: "blood_pressure_panel"`, LOINC `85354-9` con componentes `8480-6` (`presion_sistolica`) y `8462-4` (`presion_diastolica`) en `mm[Hg]`.
         - **Recurso 3B:** `observation_type: "body_temperature"`, LOINC `8310-5` (`temperatura_celsius`), unidad `Cel`.
         - **Recurso 3C:** `observation_type: "heart_rate"`, perfil `http://hl7.org/fhir/StructureDefinition/heartrate`, LOINC `8867-4` (`display: "Heart rate"`), columna `frecuencia_cardiaca`, `target_path: "valueQuantity.value"`, `unit: "/min"`, `code: "/min"`, `system: "http://unitsofmeasure.org"`.
         - **Recurso 3D:** `observation_type: "respiratory_rate"`, perfil `http://hl7.org/fhir/StructureDefinition/resprate`, LOINC `9279-1` (`display: "Respiratory rate"`), columna `frecuencia_respiratoria`, `target_path: "valueQuantity.value"`, `unit: "/min"`, `code: "/min"`, `system: "http://unitsofmeasure.org"`, con **`optional: true`**.
       - En `Condition` (`tbl_diagnosticos`):
         - En `tipo_diagnostico` (`verificationStatus.coding[0].code`, diccionario `CONFIRMADO: confirmed`, `PRESUNTIVO: provisional`): **eliminar** `fallback_value: "provisional"` y **agregar** `optional: true`.
         - En `fecha_diagnostico` (`recordedDate`): **eliminar** `use_data_absent_reason: true`.
    2. En `crates/medsys-core/src/model/mapping.rs` y `crates/medsys-core/src/engine/evaluator.rs`:
       - Eliminar el campo `pub fallback_value: Option<String>` de `FieldMapping` y toda lógica que inyecte valores de reemplazo inventados en `evaluator.rs` (`parse_date_iso8601_with_fallback`, `parse_datetime_iso8601_with_fallback`).
       - En `evaluator.rs`, cuando una columna tenga `optional: true` y su valor en la fila sea `Null` o cadena vacía `""`, retornar `Ok(())` sin modificar `target_json`. Si la columna tiene un valor no nulo pero **falla** en `apply_dictionary` o `apply_transform` (incluso si `optional: true`), retornar `Err(MedSysError::ProcessingError(...))` (política *Fail-Closed*: opcional permite ausencia `NULL`, pero nunca permite datos corruptos).
       - En `evaluator.rs`, cuando `rule.system` sea `"phone"` sobre `telecom[0].value`, inyectar `"system": "phone"` en `telecom[0].system` y `"use": "mobile"` en `telecom[0].use`.
  - **Criterio de Aceptación:** `mapping_rules.yaml` contiene exactamente 7 bloques de recursos (1 `Patient`, 1 `Encounter`, 4 `Observation`, 1 `Condition`) y 0 apariciones de `fallback_value`.

- [x] **Tarea 5.2: Validación Regex de CURP (`T5`), `Patient.telecom`, `Condition` Fail-Closed y `transform_observation_respiratory_rate` en `medsys-core`**
  - **Archivos a modificar:**
    - `Cargo.toml` y `crates/medsys-core/Cargo.toml` (agregar `regex = "1.10"` si no está presente, o validador determinista de 18 caracteres de RENAPO)
    - `crates/medsys-core/src/engine/transform.rs`
    - `crates/medsys-core/src/lib.rs`
  - **Instrucciones exactas:**
    1. **Validación estricta de CURP (`T5`) en `transform_patient` (`transform.rs`) y `evaluator.rs`:**
       - Implementar la función pública `pub fn is_valid_curp(curp: &str) -> bool` que verifique exactamente la estructura oficial de 18 caracteres de RENAPO:
         - 4 letras iniciales `[A-Z]{4}`, 6 dígitos de fecha `YYMMDD` (con mes `01..=12` y día `01..=31`), sexo `[HMX]`, clave de entidad federativa de 2 letras (`AS, BC, BS, CC, CL, CM, CS, CH, DF, DG, GT, GR, HG, JC, MC, MN, MS, NT, NL, OC, PL, QT, QR, SP, SL, SR, TC, TS, TL, VZ, YN, ZS, NE`), 3 consonantes internas `[B-DF-HJ-NP-TV-Z]{3}`, 1 carácter de homoclave `[A-Z0-9]` y 1 dígito verificador `[0-9]`.
       - Si `paciente.curp` está vacío o `!is_valid_curp(&paciente.curp)`, retornar `Err(MedSysError::ValidationError(format!("T5 Identificador CURP inválido: '{}'", paciente.curp)))`.
    2. **Mapeo opcional de `telefono_contacto` en `transform_patient`:**
       - Si `paciente.telefono_contacto` es `Some(tel)` y `!tel.trim().is_empty()`, construir `ContactPoint` de `helios_fhir::r4` con `system: Some(fhir_code("phone"))`, `value: Some(fhir_string(tel.trim()))`, `r#use: Some(fhir_code("mobile"))` y asignarlo a `patient.telecom = Some(vec![contact_point])`. Si es `None` o vacío, dejar `telecom: None`.
    3. **Validación estricta de `sexo_biologico` en `transform_patient`:**
       - Si `sexo_biologico` es `None`, vacío o distinto de `"M"`, `"F"`, `"I"`, retornar `Err(MedSysError::ProcessingError(...))` (HTTP 422).
    4. **Implementación de `transform_observation_respiratory_rate` en `transform.rs`:**
       - Firma: `pub fn transform_observation_respiratory_rate(signo: &LegacySignoVital, _rules: Option<&MappingRules>) -> Result<Option<Observation>>`.
       - Si `signo.frecuencia_respiratoria` es `None`, retornar `Ok(None)` (degradación limpia por omisión `optional: true`).
       - Si es `Some(rr)` con `rr <= 0`, retornar `Err(MedSysError::ProcessingError("Frecuencia respiratoria fuera de rango fisiológico positivo".into()))`.
       - Si es `Some(rr)` válido, construir `Observation` con `id: Some(fhir_string(format!("resp-{}", signo.id_signo)))`, `status: fhir_code("final")`, categoría `vital-signs`, código LOINC `9279-1` (`display: "Respiratory rate"`, `text: "Frecuencia respiratoria"`), `subject: Patient/{id_paciente}`, `encounter: Encounter/{id_consulta}`, `effectiveDateTime` y `valueQuantity` con valor `rr`, `unit: "/min"`, `code: "/min"`, `system: "http://unitsofmeasure.org"`.
       - Exportar `transform_observation_respiratory_rate` e `is_valid_curp` en `crates/medsys-core/src/lib.rs`.
    5. **Política `optional: true` + *Fail-Closed* en `transform_condition`:**
       - Si `diagnostico.tipo_diagnostico` es `None` o cadena vacía `""`, establecer `verification_status: None` en el struct `Condition` (omisión limpia dado que en FHIR R4 `Condition.verificationStatus` tiene cardinalidad `0..1`).
       - Si `diagnostico.tipo_diagnostico` es `Some(other)` y no está en el diccionario (`CONFIRMADO`, `PRESUNTIVO`), retornar `Err(MedSysError::ProcessingError(format!("T3 Código fuera de catálogo en tipo_diagnostico: '{other}'")))`.
  - **Criterio de Aceptación:** `cargo check -p medsys-core` compila sin errores ni advertencias.

- [x] **Tarea 5.3: Exposición de `Observation` 1:4 en `medsys-server` y Homologación de Códigos `OperationOutcome`**
  - **Archivos a modificar:**
    - `crates/medsys-server/src/handlers/observation.rs`
    - `crates/medsys-server/src/handlers/legacy.rs`
    - `crates/medsys-server/src/error.rs`
  - **Instrucciones exactas:**
    1. En `crates/medsys-server/src/handlers/observation.rs`:
       - En `get_observation`: soportar tanto prefijos como sufijos para los 4 subtipos de signos vitales:
         - Presión arterial: `bp-{id}` o `{id}-bp` o `{id}` numérico puro.
         - Temperatura: `temp-{id}` o `{id}-temp`.
         - Frecuencia cardíaca: `hr-{id}` o `{id}-hr`.
         - Frecuencia respiratoria: `resp-{id}`, `rr-{id}`, `{id}-resp` o `{id}-rr`. Si `transform_observation_respiratory_rate` devuelve `Ok(None)` (porque `frecuencia_respiratoria` es `NULL` en esa fila), retornar `Err(ServerError::NotFound(format!("La consulta de signos vitales '{id}' no registra frecuencia respiratoria (campo opcional NULL).")))` (HTTP 404 con `OperationOutcome`).
       - En `list_observations`: por cada fila de `signos`, agregar `obs_bp`, `obs_temp`, `obs_hr` y, si `transform_observation_respiratory_rate` devuelve `Ok(Some(obs_resp))`, agregar también `obs_resp` al vector `resources`.
    2. En `crates/medsys-server/src/handlers/legacy.rs` (`get_legacy_patient_full`):
       - Incluir en `fhir_observations` los 4 recursos (`bp`, `temp`, `hr` y `resp` cuando esté presente), en lugar de solo `bp` y `temp`.
    3. En `crates/medsys-server/src/error.rs`:
       - Homologar la tupla `(status, issue_code, diagnostics)` conforme al catálogo oficial `http://hl7.org/fhir/issue-type` (Cap. II §2.1.3 y Cap. III §3.3.3):
         - `ServerError::InvalidPath(msg)` $\rightarrow$ `(StatusCode::BAD_REQUEST, "invalid", msg.clone())`.
         - `ServerError::Domain(MedSysError::NotFound(msg)) | ServerError::NotFound(msg)` $\rightarrow$ `(StatusCode::NOT_FOUND, "not-found", msg.clone())`.
         - `ServerError::Domain(MedSysError::ValidationError(msg)) | ServerError::Domain(MedSysError::InvalidRule(msg)) | ServerError::Domain(MedSysError::MissingField { .. }) | ServerError::Domain(MedSysError::ProcessingError(msg)) | ServerError::Domain(MedSysError::TransformationError(msg))` $\rightarrow$ `(StatusCode::UNPROCESSABLE_ENTITY, "processing", ...)`.
         - `ServerError::Domain(MedSysError::DatabaseError(msg)) | ServerError::Database(_)` $\rightarrow$ `(StatusCode::SERVICE_UNAVAILABLE, "transient", ...)` (o `StatusCode::INTERNAL_SERVER_ERROR` con `"exception"` para fallos internos).
  - **Criterio de Aceptación:** Todas las respuestas HTTP 400 tienen `"code": "invalid"`, todas las 404 tienen `"code": "not-found"` y todas las 422 tienen `"code": "processing"`.

- [x] **Tarea 5.4: Actualización Integral de Pruebas Unitarias e Integración de `medsys-core`, `medsys-db` y `medsys-server`**
  - **Archivos a modificar:**
    - `crates/medsys-core/src/engine/mod.rs`
    - `crates/medsys-db/tests/persistence_integration.rs`
    - `crates/medsys-server/tests/e2e_interoperability.rs`
    - `crates/medsys-server/tests/server_integration.rs`
  - **Instrucciones exactas:**
    1. En `crates/medsys-core/src/engine/mod.rs`:
       - Actualizar `test_parse_embedded_specification_yaml` para verificar `assert_eq!(rules.version, "1.2.0")` y `assert_eq!(rules.resources.len(), 7)`.
       - Actualizar `test_observation_resource_mappings` para verificar los **4 bloques de `Observation`** (`blood_pressure_panel` `85354-9`, `body_temperature` `8310-5`, `heart_rate` `8867-4` y `respiratory_rate` `9279-1` con `optional: true`).
       - Actualizar `test_patient_resource_mappings` y `test_tarea_2_1_transform_patient_canonical` para validar que `telefono_contacto` se mapea a `telecom[0].value` cuando está presente y se omite cuando es `None`, y añadir prueba de rechazo HTTP 422 ante CURP malformada (`test_patient_invalid_curp_fails_closed`).
       - Reemplazar las pruebas antiguas de `fallback_value` y `data_absent_reason` por pruebas de la política de tesis:
         - `test_transform_condition_omits_verification_status_when_tipo_diagnostico_is_none` (verifica que `tipo_diagnostico: None` produce `verification_status: None` sin error).
         - `test_transform_condition_fails_closed_when_tipo_diagnostico_invalid` (verifica que `tipo_diagnostico: Some("DUDOSO")` retorna `Err(MedSysError::ProcessingError)`).
         - `test_transform_observation_respiratory_rate_some_and_none` (verifica emisión de LOINC `9279-1` cuando es `Some(18)` y retorno de `Ok(None)` cuando es `None`).
    2. En `crates/medsys-server/tests/e2e_interoperability.rs` y `server_integration.rs`:
       - Agregar prueba del endpoint `/fhir/r4/Observation/resp-1` y `/fhir/r4/Observation/1-resp`.
       - Actualizar aserciones de `OperationOutcome` para verificar `"code": "invalid"` en HTTP 400 y `"code": "processing"` en HTTP 422.
  - **Criterio de Aceptación:** Ejecutar `cargo test --workspace` y `cargo clippy --workspace --all-targets -- -D warnings`. **100 % de las pruebas deben pasar (0 fallos, 0 warnings).**
