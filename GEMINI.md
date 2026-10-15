# GEMINI.md — Punto de Entrada Nativo para Google Antigravity 2.0

Lee y aplica obligatoriamente las directrices definidas en:
1. `./AGENTS.md` (Fuente Única de Verdad del Coordinador Maestro y protocolo de respuesta al comando **`continúa`**).
2. `./.agents/rules/` (Reglas auto-activadas mediante `trigger: always_on`).
3. `./STATE.md`, `./BACKLOG.md` y `./DECISIONS.md` (Memoria persistente del proyecto).

Cuando el usuario escriba **`continúa`** (o `/continue`), ejecuta inmediatamente el flujo de trabajo `.agents/workflows/continue.md`.
