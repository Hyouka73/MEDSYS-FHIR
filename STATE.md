# ESTADO ACTIVO DEL PROYECTO (STATE.md) — MedSys-FHIR

## Metadatos de Control
- **Última Actualización:** 2026-09-24T13:35:00-06:00
- **Sprint Activo:** Sprint 3 (`.agents/backlog/sprint_3_sqlx_persistence.md`)
- **Estado General:** 8 / 16 tareas completadas (50%)
- **Tarea en Curso:** Ninguna (Sprint 2 concluido al 100%, listo para Tarea 3.1)
- **Última Tarea Cerrada:** Tareas 2.1 - 2.4: Modelado canónico de recursos HL7 FHIR R4 (`Patient`, `Encounter`, `Observation`, `Condition`) con crate `helios-fhir` (9/9 tests PASS, Clippy sin warnings).
- **Siguiente Tarea Inmediata:** Tarea 3.1: Configuración de infraestructura Docker Compose con PostgreSQL 16 y esquema sintético NOM-004.
- **Estado del Build:** PASS (Compilación estática GNU/MinGW, Clippy sin warnings, tests al 100%).

---

## 1. Decisiones Arquitectónicas Establecidas
1. **Lenguaje y Stack:** Rust 2021, runtime Tokio, framework Axum, SQLx para persistencia de solo lectura en PostgreSQL 16.
2. **Entorno de Compilación:** Toolchain Rust GNU `stable-x86_64-pc-windows-gnu` con MinGW-w64 (`C:\msys64\mingw64\bin`) configurado en el entorno de usuario.
3. **Estándar:** HL7 FHIR R4 oficial vía crate `helios-fhir` v0.2 (`R4`).
4. **Modelos Canónicos Implementados:**
   - `Patient`: Identificador oficial CURP (`urn:oid:2.16.840.1.113883.4.629`, use "official"), desglose de nombres de pila y apellidos, mapeo normativo de género clínico (`male`, `female`, `other`), fecha de nacimiento y telecomunicación telefónica.
   - `Encounter`: Clasificación ambulatoria obligatoria `AMB` (`http://terminology.hl7.org/CodeSystem/v3-ActCode`), periodo de atención `start`/`end`, participante médico tratante con Cédula SEP (`http://cedulaprofesional.sep.gob.mx`).
   - `Observation`: Categoría `vital-signs` (`http://terminology.hl7.org/CodeSystem/observation-category`), Panel de Presión Arterial (LOINC `85354-9`) con subcomponentes sistólica (LOINC `8480-6`) y diastólica (LOINC `8462-4`) en `mmHg`, y Temperatura Corporal (LOINC `8310-5`) en unidad `Cel`.
   - `Condition`: Estados `clinicalStatus` ("active") y `verificationStatus` ("confirmed"/"provisional"), catálogo internacional CIE-10 (`http://hl7.org/fhir/sid/icd-10`) con descripción y fecha de registro.
5. **Mapeo:** Archivo declarativo YAML (`mapping_rules_specification.yaml`) interoperable con transformadores tipados en `crates/medsys-core/src/engine/transform.rs`.
6. **Manejo de Errores:** Excepciones gestionadas estrictamente con `MedSysError` (cero `unwrap()` y cero `expect()` en código de producción).

---

## 2. Archivos Creados / Modificados en este Turno
- `Cargo.toml`: Adición de dependencia `rust_decimal` al workspace.
- `crates/medsys-core/Cargo.toml`: Adición de `rust_decimal` para operaciones de precisión en somatometría y signos vitales.
- `crates/medsys-core/src/lib.rs`: Re-exportación canónica de los transformadores y modelos del Sprint 2.
- `crates/medsys-core/src/model/legacy.rs`: Modelos de datos del esquema legado relacional (`LegacyPaciente`, `LegacyConsulta`, `LegacySignoVital`, `LegacyDiagnostico`).
- `crates/medsys-core/src/model/fhir_helpers.rs`: Constructores seguros y ergonómicos para primitivas y tipos complejos de FHIR R4 (`fhir_string`, `fhir_code`, `fhir_uri`, `fhir_date`, `fhir_datetime`, `fhir_decimal`, `fhir_concept`, `fhir_reference`, `fhir_identifier`).
- `crates/medsys-core/src/engine/transform.rs`: Transformadores de negocio para `Patient`, `Encounter`, `Observation` (PA y temperatura) y `Condition`, además del serializador canónico `serialize_to_fhir_json`.
- `crates/medsys-core/src/engine/mod.rs`: Integración del módulo de transformación y suite de 9 pruebas unitarias exhaustivas.
- `.agents/backlog/sprint_2_helios_fhir.md`: Marcado al 100% de las 4 tareas del Sprint 2.
- `BACKLOG.md`: Actualización del progreso general al 50% (8 / 16 tareas).
- `STATE.md`: Consolidación del estado del sistema.

---

## 3. Comando de Arranque para el Siguiente Turno
Para continuar de inmediato con el Sprint 3:
> "Lee .agents/rules/rules.md, .agents/orchestrator/workflow.md, STATE.md y BACKLOG.md. Continúa con la Tarea 3.1 del Sprint 3."
