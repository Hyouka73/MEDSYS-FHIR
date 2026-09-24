# ESTADO ACTIVO DEL PROYECTO (STATE.md) — MedSys-FHIR

## Metadatos de Control
- **Última Actualización:** 2026-09-24T15:50:00-06:00
- **Sprint Activo:** Sprint 3 (`.agents/backlog/sprint_3_sqlx_docker.md`)
- **Estado General:** 8 / 16 tareas completadas (50%)
- **Tarea en Curso:** Ninguna (Sprint 2 concluido al 100%, especificación v1.1.0 sincronizada, listo para Tarea 3.1)
- **Última Tarea Cerrada:** Sincronización metodológica de especificación YAML v1.1.0 y backlog (9/9 tests PASS, Clippy 0 warnings, rustfmt PASS).
- **Siguiente Tarea Inmediata:** Tarea 3.1: Configuración de infraestructura Docker Compose con PostgreSQL 16 y esquema sintético NOM-004.
- **Estado del Build:** PASS (Compilación estática GNU/MinGW, Clippy sin warnings, cargo fmt en regla, tests al 100%).

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
5. **Endpoints HTTP y Pruebas de Carga (Sprint 4):**
   - Los endpoints REST FHIR operarán canónicamente bajo el prefijo `/fhir/r4/` (`GET /fhir/r4/Patient/{id}`, `GET /fhir/r4/Encounter/{id}`, etc.).
   - Validación de rendimiento, latencia y concurrencia integrada con suites de k6.
6. **Manejo de Errores:** Excepciones gestionadas estrictamente con `MedSysError` (cero `unwrap()` y cero `expect()` en código de producción).

---

## 2. Archivos Creados / Modificados en este Turno
- `mapping_rules_specification.yaml`: Actualizado a la versión 1.1.0 con separación de Observation y soporte declarativo de apellido materno.
- `.agents/backlog/overview.md`: Sincronizado a 8/16 tareas (50%), marcando Sprint 2 completado al 100%.
- `.agents/backlog/sprint_4_axum_dashboard.md`: Actualizado con prefijo canónico `/fhir/r4/` y pruebas de carga con k6 en Tarea 4.4.
- `BACKLOG.md`: Homologación de Sprint 4 (/fhir/r4/ y k6).
- `crates/medsys-core/src/engine/mod.rs`: Homologación de aserciones de prueba unitaria a la especificación v1.1.0 (9/9 tests PASS).
- `STATE.md`: Sincronización del estado del proyecto apuntando a `.agents/backlog/sprint_3_sqlx_docker.md`.

---

## 3. Comando de Arranque para el Siguiente Turno
Para continuar de inmediato con el Sprint 3:
> "Lee .agents/rules/rules.md, .agents/orchestrator/workflow.md, STATE.md y BACKLOG.md. Continúa con la Tarea 3.1 del Sprint 3."
