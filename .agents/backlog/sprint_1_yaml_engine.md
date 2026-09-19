# SPRINT 1: Motor de Mapeo Declarativo YAML y Workspace — MedSys-FHIR

## Objetivo del Sprint
Establecer la estructura monorepo Cargo Workspace, el crate de dominio puro `medsys-core`, el sistema centralizado de errores (`MedSysError`), y la deserialización tipada y validación de las reglas declarativas definidas en `mapping_rules_specification.yaml`.

---

## Lista de Tareas

- [ ] **Tarea 1.1: Inicialización del Monorepo Cargo Workspace**
  - Crear `Cargo.toml` raíz con `[workspace]`, `members` y `[workspace.dependencies]`.
  - Configurar esqueletos de crates `medsys-core`, `medsys-db`, y `medsys-server`.
  - Criterio de aceptación: `cargo check --workspace` compila exitosamente.

- [ ] **Tarea 1.2: Definición del Sistema Centralizado de Errores y Modelos**
  - Implementar enum `MedSysError` usando `thiserror` en `crates/medsys-core/src/error.rs`.
  - Tipar variantes para errores de parseo YAML, reglas inválidas y transformaciones.
  - Criterio de aceptación: Código sin `unwrap()` ni `expect()`.

- [ ] **Tarea 1.3: Parser y Deserialización de Reglas Declarativas YAML**
  - Definir estructuras Serde (`MappingRules`, `ResourceMapping`, `FieldMapping`, `TransformRule`) en `crates/medsys-core/src/model/mapping.rs`.
  - Implementar parser en `crates/medsys-core/src/engine/mod.rs` que cargue y valide la especificación YAML.
  - Criterio de aceptación: Deserialización completa de los 4 recursos sin pérdida de atributos.

- [ ] **Tarea 1.4: Pruebas Unitarias del Motor de Reglas en Memoria**
  - Escribir suite de pruebas unitarias en `crates/medsys-core/src/engine/tests.rs` empleando `include_str!("../../../mapping_rules_specification.yaml")`.
  - Validar estáticamente las tablas fuentes, claves primarias y campos de los recursos `Patient`, `Encounter`, `Observation` y `Condition`.
  - Criterio de aceptación: `cargo test -p medsys-core` pasa al 100%.
