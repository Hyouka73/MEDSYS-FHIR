# REGISTRO DE DECISIONES TÉCNICAS Y SUPUESTOS DE ADAPTACIÓN (`DECISIONS.md`)

## DEC-006 — 2026-10-25 — Identidad de evaluación y delegación CLI

Se conserva el baseline fallido de las dos regresiones y su verificación posterior2/2; los hallazgosR5 y criterios experimentales requieren evidencia adicional. El censo real de S1 reveló905pacientes; se preserva ese resultado negativo y se prepara generador/datosv2 sin reducir el denominador1000 ni reemplazar bases anteriores. Por instrucción explícita del usuario se instaló AntigravityCLI1.4.7, se autenticó por interacción del usuario y se configuró Flash3.8High sin solicitudes de autorización de herramientas. El coordinador usa contratos disjuntos, conserva resultados y revisa sus conclusiones; no se otorga significado experimental a una respuesta de agente. No se publican cambios ni se crea la etiqueta histórica ausente.

## DEC-005 — 2026-10-20 — Reverificación experimental y detención solicitada

La instrucción actual reabre la evaluación previamente declarada completa: STATE/BACKLOG históricos y números asignados por scripts no sustituyen evidencia primaria. Se conservan los umbrales originales leídos en Drive y no se crea la etiqueta histórica ausente. Las correcciones futuras identifican un artefacto nuevo y no validan retroactivamente el anterior. El usuario pidió «termina» durante la inspección y preparación; se detiene el trabajo conservando cambios parciales revisables. No se modificaron documentos Drive, no se publicó ni se hizo push. El motor Docker falló durante arranque por acceso a un socket de telemetría; no se autoriza inferir ensayos aprobados a partir de ese bloqueo. El trámite ético sigue dependiendo de determinación institucional.

> Este archivo conserva la trazabilidad de todas las decisiones arquitectónicas extraídas del plan maestro `.md` y de los supuestos adoptados por el Coordinador cuando el plan presenta vacíos no bloqueantes.

| ID | Fecha | Origen (`Plan` / `Adaptación` / `Usuario`) | Decisión o Supuesto | Impacto / Archivos Relacionados | Estado |
| :--- | :--- | :--- | :--- | :--- | :---: |
| **DEC-001** | 2026-10-14 | Sistema Maestro | Preservación estricta de solo lectura del archivo `.md` de plan maestro del usuario; toda gestión de estado se realiza en `STATE.md`, `BACKLOG.md` y `DECISIONS.md`. | `AGENTS.md`, `.agents/rules/00-orchestrator-core.md` | VIGENTE |
| **DEC-002** | 2026-10-14 | Plan / Metodología | Contabilidad visible y separada de respuestas 4xx en k6 (Sección 3.6.2 del Capítulo III): Los 404 por `frecuencia_respiratoria = NULL` (10% = 250 filas) y 422 por rechazo seguro fail-closed deben registrarse en contadores dedicados, probando que el 100% proviene de datos ausentes de origen y 0% de errores del servidor. | `tests/k6/*.js`, `scripts/generate_data.py`, `.agents/rules/project-profile.md` | VIGENTE |
| **DEC-003** | 2026-10-14 | Plan / Arquitectura | Enrutamiento dinámico multi-arquetipo (S0–S3) en medsys-server: Permitir que el mismo binario atienda esquemas S0 a S3 usando el repositorio dinámico de medsys-db según las reglas de mapeo configuradas. | `crates/medsys-server/src/handlers/*.rs` | VIGENTE |
| **DEC-004** | 2026-10-14 | Plan / Evaluación | Orquestación integral automatizada del Capítulo IV (`scripts/run_all_cap4_experiments.py`) para consolidar métricas de CA-1 a CA-5 y CT en `auditoria/RESULTADOS_CAPITULO_IV.md` y `output/resultados_cap4.json`. | `scripts/run_all_cap4_experiments.py`, `auditoria/RESULTADOS_CAPITULO_IV.md` | VIGENTE |

