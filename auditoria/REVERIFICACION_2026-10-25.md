# Reverificación técnica MedSys-FHIR — 25 de octubre de 2026

Informe en curso. Base Git local: `293396423cc993cc5b191d727ae0609e15a5aeb6`, rama `main`, con cambios posteriores sin publicar. El cierre histórico40/40 y ALL_TESTS_PASSING no son el dictamen vigente. No se creó la etiqueta histórica ausente `v1.2.0-eval`.

### Avance posterior comprobado

- Runtime: 75 pruebas aprobadas (35 core, 33 db, 2 server y 5 regresiones del contrato), ninguna omitida, salida 0 en `evidencias/runtime-development/runtime-after.log`. Se está verificando además la precisión fecha-only en timestamps S3 antes de congelar el ejecutable.
- Datos corregidos: bases nuevas `medsys_eval_v2_s0`–`s3`, marcador `reeval20261025-v2`. S1 anterior conserva 905 pacientes; S1 v2 contiene 1,000 en 2,500 filas. SELECT permitido/DML denegado para medsys_ro. Evidencia: `evidencias/harness-development/cohorts-database-verification-20261025.json`. Las fuentes v1 no se reemplazaron.
- WSL volvió a fallar con `HCS_E_CONNECTION_TIMEOUT` durante la coexistencia con Docker. Detener sólo Docker Desktop devolvió inmediatamente Ubuntu y PostgreSQL a un estado que acepta conexiones. No se afirma que esté resuelta definitivamente la estabilidad de coexistencia ni el circuito en contenedores.
- Antigravity se comprobó por comando interno `/model`: salida 0 y `gemini-3.8-flash-high`, sin turnos de modelo ni consumo de tokens. Su primera revisión continúa; no se presume concluida.
- Hay un paquete de revisión provisional con manifiesto SHA256: `revision/medsys-review-20261026T035404Z.zip` (hora UTC; 25 de octubre local). Es un checkpoint, no un cierre experimental; se actualizará después de las corridas pertinentes.

## Qué ocurrió y qué quedó comprobado

La intervención anterior añadió dos regresiones, preservó fuentes documentales y se detuvo por instrucción del usuario. El commit2933964 no corrigió producción. Tras reanudar, la compilación inicial con símbolos debug sufrió paginación extrema y fue interrumpida: eso no es un fallo funcional. La compilación con perfiles dev/test debug0 y `-j1` terminó y ambas regresiones fallaron con salida101.

| Prueba en crates/medsys-db/src/dynamic_repo.rs | Antes | Después de corrección |
| --- | --- | --- |
| regression_s2_lookup_aliases_and_required_encounter_fields | sexo None en lugar de female | PASS; incluye alias, desbordamiento, estado requerido y fin inválido |
| regression_s3_actual_columns_and_optional_corruption | MissingField temperatura_celsius pese a temp_celsius presente | PASS; incluye columnas reales S3 y FR corrupta |

Evidencia primaria: `evidencias/reanudacion/regresiones-antes-sin-debug.log` y `.exit.txt` (101); `evidencias/runtime-development/regressions-after.log`; repetición independiente `evidencias/reanudacion/regresiones-verificacion-coordinador.log` y `.exit.txt` (0,2/2). Esto demuestra defectos existentes y esas regresiones corregidas; no demuestra todavía el conjunto del contrato HTTP ni CA-1–CA-5.

## Docker y PostgreSQL

Los comandos iniciales docker version/info no alcanzaban el pipe dockerDesktopLinuxEngine. El backend fallaba al retirar userAnalyticsOtlpHttp.sock: The file cannot be accessed by the system. Windows también devolvió Error1920 al consultar/mover ese archivo. Con Docker detenido se preservó toda su carpeta temporal run como run.reverificacion-preservado y se recreó run. No se borraron bases ni volúmenes. Posteriormente apareció timeout de integración Ubuntu, informado por el usuario y registrado por backend. El25oct se volvió a iniciar Desktop y el daemon respondió27.4.0/cgroupsv2. Esto acredita disponibilidad del motor, todavía no todo el circuito SQL/HTTP del ensayo en contenedores.

docker/start-db.ps1 y docker/README.md seleccionan docker/docker-compose.yml, con build ../docker/Dockerfile.server y puertoHTTP predeterminado8080. DEPLOYMENT.md usa el Compose raíz, Dockerfile raíz y puerto3000. Ambos config se resolvieron; no se mezclaron para inicializar datos ajenos. El Compose nuevo tools/experimental/lab-compose.yaml define proyecto/red/volumen dedicados y puertoPG55433 en loopback; sólo se validó su configuración hasta este checkpoint.

Se instaló PostgreSQL16 en UbuntuWSL y se prepararon cuatro bases sintéticas. El cluster previo en /tmp desapareció entre sesiones; sus logs se conservaron. El25oct se verificó un cluster persistente dedicado `/var/lib/postgresql/medsys-eval-2933964`, puerto55432, bases medsys_eval_s0..s3. Evidencia desdeWindows: `evidencias/lab-development/WINDOWS_VERIFICATION_25OCT.json`; SELECT permitido y DML denegado para medsys_ro en fuentes. No se tocó el cluster main.

## Instrumentos y datos fuente

Se corrigió el oráculo que antes igualaba instantes distintos al eliminar offsets y aceptaba reasonCode alterado. Se eliminó la consolidación de cifras asignadas; la falta de evidencia, hashes incorrectos y cobertura incompleta son errores visibles. Se verificaron15/15 regresiones de instrumentos: `evidencias/reanudacion/instrumentos-25oct-final.log`. Las autopruebas no son evaluación del middleware. Detalle: `evidencias/harness-development/INFORME_INSTRUMENTOS.md`.

Los snapshots de las bases reales están en `evidencias/harness-development/sources-20261025/`. El censo S1 reveló905Patient distintos, no1000: los2500registros planos no representaban a todos los pacientes generados. Las fuentes originales y sus hashes se conservan. Se prepara un generador corregido y basesv2 nuevas; no se reduce el denominador ni se reemplaza la evidencia negativa. Las20mutaciones por esquema miden sensibilidad del oráculo, separada de la corrección de Rust.

El validador oficial7.0.2 y k6v2.3.0 tienen versiones, URL y SHA256 en `evidencias/validator-development/TOOL_IDENTITIES.json`. El control negativo oficial devolvió Javaexit0 con dosError; el runner leyó OperationOutcome y marcóFAIL. No cuenta en CA-1 del middleware. -txn/a limita validación terminológica; no equivale a validación completa de terminología.

## Matriz experimental vigente

| Criterio original | Denominador requerido | Resultado de evaluación Rust acreditado en este checkpoint | Límite |
| --- | --- | --- | --- |
| CA-1 | 16,250recursos por esquema, ceroFatal/Error | PENDIENTE | Control del instrumento sí; falta censo real de Rust y reportes oficiales correspondientes |
| CA-2 | Cero discrepancias en atributos proyectados;20mutaciones | PENDIENTE | Fuentes reales y sensibilidad del oráculo disponibles; faltan salidas del nuevo Rust |
| CA-3 | CorpusT1–T5 S0–S3, rechazo422OperationOutcome100%;0%falsos rechazos en censo válido | PENDIENTE | CLI conserva inyección/respuesta/restauración/controles; cobertura de instrumentos no acredita ejecuciónHTTP |
| CA-4 | Mismo ejecutable después de congelamiento identificado | PENDIENTE | Se modificó producción antes de nuevo congelamiento; no acredita invarianza histórica |
| CA-5 | Cobertura completa S0–S3 | PENDIENTE | CensoS1negativo conservado; body_weightS3 adicional fuera del despachoHTTP base |
| CT | Tres réplicas90+300+30s,50VU,pausas0.1–0.3s y seriesCPU/RAM | PENDIENTE | Runner preparado; modo nativoWindows/WSL sería alternativo y no acreditaría límitescgroups originales |

No se aplica un dictamen favorable por falta de evidencia. Se conserva PENDIENTE DE VERIFICACIÓN hasta aplicar la regla originalTabla3.1 sobre resultados identificados. Los negativos se mantienen y no se trasladan entre versiones.

## Drive, delegación y revisión

Se modificó realmente el registro de auditorías con Ronda9 inicial y seguimiento25oct; ambas escrituras tuvieron lectura posterior. El capítuloIV y el paquete de prompts fueron leídos, sin sustituir N/V ni alterar tablas/figuras/referencias.

Por solicitud del usuario se instaló AntigravityCLI1.4.7 desde el distribuidor oficial, con checksum verificado. El usuario inició sesión; no se leyeron ni guardaron tokens. Se configuró Gemini3.8FlashHigh con herramientas/ediciones sin solicitudes de autorización. `.agents/tools/invoke-antigravity.ps1` fija modelo, contrato y logs. Primera revisión independiente sólolectura en `evidencias/antigravity-review/review-20261025/`; no se atribuye verificación empírica a una respuesta de agente.

Pendientes: completar suitesruntime/HTTP, congelar ejecutable/hash, ejecutar adaptación/censos/comparación/corpus/validador/CT y revisar resultados. La determinación ética institucional requiere documento o decisión de la instancia académica competente; el código no la resuelve. No se hizo push.
