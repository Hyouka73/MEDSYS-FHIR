# Ejemplo Mínimo de Selección y Lanzamiento de Subagentes (Proyecto Nuevo)

Cuando el usuario coloca `MASTER_PLAN_INVENTORY_API.md` en un proyecto nuevo y escribe **`continúa`**, ocurre exactamente esta secuencia:

1. **Detección del Plan (`init-from-plan`):**
   - El Coordinador ejecuta `python3 .agents/skills/init-from-plan/scripts/discover_plan.py --root . --json`.
   - Detecta `MASTER_PLAN_INVENTORY_API.md` (sin modificarlo) y delega al subagente **`planner-analyst`** la creación de `BACKLOG.md` con 3 tareas (Tarea 1.1 Modelos, Tarea 1.2 Endpoints, Tarea 1.3 Pruebas `pytest`) y configura `.agents/rules/project-profile.md` para Python/FastAPI.
2. **Delegación de Diseño e Implementación (`architect-researcher` $\rightarrow$ `implementer`):**
   - Al avanzar a la Tarea 1.1 y 1.2, el Coordinador emite el **Contrato de Delegación** al subagente **`implementer`**:
     - `Task ID`: `Tarea 1.1 y 1.2`
     - `Allowed Files to Modify/Create`: `app/models.py`, `app/schemas.py`, `app/main.py`
     - `Verification Command`: `python3 -m py_compile app/main.py`
   - El subagente `implementer` escribe los archivos y devuelve el estado `[~]` (Implementado).
3. **Delegación de Verificación Empírica (`tester-qa`):**
   - El Coordinador lanza al subagente **`tester-qa`** con alcance en `tests/test_api.py` y comando `pytest -v`.
   - `tester-qa` ejecuta `pytest`, comprueba que `RF-01`, `RF-02` y `RF-03` pasan al 100 % y devuelve la evidencia real marcando `[v]` (Verificado).
4. **Cierre y Continuidad (`close-milestone`):**
   - El Coordinador actualiza `BACKLOG.md` a `[x]` y registra el avance en `STATE.md`.
