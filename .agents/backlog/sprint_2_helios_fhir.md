# SPRINT 2: Recursos HL7 FHIR R4 Canónicos (helios-fhir) — MedSys-FHIR

## Objetivo del Sprint
Modelar canónicamente los 4 recursos HL7 FHIR R4 requeridos (`Patient`, `Encounter`, `Observation`, `Condition`) empleando el crate `helios-fhir` (feature `r4`), implementando los transformadores de tipos clínicos (CURP oficial, códigos LOINC, clasificaciones CIE-10) y serialización canónica en `application/fhir+json`.

---

## Lista de Tareas

- [ ] **Tarea 2.1: Modelado Canónico del Recurso Patient**
  - Implementar mapeo de `tbl_pacientes` hacia `helios_fhir::r4::Patient`.
  - Integrar identificador nacional oficial CURP (`urn:oid:2.16.840.1.113883.4.629`, use "official") y mapeo de género clínico.

- [ ] **Tarea 2.2: Modelado Canónico del Recurso Encounter**
  - Implementar mapeo de `tbl_consultas` hacia `helios_fhir::r4::Encounter`.
  - Configurar clase ambulatoria "AMB" (`http://terminology.hl7.org/CodeSystem/v3-ActCode`), periodo y cédula del médico tratante (`http://cedulaprofesional.sep.gob.mx`).

- [ ] **Tarea 2.3: Modelado Canónico del Recurso Observation (Signos Vitales)**
  - Implementar mapeo de `tbl_signos_vitales` hacia `helios_fhir::r4::Observation`.
  - Mapear categoría `vital-signs`, panel de presión arterial con códigos LOINC 8480-6 / 8462-4 y temperatura corporal con código 8310-5 en unidad Cel.

- [ ] **Tarea 2.4: Modelado Canónico del Recurso Condition (Diagnósticos CIE-10)**
  - Implementar mapeo de `tbl_diagnosticos` hacia `helios_fhir::r4::Condition`.
  - Mapear estados clínicos y de verificación, codificación oficial CIE-10 (`http://hl7.org/fhir/sid/icd-10`) y fecha de registro.
