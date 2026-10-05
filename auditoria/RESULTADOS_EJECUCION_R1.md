# EVIDENCIA DE EJECUCIÓN DEL REPOSITORIO — RONDA 1 (R1)
**Proyecto:** MedSys-FHIR — Middleware de Mediación Relacional a HL7 FHIR R4 (Tesis de Licenciatura UNACH)  
**Rama auditada:** `main`  
**Fecha de corte y ejecución:** 2026-10-05 13:00 CST (19:00 UTC)  
**Propósito:** Producir evidencia empírica verificable de ejecución sin modificar el código fuente para que los auditores técnicos puedan contrastar la hipótesis de investigación contra el comportamiento real del sistema.

---

## 1. IDENTIDAD

Esta sección documenta los metadatos de control de versiones y el conjunto de herramientas de software instaladas en el entorno donde se llevaron a cabo las mediciones.

### 1.1 Metadatos del Repositorio Git
- **URL Remota:**
  ```text
  origin  https://github.com/Hyouka73/MEDSYS-FHIR.git (fetch)
  origin  https://github.com/Hyouka73/MEDSYS-FHIR.git (push)
  ```
  *Comando:* `git remote -v` | *Salida Exit Code:* 0

- **Rama Actual:** `main`  
  *Comando:* `git rev-parse --abbrev-ref HEAD` | *Salida Exit Code:* 0

- **Hash de HEAD:** `69a592696241ba987068116db3a1000a46ad3cb5`  
  *Comando:* `git rev-parse HEAD` | *Salida Exit Code:* 0

- **Estado de Git (`git status`):**
  ```text
  On branch main
  Your branch is up to date with 'origin/main'.

  Untracked files:
    (use "git add <file>..." to include in what will be committed)
          auditoria/QAP-handoff-orquestador-v2.md
          auditoria/gestor_de_prompts_y_checklist.html

  nothing added to commit but untracked files present (use "git add" to track)
  ```
  *Comando:* `git status` | *Salida Exit Code:* 0  
  *Observación:* El repositorio no presenta modificaciones sobre archivos rastreados; existen dos archivos de documentación de auditoría sin seguimiento previo (`QAP-handoff-orquestador-v2.md` y `gestor_de_prompts_y_checklist.html`).

### 1.2 Versiones de Herramientas y Compiladores
- **Rustc:** `rustc 1.98.1 (48a229cea 2026-09-01)`
- **Cargo:** `cargo 1.98.1 (797e8a9bc 2026-08-05)`
- **Docker:** `Docker version 29.5.3, build d1c06ef`
- **Docker Compose:** `Docker Compose version v5.1.4`
- **k6:** `k6.exe v2.2.0 (commit/00a9a1b7f5, go1.26.5, windows/amd64)`
- **Java:** `java version "1.8.0_361" Java(TM) SE Runtime Environment (build 1.8.0_361-b09)`
- **HL7 Validator CLI (`validator-cli`):** No disponible en el PATH del sistema ni como archivo JAR dentro del repositorio.

---

## 2. ENTORNO

Información de la plataforma de hardware y virtualización del sistema anfitrión:

- **Modelo de Procesador (CPU):** `AMD Ryzen 7 5700U with Radeon Graphics`
- **Topología de CPU:** 8 núcleos físicos, 16 procesadores lógicos
- **Memoria RAM Física:** 14,521,784 KB (~14.5 GB) disponible a nivel de sistema operativo
- **Sistema Operativo Anfitrión:** `Microsoft Windows 11 Home` (Versión `10.0.26200`, 64 bits)
- **Motor de Contenedores:** `Docker Desktop 29.5.3` operando sobre backend WSL2 (`docker-desktop` distro)
- **Topología de Coexistencia:** Sí. La herramienta de carga (`k6`), la base de datos PostgreSQL 16 (`medsys_fhir_postgres`) y el contenedor del middleware (`medsys-server`) compartieron los recursos del mismo anfitrión físico durante las pruebas.

---

## 3. PRUEBAS DE SOFTWARE (SUITE DEL WORKSPACE)

Se ejecutó la suite completa de pruebas unitarias y de integración para todos los crates del workspace mediante Cargo.

- **Comando literal:** `cargo test --workspace --no-fail-fast`
- **Código de salida:** `1` (Fallo controlado en crate `medsys-core`)

### 3.1 Desglose por Crate y Tipo de Prueba

| Crate / Módulo | Tipo de Prueba | Pasadas | Falladas | Ignoradas | Total | Estado |
| :--- | :--- | :---: | :---: | :---: | :---: | :--- |
| **`medsys-core`** | Pruebas Unitarias (`src/lib.rs`) | 22 | 2 | 0 | 24 | **FALLIDO** |
| **`medsys-core`** | Doc-tests | 0 | 0 | 0 | 0 | APROBADO |
| **`medsys-db`** | Pruebas Unitarias (`src/lib.rs`) | 19 | 0 | 0 | 19 | **APROBADO** |
| **`medsys-db`** | Integración Persistencia (`persistence_integration.rs`) | 3 | 0 | 0 | 3 | **APROBADO** |
| **`medsys-db`** | Doc-tests | 0 | 0 | 0 | 0 | APROBADO |
| **`medsys-server`** | Pruebas Unitarias (`src/lib.rs` / `src/main.rs`) | 0 | 0 | 0 | 0 | APROBADO |
| **`medsys-server`** | Integración E2E (`e2e_interoperability.rs`) | 8 | 0 | 0 | 8 | **APROBADO** |
| **`medsys-server`** | Integración Servidor (`server_integration.rs`) | 5 | 0 | 0 | 5 | **APROBADO** |
| **`medsys-server`** | Doc-tests | 0 | 0 | 0 | 0 | APROBADO |
| **TOTAL WORKSPACE** | **Todas las pruebas** | **57** | **2** | **0** | **59** | **FALLO (2/59)** |

### 3.2 Comparación con la Línea Base Declarada
- **Línea base declarada en protocolo:** 59 pruebas distribuidas en `24 + 19 + 3 + 8 + 5`.
- **Conteo real ejecutado:** 59 pruebas coincidentes con la distribución cuantitativa exacta:
  - 24 pruebas en `medsys-core`
  - 19 pruebas en `medsys-db` (unitarias)
  - 3 pruebas en `medsys-db` (integración `tests/persistence_integration.rs`)
  - 8 pruebas en `medsys-server` (integración `tests/e2e_interoperability.rs`)
  - 5 pruebas en `medsys-server` (integración `tests/server_integration.rs`)
- **Diferencia observada:** Cuantitativamente 0 de diferencia en número de pruebas (59 ejecutadas). Cualitativamente, 2 pruebas fallaron en `medsys-core`.

### 3.3 Detalle Literal de Pruebas Fallidas (Sin modificar el código)

#### Fallo 1: `engine::tests::test_parse_embedded_specification_yaml`
- **Archivo:** `crates\medsys-core\src\engine\mod.rs:97:9`
- **Mensaje literal de aserción:**
  ```text
  thread 'engine::tests::test_parse_embedded_specification_yaml' (23892) panicked at crates\medsys-core\src\engine\mod.rs:97:9:
  assertion `left == right` failed
    left: 6
   right: 5
  ```
- **Causa técnica evidenciada en el código:** En la Fase 0 se introdujo la regla declarativa de Frecuencia Respiratoria en `mapping_rules.yaml` (D-016), elevando los recursos declarados a 6 (`Patient`, `Encounter`, `Observation (bp)`, `Observation (temp)`, `Observation (hr)`, `Condition`), pero la aserción de prueba unitaria conservaba `assert_eq!(rules.resources.len(), 5)`.

#### Fallo 2: `engine::tests::test_observation_resource_mappings`
- **Archivo:** `crates\medsys-core\src\engine\mod.rs:171:9`
- **Mensaje literal de aserción:**
  ```text
  thread 'engine::tests::test_observation_resource_mappings' (23336) panicked at crates\medsys-core\src\engine\mod.rs:171:9:
  assertion `left == right` failed
    left: 3
   right: 2
  ```
- **Causa técnica evidenciada en el código:** La regla añadida de Frecuencia Respiratoria aumentó el conteo de bloques de tipo `Observation` en `mapping_rules.yaml` a 3 (`blood_pressure_panel`, `body_temperature`, `respiratory_rate`), mientras que la prueba mantenía `assert_eq!(observations.len(), 2)`.

---

## 4. INVENTARIO ESTÁTICO (INSPECCIÓN DIRECTA DEL REPOSITORIO)

### 4.1 Rutas HTTP y Recursos Servidos
Inspección de `crates/medsys-server/src/router.rs` y `crates/medsys-server/src/handlers/`:

| Ruta HTTP | Método | Handler | Recurso FHIR R4 Resultante | Código de Retorno |
| :--- | :---: | :--- | :--- | :---: |
| `/fhir/r4/Patient` | GET | `list_patients` | `Bundle` (tipo `searchset` de `Patient`) | 200 OK |
| `/fhir/r4/Patient/{id}` | GET | `get_patient` | `Patient` (o `OperationOutcome`) | 200 OK / 404 / 422 |
| `/fhir/r4/Encounter` | GET | `list_encounters` | `Bundle` (tipo `searchset` de `Encounter`) | 200 OK |
| `/fhir/r4/Encounter/{id}` | GET | `get_encounter` | `Encounter` (o `OperationOutcome`) | 200 OK / 404 / 422 |
| `/fhir/r4/Observation` | GET | `list_observations` | `Bundle` (`searchset` con tríada: bp, temp, hr) | 200 OK |
| `/fhir/r4/Observation/{id}` | GET | `get_observation` | `Observation` (según prefijo `bp-`, `temp-`, `hr-`) | 200 OK / 404 / 422 |
| `/fhir/r4/Condition` | GET | `list_conditions` | `Bundle` (tipo `searchset` de `Condition`) | 200 OK |
| `/fhir/r4/Condition/{id}` | GET | `get_condition` | `Condition` (o `OperationOutcome`) | 200 OK / 404 / 422 |
| `/health` | GET | `health_check` | JSON nativo de diagnóstico y estado de conexión BD | 200 OK |
| `/api/health` | GET | `health_check` | JSON nativo idéntico al endpoint `/health` | 200 OK |
| `/api/legacy/patients` | GET | `list_legacy_patients` | JSON de entidades relacionales legadas | 200 OK |
| `/api/legacy/patients/{id}/full` | GET | `get_legacy_patient_full` | JSON con expediente unificado relacional | 200 OK / 404 |
| `/*` (Cualquier ruta desconocida) | ANY | `not_found_fallback` | `OperationOutcome` canónico con severidad error | 404 Not Found |

### 4.2 Recursos y Campos Generados por el Motor
1. **`Patient`:**
   - Campos generados: `resourceType`, `id`, `identifier[0].value` (CURP), `identifier[0].system` (`urn:oid:2.16.840.1.113883.4.629`), `identifier[0].use` (`official`), `name[0].given[0]`, `name[0].given[1]` (opcional), `name[0].family` (concatenación paterno + materno), `birthDate` (ISO 8601), `gender` (`male`/`female`/`other` o extensión `data-absent-reason`).
2. **`Encounter`:**
   - Campos generados: `resourceType`, `id`, `identifier[0].value`, `identifier[0].system`, `status` (`finished`, `in-progress`, `cancelled`), `class.code` (`AMB`), `subject.reference` (`Patient/{id}`), `reasonCode[0].text`, `period.start`, `period.end`, `participant[0].individual.identifier.value` (Cédula profesional), `participant[0].individual.display`.
3. **`Observation` (Panel de Presión Arterial):**
   - Campos generados: `resourceType`, `id`, `status` (`final`), `category[0].coding[0].code` (`vital-signs`), `code.coding[0].code` (`85354-9`), `subject.reference`, `encounter.reference`, `effectiveDateTime`, `component[0]` (sistólica LOINC `8480-6`, valor en `mmHg`), `component[1]` (diastólica LOINC `8462-4`, valor en `mmHg`).
4. **`Observation` (Temperatura Corporal):**
   - Campos generados: `resourceType`, `id`, `status` (`final`), `category[0].coding[0].code` (`vital-signs`), `code.coding[0].code` (`8310-5`), `subject.reference`, `encounter.reference`, `effectiveDateTime`, `valueQuantity.value`, `valueQuantity.unit` (`Cel`).
5. **`Observation` (Frecuencia Cardíaca):**
   - Campos generados: `resourceType`, `id`, `status` (`final`), `category[0].coding[0].code` (`vital-signs`), `code.coding[0].code` (`8867-4`), `subject.reference`, `encounter.reference`, `effectiveDateTime`, `valueQuantity.value`, `valueQuantity.unit` (`/min`).
6. **`Condition`:**
   - Campos generados: `resourceType`, `id`, `clinicalStatus.coding[0].code` (`active`), `verificationStatus.coding[0].code` (`confirmed`/`provisional`), `subject.reference`, `encounter.reference`, `code.coding[0].code` (CIE-10), `code.coding[0].display`, `recordedDate` (o `data-absent-reason`).

#### Hallazgo Específico: Frecuencia Respiratoria en Observation
- **¿Está en `mapping_rules.yaml`?** **SÍ.** Líneas 163–195 definen el bloque `respiratory_rate` con código LOINC `9279-1` y unidad `/min`.
- **¿Se incluye en los manejadores HTTP del servidor?** **NO.**
- **Inspección de manejadores (`crates/medsys-server/src/handlers/observation.rs`):**
  - Líneas 40–73 (`get_observation`): Solo resuelve prefijos `temp-`, `hr-`, `bp-` o ID numérico directo (mapeado por defecto a presión arterial). No existe prefijo ni rama para `rr-` ni `resprate-`.
  - Líneas 107–121 (`list_observations`): El bucle que puebla el `Bundle` searchset instancia exclusivamente:
    1. `transform_observation_blood_pressure`
    2. `transform_observation_temperature`
    3. `transform_observation_heart_rate`
  - **Conclusión técnica:** Ningún handler HTTP en `medsys-server` sirve ni genera el recurso `Observation` de Frecuencia Respiratoria.

### 4.3 Archivos YAML de Reglas
- **Archivos localizados:** 1 único archivo físico en el repositorio: `mapping_rules.yaml` (raíz).
- **Número de bloques de recursos:** 6 bloques (`Patient`, `Encounter`, `Observation (bp)`, `Observation (temp)`, `Observation (hr)`, `Condition`).
- **Número total de reglas de mapeo:** 37 reglas individuales de campo.

### 4.4 Archivos DDL y Tablas Definidas
1. **`schema_legado_simulado_nom004.sql`** (93 líneas):
   - Tablas creadas:
     - `tbl_pacientes` (columnas: `id_paciente`, `curp`, `primer_nombre`, `segundo_nombre`, `apellido_paterno`, `apellido_materno`, `fecha_nacimiento`, `sexo_biologico`, `telefono_contacto`, `fecha_registro`).
     - `tbl_consultas` (columnas: `id_consulta`, `id_paciente`, `cedula_medico_tratante`, `nombre_medico`, `estado_consulta`, `motivo_consulta`, `fecha_hora_inicio`, `fecha_hora_fin`, `unidad_medica`).
     - `tbl_signos_vitales` (columnas: `id_signo`, `id_consulta`, `id_paciente`, `presion_sistolica`, `presion_diastolica`, `frecuencia_cardiaca`, `frecuencia_respiratoria`, `temperatura_celsius`, `peso_kg`, `talla_cm`, `fecha_registro`).
     - `tbl_diagnosticos` (columnas: `id_diagnostico`, `id_consulta`, `id_paciente`, `codigo_cie10`, `descripcion_diagnostico`, `tipo_diagnostico`, `fecha_diagnostico`).
2. **`docker/sql/02_bulk_data.sql`** (Script masivo con inserciones deterministas para las 4 tablas).

### 4.5 Modificación de Datos fuera de Siembra (INSERT, UPDATE, DELETE, ALTER, DROP, GRANT)
- **Búsqueda en código de crates (`crates/**`):**
  - `INSERT`: 0 apariciones.
  - `UPDATE`: 0 apariciones.
  - `DELETE`: 0 apariciones.
  - `ALTER`: 0 apariciones.
  - `DROP`: 0 apariciones.
  - `GRANT`: 0 apariciones.
- **Resultado:** La capa de persistencia del middleware (`medsys-db`) opera de forma 100% de solo lectura (`SELECT`) sobre el esquema relacional legado.

### 4.6 Apariciones de `fallback_value` en el Repositorio
Grep literal en todo el árbol de código (excluyendo documentación histórica y carpetas `target/` y `.git/`):

1. `mapping_rules.yaml:212`:
   ```yaml
   fallback_value: "provisional"
   ```
2. `crates/medsys-core/src/model/mapping.rs:121`:
   ```rust
   pub fallback_value: Option<String>,
   ```
3. `crates/medsys-core/src/engine/evaluator.rs:276`:
   ```rust
   if let Some(ref fb) = rule.fallback_value {
   ```
4. `crates/medsys-core/src/engine/evaluator.rs:321`:
   ```rust
   if let Some(ref fb) = rule.fallback_value {
   ```
5. `crates/medsys-core/src/engine/evaluator.rs:353`:
   ```rust
   if let Some(ref fb) = rule.fallback_value {
   ```
6. `crates/medsys-core/src/engine/mod.rs:277`:
   ```rust
   tipo_diag.fallback_value.as_deref(),
   ```
7. `crates/medsys-core/src/engine/mod.rs:932, 973, 1019, 1057`:
   ```rust
   fallback_value: None,
   ```
8. `crates/medsys-core/src/engine/mod.rs:1240, 1255, 1275, 1296, 1321, 1338, 1350, 1362, 1366`:
   - Pruebas unitarias de regresión validando el comportamiento con y sin `fallback_value`.
9. `crates/medsys-core/src/engine/transform.rs:787-788`:
   ```rust
   // con soporte de fallback_value ante valores nulos o desconocidos
   let fallback = ver_rule.and_then(|r| r.fallback_value.as_deref());
   ```
10. `crates/medsys-core/src/engine/transform.rs:862`:
    ```rust
    "Data corruption: Campo obligatorio 'tipo_diagnostico' ausente o nulo sin fallback_value configurado".to_string(),
    ```

### 4.7 Implementación de la Política Fail-Closed
- **Núcleo de Transformación:** `crates/medsys-core/src/engine/transform.rs`:
  - Líneas 820–822 y 839–841: Ante un tipo de diagnóstico desconocido fuera de catálogo sin fallback ni `data_absent_reason`, el motor aborta la transformación y retorna `Err(MedSysError::ProcessingError("Data corruption: Tipo de diagnóstico desconocido '{other}'"))`.
  - Líneas 861–864: Ante un valor nulo o ausente en campos obligatorios sin regla de omisión válida, retorna `Err(MedSysError::ProcessingError("Data corruption: Campo obligatorio ... ausente o nulo"))`.
- **Capa Servidor HTTP:** `crates/medsys-server/src/error.rs`:
  - Líneas 48–52: `MedSysError::ProcessingError` se captura en el trait `IntoResponse` y se mapea a código HTTP `422 Unprocessable Entity`, serializando un recurso `OperationOutcome` canónico con `issue[0].code = "processing"` y cabecera `Content-Type: application/fhir+json; charset=utf-8`.

### 4.8 Manejo de `tipo_diagnostico` NULL
- **En `mapping_rules.yaml`:** Línea 212 especifica `fallback_value: "provisional"`.
- **En `transform.rs` (líneas 844–865):** Al recibir `None` en `tipo_diagnostico`, el motor comprueba la presencia de `fallback_value`. Como está configurado `"provisional"`, inyecta dicho código en `verificationStatus` en lugar de omitir el campo o fallar. Si se retira dicha regla de fallback, el motor aplica Fail-Closed y rechaza con HTTP 422.

### 4.9 Apariciones de CURP, `/health` y Paneles de Observabilidad
- **CURP:**
  - `schema_legado_simulado_nom004.sql:23`: `curp VARCHAR(18) UNIQUE NOT NULL`
  - `crates/medsys-core/src/model/legacy.rs:9`: `pub curp: String`
  - `crates/medsys-db/src/repository/pacientes.rs:22-29, 90-104`: Consulta `SELECT ... WHERE curp = $1` y método `find_by_curp`.
  - `crates/medsys-core/src/engine/transform.rs:44-53`: Validación obligatoria de CURP con asignación al sistema `urn:oid:2.16.840.1.113883.4.629` y `use: "official"`. Si está vacía, falla con `ProcessingError`.
- **`/health`:**
  - Servidor Axum: `crates/medsys-server/src/router.rs:34-35` registra `/health` y `/api/health`.
  - Handler: `crates/medsys-server/src/handlers/health.rs:20-43` responde JSON con `server_name`, `server_version`, `uptime_seconds`, `database_status` y `fhir_version`.
  - Dockerfile: Línea 53 define `HEALTHCHECK ... CMD curl -f http://localhost:3000/health || exit 1`.
- **Paneles de Observabilidad:**
  - Carpeta `dashboard/`: Aplicación frontend en React 19 + Vite + Tailwind CSS (`dashboard/src/components/MetricsPanel.jsx`) diseñada como consola de supervisión de `/health`, latencia de peticiones e inspección comparativa de datos relacionales vs FHIR.

### 4.10 Dependencias por Crate (Cargo.toml)

#### Workspace (`Cargo.toml` raíz)
- `tokio = { version = "1.38", features = ["full"] }`
- `serde = { version = "1.0", features = ["derive"] }`
- `serde_json = "1.0"`
- `serde_yaml = "0.9"`
- `helios-fhir = { version = "0.2", features = ["R4"] }`
- `thiserror = "1.0"`
- `tracing = "0.1"`
- `tracing-subscriber = "0.3"`
- `chrono = { version = "0.4", features = ["serde"] }`
- `rust_decimal = "1.43"`
- `sqlx = { version = "0.8", default-features = false, features = ["runtime-tokio", "tls-rustls", "postgres", "chrono", "rust_decimal", "macros"] }`
- `dotenvy = "0.15"`
- `axum = "0.8"`
- `tower = "0.5"`
- `tower-http = { version = "0.6", features = ["cors", "trace"] }`

#### `crates/medsys-core/Cargo.toml`
- `serde`, `serde_yaml`, `serde_json`, `thiserror`, `chrono`, `helios-fhir`, `tracing`, `rust_decimal` (heredadas del workspace)
- Dev-dependencies: `tokio`

#### `crates/medsys-db/Cargo.toml`
- `medsys-core = { path = "../medsys-core" }`
- `tokio`, `serde`, `serde_json`, `thiserror`, `tracing`, `chrono`, `rust_decimal`, `sqlx`, `dotenvy` (workspace)
- Dev-dependencies: `helios-fhir`

#### `crates/medsys-server/Cargo.toml`
- `medsys-core = { path = "../medsys-core" }`
- `medsys-db = { path = "../medsys-db" }`
- `tokio`, `serde`, `serde_json`, `thiserror`, `tracing`, `tracing-subscriber`, `axum`, `tower`, `tower-http`, `helios-fhir`, `dotenvy`, `sqlx`, `chrono` (workspace)
- Dev-dependencies: `http-body-util = "0.1"`, `tower`

### 4.11 Dockerfile y docker-compose.yml
- **`Dockerfile`:**
  - Builder: `rust:slim-bookworm` (compilación en modo `cargo build --release --bin medsys-server`)
  - Runner: `debian:bookworm-slim` (binario estripado + `mapping_rules.yaml`, usuario estándar, `HEALTHCHECK` activo en `/health`).
- **`docker-compose.yml` y `docker/docker-compose.yml`:**
  - Base de datos: `postgres:16-alpine`
  - Límites de CPU y memoria declarados para `medsys-server`:
    ```yaml
    deploy:
      resources:
        limits:
          cpus: '1.0'
          memory: 256M
    mem_limit: 256m
    cpus: 1.0
    ```

---

## 5. DATOS SINTÉTICOS Y DETERMINISMO

### 5.1 Ejecución del Generador Sintético (`scripts/generate_data.py`)
- **Semilla pseudoaleatoria:** `20260930`
- **Comprobación de determinismo:** Se ejecutó el generador en dos corridas consecutivas hacia archivos temporales independientes y se calculó el hash SHA256, comparándolo contra el archivo semilla del repositorio `docker/sql/02_bulk_data.sql`:

| Archivo / Corrida | Comando / Hash SHA-256 | Estado |
| :--- | :--- | :---: |
| Corrida 1 | `524131150636BAD7C36D6EF4A68CBADAA43702A2A3ED0D3A8C3D1BAE6959F823` | Determinista |
| Corrida 2 | `524131150636BAD7C36D6EF4A68CBADAA43702A2A3ED0D3A8C3D1BAE6959F823` | Determinista |
| `docker/sql/02_bulk_data.sql` | `524131150636BAD7C36D6EF4A68CBADAA43702A2A3ED0D3A8C3D1BAE6959F823` | **Idéntico (100%)** |

### 5.2 Conteos SQL en Base de Datos (`medsys_legacy`)
Consulta SQL ejecutada en el contenedor PostgreSQL 16:
```sql
SELECT 'tbl_pacientes' AS tabla, count(*) AS total_filas FROM tbl_pacientes 
UNION ALL SELECT 'tbl_consultas', count(*) FROM tbl_consultas 
UNION ALL SELECT 'tbl_signos_vitales', count(*) FROM tbl_signos_vitales 
UNION ALL SELECT 'tbl_diagnosticos', count(*) FROM tbl_diagnosticos;
```
*Salida literal de psql:*
```text
       tabla        | total_filas 
--------------------+-------------
 tbl_pacientes      |        1000
 tbl_consultas      |        2500
 tbl_signos_vitales |        2500
 tbl_diagnosticos   |        3000
(4 rows)
```

### 5.3 Derivación Teórica y Real de Recursos FHIR
- **`tbl_pacientes` (1,000 filas):** Genera **1,000** recursos `Patient`.
- **`tbl_consultas` (2,500 filas):** Genera **2,500** recursos `Encounter`.
- **`tbl_signos_vitales` (2,500 filas):**
  - Desacoplamiento actual en middleware (TA, Temp, FC): **7,500** recursos `Observation`.
  - Con Frecuencia Respiratoria declarada en D-016 (4 recursos/fila): **10,000** recursos `Observation`.
- **`tbl_diagnosticos` (3,000 filas):** Genera **3,000** recursos `Condition`.

---

## 6. CONTENEDORIZACIÓN DOCKER Y LÍMITES EFECTIVOS

### 6.1 Construcción e Imágenes
- **Comando:** `docker images` | *Exit Code:* 0
- **Imagen `docker-medsys-server:latest`:** Tamaño final en disco: `145 MB` (Content size: `36.8 MB`).
- **Imagen `postgres:16-alpine`:** Tamaño final en disco: `420 MB` (Content size: `117 MB`).

### 6.2 Verificación de Límites Efectivos de cgroups con `docker inspect`
- **Comando literal:**
  ```powershell
  docker inspect medsys-server --format="Memory={{.HostConfig.Memory}} NanoCpus={{.HostConfig.NanoCpus}} MemorySwap={{.HostConfig.MemorySwap}}"
  ```
- **Salida literal:**
  ```text
  Memory=268435456 NanoCpus=1000000000 MemorySwap=536870912
  ```
- **Análisis de los límites efectivos:**
  - `Memory = 268,435,456 bytes`: Exactamente **256 MB** de memoria física asignada por cgroups.
  - `NanoCpus = 1,000,000,000`: Exactamente **1.0 vCPU** asignada.
  - `MemorySwap = 536,870,912 bytes`: 512 MB totales (256 MB RAM + 256 MB Swap).
- **Conclusión:** El repositorio SÍ define formalmente y aplica los límites de 1 vCPU y 256 MB requeridos por la hipótesis de investigación.

---

## 7. MEDICIONES DE RENDIMIENTO Y TELEMETRÍA

### 7.1 Prueba de Carga Nominal (`tests/k6/load_test.js`)

- **Ruta del script:** `tests/k6/load_test.js`
- **Parámetros y Escenarios:**
  - `warmup`: Rampa de 0 a 50 VUs (1m 30s)
  - `plateau`: Meseta continua de 50 VUs (5m 00s)
  - `cooldown`: Rampa descendente de 50 a 0 VUs (30s)
  - Duración total de la corrida: 7 minutos
- **Umbrales declarados en el script:**
  - `http_req_failed`: `<0.005` (< 0.5%)
  - `fhir_success_rate`: `>=0.995` (>= 99.5%)
  - `checks`: `>=0.995`
  - `http_req_duration`: `p(95)<=200` (<= 200 ms)
  - `fhir_read_duration_ms`: `p(95)<=200`
  - `http_req_failed{phase:plateau}`: `<0.005`
  - `http_req_duration{phase:plateau}`: `p(95)<=200`
  - `plateau_fhir_latency_ms`: `p(95)<=200`
  - `plateau_fhir_success_rate`: `>=0.995`
- **Tratamiento de códigos 422 esperados:** En `load_test.js` NO se emiten códigos 422 deliberados porque la prueba evalúa exclusivamente carga nominal sobre identificadores existentes (1-1000 Patient, 1-2500 Encounter, 1-2500 Observation, 1-3000 Condition). Los códigos 422 esperados (Fail-Closed) se aíslan metodológicamente en `tests/k6/resilience_and_errors_test.js`.
- **Número de corridas:** El repositorio define **1 corrida oficial** en su pipeline (`run_benchmark_with_stats.ps1`).

#### Resultados Consolidados de la Corrida Oficial (Corrida 1 — 7 Minutos, 50 VUs en Meseta)
- **Comando:** `k6 run --env BASE_URL=http://localhost:8080 tests/k6/load_test.js`
- **Código de salida:** `0`
- **Extracto relevante de la salida k6:**
  ```text
    █ THRESHOLDS 

      checks
      ✓ 'rate>=0.995' rate=100.00%

      fhir_success_rate
      ✓ 'rate>=0.995' rate=100.00%

      http_req_duration
      ✓ 'p(95)<=200' p(95)=14.76ms

        {phase:plateau}
          ✓ 'p(95)<=200' p(95)=15.91ms

      http_req_failed
      ✓ 'rate<0.005' rate=0.00%

        {phase:plateau}
          ✓ 'rate<0.005' rate=0.00%

      plateau_fhir_latency_ms
      ✓ 'p(95)<=200' p(95)=16ms

      plateau_fhir_success_rate
      ✓ 'rate>=0.995' rate=100.00%

    █ TOTAL RESULTS 

      checks_total.......: 634624  1510.523742/s
      checks_succeeded...: 100.00% 634624 out of 634624
      checks_failed......: 0.00%   0 out of 634624

      CUSTOM
      fhir_read_duration_ms..........: avg=4.77ms   med=3ms      p(90)=8ms    p(95)=15ms   max=217ms
      fhir_success_rate..............: 100.00% 317312 out of 317312
      fhir_total_transactions........: 317312  755.261871/s
      plateau_fhir_latency_ms........: avg=4.93ms   med=3ms      p(90)=8ms    p(95)=16ms   max=217ms
      plateau_fhir_success_rate......: 100.00% 271936 out of 271936

      HTTP
      http_req_duration..............: avg=4.62ms   med=2.78ms   p(90)=7.59ms p(95)=14.76ms max=204.77ms
        { expected_response:true }...: avg=4.62ms   med=2.78ms   p(90)=7.59ms p(95)=14.76ms max=204.77ms
        { phase:plateau }............: avg=4.78ms   med=2.79ms   p(90)=8.17ms p(95)=15.91ms max=204.77ms
      http_req_failed................: 0.00%   0 out of 317312
        { phase:plateau }............: 0.00%   0 out of 271936
      http_reqs......................: 317312  755.261871/s

      EXECUTION
      iteration_duration.............: avg=220.11ms min=108.86ms med=219.28ms p(90)=300ms  p(95)=310.13ms max=655.56ms
      iterations.....................: 79328   188.815468/s
      vus............................: 1       min=0                max=50
      vus_max........................: 100     min=100              max=100
      data_received..................: 374 MB  890 kB/s
      data_sent......................: 49 MB   117 kB/s
  ```

### 7.2 Telemetría de Recursos con `docker stats` y CGroup

- **Comando de muestreo:** `docker stats medsys-server --no-stream --format "{{.CPUPerc}},{{.MemUsage}}"` ejecutado cada 5 segundos en paralelo a la corrida de 7 minutos.
- **Número de muestras recolectadas:** 64 muestras.
- **Estadísticas calculadas:**
  - **Consumo de CPU (`docker stats`):**
    - Promedio: **47.86%** (Criterio de la hipótesis: $\le 50.0\%$ de 1 vCPU $\rightarrow$ **CUMPLIDO**)
    - Percentil 95: **50.0%**
    - Máximo: **93.33%** (Pico transitorio inicial de rampa de concurrencia)
  - **Consumo de Memoria (`docker stats`):**
    - Promedio: **19.42 MiB**
    - Máximo: **20.16 MiB** (Criterio de la hipótesis: $\le 150\text{ MB}$ $\rightarrow$ **CUMPLIDO**)

### 7.3 Lectura Directa dentro del Contenedor (`VmRSS` vs `docker stats`)
- **Comando dentro del contenedor:**
  ```bash
  grep -i vmrss /proc/1/status
  cat /sys/fs/cgroup/memory.current
  ```
- **Resultados de las mediciones directas (64 muestras cada 5s):**
  - **`VmRSS` del proceso `medsys-server` (PID 1):**
    - Promedio: **11.74 MiB**
    - Máximo: **11.92 MiB**
  - **Uso de memoria según cgroup v2 (`memory.current`):**
    - Promedio: **36.84 MiB**
    - Máximo: **38.58 MiB**
- **Documentación de la discrepancia técnica:**
  - `docker stats` reportó un consumo medio de **19.42 MiB**.
  - La memoria cgroup física (`memory.current`) reportó **36.84 MiB**.
  - El espacio residente real del binario compilado en Rust (`VmRSS`) fue de tan solo **11.74 MiB**.
  - **Explicación de bajo nivel:** `docker stats` sustrae de la memoria del cgroup la memoria inactiva en búferes de archivos (`inactive_file`). Por su parte, el total de cgroup incluye páginas de mapeo de bibliotecas y caché del sistema de archivos interno de Debian. En todos los casos, el valor real de `VmRSS` (11.74 MiB) demuestra la extrema ligereza del binario en Rust, operando al **7.8%** del umbral máximo permitido por la hipótesis (150 MB).

### 7.4 Prueba de Resiliencia, Fallos y Errores (`tests/k6/resilience_and_errors_test.js`)
Se ejecutaron **3 corridas consecutivas** de la prueba de resiliencia e inyección de fallos controlados (IDs negativos, alfanuméricos, rutas 404, fallos 422 Fail-Closed):

- **Parámetros:** 10 VUs continuas durante 15 segundos por corrida.
- **Resultados comparativos de las 3 corridas:**

| Métrica | Corrida 1 | Corrida 2 | Corrida 3 | Umbral Hipótesis |
| :--- | :---: | :---: | :---: | :---: |
| **Peticiones Totales** | 11,560 | 11,370 | 11,755 | — |
| **Conformidad `OperationOutcome`** | **100.00%** (9,248/9,248) | **100.00%** (9,096/9,096) | **100.00%** (9,404/9,404) | 100.0% (`rate == 1.0`) |
| **Validaciones (Checks)** | **100.00%** (34,680/34,680) | **100.00%** (34,110/34,110) | **100.00%** (35,265/35,265) | 100.0% |
| **Latencia Media (`http_req_duration`)** | 2.68 ms | 2.86 ms | 2.48 ms | — |
| **Latencia p90** | 4.00 ms | 4.35 ms | 3.62 ms | — |
| **Latencia p95** | **4.72 ms** | **5.16 ms** | **4.10 ms** | < 30 ms |
| **Latencia Máxima** | 24.52 ms | 14.18 ms | 8.99 ms | — |
| **Throughput** | 769.4 req/s | 756.8 req/s | 782.0 req/s | — |

---

## 8. VALIDATOR HL7 FHIR (OFICIAL)

- **Comando intentado:** `powershell -ExecutionPolicy Bypass -File scripts/validate_fhir.ps1`
- **Código de salida:** `1`
- **Resultado:** **NO EJECUTADO**
- **Salida literal:**
  ```text
  ============================================================
  MedSys-FHIR — VALIDADOR HL7 FHIR R4 (OFFLINE)
  ============================================================
  [*] Directorio de entrada : C:\Users\Judirico\Documents\MedSys-FHIR\MEDSYS-FHIR\output
  [*] Validador JAR         : C:\Users\Judirico\Documents\MedSys-FHIR\MEDSYS-FHIR\validator_cli.jar
  [*] Version FHIR          : 4.0.1
  ------------------------------------------------------------
  [*] Archivos JSON a validar: 60
  ADVERTENCIA: [WARN] No se localizó validator_cli.jar en: 
  C:\Users\Judirico\Documents\MedSys-FHIR\MEDSYS-FHIR\validator_cli.jar

  Para descargarlo ejecute (PowerShell):
    Invoke-WebRequest -Uri 'https://github.com/hapifhir/org.hl7.fhir.core/releases/latest/download/validator_cli.jar' -OutFile validator_cli.jar

  Comando de validacion manual una vez descargado:
    java -jar C:\Users\Judirico\Documents\MedSys-FHIR\MEDSYS-FHIR\validator_cli.jar C:\Users\Judirico\Documents\MedSys-FHIR\MEDSYS-FHIR\output\*.json -version 4.0.1 -tx n/a
  ```
- **Motivo técnico:**
  1. El archivo `validator_cli.jar` no existe en la raíz ni en ningún subdirectorio del repositorio.
  2. La versión de Java instalada en el sistema anfitrión es Java 8 (`1.8.0_361`), mientras que el script oficial del repositorio (`scripts/validate_fhir.ps1:14`) exige Java 11+.
  3. De acuerdo con las prohibiciones operativas de la auditoría ("No agregues dependencias; si un comando no se puede ejecutar, escribe NO EJECUTADO"), no se descargó el binario JAR externo ni se actualizó la versión de Java del anfitrión.
- **Muestra existente en `output/`:** Existen 60 archivos JSON exportados previamente (10 Patient, 10 Encounter, 10 Condition, 10 Observation_bp, 10 Observation_temp, 10 Observation_hr; no existen muestras de Observation con frecuencia respiratoria).

---

## 9. RESUMEN DE ELEMENTOS NO EJECUTADOS O NO VERIFICABLES

A continuación se consolida la lista formal de todos los procedimientos que no pudieron ejecutarse o verificarse empíricamente en esta ronda, indicando la causa objetiva:

1. **Validación Sintáctica con `org.hl7.fhir.validator-cli`:**
   - **Estado:** NO EJECUTADO.
   - **Causa:** El archivo binario `validator_cli.jar` no se distribuye dentro del repositorio y el anfitrión cuenta con Java 8 (el script requiere Java 11+). No se introdujeron descargas externas para respetar las restricciones de inmutabilidad.
2. **Script de Orquestación PowerShell `tests/k6/run_benchmark_with_stats.ps1`:**
   - **Estado:** NO EJECUTABLE NATIVAMENTE (Fallo de sintaxis del script).
   - **Causa:** Error de sintaxis en el intérprete de PowerShell por caracteres de tubería no escapados en strings (`ExpressionsMustBeFirstInPipeline` en línea 24). La medición de `load_test.js` y `docker stats` se ejecutó mediante invocación directa del motor k6 y subprocesos según las instrucciones del `README.md`.
3. **Script de Orquestación PowerShell `tests/k6/run_all_benchmarks.ps1`:**
   - **Estado:** NO EJECUTABLE NATIVAMENTE (Fallo de sintaxis del script).
   - **Causa:** Error de sintaxis en línea 75 por falta de delimitador de cierre de cadena (`TerminatorExpectedAtEndOfString`). Las suites de `smoke_test.js`, `load_test.js` y `resilience_and_errors_test.js` fueron ejecutadas directamente con k6.
4. **Script Bash `tests/k6/run_benchmark_with_stats.sh`:**
   - **Estado:** NO EJECUTADO.
   - **Causa:** Diseñado exclusivamente para entornos POSIX/Linux. En el host Windows, la distro WSL2 (`docker-desktop`) no contiene `/bin/bash` instalada.
5. **Generación y Servicio de Recursos `Observation` de Frecuencia Respiratoria:**
   - **Estado:** NO VERIFICABLE EN TIEMPO DE EJECUCIÓN HTTP.
   - **Causa:** Aunque la regla declarativa fue añadida en `mapping_rules.yaml` (D-016), los manejadores HTTP en `crates/medsys-server/src/handlers/observation.rs` no la incluyen ni en `list_observations` ni en `get_observation`.
