# Contrato de delegación — revisión independiente MedSys-FHIR

- Task ID: AUD-R5-REVIEW-20261025.
- Rol: reviewer-auditor; leer `.agents/agents/reviewer-auditor/agent.md` y reglas aplicables.
- Objetivo: auditar los cambios locales de runtime, instrumentos y evidencia; identificar afirmaciones no justificadas o defectos materiales antes de cerrar criterios.
- Contexto: commit base2933964; dos regresiones originales fallaron exit101 y después aprobaron2/2. Hay cambios posteriores aún en curso. CA-1–CA-5 y CT no se presumen aprobados.
- Referencias de sólo lectura: STATE.md, BACKLOG.md, DECISIONS.md, diff de crates/, scripts/, tools/experimental/ y auditoria/evidencias/. Consultar fuentes de tesis guardadas en auditoria/evidencias/2026-10-20-reverificacion/ si son necesarias.
- Allowed Files to Modify/Create: ninguno. Responder únicamente en stdout; el coordinador conservará el resultado en auditoria/evidencias/antigravity-review/.
- Restricciones: no editar ni ejecutar git add/commit/push/reset; no iniciar pruebas, servidores, contenedores ni modificar bases; no leer .env, keyrings, tokens, credenciales ni archivos ajenos al proyecto; no delegar nuevos agentes.
- Verificación permitida: lectura y búsqueda dirigida; git status/diff/show. No modificar archivos para demostrar un hallazgo.
- Criterios de aceptación: hallazgos concretos con archivo/línea, impacto y evidencia; distinguir inspección estática de salida ejecutada y de pendiente. No elevar nombres de pruebas o autopruebas a validación experimental.
- Entrega: resumen de alcance, hallazgos priorizados y límites. Si no hay defectos materiales demostrables, decirlo explícitamente sin declarar evaluación aprobada.
