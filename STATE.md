# ESTADO ACTIVO DEL PROYECTO (STATE.md) — MedSys-FHIR

## Metadatos de Control
- **Última Actualización:** 2026-09-20T13:21:00-06:00
- **Sprint Activo:** Sprint 2 (`.agents/backlog/sprint_2_helios_fhir.md`)
- **Estado General:** 4 / 16 tareas completadas (25%)
- **Tarea en Curso:** Ninguna (Sprint 1 concluido, listo para Tarea 2.1)
- **Última Tarea Cerrada:** Tarea 1.4: Pruebas unitarias del motor de mapeo con macro `include_str!` en memoria (5/5 PASS).
- **Siguiente Tarea Inmediata:** Tarea 2.1: Modelado canónico del recurso `Patient` (CURP oficial y demografía).
- **Estado del Build:** PASS (Compilación estática, Clippy sin warnings, tests al 100%).

---

## 1. Decisiones Arquitectónicas Establecidas
1. **Lenguaje y Stack:** Rust 2021, runtime Tokio, framework Axum, SQLx para persistencia de solo lectura en PostgreSQL 16.
2. **Estándar:** HL7 FHIR R4 oficial vía crate `helios-fhir` v0.2 con feature flag `R4`.
3. **Mapeo:** Archivo declarativo YAML (`mapping_rules_specification.yaml`) deserializado con Serde en memoria.
4. **Manejo de Errores:** Excepciones gestionadas estrictamente con `MedSysError` (sin `unwrap()` ni `expect()` en producción).
5. **Backlog:** Arquitectura modular en `.agents/backlog/` dividida en 4 Sprints canónicos (16 tareas).

---

## 2. Archivos Creados / Modificados en este Turno
- `Cargo.toml`: Configuración del Cargo Workspace y dependencias compartidas.
- `crates/medsys-core/*`: Núcleo de dominio, modelos `MappingRules`, `MedSysError` y motor de parseo/validación con tests unitarios.
- `crates/medsys-db/*`: Esqueleto inicial del adaptador de persistencia.
- `crates/medsys-server/*`: Esqueleto del servidor HTTP Axum.
- `.agents/backlog/*`: Backlog modular por Sprints (`overview.md`, `sprint_1_yaml_engine.md`, `sprint_2_helios_fhir.md`, etc.).
- `BACKLOG.md`: Backlog unificado raíz.
- `.agents/orchestrator/workflow.md`: Protocolo de orquestación autónoma.

---

## 3. Comando de Arranque para el Siguiente Turno
Para continuar de inmediato sin explicaciones previas, solo indica al agente:
> "Lee .agents/orchestrator/workflow.md, STATE.md y BACKLOG.md. Continúa con la Tarea 2.1 del Sprint 2."
