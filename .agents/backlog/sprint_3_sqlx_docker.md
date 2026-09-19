# SPRINT 3: Persistencia Asíncrona SQLx y Entorno Docker PostgreSQL 16 — MedSys-FHIR

## Objetivo del Sprint
Configurar la infraestructura de persistencia relacional en PostgreSQL 16 contenerizado con Docker Compose, e implementar en el crate `medsys-db` las consultas asíncronas parametrizadas de solo lectura sobre las tablas legadas simuladas.

---

## Lista de Tareas

- [ ] **Tarea 3.1: Configuración de Infraestructura Docker para PostgreSQL 16**
  - Crear `docker/docker-compose.yml` con servicio PostgreSQL 16 y volumen de inicialización que monte `schema_legado_simulado_nom004.sql`.
  - Configurar variables de entorno y scripts de arranque.

- [ ] **Tarea 3.2: Configuración del Pool Asíncrono SQLx en medsys-db**
  - Implementar inicialización de `sqlx::PgPool` con timeouts, límites de conexión y manejo seguro de errores.
  - Integrar conexión en `crates/medsys-db`.

- [ ] **Tarea 3.3: Implementación de Repositorios de Lectura Parametrizada**
  - Implementar structs de acceso a datos para `tbl_pacientes`, `tbl_consultas`, `tbl_signos_vitales` y `tbl_diagnosticos`.
  - Garantizar uso estricto de consultas parametrizadas (`$1`, `$2`) sin concatenación ni mutaciones (solo lectura).

- [ ] **Tarea 3.4: Pruebas de Integración de Persistencia**
  - Pruebas automatizadas de lectura y mapeo de filas de PostgreSQL a estructuras intermedias en Rust.
