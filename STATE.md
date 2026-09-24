# ESTADO ACTIVO DEL PROYECTO (STATE.md) — MedSys-FHIR

## Metadatos de Control
- **Última Actualización:** 2026-09-24T15:56:00-06:00
- **Sprint Activo:** Sprint 3 (`.agents/backlog/sprint_3_sqlx_docker.md`)
- **Estado General:** 9 / 16 tareas completadas (56.25%)
- **Tarea en Curso:** Ninguna (Tarea 3.1 concluida al 100%, infraestructura Docker y PostgreSQL 16 lista)
- **Última Tarea Cerrada:** Tarea 3.1: Configuración de infraestructura Docker Compose con PostgreSQL 16 y esquema sintético NOM-004.
- **Siguiente Tarea Inmediata:** Tarea 3.2: Configuración del pool asíncrono SQLx en `medsys-db`.
- **Estado del Build:** PASS (Compilación estática GNU/MinGW, Clippy sin warnings, cargo test 9/9 PASS, validación `docker compose config` PASS).

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
   - Automatización de ciclo de vida con scripts nativos PowerShell (`start-db.ps1`, `stop-db.ps1`, `reset-db.ps1`) y Bash (`start-db.sh`, `stop-db.sh`, `reset-db.sh`).
6. **Endpoints HTTP y Pruebas de Carga (Sprint 4):**
   - Los endpoints REST FHIR operarán canónicamente bajo el prefijo `/fhir/r4/` (`GET /fhir/r4/Patient/{id}`, `GET /fhir/r4/Encounter/{id}`, etc.).
   - Validación de rendimiento, latencia y concurrencia integrada con suites de k6.
7. **Manejo de Errores:** Excepciones gestionadas estrictamente con `MedSysError` (cero `unwrap()` y cero `expect()` en código de producción).

---

## 2. Archivos Creados / Modificados en este Turno
- `docker/docker-compose.yml`: Definición de servicio PostgreSQL 16 Alpine, volumen persistente, montaje de esquema NOM-004 y healthcheck.
- `docker-compose.yml`: Archivo compose en la raíz para permitir arranque directo.
- `docker/.env.example` y `.env.example`: Plantillas de variables de entorno para Docker, SQLx y servidor Axum.
- `docker/.env` y `.env`: Configuración local de entorno para desarrollo inmediato (ignorado en git).
- `docker/start-db.ps1` y `docker/start-db.sh`: Scripts de arranque con comprobación del daemon de Docker y polling de healthcheck.
- `docker/stop-db.ps1` y `docker/stop-db.sh`: Scripts de detención limpia del contenedor.
- `docker/reset-db.ps1` y `docker/reset-db.sh`: Scripts para reseteo completo y reejecución de datos sintéticos eliminando volúmenes.
- `docker/README.md`: Documentación completa de acceso, credenciales, verificación con psql y estructura de tablas NOM-004.
- `BACKLOG.md`: Marcada Tarea 3.1 como completada `[x]`, Sprint 3 en curso.
- `.agents/backlog/sprint_3_sqlx_docker.md`: Tarea 3.1 marcada como completada `[x]` con nota técnica de cierre.
- `.agents/backlog/overview.md`: Progreso actualizado a 9/16 tareas (56.25%), Sprint 3 al 25%.
- `STATE.md`: Consolidación del estado del proyecto tras finalizar la Tarea 3.1.

---

## 3. Comando de Arranque para el Siguiente Turno
Para continuar de inmediato con la Tarea 3.2 del Sprint 3:
> "Lee .agents/rules/rules.md, .agents/orchestrator/workflow.md, STATE.md y BACKLOG.md. Continúa con la Tarea 3.2 del Sprint 3."
