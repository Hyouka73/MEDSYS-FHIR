---
name: reconcile-state
description: Reconcilia el estado persistente (STATE.md y BACKLOG.md) frente a los archivos reales del repositorio y extrae la siguiente tarea ejecutable. Úsese al inicio de cada ciclo «continúa».
---

# Skill: Reconciliación de Estado y Selección de Tareas (`reconcile-state`)

## Cuándo Usar
- En los pasos 4, 5, 6 y 7 del ciclo **`continúa`**.
- Después de recuperar una sesión interrumpida.

## Procedimiento
1. Ejecuta el analizador de estado y backlog:
   ```bash
   python3 .agents/skills/reconcile-state/scripts/reconcile_status.py --root .
   ```
2. Revisa el conteo de tareas por estado (`[ ]`, `[/]`, `[~]`, `[v]`, `[x]`, `[!]`) y la primera tarea accionable reportada por el script.
3. Si existe alguna tarea en `[~]` (Implementada pero no verificada), prioriza inmediatamente su verificación con `verify-and-audit` antes de abrir una tarea nueva `[ ]`.
4. Si existe alguna tarea en `[/]` (En progreso de una sesión previa interrumpida), inspecciona los archivos modificados con `git status` / `git diff` y retómala sin rehacer desde cero lo que ya esté bien escrito.
