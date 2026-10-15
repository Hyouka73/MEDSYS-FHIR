---
name: close-milestone
description: Consolida la memoria persistente (STATE.md, BACKLOG.md y DECISIONS.md) al concluir una tarea verificada [v] o al cerrar una fase/sprint completo.
---

# Skill: Cierre de Tarea, Hito o Sprint (`close-milestone`)

## Cuándo Usar
- En el paso 15 del ciclo **`continúa`** inmediatamente después de que `verify-and-audit` certifique que una tarea está verificada (`[v]`).

## Procedimiento
1. En `BACKLOG.md`:
   - Cambia el marcador de la tarea de `[v]` (o `[/]` / `[~]` tras verificar) a `[x]`.
   - Añade al final de la línea o sub-bullet la evidencia resumida (ej. `*(Verificado: pytest 12/12 PASS)*`).
2. En `STATE.md`:
   - Actualiza `Última Actualización`, `Progreso Global`, `Última Tarea Cerrada`, `Archivos Modificados/Creados`, `Evidencia de Verificación` y `Siguiente Tarea Inmediata`.
3. En `DECISIONS.md`:
   - Si durante la tarea se tomó una decisión técnica relevante o se documentó una limitación, agrégala con fecha e ID correlativo (`DEC-XXX`).
