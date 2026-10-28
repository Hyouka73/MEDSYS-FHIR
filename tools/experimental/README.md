# Runners de evidencia experimental real

Herramientas locales descargadas de publicaciones oficiales (identidad en `auditoria/evidencias/validator-development/TOOL_IDENTITIES.json`): validador HL7 7.0.2 y k6 2.3.0. No se incluyen JAR/ZIP/binarios en Git; se conservan en `tools/experimental/bin` para reproducción local. Los JSON de release originales conservan URL, fecha e identidad publicada.

## CA-1

Desde la raíz del repositorio, usando un directorio de exportaciones HTTP reales de un único arquetipo:

```powershell
python tools/experimental/evidence_runners.py validate --schema s0 --input RUTA_EXPORTACIONES_S0/resources --output RUTA_NUEVA_CA1_S0 --expected-count 16250
```

Repetir para S1–S3. El runner ejecuta Java con FHIR 4.0.1, `-tx n/a`, heap 768 MiB y lotes de 500 recursos; conserva comandos, retornos, tiempos, logs y OperationOutcome oficiales. El informe consolidado asocia cada resultado al fichero mediante la extensión oficial `operationoutcome-file`; si falta un reporte, hay entradas duplicadas o no coinciden los denominadores, no aprueba. `resources.json` conserva incidencias por recurso y huellas del JSON realmente leído. `summary.json` incluye hashes de archivos de paquetes del cache usado. `--tx URL` habilita un servidor terminológico explícito; `-tx n/a` **no acredita validación terminológica completa**. El cache de paquetes FHIR puede requerir Internet incluso con `-tx n/a`.

Cada ejecución exige directorio de salida nuevo. Un retorno 1 indica FAIL/BLOCKED; el resumen distingue ambas situaciones. No deben incluirse en CA-1 los fixtures de control como si fueran salidas de Rust. El censo de 16,250 pertenece al contrato Patient/Encounter/cuatro Observation/Condition; body_weight S3 permanece fuera del despacho HTTP evaluado.

## CT (requiere Docker operativo y servicio Linux aislado)

```powershell
python tools/experimental/evidence_runners.py ct --k6 tools/experimental/bin/k6/k6-v2.3.0-windows-amd64/k6.exe --base-url http://localhost:18080 --container NOMBRE_SERVIDOR_LAB --container NOMBRE_POSTGRES_LAB --output RUTA_NUEVA_CT
python tools/experimental/aggregate_ct.py RUTA_NUEVA_CT
```

No cambia `tests/k6/load_test.js`, umbrales, fases ni pausas. Ejecuta tres réplicas de 7 minutos (90 s rampa, 300 s meseta a 50 VU, 30 s descenso) con datos brutos k6 gzip JSON, resumen, retorno y logs. Muestrea ambos contenedores en una llamada `docker stats` y conserva inicio/fin de cada muestra más `cpu.stat`, `memory.current` y VmRSS de PID 1. Docker working set, memoria de cgroup y RSS son definiciones separadas; en PostgreSQL PID1 no representa todos los procesos. El agregado conjunto se forma por muestra antes de calcular p95; no suma percentiles individuales. El desfase de lecturas secuenciales queda registrado en los tiempos.

El agregador exige las tres réplicas y puntos reales de meseta y telemetría conjunta; falla ante evidencia ausente. No convierte el resultado descriptivo CT en aprobación vinculante. Los códigos por ruta/fase se derivan de puntos HTTP, manteniendo los 404 esperados visibles. El callback y las pausas siguen definidos en el script original conservado por hash. Ejecutar CT con el mismo artefacto identificado que los criterios funcionales; no se atribuye equivalencia con cgroups a un servidor nativo de Windows.

### Alternativa explícita sin Docker

```powershell
python tools/experimental/evidence_runners.py ct --k6 tools/experimental/bin/k6/k6-v2.3.0-windows-amd64/k6.exe --base-url http://localhost:18080 --native-pid PID_REAL_RUST --output RUTA_NUEVA_CT_NATIVE
python tools/experimental/aggregate_ct.py RUTA_NUEVA_CT_NATIVE
```

El script ejecuta la misma carga k6 y conserva series del proceso Rust en Windows (CPU acumulada, WorkingSet64 y PrivateMemorySize64) y los procesos del cluster PostgreSQL de laboratorio en WSL (CPU acumulada y RSS). El agregado mantiene CPU por proceso y suma las tasas de CPU observadas en intervalos alineados; no suma percentiles. Los procesos PostgreSQL nacidos o terminados entre muestras introducen subcobertura de CPU; el RSS sumado duplica páginas compartidas. No se interpreta la suma como RAM física conjunta. No hay cuotas cgroup: el agregado queda `INCOMPLETE` respecto al protocolo CT original y retorna 1 aunque las tres cargas terminen correctamente. El resultado descriptivo alternativo permanece disponible y debe identificarse por separado en la tesis.

## PostgreSQL de laboratorio

```powershell
wsl -d Ubuntu -- bash '/mnt/d/Proyectos X/MedSys-FHIR/tools/experimental/lab-postgres.sh'
```

Usa exclusivamente `/var/lib/postgresql/medsys-eval-2933964`, puerto 55432, bases `medsys_eval_s0` a `medsys_eval_s3` y marcador `reanudacion-2933964`. Nunca usa el cluster `main` de Ubuntu. Se niega a inicializar sobre un directorio existente sin `PG_VERSION`, no sobrescribe bases existentes y conserva logs en un subdirectorio UTC nuevo de `auditoria/evidencias/lab-development`. La autenticación trust se limita a loopback y sockets del laboratorio sintético; no se emplean credenciales productivas. Las fuentes DDL y bulk se conservan sin cambios. La ubicación previa en `/tmp` desapareció al reiniciar WSL; se migró la recreación del 25 de octubre a la ruta persistente dedicada.

## Controles ejecutados de los runners

- `python -m py_compile tools/experimental/evidence_runners.py tools/experimental/aggregate_ct.py`: sintaxis válida.
- JAR ausente y entrada ausente: retorno 1 y resumen BLOCKED, sin aprobación falsa.
- Agregado CT sin evidencia: retorno 1.
- Ensayo oficial nominal/código inválido: ver `auditoria/evidencias/validator-development/controlled-invalid`; su estado debe leerse del resumen, no de este documento.
- En dicho control, Java retornó 0 aunque emitió dos incidencias Error para el código inválido. El runner devolvió FAIL al inspeccionar OperationOutcome; el recurso nominal tuvo cero Error y un Warning de narrativa. Esta evidencia verifica el control del envoltorio, no CA-1 de Rust.
