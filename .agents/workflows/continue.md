---
description: Ejecuta el bucle autónomo completo (17 pasos) a partir del estado actual y del plan maestro. Equivalente a escribir «continúa».
---

# Workflow Autónomo Principal (`/continue` o `continúa`)

1. Ejecuta `python3 .agents/skills/reconcile-state/scripts/reconcile_status.py --root . --json`.
2. Si `STATE.md` está en estado `UNINITIALIZED`, aplica primero la skill `init-from-plan` (`python3 .agents/skills/init-from-plan/scripts/discover_plan.py --root . --json`) para inicializar el backlog y el perfil desde el archivo `.md` del proyecto sin sobrescribir el plan original.
3. Toma la tarea prioritaria indicada por `reconcile_status.py` (y hasta 2 tareas independientes adicionales si sus alcances de archivos son disjuntos).
4. Marca la tarea como `[/]` en `BACKLOG.md` y `STATE.md`.
5. Decide si ejecutar directamente (cambios triviales $< 30$ líneas) o delegar al subagente correspondiente de `.agents/agents/<rol>/agent.md` mediante la skill `delegate-subagent`.
6. Una vez implementada (`[~]`), ejecuta obligatoriamente la verificación empírica real (`verify-and-audit`) para pasarla a `[v]`.
7. Ejecuta `close-milestone` para marcar `[x]` en `BACKLOG.md` y actualizar `STATE.md`.
8. Si quedan tareas pendientes en el sprint/fase activa y no hay bloqueos `[!]` ni operaciones de Nivel Rojo pendientes de aprobación humana, continúa automáticamente con la siguiente tarea.
