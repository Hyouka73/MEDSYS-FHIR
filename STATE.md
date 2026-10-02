# ESTADO ACTIVO DEL PROYECTO (STATE.md) — MedSys-FHIR

## Metadatos de Control
- **Última Actualización:** 2026-10-02T09:45:00-06:00
- **Sprint Activo:** Mantenimiento y cumplimiento normativo de privacidad LGPDPPSO (2025).
- **Estado General:** 16 / 16 tareas completadas (100.0%) — Conformidad estricta LGPDPPSO.
- **Tarea en Curso:** Ninguna (Corrección de privacidad completada).
- **Última Tarea Cerrada:** Eliminación de directiva telefono_contacto en reglas YAML y telecom en recurso Patient por LGPDPPSO (2025).
- **Siguiente Tarea Inmediata:** Despliegue en producción / Defensa de Tesis UNACH 2026.
- **Estado del Build:** PASS (Backend Rust GNU/MinGW: 55/55 tests PASS, Clippy 0 warnings con `-D warnings`, rustfmt PASS; Frontend Dashboard React 19 + Vite: build PASS, oxlint 0 warnings/0 errors).

---

## 1. Decisiones Arquitectónicas Establecidas
1. **Lenguaje y Stack:** Rust 2021, runtime Tokio, framework Axum 0.8, Tower / Tower-HTTP 0.6, SQLx 0.8 para persistencia de solo lectura en PostgreSQL 16.
2. **Entorno de Compilación:** Toolchain Rust GNU `stable-x86_64-pc-windows-gnu` con MinGW-w64 (`C:\msys64\mingw64\bin`) configurado en el entorno de usuario.
3. **Estándar:** HL7 FHIR R4 oficial vía crate `helios-fhir` v0.2 (`R4`).
4. **Reglas Declarativas de Mapeo (v1.1.0):**
   - El archivo `mapping_rules.yaml` opera bajo la versión `1.1.0`, completamente homologado con `crates/medsys-core/src/engine/transform.rs`, la NOM-004-SSA3-2012 y HL7 FHIR R4.
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
- `crates/medsys-server/src/config.rs`: Soporte dual para variables `SERVER_PORT`/`SERVER_HOST` con fallback a 3000.
- `dashboard/package.json`: Configuración de dependencias React 19 (`^19.2.8`), Vite 8, Tailwind CSS y Lucide Icons.
- `dashboard/vite.config.js`: Configuración de proxy HTTP hacia `http://localhost:3000` (`/fhir`, `/health`, `/api`).
- `dashboard/src/services/api.js`: Cliente API con medición de latencia en ms, emisor de eventos y fallback automático a datos sintéticos.
- `dashboard/src/services/mockData.js`: Datos sintéticos de `tbl_pacientes`, `tbl_consultas`, `tbl_signos_vitales`, `tbl_diagnosticos` y sus recursos FHIR correspondientes.
- `dashboard/src/components/Header.jsx`: Encabezado sticky con badges de arquitectura, estado en vivo y pestañas de navegación.
- `dashboard/src/components/MetricsPanel.jsx`: 4 tarjetas de métricas en tiempo real (salud, latencia, base de datos, total transacciones).
- `dashboard/src/components/FhirViewer.jsx`: Navegador interactivo de recursos FHIR R4 (`Patient`, `Encounter`, `Observation`, `Condition`) con soporte para instancias individuales y Bundles searchset.
- `dashboard/src/components/InteroperabilityComparator.jsx`: Demostración visual trifásica lado a lado (Fila relacional NOM-004 ⇄ Reglas YAML ⇄ Recurso FHIR R4).
- `dashboard/src/components/EventConsole.jsx`: Stream de peticiones HTTP, códigos de estado, disparador de fallos diagnósticos e inspección de `OperationOutcome`.
- `dashboard/src/components/JsonSyntaxHighlighter.jsx`: Resaltador sintáctico para JSON FHIR con botones de copia y descarga.
- `dashboard/src/App.jsx`: Ensamblado de componentes, polling periódico y footer académico UNACH 2026.
- `dashboard/README.md`: Documentación completa de arquitectura y ejecución del Dashboard.
- `crates/medsys-server/tests/e2e_interoperability.rs`: Suite de pruebas E2E con 8 casos validando el flujo completo HTTP ➔ FHIR R4.
- `tests/k6/smoke_test.js`: Prueba de humo de disponibilidad y esquemas FHIR R4 en k6.
- `tests/k6/load_test.js`: Prueba de carga y concurrencia clínica en k6 (20 VUs sostenidas).
- `tests/k6/resilience_and_errors_test.js`: Prueba de estrés de errores OperationOutcome (100% conformidad, p95=1.14ms).
- `tests/k6/run_all_benchmarks.ps1`: Automatizador de ejecución de benchmarks en PowerShell.
- `tests/k6/README.md`: Documentación de la suite k6 para la tesis.
- `DEPLOYMENT.md`: Manual integral de arquitectura, despliegue y entrega de tesis.
- `Cargo.toml`: Configuración de perfil release con LTO, strip y optimizaciones de compilador.
- `BACKLOG.md`: Tarea 4.4 completada `[x]`, 16/16 tareas (100.0%).
- `.agents/backlog/sprint_4_axum_dashboard.md`: Tarea 4.4 completada `[x]`.
- `mapping_rules.yaml`: Supresión completa del mapeo de `telefono_contacto` hacia `telecom[0].value` en el recurso `Patient`.
- `crates/medsys-core/src/engine/transform.rs`: Remoción del cálculo de `telecom`, fijación de `telecom: None` y eliminación de la importación no utilizada `ContactPoint`.
- `crates/medsys-core/src/engine/mod.rs`: Actualización de aserciones en pruebas unitarias para comprobar que `telecom` es `None`, que el JSON serializado no contiene `"telecom"` y que las reglas YAML no contienen mapeo para `telefono_contacto`.
- `crates/medsys-server/tests/e2e_interoperability.rs`: Aserción de exclusión de `"telecom"` en respuesta HTTP de `GET /fhir/r4/Patient/1`.
- `crates/medsys-server/tests/server_integration.rs`: Aserción de exclusión de `"telecom"` en la respuesta JSON canónica de `Patient`.
- `dashboard/src/services/mockData.js`: Remoción del campo `telecom` en los datos mock de `fhir_patients` para reflejar la política de privacidad.

---

## 3. Estado de Entrega y Cierre
El proyecto **MedSys-FHIR** ha culminado el 100% de su cronograma de desarrollo e investigación para la titulación de licenciatura en la UNACH.
Todos los artefactos de código, suites de pruebas unitarias/integración/E2E, suite de benchmarking con k6, panel analítico en React 19 y manuales de despliegue se encuentran probados, compilados y verificados bajo cero errores y cero advertencias.
