# BACKLOG MAESTRO DE EJECUCIÓN (`BACKLOG.md`)

> **Leyenda de Estados Verificables:**
> - `[ ]` **Pendiente (`PENDING`):** No iniciada.
> - `[/]` **En progreso (`IN_PROGRESS`):** En ejecución o delegada a un subagente activo.
> - `[~]` **Implementada (`IMPLEMENTED`):** Cambios realizados, pendiente de correr verificación empírica.
> - `[v]` **Verificada (`VERIFIED`):** Comandos de prueba ejecutados con éxito (evidencia real).
> - `[x]` **Completada (`COMPLETED`):** Verificada y consolidada en `STATE.md`.
> - `[!]` **Bloqueada (`BLOCKED`):** Requiere insumo externo o autorización humana de Nivel Rojo.

---

## Fase 0: Inicialización y Adaptación Automática del Proyecto
- [ ] **Tarea 0.1:** Ejecutar `init-from-plan` (`python3 .agents/skills/init-from-plan/scripts/discover_plan.py --root . --json`) para localizar el plan maestro `.md`, inspeccionar el repositorio sin alterar el plan original y poblar `.agents/rules/project-profile.md`, `DECISIONS.md` y las fases operativas de este `BACKLOG.md`.
