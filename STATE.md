# ESTADO ACTIVO DEL PROYECTO (STATE.md) — MedSys-FHIR

## Metadatos de Control
- **Última Actualización:** 2026-09-24T16:16:00-06:00
- **Sprint Activo:** Sprint 3 (`.agents/backlog/sprint_3_sqlx_docker.md`)
- **Estado General:** 10 / 16 tareas completadas (62.5%)
- **Tarea en Curso:** Ninguna (Tarea 3.2 concluida al 100%, pool asíncrono SQLx y DbManager listos)
- **Última Tarea Cerrada:** Tarea 3.2: Configuración del pool asíncrono SQLx en `medsys-db`.
- **Siguiente Tarea Inmediata:** Tarea 3.3: Implementación de repositorios de lectura parametrizada ($1, $2).
- **Estado del Build:** PASS (Compilación GNU/MinGW, Clippy 0 warnings, rustfmt PASS, cargo test 19/19 PASS [9 en medsys-core, 10 en medsys-db]).

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
   - `DbManager`: inicialización eager (`init_pool`) y lazy (`init_pool_lazy`), healthcheck tipado no bloqueante (`SELECT 1`), cierre ordenado (`close`).
   - Mapeo unificado de errores (`map_sqlx_error`) hacia `MedSysError::DatabaseError` y `MedSysError::NotFound`, asegurando compatibilidad directa con `OperationOutcome`.
7. **Endpoints HTTP y Pruebas de Carga (Sprint 4):**
   - Los endpoints REST FHIR operarán canónicamente bajo el prefijo `/fhir/r4/` (`GET /fhir/r4/Patient/{id}`, `GET /fhir/r4/Encounter/{id}`, etc.).
   - Validación de rendimiento, latencia y concurrencia integrada con suites de k6.
8. **Manejo de Errores:** Excepciones gestionadas estrictamente con `MedSysError` (cero `unwrap()` y cero `expect()` en código de producción).

---

## 2. Archivos Creados / Modificados en este Turno
- `Cargo.toml`: Adición de `sqlx` (v0.8 con soporte Tokio, PostgreSQL, Chrono, RustDecimal) y `dotenvy` a `[workspace.dependencies]`.
- `crates/medsys-db/Cargo.toml`: Inclusión de dependencias de persistencia relacional.
- `crates/medsys-core/src/error.rs`: Adición de variante `MedSysError::DatabaseError(String)`.
- `crates/medsys-db/src/config.rs`: Implementación de `DbConfig` con parámetros de pool, lectura de `.env`, timeouts tipados y pruebas unitarias.
- `crates/medsys-db/src/error.rs`: Tipado de `DbError` y función `map_sqlx_error` con pruebas unitarias.
- `crates/medsys-db/src/pool.rs`: Implementación de `create_pool_options`, `init_pool`, `init_pool_lazy`, `DbManager`, healthchecks y pruebas asíncronas Tokio.
- `crates/medsys-db/src/lib.rs`: Exposición pública de los módulos de configuración, pool y gestión de errores.
- `BACKLOG.md`: Marcada Tarea 3.2 como completada `[x]`.
- `.agents/backlog/sprint_3_sqlx_docker.md`: Tarea 3.2 marcada como completada `[x]` con nota técnica detallada.
- `.agents/backlog/overview.md`: Progreso actualizado a 10/16 tareas (62.5%), Sprint 3 al 50%.
- `STATE.md`: Consolidación del estado del proyecto tras finalizar la Tarea 3.2.

---

## 3. Comando de Arranque para el Siguiente Turno
Para continuar de inmediato con la Tarea 3.3 del Sprint 3:
> "Lee .agents/rules/rules.md, .agents/orchestrator/workflow.md, STATE.md y BACKLOG.md. Continúa con la Tarea 3.3 del Sprint 3."
