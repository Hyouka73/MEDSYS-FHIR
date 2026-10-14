# REGLAS DEL SISTEMA Y DESARROLLO — MedSys-FHIR (v1.2.0-eval)

## 1. Contexto y Objetivos del Proyecto
MedSys-FHIR es un middleware de traducción clínica en salud desarrollado en Rust para la tesis de licenciatura en Ingeniería en Desarrollo y Tecnologías de Software de **Carlos Eduardo Iglesias de la Cruz** y **Alexis Andrey Gálvez Roblero** (UNACH, 2026).
- **Título Oficial:** *Desarrollo y evaluación de un middleware en Rust basado en reglas declarativas para la interoperabilidad de bases de datos clínicas relacionales con el estándar HL7 FHIR R4 en entornos de recursos limitados*.
- **Propósito:** Traducir en tiempo real registros clínicos almacenados en bases de datos relacionales heredadas (PostgreSQL 16 en Docker) hacia 4 recursos del estándar HL7 FHIR Release 4 (R4) v4.0.1: `Patient`, `Encounter`, `Observation` y `Condition`, más `OperationOutcome` para el reporte normativo de errores.
- **Enfoque Arquitectónico:** Arquitectura de 4 capas desacoplada mediante patrones *Adapter*, *Facade* y *Data Mapper*, gobernada por el lenguaje declarativo `mapping_rules.yaml` (DSL v1.2) evaluado sobre 4 arquetipos relacionales (`S0` base NOM-004, `S1` sábana plana desnormalizada, `S2` catálogos externos normalizados con `JOIN` y `S3` legado con deuda técnica: fechas en texto, presión arterial concatenada `"120/80"`, peso en libras y claves primarias de texto).
- **Restricción Ético-Legal:** Los datos y esquemas son arquetipos sintéticos deterministas (`SEED=20260930`) deducidos de la NOM-004-SSA3-2012 y NOM-024-SSA3-2012. Prohibido operar sobre datos de pacientes reales (LGPDPPSO / LFPDPPP).

---

## 2. Reglas Innegociables del Dominio Clínico y Normativo (Capítulos I, II y III)

1. **Principio Rector de Fidelidad Clínica (Cero Invención de Datos):**
   - *"El middleware transforma; no corrige ni completa. Nunca sustituye un dato clínico ausente o corrupto por un valor inventado."*
   - **PROHIBIDO** el uso de `fallback_value` o `use_data_absent_reason` para enmascarar corrupción de datos o inventar estados diagnósticos (`"provisional"`), fechas (`1970-01-01`) o géneros.
   - **Omisión controlada (`optional: true`):** Si una columna marcada con `optional: true` en el YAML contiene `NULL` o cadena vacía en SQL (ej. `segundo_nombre`, `apellido_materno`, `telefono_contacto`, `frecuencia_respiratoria`, `tipo_diagnostico`), el motor simplemente omite esa propiedad (o ese recurso `Observation` en el caso de frecuencia respiratoria nula) sin abortar la transacción.
   - **Rechazo por seguridad (*Fail-Closed*):** Si un campo obligatorio (`optional: false` por defecto) es `NULL`/vacío, o si cualquier campo (obligatorio u opcional) contiene un valor corrupto, malformado, fuera del diccionario YAML, con llave foránea padre inexistente o con CURP inválida, el motor **aborta inmediatamente** la generación del recurso y responde **HTTP `422 Unprocessable Entity`** con un recurso `OperationOutcome` oficial.

2. **Expansión 1:4 de Signos Vitales (`Observation`):**
   - Cada fila de `tbl_signos_vitales` se desacopla hasta en **4 recursos `Observation` independientes** bajo la categoría `vital-signs` (`http://terminology.hl7.org/CodeSystem/observation-category`):
     1. **Presión Arterial (`{id}-bp` / `bp-{id}`):** Panel compuesto LOINC `85354-9` (*Blood pressure panel with all children optional*) con `component[0]` Sistólica (LOINC `8480-6`, UCUM `mm[Hg]`, unit `mmHg`) y `component[1]` Diastólica (LOINC `8462-4`, UCUM `mm[Hg]`, unit `mmHg`).
     2. **Temperatura Corporal (`{id}-temp` / `temp-{id}`):** LOINC `8310-5` (*Body temperature*), `valueQuantity` en UCUM `Cel`.
     3. **Frecuencia Cardíaca (`{id}-hr` / `hr-{id}`):** LOINC `8867-4` (*Heart rate*), `valueQuantity` en UCUM `/min`.
     4. **Frecuencia Respiratoria (`{id}-resp` / `resp-{id}`):** LOINC `9279-1` (*Respiratory rate*), `valueQuantity` en UCUM `/min`, marcada con `optional: true`. Si `frecuencia_respiratoria` es `NULL` en la fila SQL (10 % del dataset = 250 filas), esta cuarta observación no se emite. Por tanto, las 2,500 filas producen exactamente **9,750 recursos `Observation`** (`7,500 + 2,250`) y el censo total del banco de pruebas es de **16,250 recursos FHIR R4** (`1,000 Patient + 2,500 Encounter + 9,750 Observation + 3,000 Condition`).

3. **Identificadores Oficiales y Catálogos Terminológicos:**
   - **`Patient`:**
     - Identificador oficial nacional: `curp` en `identifier[0].value` con `system: "urn:oid:2.16.840.1.113883.4.629"` y `use: "official"`. Debe validarse contra la expresión regular oficial de RENAPO de 18 caracteres: `^[A-Z]{4}\d{6}[HMX][A-Z]{2}[B-DF-HJ-NP-TV-Z]{3}[A-Z0-9]\d$`. Si no cumple la regex, rechazar con HTTP 422 (`T5`).
     - Teléfono de contacto: `telefono_contacto` mapeado a `telecom[0].value` con `system: "phone"`, `use: "mobile"` y `optional: true`.
     - Segundo nombre: `segundo_nombre` mapeado a `name[0].given[1]` con `optional: true`.
     - Apellidos: `apellido_paterno` combinado con `apellido_materno` (`optional: true`) en `name[0].family`.
   - **`Encounter`:**
     - Clase ambulatoria obligatoria `AMB` (`http://terminology.hl7.org/CodeSystem/v3-ActCode`).
     - Cédula profesional del médico tratante en `participant[0].individual.identifier.value` bajo `http://cedulaprofesional.sep.gob.mx`.
   - **`Condition`:**
     - Código diagnóstico en `code.coding[0].code` bajo `http://hl7.org/fhir/sid/icd-10`.
     - `tipo_diagnostico` mapeado a `verificationStatus.coding[0].code` (`CONFIRMADO -> confirmed`, `PRESUNTIVO -> provisional`) con `optional: true` y **sin `fallback_value`**. Si `tipo_diagnostico` es `NULL`, se omite `verificationStatus`. Si tiene un string distinto de `CONFIRMADO` o `PRESUNTIVO`, falla cerrado con HTTP 422 (`T3`).

4. **Correspondencia Canónica de Códigos `OperationOutcome` (`http://hl7.org/fhir/issue-type`):**
   - Toda respuesta de error debe incluir cabecera `Content-Type: application/fhir+json; charset=utf-8` y cuerpo `OperationOutcome` con `severity: "error"` y el código exacto de `IssueType`:
     - **HTTP `400 Bad Request`** (parámetro de ruta malformado): `issue[0].code = "invalid"`.
     - **HTTP `404 Not Found`** (registro inexistente en BD o ruta desconocida): `issue[0].code = "not-found"`.
     - **HTTP `422 Unprocessable Entity`** (violación de reglas `T1` a `T5`: campo obligatorio nulo, tipo/fecha corruptos, valor fuera de diccionario, llave foránea padre rota, sintaxis/CURP inválida): `issue[0].code = "processing"`.
     - **HTTP `500 Internal Server Error` / `503 Service Unavailable`** (fallo de conexión o timeout de BD): `issue[0].code = "exception"` o `"transient"`.

---

## 3. Reglas de Codificación en Rust, SQLx y Docker
1. **Cero `unwrap()` / `expect()` en Producción:** Toda función falible en `crates/medsys-core`, `crates/medsys-db` y `crates/medsys-server` debe retornar `Result<T, MedSysError>` o `Result<T, ServerError>`.
2. **Seguridad y Solo Lectura en Base de Datos (`medsys-db`):**
   - Conexión por defecto mediante el rol restringido `medsys_ro` (solo permiso `SELECT`).
   - Toda lectura clínica debe ejecutarse bajo transacción de solo lectura `BEGIN TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY`.
   - Prohibida la concatenación de parámetros de usuario en SQL: usar siempre placeholders `$1`, `$2`.
   - Los nombres de tablas y columnas leídos desde el archivo YAML (`source_table`, `primary_key`, `source_column`, `lookup.table`, `lookup.key`, `lookup.value_column`) deben validarse al arranque del servidor contra una **lista blanca (*allowlist*)** estricta de identificadores SQL válidos (*Fail-Fast*).
3. **Límites de Hardware en Contenedores (`cgroups v2`):**
   - `medsys-server`: `cpus: '1.0'`, `mem_limit: 256m` (umbral de tesis: RSS $\le 150\text{ MB}$, CPU $\le 50\%$, latencia $p_{95} \le 200\text{ ms}$ a 50 VUs).
   - `postgres:16-alpine`: `cpus: '1.0'`, `mem_limit: 512m`.

---

## 4. Reglas de Continuidad y Autonomía de Agentes
1. **Protocolo de Inicio:** Antes de modificar cualquier archivo, leer siempre `STATE.md`, `BACKLOG.md` y el archivo del Sprint correspondiente en `.agents/backlog/sprint_X_*.md`.
2. **Protocolo de Validación Obligatoria:** Ninguna tarea puede marcarse como `[x]` sin antes ejecutar y aprobar:
   - `cargo check --workspace`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test --workspace`
   - (Y cuando la tarea involucre scripts Python/SQL/k6, el comando de verificación indicado en el criterio de aceptación de esa tarea).
3. **Protocolo de Cierre:** Al terminar una tarea o un Sprint, actualizar `BACKLOG.md`, el archivo `.agents/backlog/sprint_X_*.md` y `STATE.md` documentando exactamente qué archivos se crearon/modificaron y cuál es la siguiente tarea pendiente.
