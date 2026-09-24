# BACKLOG PRINCIPAL — MedSys-FHIR

## Cronograma Canónico del Proyecto (16 Tareas / 4 Sprints)

### Sprint 1: Motor de Mapeo Declarativo YAML y Monorepo Workspace (COMPLETADO)
- [x] **Tarea 1.1:** Inicialización del Monorepo Cargo Workspace (`Cargo.toml`) y estructura de crates (`medsys-core`, `medsys-db`, `medsys-server`).
- [x] **Tarea 1.2:** Definición del sistema centralizado de errores `MedSysError` y modelos de mapeo en `medsys-core`.
- [x] **Tarea 1.3:** Parser y deserialización de reglas YAML (`mapping_rules_specification.yaml`).
- [x] **Tarea 1.4:** Pruebas unitarias del motor de mapeo con macro `include_str!` en memoria (5/5 tests PASS).

### Sprint 2: Recursos HL7 FHIR R4 Canónicos (`helios-fhir`) (COMPLETADO)
- [x] **Tarea 2.1:** Modelado canónico del recurso `Patient` (CURP oficial y demografía).
- [x] **Tarea 2.2:** Modelado canónico del recurso `Encounter` (clase AMB y médico tratante).
- [x] **Tarea 2.3:** Modelado canónico del recurso `Observation` (códigos LOINC de signos vitales).
- [x] **Tarea 2.4:** Modelado canónico del recurso `Condition` (codificación internacional CIE-10).

### Sprint 3: Persistencia Asíncrona SQLx y Entorno Docker PostgreSQL 16 (COMPLETADO)
- [x] **Tarea 3.1:** Configuración de infraestructura Docker Compose con PostgreSQL 16 y esquema sintético.
- [x] **Tarea 3.2:** Configuración del pool asíncrono SQLx en `medsys-db`.
- [x] **Tarea 3.3:** Implementación de repositorios de lectura parametrizada ($1, $2).
- [x] **Tarea 3.4:** Pruebas de integración de persistencia sobre datos sintéticos.

### Sprint 4: API REST Axum, OperationOutcome y Dashboard React 19 (SIGUIENTE)
- [ ] **Tarea 4.1:** Enrutamiento HTTP en Axum y endpoints REST FHIR canónicos (`/fhir/r4/`).
- [ ] **Tarea 4.2:** Manejador global de excepciones traduciendo a `OperationOutcome`.
- [ ] **Tarea 4.3:** Dashboard interactivo en React 19 + Vite + Tailwind CSS.
- [ ] **Tarea 4.4:** Pruebas E2E de interoperabilidad, validación con k6, auditoría Clippy y release final.
