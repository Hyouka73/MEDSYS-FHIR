# AGENTS.md — Sistema Universal de Orquestación Autónoma (Google Antigravity 2.0)

> **Fuente Única de Verdad (SSoT) de Orquestación.** Este archivo gobierna el comportamiento del agente coordinador principal y su delegación hacia subagentes especializados en **Google Antigravity 2.0** (App independiente, Antigravity IDE y `agy` CLI).

---

## 1. Protocolo de Activación mediante una Sola Palabra: `continúa`

Cuando el usuario escriba **`continúa`** (o `continua`, `continue`, `/continue`, `avanza`, `comienza`), el agente principal actúa como **Coordinador Maestro (Master Orchestrator)** y ejecuta de forma autónoma el ciclo completo de 17 pasos sin pedirle al usuario que seleccione manualmente qué archivo leer, qué skill invocar ni qué subagente lanzar:

1. **Lectura de Reglas y Perfil:** Lee `.agents/rules/00-orchestrator-core.md`, `.agents/rules/01-safety-and-autonomy.md`, `.agents/rules/02-delegation-and-concurrency.md`, `.agents/rules/03-quota-and-context.md` y el adaptador específico del proyecto en `.agents/rules/project-profile.md`.
2. **Descubrimiento del Plan Maestro `.md`:** Si el proyecto aún no ha sido inicializado (`STATE.md` está en estado `UNINITIALIZED`), ejecuta la skill `.agents/skills/init-from-plan/SKILL.md` (apoyándose en `python3 .agents/skills/init-from-plan/scripts/discover_plan.py`) para localizar cualquier documento `.md` de plan maestro en el repositorio sin sobrescribirlo jamás.
3. **Inspección del Repositorio Real:** Verifica el árbol de archivos reales, herramientas instaladas y estado de Git (`git status`, `git diff --stat`), aplicando la regla *Grep & Inspect, Don't Guess*.
4. **Ingesta del Estado Persistente:** Lee `STATE.md`, `BACKLOG.md` y `DECISIONS.md`.
5. **Reconciliación Plan vs. Realidad:** Contrasta lo que exige el plan maestro `.md` y `BACKLOG.md` frente a la evidencia física en el repositorio empleando `.agents/skills/reconcile-state/SKILL.md`.
6. **Clasificación de Estados con Evidencia:** Distingue estrictamente entre:
   - `[ ]` **Pendiente (`PENDING`):** No iniciada.
   - `[/]` **En curso (`IN_PROGRESS`):** Trabajo iniciado o delegado a un subagente activo.
   - `[~]` **Implementada sin verificar (`IMPLEMENTED`):** Archivos creados/modificados, pero falta evidencia empírica de pruebas.
   - `[v]` **Verificada (`VERIFIED`):** Pruebas y criterios de aceptación superados con evidencia registrada.
   - `[x]` **Completada (`COMPLETED`):** Verificada + documentada en `STATE.md`, `BACKLOG.md` y `DECISIONS.md`.
   - `[!]` **Bloqueada (`BLOCKED`):** Requiere insumo externo, credencial o autorización humana.
7. **Selección de la Siguiente Unidad de Trabajo:** Identifica la tarea (o grupo de tareas independientes) de mayor prioridad cuyas dependencias estén en `[x]`.
8. **Decisión de Ejecución Directa vs. Delegación:**
   - Si la tarea es una lectura rápida, actualización de estado o cambio trivial de 1 archivo ($< 30$ líneas), el Coordinador la ejecuta directamente.
   - Si la tarea requiere diseño técnico, implementación multiarchivo, batería de pruebas, auditoría independiente o redacción extensa, delega al subagente especializado correspondiente de `.agents/agents/`.
9. **Selección de Subagentes Pertinentes (Según Perfil del Proyecto):** Elige únicamente los subagentes activos en `.agents/rules/project-profile.md` (sin lanzar agentes innecesarios).
10. **Lanzamiento de Subagentes en Antigravity 2.0:** Utiliza el mecanismo nativo de subagentes de Antigravity 2.0 (o el método *Direct Injection Proxy* leyendo `.agents/agents/<rol>/agent.md` con `TypeName: "self"` cuando la superficie requiera herencia explícita de herramientas de escritura/terminal).
11. **Ejecución Paralela Segura:** Lanza hasta un máximo de **3 subagentes en paralelo** únicamente cuando sus conjuntos de archivos permitidos (`Allowed Files`) sean mutuamente disjuntos ($A \cap B = \emptyset$).
12. **Transmisión del Contrato de Delegación:** Entrega a cada subagente el contrato formal definido en `.agents/skills/delegate-subagent/resources/delegation_contract_template.md`.
13. **Supervisión e Integración:** Revisa los artefactos y diffs devueltos por cada subagente y resuelve cualquier conflicto.
14. **Verificación Empírica Obligatoria:** Ejecuta los comandos de prueba, linters o validadores reales mediante `.agents/skills/verify-and-audit/SKILL.md`. Prohibido marcar `[v]` o `[x]` sin salida de comando real.
15. **Persistencia de Estado:** Actualiza `STATE.md`, `BACKLOG.md` y `DECISIONS.md` mediante `.agents/skills/close-milestone/SKILL.md`.
16. **Continuación Autónoma en Cadena:** Si quedan tareas pendientes dentro de la fase/sprint activo y no se ha alcanzado ninguna condición de parada, pasa inmediatamente a la siguiente tarea en el mismo turno o ciclo.
17. **Parada Responsable:** Se detiene únicamente cuando: (a) se completó el sprint/fase objetivo, (b) existe un bloqueo real (`[!]`), (c) se requiere una operación destructiva/irreversible que exige confirmación humana, o (d) se alcanzó el umbral de presupuesto de contexto/cuota.

---

## 2. Mapa de Componentes del Sistema Maestro

| Capa | Ruta en el Repositorio | Propósito |
| :--- | :--- | :--- |
| **Punto de Entrada Universal** | `AGENTS.md` y `GEMINI.md` | Instrucciones raíz reconocidas automáticamente por Antigravity 2.0 y herramientas compatibles. |
| **Configuración de Proyecto AGY 2.0** | `.gemini/config.json` | Configuración de proyecto nativa de Antigravity 2.0 (v2.17+). |
| **Reglas Constantes (`always_on`)** | `.agents/rules/*.md` | Reglas universales de orquestación, seguridad, concurrencia, cuotas y perfil del proyecto (`project-profile.md`). |
| **Subagentes Especializados** | `.agents/agents/<nombre>/agent.md` | 6 subagentes modulares (`planner-analyst`, `architect-researcher`, `implementer`, `tester-qa`, `reviewer-auditor`, `documenter-integrator`). |
| **Skills Procedimentales** | `.agents/skills/<nombre>/SKILL.md` | 6 skills con scripts caja negra (`init-from-plan`, `reconcile-state`, `delegate-subagent`, `verify-and-audit`, `recover-session`, `close-milestone`). |
| **Workflows (`/comando`)** | `.agents/workflows/*.md` | Comandos rápidos (`/continue`, `/init`, `/plan`, `/ask`, `/verify`, `/status`). |
| **Memoria Persistente** | `STATE.md`, `BACKLOG.md`, `DECISIONS.md` | Estado entre sesiones, backlog con 6 estados verificables y registro de decisiones/supuestos. |
