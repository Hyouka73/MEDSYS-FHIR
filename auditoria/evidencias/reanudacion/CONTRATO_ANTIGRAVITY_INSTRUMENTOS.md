# Contrato AUD-INSTRUMENTOS-20261025

Rol reviewer-auditor de `.agents/agents/reviewer-auditor/agent.md`. Tarea acotada: revisar sólo instrumentos para detectar aprobaciones falsas, pérdida de evidencia, errores de denominador o peligros de restauración. No hacer una auditoría general del proyecto.

Allowed Files to Modify/Create: ninguno. Responder únicamente por stdout; no delegar agentes ni hacer pruebas/servidores/SQL/ediciones. No leer .env, tokens, keyrings o archivos externos al repositorio. Permite lectura, búsqueda y git diff/show.

Archivos pertinentes: scripts/experimental_evidence.py, scripts/oracle_verifier.py, scripts/run_all_cap4_experiments.py, scripts/test_experimental_evidence.py, tools/experimental/evidence_runners.py y tools/experimental/aggregate_ct.py. Puedes leer sus informes de autoprueba, sin confundirlos con evaluación de Rust.

Concentrarse en fallos demostrables por código: exportación conserva cuerpo original, cero discrepancias requiere censo real, mutaciones no son corrección del middleware, corpus conserva errores de inyección/restauración, reportesHL7 se asocian por recurso, CT no decide viabilidad. Devolver como máximo5hallazgos con archivo/línea/mecanismo, o indicar que no hay fallos demostrables en el alcance. Evitar predicciones numéricas sin ejecutar. Máximo5minutos; no repetir contradicciones históricas ya archivadas ni afirmar un estado de build a partir de logs en curso.
