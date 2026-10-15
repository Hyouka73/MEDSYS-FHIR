---
description: Flujo de trabajo autónomo para el desarrollo y evaluación de MedSys-FHIR.
---

# FLUJO DE TRABAJO AUTÓNOMO (WORKFLOW) — MedSys-FHIR (`v1.2.0-eval`)

## Descripción General
Este workflow gobierna la ejecución autónoma, incremental y sin pérdida de contexto para completar los **Sprints 5 al 9** del middleware **MedSys-FHIR** hasta alcanzar la versión congelada de evaluación experimental `v1.2.0-eval` (alineada al 100 % con los Capítulos I, II y III de la tesis).

Permite al usuario controlar todo el desarrollo con órdenes cortas como:
- `"Continúa"` o `"Avanza"` (ejecuta la siguiente tarea pendiente `[ ]`).
- `"Haz el Sprint 5"` (o `Sprint 6`, `Sprint 7`, `Sprint 8`, `Sprint 9`: ejecuta de principio a fin las 4 tareas de ese Sprint sin detenerse hasta dejarlo al 100 % PASS).
- `"Haz todos los Sprints pendientes"` (ejecuta secuencialmente desde la tarea actual hasta cerrar el Sprint 9).

---

## 1. El Ciclo de Ejecución Autónomo (Loop de 6 Fases)

```
[1. INGESTA DE ESTADO] ---> [2. ASIGNACIÓN ORQUESTADOR] ---> [3. DESARROLLO (CODER)]
          ^                                                               |
          |                                                               v
[6. CONSOLIDACIÓN MEMORIA] <-- [5. ACTUALIZACIÓN BACKLOG] <-- [4. VALIDACIÓN (QA)]
```

### Fase 1: Ingesta de Contexto y Estado Actual
1. Lee `STATE.md` y `BACKLOG.md` en la raíz del proyecto.
2. Identifica el Sprint activo (o el Sprint que el usuario pidió ejecutar) y abre su especificación detallada en `.agents/backlog/sprint_X_*.md`.
3. Localiza la primera tarea con estado `[ ]` (o `[/]`).

### Fase 2: Planificación Técnica (Rol: Orquestador)
1. Revisa los archivos exactos, firmas de funciones, estructuras YAML/SQL y criterios de aceptación listados en la tarea dentro de `.agents/backlog/sprint_X_*.md`.
2. Verifica que se respeten todas las invariantes de `.agents/rules/rules.md` (cero `unwrap()`, cero `fallback_value`, expansión 1:4 de `Observation`, códigos `IssueType` oficiales en `OperationOutcome`).

### Fase 3: Implementación (Rol: Desarrollador)
1. Modifica o crea únicamente los archivos requeridos por la tarea activa.
2. Implementa las pruebas unitarias o de integración correspondientes a los cambios realizados.

### Fase 4: Auditoría y Verificación (Rol: Validador)
1. Ejecuta la batería obligatoria:
   - `cargo check --workspace`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo test --workspace`
   - Comandos adicionales de Python/SQL/Docker especificados en la tarea.
2. Si aparece cualquier error o warning, corrígelo antes de avanzar a la Fase 5.

### Fase 5: Actualización del Backlog (Rol: Gestor de Backlog)
1. Cambia el estado de la tarea de `[ ]` a `[x]` tanto en `BACKLOG.md` como en `.agents/backlog/sprint_X_*.md` y actualiza la tabla de progreso en `.agents/backlog/overview.md`.
2. Registra la marca de tiempo de finalización y el resultado de las pruebas.

### Fase 6: Consolidación de Memoria (Rol: Gestor de Memoria)
1. Actualiza `STATE.md` con:
   - Última tarea cerrada.
   - Archivos creados o modificados.
   - Siguiente tarea inmediata.
   - Estado del build y conteo de tests.
2. Si el usuario pidió ejecutar un Sprint completo (ej. `"Haz el Sprint 5"`), repite inmediatamente el ciclo desde la Fase 1 con la siguiente tarea de ese Sprint hasta completar las 4 tareas del Sprint.

---

## 2. Comandos Rápidos del Usuario

| Comando del Usuario | Acción Autónoma del Agente |
| :--- | :--- |
| **`Continúa` / `Avanza`** | Lee `STATE.md` y `BACKLOG.md`, toma la siguiente tarea `[ ]`, la implementa, valida con `cargo test` y actualiza el estado. |
| **`Haz el Sprint 5`** *(o 6, 7, 8, 9)* | Ejecuta de corrido las 4 tareas del Sprint indicado, validando cada una y actualizando `BACKLOG.md` y `STATE.md` al finalizar. |
| **`Valida`** | Ejecuta `cargo fmt --check`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo test --workspace` y reporta el estado. |
| **`Estado`** | Muestra la tabla de avance de los Sprints 1 al 9 (`36 tareas` en total: 16 históricas + 20 de `v1.2.0-eval`). |
| **`Pausa`** | Guarda el progreso exacto en `STATE.md` y deja listo el repositorio para reanudar en otra sesión. |
