# ESTADO ACTIVO DEL PROYECTO (STATE.md) — MedSys-FHIR

## Metadatos de Control
- **Última Actualización:** 2026-09-24T17:20:00-06:00
- **Sprint Activo:** Sprint 4 (`.agents/backlog/sprint_4_axum_dashboard.md`)
- **Estado General:** 14 / 16 tareas completadas (87.5%)
- **Tarea en Curso:** Tarea 4.3: Dashboard interactivo en React 19 + Vite + Tailwind CSS.
- **Última Tarea Cerrada:** Tarea 4.2: Manejador global de excepciones traduciendo a `OperationOutcome` (junto con Tarea 4.1).
- **Siguiente Tarea Inmediata:** Tarea 4.3: Desarrollo del Dashboard Frontend (React 19 + Vite + Tailwind CSS + Lucide Icons).
- **Estado del Build:** PASS (Compilación GNU/MinGW, Clippy 0 warnings, rustfmt PASS, cargo test 38/38 PASS [11 en medsys-core, 19 en medsys-db, 3 en medsys-db integration, 5 en medsys-server integration]).

---

## 1. Decisiones Arquitectónicas Establecidas
1. **Lenguaje y Stack:** Rust 2021, runtime Tokio, framework Axum 0.8, Tower / Tower-HTTP 0.6, SQLx 0.8 para persistencia de solo lectura en PostgreSQL 16.
2. **Entorno de Compilación:** Toolchain Rust GNU `stable-x86_64-pc-windows-gnu` con MinGW-w64 (`C:\msys64\mingw64\bin`) configurado en el entorno de usuario.
3. **Estándar:** HL7 FHIR R4 oficial vía crate `helios-fhir` v0.2 (`R4`).
4. **Reglas Declarativas de Mapeo (v1.1.0):**
   - El archivo `mapping_rules_specification.yaml` opera bajo la versión `1.1.0`, completamente homologado con `crates/medsys-core/src/engine/transform.rs`, la NOM-004-SSA3-2012 y HL7 FHIR R4.
   - `Patient`: CURP oficial (`urn:oid:2.16.840.1.113883.4.629`, use "official"), desglose de nombres de pila y combinación declarativa de apellidos (`combine_with: "apellido_materno"`).
   - `Encounter`: Clasificación ambulatoria obligatoria `AMB` (`http://terminology.hl7.org/CodeSystem/v3-ActCode`), periodo de atención `start`/`end`, participante médico con Cédula SEP (`http://cedulaprofesional.sep.gob.mx`).
   - `Observation`: Desacoplado formalmente en 2 perfiles/recursos canónicos independientes:
     - *Recurso 3A:* Panel de Presión Arterial (`http://hl7.org/fhir/StructureDefinition/bp`, LOINC `85354-9`) con subcomponentes sistólica (LOINC `8480-6`) y diastólica (LOINC `8462-4`) en `mmHg`.
     - *Recurso 3B:* Temperatura Corporal (`http://hl7.org/fhir/StructureDefinition/bodytemp`, LOINC `8310-5`) en unidad UCUM `Cel`.
   - `Condition`: Catálogo internacional CIE-10 (`http://hl7.org/fhir/sid/icd-10`), estado clínico `active` y estado de verificación tipado desde `tipo_diagnostico` (`confirmed` / `provisional`).
5. **Servidor Axum y Endpoints REST FHIR Canónicos (Tarea 4.1):**
   - Servidor montado en `crates/medsys-server` exponiendo los recursos FHIR bajo el prefijo canónico `/fhir/r4/`:
     - `GET /fhir/r4/Patient/{id}` y `GET /fhir/r4/Patient` (Bundle searchset).
     - `GET /fhir/r4/Encounter/{id}` y `GET /fhir/r4/Encounter` (Bundle searchset, filtro opcional `?patient={id}`).
     - `GET /fhir/r4/Observation/{id}` (soporte para prefijos `bp-{id}`, `temp-{id}` o id puro) y `GET /fhir/r4/Observation` (Bundle searchset que emite las 2 observaciones desacopladas por cada signo vital).
     - `GET /fhir/r4/Condition/{id}` (soporte para prefijo `cond-{id}` o id puro) y `GET /fhir/r4/Condition` (Bundle searchset, filtros por paciente, consulta o código CIE-10).
   - Negociación estricta de contenido con cabecera `Content-Type: application/fhir+json; charset=utf-8` mediante struct `FhirResponse`.
   - Middlewares de telemetría con `tracing` (`TraceLayer::new_for_http()`) y CORS permisivo (`CorsLayer`) para el Dashboard React.
6. **Manejo Global de Excepciones Clínicas con OperationOutcome (Tarea 4.2):**
   - Implementado el trait `IntoResponse` en `ServerError` mapeando todas las excepciones de dominio (`MedSysError`), persistencia (`sqlx::Error`), errores de ruta y fallos JSON.
   - Todo error 404 (`not-found`), 422 (`invalid` / `required`), 500 (`transient` / `processing` / `exception`) y 400 (`value`) emite invariablemente el recurso canónico `OperationOutcome` con cabecera `Content-Type: application/fhir+json; charset=utf-8` y diagnósticos en español técnico. Cero respuestas de error con JSON genérico.
   - Implementado fallback universal de enrutador `not_found_fallback` que captura cualquier URI inexistente y devuelve `OperationOutcome` 404.
   - Endpoints auxiliares de salud y monitoreo (`/health`, `/api/health`) e inspección relacional/comparativa para el frontend (`/api/legacy/patients`, `/api/legacy/patients/{id}/full`).
7. **Infraestructura de Persistencia Relacional y Pool SQLx:**
   - PostgreSQL 16 contenerizado con esquema `schema_legado_simulado_nom004.sql`.
   - Pool asíncrono con `sqlx::PgPool` gestionado por `medsys_db::DbManager` y `MedsysRepositories`. Invariantes de seguridad: Cero mutaciones, consultas 100% parametrizadas con placeholders `$1`, `$2`... previniendo inyección SQL.

---

## 2. Archivos Creados / Modificados en este Turno
- `Cargo.toml`: Adición de `axum`, `tower` y `tower-http` a dependencias del workspace.
- `crates/medsys-core/src/engine/transform.rs`: Funciones constructoras canónicas `create_operation_outcome` y `create_searchset_bundle`.
- `crates/medsys-core/src/engine/mod.rs`: Pruebas unitarias de serialización canónica para `OperationOutcome` y `Bundle`.
- `crates/medsys-core/src/lib.rs`: Exportación pública de `create_operation_outcome` y `create_searchset_bundle`.
- `crates/medsys-server/Cargo.toml`: Configuración de dependencias (`axum`, `tower`, `tower-http`, `helios-fhir`, `sqlx`, `chrono`, `http-body-util`).
- `crates/medsys-server/src/error.rs`: Manejador centralizado de errores `ServerError` implementando `IntoResponse` hacia `OperationOutcome`, constante `FHIR_JSON_CONTENT_TYPE` y `not_found_fallback`.
- `crates/medsys-server/src/response.rs`: Wrapper `FhirResponse` que garantiza emisión de cabecera `application/fhir+json`.
- `crates/medsys-server/src/state.rs`: `AppState` thread-safe conteniendo repositorios, reglas de mapeo, pool y temporizador de uptime.
- `crates/medsys-server/src/config.rs`: `ServerConfig` para lectura de host, puerto, URL de BD y ruta de especificación YAML.
- `crates/medsys-server/src/handlers/patient.rs`: Endpoints `GET /fhir/r4/Patient/{id}` y `GET /fhir/r4/Patient`.
- `crates/medsys-server/src/handlers/encounter.rs`: Endpoints `GET /fhir/r4/Encounter/{id}` y `GET /fhir/r4/Encounter`.
- `crates/medsys-server/src/handlers/observation.rs`: Endpoints `GET /fhir/r4/Observation/{id}` y `GET /fhir/r4/Observation`.
- `crates/medsys-server/src/handlers/condition.rs`: Endpoints `GET /fhir/r4/Condition/{id}` y `GET /fhir/r4/Condition`.
- `crates/medsys-server/src/handlers/health.rs`: Endpoints `/health` y `/api/health`.
- `crates/medsys-server/src/handlers/legacy.rs`: Endpoints `/api/legacy/patients` y `/api/legacy/patients/{id}/full` para el Dashboard.
- `crates/medsys-server/src/handlers/mod.rs`: Módulo agrupador y re-exportador de controladores.
- `crates/medsys-server/src/router.rs`: Ensamblado de rutas, capas de middleware (CORS, TraceLayer) y fallback de error.
- `crates/medsys-server/src/lib.rs`: Exposición pública de la biblioteca `medsys-server`.
- `crates/medsys-server/src/main.rs`: Punto de entrada del binario del servidor con bindeo TCP en `0.0.0.0:8080`.
- `crates/medsys-server/tests/server_integration.rs`: Suite de pruebas de integración con 5 casos validando healthcheck, OperationOutcome, fallback, validación de parámetros y endpoints canónicos FHIR.
- `BACKLOG.md`: Tareas 4.1 y 4.2 marcadas como completadas `[x]`, progreso 14/16 (87.5%).
- `.agents/backlog/sprint_4_axum_dashboard.md`: Tareas 4.1 y 4.2 marcadas como completadas `[x]`.
- `.agents/backlog/overview.md`: Progreso global actualizado al 87.5%.

---

## 3. Comando de Arranque para el Siguiente Turno
Para continuar de inmediato con la Tarea 4.3 del Sprint 4 (Dashboard React 19 + Vite):
> "Lee .agents/rules/rules.md, .agents/orchestrator/workflow.md, STATE.md y BACKLOG.md. Continúa con la Tarea 4.3 del Sprint 4."
