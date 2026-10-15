# REGISTRO DE DECISIONES TÉCNICAS Y SUPUESTOS DE ADAPTACIÓN (`DECISIONS.md`)

> Este archivo conserva la trazabilidad de todas las decisiones arquitectónicas extraídas del plan maestro `.md` y de los supuestos adoptados por el Coordinador cuando el plan presenta vacíos no bloqueantes.

| ID | Fecha | Origen (`Plan` / `Adaptación` / `Usuario`) | Decisión o Supuesto | Impacto / Archivos Relacionados | Estado |
| :--- | :--- | :--- | :--- | :--- | :---: |
| **DEC-001** | 2026-10-14 | Sistema Maestro | Preservación estricta de solo lectura del archivo `.md` de plan maestro del usuario; toda gestión de estado se realiza en `STATE.md`, `BACKLOG.md` y `DECISIONS.md`. | `AGENTS.md`, `.agents/rules/00-orchestrator-core.md` | VIGENTE |
| **DEC-002** | 2026-10-14 | Plan / Metodología | Contabilidad visible y separada de respuestas 4xx en k6 (Sección 3.6.2 del Capítulo III): Los 404 por `frecuencia_respiratoria = NULL` (10% = 250 filas) y 422 por rechazo seguro fail-closed deben registrarse en contadores dedicados, probando que el 100% proviene de datos ausentes de origen y 0% de errores del servidor. | `tests/k6/*.js`, `scripts/generate_data.py`, `.agents/rules/project-profile.md` | VIGENTE |
| **DEC-003** | 2026-10-14 | Plan / Arquitectura | Enrutamiento dinámico multi-arquetipo (S0–S3) en medsys-server: Permitir que el mismo binario atienda esquemas S0 a S3 usando el repositorio dinámico de medsys-db según las reglas de mapeo configuradas. | `crates/medsys-server/src/handlers/*.rs` | VIGENTE |
| **DEC-004** | 2026-10-14 | Plan / Evaluación | Orquestación integral automatizada del Capítulo IV (`scripts/run_all_cap4_experiments.py`) para consolidar métricas de CA-1 a CA-5 y CT en `auditoria/RESULTADOS_CAPITULO_IV.md` y `output/resultados_cap4.json`. | `scripts/run_all_cap4_experiments.py`, `auditoria/RESULTADOS_CAPITULO_IV.md` | VIGENTE |

