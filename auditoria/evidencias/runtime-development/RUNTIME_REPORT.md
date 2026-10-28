# Correcciones runtime y verificación Rust — corte 27 de octubre de 2026

El artefacto se identifica por el commit base `293396423cc993cc5b191d727ae0609e15a5aeb6` más el diff local, no por una etiqueta histórica. No se creó `v1.2.0-eval`, ni se hizo commit o push desde esta delegación.

**Ejecutable:** `target/debug/medsys-server.exe`, SHA-256 `ea7e9e29bc730558cf0b0de993203bf0413d06c29478e273b6aa140eb8f5f9ad`. Perfil Windows `dev` sin optimización, símbolos debug desactivados y compilación `-j1`. Esto debe declararse al interpretar CT y no equivale al despliegue Linux/cgroups del diseño original.

## Fallos conservados y resultados posteriores

| Evidencia inicial | Causa | Verificación posterior |
|---|---|---|
| `../reanudacion/regresiones-antes-sin-debug.log`, salida 101, 0/2 | El adaptador ignoraba `id_sexo` resuelto por JOIN S2 y `temp_celsius` real de S3. Las dos pruebas se añadieron antes de modificar producción. | `regressions-after.log`, salida 0, 2/2; la suite final incluye las mismas aserciones de alias, overflow, estado/fin obligatorios y FR corrupta. |
| `date-precision-before.log`, salida 101, 0/1 | Una fecha sin hora admitida por el YAML S3 seguía siendo rechazada por el camino tipado. | `workspace-final3.log`: normalización y restauración de precisión de fecha verificadas para Observation/Encounter; Condition conserva fecha sin hora. |
| `workspace-final.log`, salida 101 | El test auxiliar legacy exigía claves que no correspondían al contrato existente. | Se verificó `PatientFullComparison` en `crates/medsys-server/src/handlers/legacy.rs` y el consumidor independiente `dashboard/src/components/InteroperabilityComparator.jsx`. La nueva prueba exige las claves reales `*_legado(s)`, tipos JSON y recursos FHIR, además de HTTP 200. No se añadieron aliases duplicados a la API. |
| `workspace-final2.log`, salida 101 | El test Patient/1 exigía ausencia de telecom aunque la semilla original SQL contiene `9611234567`. | Se exige ahora ese valor y `system=phone`, conforme a `schema_legado_simulado_nom004.sql`. La omisión con NULL se mantiene en los tests de dominio y debe contrastarse también con el censo HTTP de la evaluación. No se alteró la semilla de la fixture unitaria. |

`runtime-contract-before.log` corresponde a una compilación adicional interrumpida antes de ejecutar pruebas; no constituye aprobación ni un resultado funcional negativo. `fmt-before.log` es una revisión de formato anterior a aplicar rustfmt, no una prueba clínica.

## Cambios y alcance

- **R5-003/004:** las rutas FHIR usan el repositorio dinámico también en S0. Observation proyecta la unión explícita de columnas de sus cuatro subtipos publicados, sin incluir `body_weight`. Se reconocen los alias S2 y los nombres reales S3 de estado, temperatura y descripción. La prueba imprime el SQL proyectado de S0–S3.
- **R5-005:** se eliminaron sustituciones de estado por FINALIZADA, fin por inicio y descripción por código. Un origen obligatorio ausente no puede usar `constant_value` como reemplazo. El entero fuera de rango y la FR opcional corrupta se rechazan; un FK de catálogo presente pero sin coincidencia de JOIN se distingue de un NULL legítimo. Las conversiones numéricas ya no inventan valores de temperatura/peso. Los adaptadores siguen siendo tipados y acotados a los esquemas de la tesis; esto no acredita interpretación universal de cualquier DSL o columna no proyectada.
- **R5-006:** los instantes sin offset se interpretan en la zona declarada (por defecto America/Mexico_City histórica); se rechazan relojes ambiguos/inexistentes. Los offsets explícitos conservan su instante y las fracciones se preservan. La ruta HTTP normaliza instantes completos a UTC. Una fecha sin hora lleva un marcador de precisión y se publica como fecha; el reloj cero usado como portador interno de Legacy no se emite como hora observada. Condition.recordedDate conserva la precisión original de fecha. PostgreSQL NUMERIC se transporta como texto decimal para evitar un paso intermedio por IEEE-754.
- **R5-008:** las lecturas FHIR y sus comprobaciones de padres comparten una transacción REPEATABLE READ READ ONLY. Se comprueba además la asociación entre el paciente del recurso y el encuentro padre. Las rutas auxiliares `/api/legacy` conservan sus repositorios tradicionales; no se afirma una instantánea universal para ellas. Los privilegios efectivos y los ensayos de inyección corresponden al laboratorio y deben registrarse aparte.
- **R5-009:** los rechazos de Query y los métodos HTTP no admitidos producen OperationOutcome; la salud devuelve HTTP 503 cuando la base está desconectada. La salud comprueba conectividad, no certifica toda transformación, configuración o privilegio.
- **R5-010:** filtros admitidos se combinan antes de paginar; el límite/offset de Observation opera sobre recursos FHIR. Bundle.total cuenta las coincidencias completas y se proporcionan enlaces de página. El SQL `DISTINCT ON` de Patient S1 ya estaba implementado: el ensayo HTTP guardado en `s1-collection-before*.json` observó 200, total=1000 y una entrada con limit=1. No se añadió una deduplicación redundante ni se atribuye un fallo de 2500 pacientes a esa ruta. Las colecciones cargan las filas del laboratorio para filtrar/contar; esta limitación de escalabilidad permanece y no permite pronosticar un percentil CT sin medirlo.
- **R5-014:** las pruebas nominales HTTP/Tower exigen HTTP 200; la integración PostgreSQL deja de contar una conexión ausente como PASS. `pg_row_to_json_map` propaga un error de decodificación/tipo desconocido en lugar de convertirlo en NULL. Se verifica con un PgRow BYTEA real y controles TEXT/NULL sobre la fixture sintética.

## Resultado de verificación

`workspace-final3.log` y su `.exit` acreditan **104 aprobadas, 0 fallidas, 0 ignoradas**, usando únicamente `medsys_eval_unit` en `127.0.0.1:55432`. `build-final.log` acredita salida 0. `clippy-final.log` y su `.exit` acreditan salida 0 con `-D warnings`. `runtime-verification-summary.json` conserva comandos, perfil, resultado y hash.

El 27 de octubre se conservó un nuevo inventario de fuentes en `freeze-recheck-2026-10-27-before.json`, se comprobó la identidad de la fixture mediante SELECT y marcador `medsys-unit-20261025` en `unit-identity-recheck-2026-10-27.json`, y se hizo una compilación incremental congelada sin modificar producción. Los archivos `recheck-2026-10-27-*` acreditan 9/9 regresiones, 1/1 decodificación PgRow real y 1/1 restauración de precisión de fecha, sin fallos ni pruebas ignoradas. Build y formato terminaron con salida 0; no hubo recompilación de fuentes ni cambio de hash del ejecutable.

**Límite del dictamen:** estos resultados verifican implementación y pruebas Rust/Tower, junto con un sondeo HTTP S1. No sustituyen el censo HTTP S0–S3, el oráculo diferencial sobre salidas reales, las veinte mutaciones, el corpus PostgreSQL+HTTP, el validador oficial ni k6. La prueba Rust del corpus no equivale a cien transacciones HTTP. El coordinador debe cerrar o mantener abiertos los hallazgos según las evidencias experimentales finales.

## Reproducción

En PowerShell, con la fixture sintética original ya preparada y su marcador verificado:

```powershell
$env:CARGO_PROFILE_DEV_DEBUG='0'
$env:CARGO_PROFILE_TEST_DEBUG='0'
$env:DATABASE_URL='postgres://medsys_ro@127.0.0.1:55432/medsys_eval_unit'
cargo test -j1 --frozen --workspace -- --nocapture
cargo build -j1 --frozen -p medsys-server --bin medsys-server
cargo clippy -j1 --frozen --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
```

`runtime-implementation.diff` contiene las modificaciones de `crates/**`, los archivos nuevos y la actualización mecánica de Cargo.lock. Los hashes finales de fuentes y evidencias permiten detectar cambios posteriores. No se escribió en Drive desde este subagente.
