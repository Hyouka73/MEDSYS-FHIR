# ESTADO ACTIVO DEL PROYECTO (STATE.md) — MedSys-FHIR

## Metadatos de Control
- **Última Actualización:** 2026-09-24T16:53:00-06:00
- **Sprint Activo:** Sprint 4 (`.agents/backlog/sprint_4_axum_dashboard.md`)
- **Estado General:** 12 / 16 tareas completadas (75%)
- **Tarea en Curso:** Ninguna (Sprint 3 concluido al 100%, listo para Sprint 4)
- **Última Tarea Cerrada:** Tarea 3.4: Pruebas de integración de persistencia sobre datos sintéticos (Cierre del Sprint 3 al 100%).
- **Siguiente Tarea Inmediata:** Tarea 4.1: Enrutamiento HTTP en Axum y endpoints REST FHIR canónicos (`/fhir/r4/`).
- **Estado del Build:** PASS (Compilación GNU/MinGW, Clippy 0 warnings, rustfmt PASS, cargo test 31/31 PASS [9 en medsys-core, 19 en medsys-db, 3 en tests de integración]).

---

## 1. Decisiones Arquitectónicas Establecidas
1. **Lenguaje y Stack:** Rust 2021, runtime Tokio, framework Axum, SQLx para persistencia de solo lectura en PostgreSQL 16.
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
5. **Infraestructura de Persistencia Relacional (PostgreSQL 16 en Docker):**
   - Servicio contenerizado bajo imagen oficial `postgres:16-alpine` montando en modo solo lectura (`:ro`) el archivo `schema_legado_simulado_nom004.sql` hacia `/docker-entrypoint-initdb.d/01_schema_legado_simulado_nom004.sql`.
   - Volumen dedicado `postgres_data` y healthcheck autónomo con `pg_isready`.
   - Cadena de conexión canónica: `postgres://medsys_user:medsys_secure_pass_2026@localhost:5432/medsys_legacy`.
   - Scripts de ciclo de vida PowerShell (`start-db.ps1`, `stop-db.ps1`, `reset-db.ps1`) y Bash (`start-db.sh`, `stop-db.sh`, `reset-db.sh`).
6. **Pool Asíncrono SQLx y Capa de Persistencia (`medsys-db`):**
   - Implementado en `crates/medsys-db` desacoplado de `medsys-core` (el cual se mantiene puro sin dependencias de base de datos ni red).
   - `DbConfig`: lectura jerárquica desde variables de entorno y archivos `.env` (dotenvy), gestión de límites (`max_connections`, `min_connections`) y timeouts de ciclo de vida (`acquire_timeout`, `idle_timeout`, `max_lifetime`). Función `masked_url()` para prevención de fuga de credenciales en logs/tracing.
   - `DbManager`: inicialización eager (`init_pool`) y lazy (`init_pool_lazy`), healthcheck tipado no bloqueante (`SELECT 1`), cierre ordenado (`close`), acceso integrado a repositorios (`repositories()`).
   - Mapeo unificado de errores (`map_sqlx_error`) hacia `MedSysError::DatabaseError` y `MedSysError::NotFound`, asegurando compatibilidad directa con `OperationOutcome`.
7. **Repositorios de Lectura Parametrizada y Modelado Relacional (NOM-004):**
   - Entidades intermedias `sqlx::FromRow`: `PacienteEntity`, `ConsultaEntity`, `SignoVitalEntity`, `DiagnosticoEntity` en `crates/medsys-db/src/entities.rs` con conversión sin pérdida hacia los modelos de dominio `LegacyPaciente`, `LegacyConsulta`, `LegacySignoVital`, `LegacyDiagnostico` de `medsys-core`.
   - Repositorios especializados en `crates/medsys-db/src/repository/`: `PacienteRepository`, `ConsultaRepository`, `SignosVitalesRepository` y `DiagnosticosRepository`, orquestados mediante el bundle `MedsysRepositories`.
   - Garantía de invariantes de seguridad: Cero mutaciones (estricto `SELECT`), cero concatenación de cadenas, todas las sentencias parametrizadas exclusivamente con placeholders `$1`, `$2`... previniendo inyecciones SQL.
8. **Suite de Integración y Transformación Canónica E2E (Tarea 3.4):**
   - Pruebas en `crates/medsys-db/tests/persistence_integration.rs`:
     - `test_schema_sql_contract_integrity`: Valida que el archivo SQL de laboratorio cumpla con todas las columnas, restricciones normativas y datos sintéticos.
     - `test_synthetic_data_persistence_mapping_to_fhir_e2e`: Valida la cadena completa: datos relacionales de prueba -> conversión a entidades -> conversión a modelos legacy -> transformación a recursos HL7 FHIR R4 canónicos (`Resource::Patient`, `Resource::Encounter`, `Resource::Observation`, `Resource::Condition`) -> serialización JSON conforme a estándar.
     - `test_live_postgresql_persistence_when_available`: Conexión en vivo contra contenedor PostgreSQL 16 con verificación de consultas reales parametrizadas y reporte no bloqueante si Docker no está activo.
9. **Endpoints HTTP y Pruebas de Carga (Sprint 4):**
   - Los endpoints REST FHIR operarán canónicamente bajo el prefijo `/fhir/r4/` (`GET /fhir/r4/Patient/{id}`, `GET /fhir/r4/Encounter/{id}`, etc.).
   - Validación de rendimiento, latencia y concurrencia integrada con suites de k6.
10. **Manejo de Errores:** Excepciones gestionadas estrictamente con `MedSysError` (cero `unwrap()` y cero `expect()` en código de producción).

---

## 2. Archivos Creados / Modificados en este Turno
- `crates/medsys-db/Cargo.toml`: Adición de `helios-fhir` a `[dev-dependencies]` para pruebas de integración de transformación a recursos FHIR.
- `crates/medsys-db/tests/persistence_integration.rs`: Suite de pruebas de integración de persistencia sobre datos sintéticos NOM-004 y transformación canónica HL7 FHIR R4.
- `BACKLOG.md`: Marcada Tarea 3.4 como completada `[x]`, Sprint 3 cerrado al 100% y Sprint 4 en estado SIGUIENTE.
- `.agents/backlog/sprint_3_sqlx_docker.md`: Tarea 3.4 marcada como completada `[x]` con nota técnica y declaración de Sprint 3 concluido al 100%.
- `.agents/backlog/overview.md`: Progreso actualizado a 12/16 tareas (75%), Sprint 3 completado (4/4, 100%).
- `STATE.md`: Consolidación del estado del proyecto tras finalizar el Sprint 3.

---

## 3. Comando de Arranque para el Siguiente Turno
Para comenzar de inmediato con el Sprint 4 (Tarea 4.1):
> "Lee .agents/rules/rules.md, .agents/orchestrator/workflow.md, STATE.md y BACKLOG.md. Continúa con la Tarea 4.1 del Sprint 4."
