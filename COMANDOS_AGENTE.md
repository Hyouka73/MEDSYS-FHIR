# GUÍA RÁPIDA DE USO — CÓMO PONER A CHAMBEAR A TU AGENTE EN `MedSys-FHIR`

## 1. Instalación en tu Repositorio Local
1. Descomprime el archivo `medsys_fhir_agents_v1.2.0_eval.zip` directamente en la raíz de tu carpeta `MEDSYS-FHIR` (reemplazando la carpeta `.agents/`, `STATE.md` y `BACKLOG.md`).
2. Abre tu editor/agente de código (Cursor, Windsurf, VS Code Copilot, Claude Code o Antigravity) en la raíz del repositorio.

---

## 2. Frases Exactas para Ordenarle al Agente

### Opción A: Para que haga un Sprint completo él solito de principio a fin
Copia y pega cualquiera de estos comandos según el Sprint que quieras que ejecute:

- **Para el Sprint 5 (Corrige los 2 tests rotos, elimina `fallback_value`, agrega las 4 `Observation` y homologa `OperationOutcome`):**
  > `Lee .agents/rules/rules.md, STATE.md y BACKLOG.md. Haz el Sprint 5 completo (Tareas 5.1 a 5.4) validando con cargo test y clippy.`

- **Para el Sprint 6 (Motor YAML v1.2, SQL dinámico con allowlist, transacciones READ ONLY y llaves foráneas T4):**
  > `Lee STATE.md y .agents/backlog/sprint_6_dsl_v1_2_sql_dinamico.md. Haz el Sprint 6 completo (Tareas 6.1 a 6.4).`

- **Para el Sprint 7 (Esquemas SQL S0 a S3, reglas YAML S0-S3 y generador determinista de 16,250 recursos):**
  > `Lee STATE.md y .agents/backlog/sprint_7_arquetipos_s1_s3_semilla.md. Haz el Sprint 7 completo (Tareas 7.1 a 7.4).`

- **Para el Sprint 8 (Oráculo diferencial en Python con mutaciones y Corpus de 100 fallas clínicas T1-T5):**
  > `Lee STATE.md y .agents/backlog/sprint_8_oraculo_y_corpus_t1_t5.md. Haz el Sprint 8 completo (Tareas 8.1 a 8.4).`

- **Para el Sprint 9 (Límites Docker cgroups, exportador de 16,250 JSON, arreglo de scripts PowerShell k6 y cierre v1.2.0-eval):**
  > `Lee STATE.md y .agents/backlog/sprint_9_docker_k6_censo_congelamiento.md. Haz el Sprint 9 completo (Tareas 9.1 a 9.4).`

---

### Opción B: Para avanzar paso por paso (una tarea a la vez)
Si prefieres ir revisando tarea por tarea, solo escríbele:
> `Continúa`

Y cuando quieras ver en qué tarea va o cuánto falta:
> `Estado`
