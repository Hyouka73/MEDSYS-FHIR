# Verificación de instrumentos experimentales

Fecha de reanudación del cliente: 25 de octubre de 2026. Este informe distingue
las pruebas de los instrumentos Python de las nuevas corridas del middleware.
No acredita CA-1, CA-2, CA-3, CA-4, CA-5 ni CT por sí solo.

## Problemas reproducidos y cambios

| Instrumento | Antes, observado | Corrección | Evidencia posterior |
| --- | --- | --- | --- |
| Oráculo/fechas | Dos instantes con la misma hora de pared y offsets distintos se consideraban iguales | Preservar zona, reglas históricas America/Mexico_City y fracciones; rechazar horas locales ambiguas/inexistentes | before-after.json y pruebas de zonas/fracciones/nanosegundos |
| Oráculo/campos | Cambiar reasonCode[0].text no producía rechazo | Comprobar todos los campos proyectados, además de restricciones de dominio | before-after.json y test_previously_unchecked_text_field_detected |
| Oráculo/números | Un valor inválido podía convertirse en 0 | Inválidos, ausentes, booleanos y no finitos no representan un cero legítimo | test_invalid_numbers_do_not_compare_equal_to_zero |
| Oráculo/entrada | La vía antigua podía aprobar un directorio vacío o sondear solo 100 Patient | Exigir snapshot PostgreSQL para comparar; exportador exhaustivo separado y censo original obligatorio | test_missing_export_is_not_pass y guardas CLI |
| Corpus | --base-url no hacía solicitudes HTTP y podía anunciar 100 rechazos HTTP | Rechazar esa vía histórica; marcar autoprueba Python; colector nuevo PostgreSQL+HTTP | fake-http-refused.log, exit 2 |
| Consolidación | Cifras asignadas y conclusión favorable sin experimentos reales | Eliminar el generador ficticio; exigir informes por criterio, SHA-256 y cobertura S0–S3 | missing-manifest.log, exit 2; pruebas de hashes/cobertura |
| CT | Mezcla posible de criterios vinculantes con caracterización | CT se consolida como descriptiva; no decide all_passed de CA-1–CA-5 | test_ct_is_descriptive_not_binding |
| Exportador histórico | Ignoraba fallos de exportación y el resultado del validador al salir | Propagar código de fallo y quitar afirmación de conformidad completa basada en parser de consola | Inspección y py_compile; usar runner HL7 estructurado para CA-1 |

`before-after.json` registra el commit real de la comparación histórica,
293396423cc993cc5b191d727ae0609e15a5aeb6, y la huella del módulo anterior leído
mediante git show. No se atribuye esa prueba al commit 18b434a ni a una etiqueta
v1.2.0-eval. No se creó esa etiqueta.

## Verificación ejecutada

```powershell
python scripts/test_experimental_evidence.py
python -m py_compile scripts/experimental_evidence.py scripts/oracle_verifier.py scripts/run_fault_corpus.py scripts/run_all_cap4_experiments.py scripts/export_fhir_samples.py
```

La primera batería tuvo 13/13 pruebas aprobadas; la reanudación incorporó 2
regresiones sobre cobertura multiesquema y CT no vinculante:15/15 aprobadas,
código 0, en `regression-tests-20261025.log`. La ejecución y su hora UTC real
están en `instrument-verification-20261025.json`. Las veinte mutaciones originales
son parte de esta autoprueba; se ejecutarán también sobre expectativas derivadas
de snapshots reales, con informes individuales separados.

## Instrumentos nuevos y límites

`scripts/experimental_evidence.py snapshot` lee fuentes PostgreSQL en una
transacción REPEATABLE READ de solo lectura. El modelo fuente se proyecta con
Python explícito sin leer YAML ni funciones Rust. Cada fuente y cada reporte
conserva SHA-256 de scripts, commit, estado Git y huella del diff evaluado.

`export` conserva cada petición/respuesta en http/ y únicamente los recursos
HTTP 200 en resources/. Comprueba identidad, Content-Type FHIR, las ausencias
respiratorias y el censo original. `compare` conserva denominadores, posiciones
proyectadas y discrepancias. Los campos FHIR adicionales no proyectados no son
parte de esta comparación. No se interpreta concordancia algorítmica como
validación médica ni interoperabilidad institucional.

`faults` exige una marca de laboratorio que coincida con la corrida y permite
únicamente insertar IDs nuevos>= 9000 en tablas conocidas. Conserva SQL/valores,
respuesta, controles y restauración. Las restricciones PostgreSQL que impidan
inyectar datos quedan BLOCKED: no equivalen a HTTP 422 observado. No modifica
ni elimina filas preexistentes.

El corpus original tiene 97 casos S0 y 3 casos S3. Su ejecución por esos dos
esquemas no demuestra cobertura de S1/S2. Los tipos, restricciones o fechas
malformadas que requieran una representación relajada necesitan un laboratorio
separado y declaración metodológica explícita; no se relaja el DDL nominal
silenciosamente. body_weight de S3 no forma parte del censo base 16,250.

El consolidador todavía requiere que los informes de adaptación del mismo
binario y los agregados CT provengan de corridas reales. No infiere por sí mismo
el dictamen de viabilidad. Los comandos y formato del manifiesto están en
scripts/EXPERIMENTAL_EVIDENCE.md.

## Fuentes reales y ensayos pendientes

Se capturaron las cuatro fuentes PostgreSQL reales del laboratorio persistente
en `sources-20261025/`: los comandos snapshot y mutations devolvieron código 0
por esquema. Los informes individuales contienen MUT-01–MUT-20, detectadas 20/20
por esquema. Son 80 ejecuciones de mutación del oráculo; no 80 pruebas de Rust.

| Fuente real anterior | Demografía disponible | Expectativas derivadas de fuente, todavía no salidas Rust |
| --- | ---: | ---: |
| S0 | 1,000 Patient | 16,250 |
| S1 | 905 Patient distintos en 2,500 filas planas | 16,155 |
| S2 | 1,000 Patient | 16,250 |
| S3 | 1,000 Patient | 16,250 |

La nueva evidencia descubre una pérdida de 95 pacientes en S1 que el censo del
modelo generado no reflejaba. Se conserva este resultado negativo y los
snapshots anteriores íntegros. El usuario autorizó corregir el generador y
crear nuevas bases medsys_eval_v2_s0..s3; no sobrescribir las bases anteriores.

La corrección reasigna el mínimo de 95 propietarios de consultas duplicadas a
los 95 pacientes ausentes, antes de generar signos y diagnósticos dependientes.
Conserva las 2,500 consultas, la semilla, el flujo RNG, las dos filas DDL iniciales
y los demás valores clínicos. `generator-before-after-20261025.json` registra
cada reasignación y las comprobaciones de preservación. El generador produce
ahora SQL con terminaciones LF y calcula SHA-256 de los bytes escritos, evitando
el hash incorrecto anterior cuando Windows traducía LF a CRLF. Las huellas
finales están en `generator-final-hashes-20261025.json` y la generación real en
`generate-data-v2-20261025.log`. La prueba de cobertura del esquema plano amplía
la batería a 16/16 aprobadas.

El primer intento de crear las bases v2 se bloqueó porque Windows volvió a perder
la conexión con PostgreSQL en 127.0.0.1:55432; se detuvo solamente el proceso
propio y se añadió connect_timeout=5. No se considera un ensayo aprobado.
Se coordina la restauración del transporte antes de crear y verificar las nuevas
bases. Las exportaciones y el corpus HTTP requieren además el servidor Rust real
corregido, todavía en preparación al escribir esta sección.

Actualización posterior de esta misma corrida: el coordinador restauró WSL y el
transporte. `lab-v2-creation-20261025-retry1/creation.json` registra la creación
exitosa de las cuatro bases nuevas, sin DROP ni cambios en las anteriores.
`cohorts-database-verification-20261025.json` confirma que S1 v1 conserva sus
905 pacientes y S1 v2 contiene 1,000, ambos con 2,500 filas. El rol medsys_ro
conserva SELECT y carece de permisos INSERT/UPDATE/DELETE.

Los snapshots y mutaciones v2 se encuentran en `sources-v2-20261025/`.
`source-census-summary.json` deriva de esas fuentes reales exactamente
1,000 Patient, 2,500 Encounter, 9,750 Observation y 3,000 Condition por esquema,
con 250 respiratorias ausentes y las 20 mutaciones detectadas en cada esquema.
Estos censos son expectativas derivadas de SQL y no se presentan todavía como
recursos realmente exportados desde Rust.

El fixture unitario independiente `medsys_eval_unit` conserva únicamente las dos
semillas Alberto/Mariana del DDL raíz, sin bulk. Está separado de las cohortes
v1/v2 y no sirve como fuente para el censo experimental. Su creación y sus
cuatro conteos de dos filas están en `unit-lab-20261025/creation.json`.

La última verificación de instrumentos añade la conservación byte a byte del
HTTP al exportar: el JSON original no se reserializa antes de validar, evitando
redondear decimales o eliminar claves duplicadas. La batería final es 17/17,
código 0, en `regression-tests-v2-20261025.log`; las huellas de scripts evaluados
están en `instrument-verification-v2-20261025.json`.

Una última regresión comprueba que un timeout de red cuenta como intento y no
como respuesta HTTP observada. El denominador CA-3 distingue inyecciones
confirmadas, solicitudes intentadas, respuestas recibidas y controles nominales
aprobados. El cierre de instrumentos queda en 18/18, código 0, registrado en
`regression-tests-final-20261025.log` y
`instrument-verification-final-20261025.json`. Se conservan las baterías previas
para no confundir sus versiones con esta última.

## Reanudación del 27 de octubre de 2026

La revisión independiente Flash en
`auditoria/evidencias/antigravity-review/instrumentos-20261025/REVIEW.md`
detectó dos defectos adicionales acotados. El estado de adapt() omitía el código
del corpus y podía propagar PASS pese a un fallo de inyección/HTTP; ahora incluye
esa salida y exige resultado cuando se pidió el corpus. Un corpus pendiente
produce INCOMPLETE y uno fallido impide PASS en ca5-contract.json.

La selección de PK en S3 ya no usa id_paciente/id_consulta/id_diagnostico:
resuelve pac_id/con_id/sig_id/diag_id por esquema y recurso. El corpus original
solo contiene tres Observation S3, por lo que ese defecto era latente en los
otros recursos; no se inventa una corrida previa de dichos casos.

Las dos nuevas regresiones amplían la batería a 20/20 aprobadas, código 0.
`reanudacion-20261027/instrument-tests.log` e `instrument-verification.json`
conservan comandos, huellas y resultados. El fallo anterior proviene de la
inspección independiente del código; todavía no se afirma una corrida completa
de adapt anterior ni posterior.

`reanudacion-20261027/database-check.json` confirma desde Windows el acceso a
las cuatro bases v2, su marcador reeval20261025-v2, 1,000 pacientes distintos por
esquema y rol medsys_ro con SELECT y sin UPDATE. Los snapshots nominales v2
anteriores siguen disponibles. La próxima corrida de adapt capturará fuentes
nuevas; espera la confirmación del ejecutable congelado antes de iniciar HTTP.
