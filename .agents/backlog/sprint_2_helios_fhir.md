# SPRINT 2: Recursos HL7 FHIR R4 Canónicos (helios-fhir) — MedSys-FHIR

## Objetivo del Sprint
Modelar canónicamente los 4 recursos HL7 FHIR R4 requeridos (`Patient`, `Encounter`, `Observation`, `Condition`) empleando el crate `helios-fhir` (feature `r4`), implementando los transformadores de tipos clínicos (CURP oficial, códigos LOINC, clasificaciones CIE-10) y serialización canónica en `application/fhir+json`. (COMPLETADO AL 100%)

---

## Lista de Tareas

- [x] **Tarea 2.1: Modelado Canónico del Recurso Patient**
  - Implementado mapeo de `tbl_pacientes` hacia `helios_fhir::r4::Patient`.
  - Integrado identificador nacional oficial CURP (`urn:oid:2.16.840.1.113883.4.629`, use "official") y mapeo de género clínico (male/female/other).
  - *Finalizado:* 2026-09-24T13:30:00-06:00.

- [x] **Tarea 2.2: Modelado Canónico del Recurso Encounter**
  - Implementado mapeo de `tbl_consultas` hacia `helios_fhir::r4::Encounter`.
  - Configurada clase ambulatoria "AMB" (`http://terminology.hl7.org/CodeSystem/v3-ActCode`), periodo y cédula del médico tratante (`http://cedulaprofesional.sep.gob.mx`).
  - *Finalizado:* 2026-09-24T13:30:00-06:00.

- [x] **Tarea 2.3: Modelado Canónico del Recurso Observation (Signos Vitales)**
  - Implementado mapeo de `tbl_signos_vitales` hacia `helios_fhir::r4::Observation`.
  - Mapeada categoría `vital-signs`, panel de presión arterial con códigos LOINC 8480-6 / 8462-4 en mmHg y temperatura corporal con código 8310-5 en unidad Cel.
  - *Finalizado:* 2026-09-24T13:30:00-06:00.

- [x] **Tarea 2.4: Modelado Canónico del Recurso Condition (Diagnósticos CIE-10)**
  - Implementado mapeo de `tbl_diagnosticos` hacia `helios_fhir::r4::Condition`.
  - Mapeados estados clínicos ("active") y de verificación ("confirmed"/"provisional"), codificación oficial CIE-10 (`http://hl7.org/fhir/sid/icd-10`) y fecha de registro.
  - *Finalizado:* 2026-09-24T13:30:00-06:00.
