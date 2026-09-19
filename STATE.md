# ESTADO ACTIVO DEL PROYECTO (STATE.md) — MedSys-FHIR

## Metadatos de Control
- **Última Actualización:** 2026-09-19T18:00:00-06:00
- **Sprint Activo:** Sprint 1 (`.agents/backlog/sprint_1_yaml_engine.md`)
- **Estado General:** 0 / 16 tareas completadas (0%)
- **Tarea en Curso:** Tarea 1.1: Inicialización del Monorepo Cargo Workspace (`Cargo.toml`)
- **Última Tarea Cerrada:** Inicialización del marco multi-agente modular y vinculación con Git remoto.
- **Siguiente Tarea Inmediata:** Tarea 1.1: Inicialización del Monorepo Cargo Workspace (`Cargo.toml`).

---

## 1. Decisiones Arquitectónicas Establecidas
1. **Lenguaje y Stack:** Rust 2021, runtime Tokio, framework Axum, SQLx para persistencia de solo lectura en PostgreSQL 16.
2. **Estándar:** HL7 FHIR R4 oficial vía crate `helios-fhir` con feature flag `r4`.
3. **Mapeo:** Archivo declarativo YAML (`mapping_rules_specification.yaml`) deserializado con Serde. Salida hacia clientes en FHIR JSON oficial.
4. **Manejo de Errores:** Excepciones traducidas obligatoriamente al recurso FHIR `OperationOutcome`.
5. **Backlog:** Arquitectura modular en `.agents/backlog/` dividida por sprints individuales (16 tareas en 4 sprints).

---

## 2. Archivos Recientes de Referencia
- `.agents/orchestrator/workflow.md`: Flujo de trabajo autónomo.
- `.agents/rules/rules.md`: Reglas del sistema y desarrollo.
- `.agents/backlog/overview.md`: Panorama general de avance.
- `.agents/backlog/sprint_1_yaml_engine.md`: Backlog del Sprint 1 activo.
- `BACKLOG.md`: Backlog principal raíz.
- `schema_legado_simulado_nom004.sql`: Script DDL con tablas y datos sintéticos.
- `mapping_rules_specification.yaml`: Especificación de mapeo de los 4 recursos.

---

## 3. Comando de Arranque para el Siguiente Turno
Para continuar de inmediato sin explicaciones previas, solo indica al agente:
> "Lee .agents/orchestrator/workflow.md, STATE.md y BACKLOG.md. Continúa con la siguiente tarea."
