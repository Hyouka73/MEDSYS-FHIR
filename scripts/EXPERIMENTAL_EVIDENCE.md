# Ensayos con evidencia primaria

Estos comandos no utilizan las cifras históricas asignadas por el antiguo
`run_all_cap4_experiments.py`. Los informes de autopruebas verifican instrumentos,
no acreditan la evaluación del middleware.

## Dependencias

```powershell
python -m pip install -r scripts/requirements-experiments.txt
python scripts/test_experimental_evidence.py
```

## Fuentes y salidas reales

Con PostgreSQL sintético y Rust ya iniciados por el operador del laboratorio,
definir `EXPERIMENT_DATABASE_URL` en el entorno sin guardar credenciales en logs.
Repetir para S0, S1, S2 y S3 cambiando configuración, conservando el mismo binario
una vez corregido y congelado. El directorio de exportación debe ser nuevo.

```powershell
python scripts/experimental_evidence.py snapshot --schema s0 --output RUN/s0-source.json
python scripts/experimental_evidence.py export --source RUN/s0-source.json --base-url http://127.0.0.1:3000 --output RUN/s0-export
python scripts/experimental_evidence.py compare --source RUN/s0-source.json --input-dir RUN/s0-export --output RUN/s0-oracle.json
python scripts/experimental_evidence.py mutations --source RUN/s0-source.json --output RUN/s0-mutations.json
```

`snapshot` consulta las tablas reales en una transacción de solo lectura y
REPEATABLE READ. El oráculo usa proyecciones Python explícitas sin leer YAML ni
código Rust. Se inspecciona la consistencia de la demografía repetida en S1.
Se conserva el censo por recurso, posiciones de campo y discrepancias. Las
expectativas del modelo pueden también contener errores: no son validación
médica. Las mutaciones conservan MUT-01–MUT-20 y miden exclusivamente sensibilidad
del oráculo, separada de la corrección de Rust.

`http/` conserva URL, estado, cabeceras, cuerpo y tiempo por petición, también
en errores. `resources/` contiene solamente recursos HTTP 200 y es el directorio
de entrada al validador HL7; no pasar el directorio raíz con metadatos al validador.
Los 250 recursos respiratorios ausentes se consultan y verifican individualmente.
Un censo menor al original 1,000/2,500/9,750/3,000 queda INCOMPLETE aunque todas sus
comparaciones coincidan. El recurso adicional body_weight de S3 está fuera de
este censo base y requiere un ensayo independiente si se expone.

## Misma compilación y cuatro configuraciones

Después de corregir y compilar Rust, congelar su ejecutable y definir por esquema
las variables `EXPERIMENT_DATABASE_URL_S0`..`S3` con el rol medsys_ro. Para
inyecciones, definir separadamente `EXPERIMENT_WRITE_DATABASE_URL_S0` y `S3`
con el escritor del laboratorio. Las credenciales se mantienen en el entorno.

```powershell
python scripts/experimental_evidence.py adapt --executable target/debug/medsys-server.exe --port-base 3310 --lab-id reeval20261025-v2 --output RUN/adaptation
```

El colector copia las cuatro configuraciones a la evidencia, comprueba que los
puertos estén libres, inicia exclusivamente sus propios servidores locales y
los detiene al terminar cada esquema. No compila ni modifica código Rust. Por
esquema ejecuta snapshot/export/compare/mutations; si se aporta lab-id, también
el corpus original S0/S3. Registra hashes del ejecutable antes/después de cada
transición, logs y códigos de salida. Genera ca4-transition.json y
ca5-contract.json. La identificación corresponde al artefacto local corregido;
no demuestra identidad con una versión histórica ni adopción por terceros.

El generador nominal se corrigió tras observar 905 pacientes distintos en S1:
se reasignan 95 consultas duplicadas para incluir los pacientes ausentes, sin
cambiar el número de consultas ni los demás valores clínicos. Las bases
anteriores medsys_eval_s0..s3 se conservan. La nueva cohorte usa
medsys_eval_v2_s0..s3 y marca reeval20261025-v2. El comando de creación explícita
solo admite esos nombres nuevos y rechaza destinos existentes:

```powershell
python scripts/generate_data.py --schema all
python scripts/create_evaluation_lab_v2.py --output RUN/new-databases
```

La segunda orden usa EXPERIMENT_ADMIN_DATABASE_URL, con host de loopback, y
conserva el DDL realmente aplicado y hashes. No ejecutar otra vez sobre una
cohorte ya creada: se debe reutilizarla y verificar su marca y sus fuentes.

## Corpus PostgreSQL + HTTP

Solo sobre una base efímera y sintética, su responsable debe crear previamente
`experimental_lab_guard(run_id text primary key)` con el identificador de corrida.
El colector exige esa marca, permite únicamente tablas del esquema seleccionado
e IDs nuevos >= 9000. No modifica ni borra filas existentes. Inyecta y confirma
una fila, realiza la petición HTTP, elimina la fila insertada y verifica su
ausencia, con controles nominales HTTP antes y después de cada caso.

```powershell
python scripts/experimental_evidence.py faults --schema s0 --lab-id RUN_ID --base-url http://127.0.0.1:3000 --output RUN/s0-faults
python scripts/experimental_evidence.py faults --schema s3 --lab-id RUN_ID --base-url http://127.0.0.1:3003 --output RUN/s3-faults
```

La conexión indicada debe corresponder a la base consultada por cada servidor.
El corpus original contiene 97 casos S0 y 3 S3; este reparto no acredita rechazo
de T1–T5 en S1/S2. El alias incorrecto `temp_c` de las tres fixtures S3 se corrige
a `temp_celsius` y se registra por caso. Una restricción PostgreSQL que impida
inyectar un dato se conserva como BLOCKED, jamás como rechazo HTTP aprobado.
Los tipos DATE/INTEGER, NOT NULL y FK originales pueden impedir algunos casos:
no se relajan silenciosamente para conseguir aprobaciones. Los informes conservan
denominadores observados y restauración; una restauración fallida detiene el ensayo.

El runner histórico `run_fault_corpus.py` mantiene su autoprueba Python claramente
rotulada y rechaza `--base-url`, porque nunca implementó esa vía real.

## Consolidación

```powershell
python scripts/run_all_cap4_experiments.py --manifest RUN/manifest.json --output RUN/consolidado.json
```

El manifiesto contiene claves CA-1, CA-2, CA-3, CA-4, CA-5 y CT. Cada valor es una
lista de objetos `{"path":"ruta-relativa.json","sha256":"hash-real"}`. Se rechazan
rutas externas, informes ausentes y hashes que no coincidan. Tipos requeridos:

| Criterio | kind del informe |
| --- | --- |
| CA-1 | hl7_validator_execution o CA-1 (runner HL7 oficial) |
| CA-2 | source_vs_rust y oracle_sensitivity_only |
| CA-3 | postgres_http_faults |
| CA-4 | binary_configuration_transition |
| CA-5 | adaptation_contract_comparison |
| CT | k6_resource_measurement |

Cada informe debe incluir `status` PASS/FAIL/BLOCKED/INCOMPLETE derivado de sus
datos primarios y `schema` s0/s1/s2/s3; para informes de transición de binario y
contrato que realmente agrupen varias corridas, `schemas` puede enumerar su
alcance. El manifiesto puede añadir `schema` para el resumen del validador que
no lo incluyera. CA-1–CA-5 requieren cobertura explícita de S0–S3: un único
informe S0 no permite cerrar el conjunto. CA-1 exige el censo validado de 16,250,
CA-2 exige censo completo sin discrepancias y las 20 mutaciones individuales,
CA-3 exige solicitudes observadas, controles y restauración de cada inyección.
Un reporte resumido no sustituye sus exportaciones, logs y
mediciones. El consolidador valida referencias e integra estados, pero no
comprueba por sí solo todos los umbrales ni aplica la regla original de viabilidad
de la tesis; el dictamen requiere contrastar esa regla y la cobertura completa.
CT es descriptiva y no vinculante: su resultado no entra en
`binding_criteria_all_passed`. La existencia de evidencia CT se informa por
separado en `characterization_evidence_complete`. Un resultado negativo medido
puede ser evidencia completa; BLOCKED e INCOMPLETE no lo son.
No genera resultados ni sobreescribe el informe histórico de Capítulo IV.

La evidencia de regresión anterior/posterior de estos instrumentos se encuentra
en `auditoria/evidencias/harness-development/`. No constituye una corrida CA o CT.
