# REGISTRO DE DECISIONES TÉCNICAS Y SUPUESTOS DE ADAPTACIÓN (`DECISIONS.md`)

> Este archivo conserva la trazabilidad de todas las decisiones arquitectónicas extraídas del plan maestro `.md` y de los supuestos adoptados por el Coordinador cuando el plan presenta vacíos no bloqueantes.

| ID | Fecha | Origen (`Plan` / `Adaptación` / `Usuario`) | Decisión o Supuesto | Impacto / Archivos Relacionados | Estado |
| :--- | :--- | :--- | :--- | :--- | :---: |
| **DEC-001** | 2026-10-14 | Sistema Maestro | Preservación estricta de solo lectura del archivo `.md` de plan maestro del usuario; toda gestión de estado se realiza en `STATE.md`, `BACKLOG.md` y `DECISIONS.md`. | `AGENTS.md`, `.agents/rules/00-orchestrator-core.md` | VIGENTE |
