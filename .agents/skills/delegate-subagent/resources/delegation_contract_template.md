# CONTRATO DE DELEGACIÓN A SUBAGENTE — Antigravity 2.0

- **Task ID:** `<ID de la tarea en BACKLOG.md>`
- **Subagent Role:** `<planner-analyst | architect-researcher | implementer | tester-qa | reviewer-auditor | documenter-integrator>`
- **Objective:** `<Descripción clara y acotada del resultado esperado>`
- **Relevant Context:** `<Resumen de 3-5 líneas del contexto necesario y decisiones de DECISIONS.md que debe respetar>`
- **Reference Files (Read-Only):**
  - `<ruta/archivo_referencia_1>`
  - `<ruta/archivo_referencia_2>`
- **Allowed Files to Modify/Create (Strict Scope):**
  - `<ruta/archivo_permitido_1>`
- **Technical Constraints & Guardrails:**
  - Prohibido modificar cualquier archivo fuera de `Allowed Files to Modify/Create`.
  - Respetar `.agents/rules/01-safety-and-autonomy.md`.
- **Acceptance Criteria & Verification Command:**
  - Comando a ejecutar: `<comando exacto de prueba/validación>`
  - Resultado esperado: `<código de salida 0 + criterio cuantitativo>`
- **Required Handover Output:**
  1. Lista exacta de archivos creados o modificados.
  2. Salida literal resumida del comando de verificación.
  3. Estado resultante (`[~]` Implementado, `[v]` Verificado o `[!]` Bloqueado con causa exacta).
