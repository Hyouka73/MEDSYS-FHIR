# ESTADO PERSISTENTE DEL PROYECTO (`STATE.md`)

## Detenido por el usuario — 2026-10-27

El usuario pidió detenerse y conocer el alcance y el tiempo restante. No continuar automáticamente. El colector de adaptación y su servidor se detuvieron durante la comparación S0; aún no se había iniciado inyección de fallos. S0 exportó 16,250 respuestas HTTP 200 y 250 respuestas 404 esperadas, con `export_report.json`; la comparación no terminó y S1–S3 no se iniciaron en esta corrida. Evidencias en `auditoria/evidencias/2026-10-27-run1/`. Ningún criterio completo CA-1–CA-5 ni CT queda aprobado por este avance.

La suite Rust tiene 104 pruebas aprobadas, el ejecutable congelado se identificó y los controles puntuales HTTP de los cuatro esquemas fueron 36/36. En una muestra de 28 recursos, cuatro Encounter omitieron `serviceType` esperado por el oráculo. No se han cambiado los YAML para ocultar ese resultado. El validador se encuentra incompleto y se pidió detener sus procesos. La configuración de Antigravity solicitada quedó instalada y dos auditorías registradas.

Pendiente cuando el usuario solicite reanudar: decidir alcance y prioridad; reanudar o repetir la comparación parcial con salida nueva, evaluar discrepancias, S1–S3, validación HL7, corpus HTTP, CA-4/5 y CT. CT prevista comprende al menos 21 minutos de carga efectiva; la duración total depende de ejecución, recursos del equipo y correcciones necesarias, sin promesa de término.

---

## Reanudación vigente — 2026-10-27

- Rama `main`, commit base `293396423cc993cc5b191d727ae0609e15a5aeb6`; cambios locales preservados y sin push.
- Suite completa de la versión corregida: 104 aprobadas, 0 fallos, 0 omitidas; compilación y clippy exitosos, evidencias `auditoria/evidencias/runtime-development/`. Perfiles dev/test sin símbolos debug; el ejecutable es no optimizado.
- Binario preservado en `output/evaluation/20261027/artifact-v1/medsys-server.exe`, SHA-256 `ea7e9e29bc730558cf0b0de993203bf0413d06c29478e273b6aa140eb8f5f9ad`. Identidad y hashes de fuentes/configuraciones/datos registrados en `auditoria/evidencias/2026-10-27-run1/artifact.json`; no se identifica con la etiqueta histórica ausente.
- PostgreSQL sintético persistente en Ubuntu WSL, puerto 55432; cuatro bases v2 y fuentes originales conservadas. Ensayo real de arranque con S0: health HTTP 200 y Patient/1 HTTP 200 FHIR, evidencia `2026-10-27-run1/smoke-results.json` y respuestas. No demuestra el censo completo.
- Dos revisiones independientes de Antigravity CLI terminaron; sus resultados y adjudicación están en `auditoria/evidencias/antigravity-review/`. Se corrigen dos problemas detectados en el recolector antes de la ejecución exhaustiva. Configuración guardada Flash 3.8 y ejecución de herramientas sin solicitudes según instrucción del usuario.
- Siguiente acción: ejecutar el mismo binario y cuatro YAML sobre las bases v2, capturar recursos reales y corpus, contrastar con oráculo y validador; caracterizar CT con las limitaciones efectivas. CA-1–CA-5 y CT permanecen pendientes; ninguna cifra histórica asignada cuenta como resultado nuevo.

---

## Reanudación vigente — 2026-10-25

- Antigravity CLI instalado por solicitud del usuario desde instalador oficial, `agy --version` =1.4.7, ruta usuario `%LOCALAPPDATA%/agy/bin/agy.exe`; checksum SHA512 contrastado por instalador. Terminal visible abierta para inicio de sesión. No se leen ni guardan tokens. Delegación externa pendiente autenticación, con contrato de archivos disjuntos y supervisión.
- Usuario confirmó sesión y solicitó ejecución sin autorizaciones. Preferencias CLI guardadas (copia previa preservada): modelo `Gemini 3.8 Flash (High)`, agentMode accept-edits, toolPermission/artifactReviewPolicy always-proceed, proyecto confiable. Launcher `.agents/tools/invoke-antigravity.ps1` fija modelo real y `--dangerously-skip-permissions`, conserva contrato/hash/resultado. Primera auditoría sólo lectura en ejecución; sus conclusiones requieren revisión del coordinador.
- Dos regresiones originales corregidas y verificadas independientemente: `regresiones-verificacion-coordinador.log`, exit0, 2/2. Antes exit101, 0/2. Harness estructural25oct:33 componentes,0 errores. Esto verifica esas regresiones unitarias, no cierra CA ni todos los hallazgos R5.

- Commit actual `2933964`, rama `main`; se preservan cambios locales de instrumentos/evidencias de la sesión interrumpida. No se hizo push.
- Baseline terminado: `regresiones-antes-sin-debug.log`, salida101, 0/2 aprobadas: S2 alias de sexo None en lugar de female; S3 MissingField temperatura_celsius. Defectos del código productivo previo; el commit2933964 sólo había añadido estas pruebas.
- Runtime se corrige en `crates/**` tras conservar baseline. Perfil de pruebas sin símbolos debug y `-j1` por memoria de laptop.
- Laboratorio anterior en `/tmp` ya no existe tras interrupción; sus logs se conservan y no acreditan servicio disponible hoy. Se recrea laboratorio sintético en ruta persistente dedicada, sin tocar cluster main.
- Runners/oráculo/consolidación modificados, autopruebas de instrumento anteriores13/13; pendientes de verificación independiente y corridas de Rust. Fixture oficial negativo muestra Error con Java exit0: se requieren incidencias estructuradas, no sólo código del proceso.
- Docker Desktop se volvió a iniciar para diagnóstico; evaluación CA-1–CA-5/CT sigue abierta. Próxima acción: completar arreglos/regresiones, conectar PostgreSQL+HTTP y congelar nuevo ejecutable antes del censo.

---


## Antecedentes preservados

El estado anterior está archivado en `auditoria/evidencias/reanudacion/STATE_ANTECEDENTES.md`. Sus cifras y marcadores históricos no acreditan la evaluación actual.
