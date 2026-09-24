# SPRINT 3: Persistencia Asíncrona SQLx y Entorno Docker PostgreSQL 16 — MedSys-FHIR

## Objetivo del Sprint
Configurar la infraestructura de persistencia relacional en PostgreSQL 16 contenerizado con Docker Compose, e implementar en el crate `medsys-db` las consultas asíncronas parametrizadas de solo lectura sobre las tablas legadas simuladas.

---

## Lista de Tareas

- [x] **Tarea 3.1: Configuración de Infraestructura Docker para PostgreSQL 16**
  - Crear `docker/docker-compose.yml` con servicio PostgreSQL 16 y volumen de inicialización que monte `schema_legado_simulado_nom004.sql`.
  - Configurar variables de entorno y scripts de arranque.
  - *Finalizado:* 2026-09-24T15:56:00-06:00. Infraestructura contenerizada con PostgreSQL 16 Alpine, volumen de datos `postgres_data`, montaje de lectura de `schema_legado_simulado_nom004.sql` en `/docker-entrypoint-initdb.d/`, healthcheck con `pg_isready`, variables de entorno (`.env`, `.env.example`, `docker/.env.example`) y scripts de arranque/parada/reinicio (`start-db.ps1`, `stop-db.ps1`, `reset-db.ps1`, `start-db.sh`, `stop-db.sh`, `reset-db.sh`). Sintaxis validada con `docker compose config`.


- [x] **Tarea 3.2: Configuración del Pool Asíncrono SQLx en medsys-db**
  - Implementar inicialización de `sqlx::PgPool` con timeouts, límites de conexión y manejo seguro de errores.
  - Integrar conexión en `crates/medsys-db`.
  - *Finalizado:* 2026-09-24T16:16:00-06:00. Integración de `sqlx` 0.8 con soporte Tokio, PostgreSQL, Chrono y RustDecimal. Implementado `DbConfig` con parámetros de pool (`max_connections`, `min_connections`, `acquire_timeout_secs`, `idle_timeout_secs`, `max_lifetime_secs`), enmascaramiento seguro de URLs para tracing, inicialización eager (`init_pool`) y lazy (`init_pool_lazy`), `DbManager` con healthcheck no bloqueante (`SELECT 1`), mapeo tipado de errores SQLx a `MedSysError` (`DatabaseError`, `NotFound`). 10/10 pruebas unitarias en `medsys-db` PASS, Clippy 0 warnings.


- [x] **Tarea 3.3: Implementación de Repositorios de Lectura Parametrizada**
  - Implementar structs de acceso a datos para `tbl_pacientes`, `tbl_consultas`, `tbl_signos_vitales` y `tbl_diagnosticos`.
  - Garantizar uso estricto de consultas parametrizadas (`$1`, `$2`) sin concatenación ni mutaciones (solo lectura).
  - *Finalizado:* 2026-09-24T16:34:00-06:00. Implementadas entidades intermedias `sqlx::FromRow` (`PacienteEntity`, `ConsultaEntity`, `SignoVitalEntity`, `DiagnosticoEntity`) con conversión bidireccional `From` hacia modelos de dominio en `medsys-core`. Implementados 4 repositorios especializados (`PacienteRepository`, `ConsultaRepository`, `SignosVitalesRepository`, `DiagnosticosRepository`) y bundle unificado `MedsysRepositories`. Todas las consultas son estrictamente de solo lectura (`SELECT`), parametrizadas con placeholders (`$1`, `$2`), sin concatenación de cadenas, con mapeo tipado a `MedSysError::NotFound` y `MedSysError::DatabaseError`. 19/19 tests en `medsys-db` PASS (28/28 global), Clippy 0 warnings.


- [x] **Tarea 3.4: Pruebas de Integración de Persistencia**
  - Pruebas automatizadas de lectura y mapeo de filas de PostgreSQL a estructuras intermedias en Rust.
  - *Finalizado:* 2026-09-24T16:52:00-06:00. Implementada suite de integración en `crates/medsys-db/tests/persistence_integration.rs`: validación de contrato estricto del archivo SQL sintético NOM-004 (`test_schema_sql_contract_integrity`), verificación end-to-end de mapeo relacional a modelos de dominio y transformación FHIR R4 canónica (`test_synthetic_data_persistence_mapping_to_fhir_e2e`) serializando a JSON con etiquetas oficiales de `Resource`, y prueba en vivo contra PostgreSQL contenerizado con fallback diagnóstico (`test_live_postgresql_persistence_when_available`). 31/31 tests PASS a nivel workspace, Clippy 0 warnings, rustfmt PASS. **Sprint 3 completado al 100% (4/4 tareas).**

