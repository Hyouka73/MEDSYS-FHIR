# Herramientas y controles — 25 de octubre de 2026

## Identidades observadas

- Validador oficial HL7 7.0.2, Git `de3463393298`, JAR SHA-256 `e3e50f06b806437821b7aef8189a1b790415cb96c5036e0812ed62981452b377`.
- k6 2.3.0, commit `e088784614`, Go 1.26.8, Windows amd64. Ejecutable SHA-256 `efec04c36452cfb9c0ef313082c021e5656caf5004f922c2d8b807fdf4a0e02f`.
- Java Microsoft OpenJDK 21.0.10. Fuentes, huellas de ZIP y comandos de versión: `TOOL_IDENTITIES.json` y `validator-help.txt`.

## Control histórico realmente ejecutado

`controlled-invalid/summary.json`, `resources.json`, `batch-0001/outcomes.json` y logs contienen la ejecución oficial de dos fixtures (no salidas del middleware):

- Patient/invalid: dos incidencias Error por `BROKEN_GENDER` y un Warning por ausencia de narrativa.
- Patient/nominal: cero Error y un Warning por narrativa.
- Java retornó **0**, mientras el runner clasificó correctamente **FAIL** al leer OperationOutcome. No se usa el retorno Java aislado como garantía de aprobación.
- `-version 4.0.1 -tx n/a`: no acredita validación terminológica completa. Los paquetes disponibles permiten detectar este código inválido aun sin servidor terminológico.
- La primera carga incluyó descargas de paquetes y duró 770.21 s en un equipo bajo presión de memoria. Es tiempo del control/herramientas, no latencia del middleware.

La reinspección del 25 de octubre se registra como lectura histórica, sin fingir una nueva ejecución. `REINSPECTION_25OCT.json` conserva hashes de los resultados originales y de los paquetes efectivamente listados por el validador. La lista corregida incluye xver-extensions; el runner actual captura únicamente paquetes observados en los logs, no todo el cache ni sus directorios temporales duplicados.

## Cambios del runner

- CA-1: censo total y por tipo, identidades duplicadas, asociación de cada OperationOutcome a archivo, SHA-256 de entradas/JAR/paquetes, comandos, retorno real y fallo explícito si falta reporte o procedencia.
- CT: tres réplicas del script original con bruto gzip JSON, fases, rutas/estados, retornos y series de recursos. Agregador computa percentiles sobre puntos y sobre sumas por muestra alineada.
- Alternativa Windows/WSL explícita: mantiene carga real y telemetría de procesos, pero marca `INCOMPLETE` para CT-3/CT-4 originales por no reproducir cuotas/cgroups. No presenta RSS sumado como memoria física conjunta.
- Controles ya ejecutados: sintaxis Python válida; JAR/entrada/ejecutable k6 ausente y agregado CT sin evidencia retornan 1. No cuentan como CA-1–CA-5 del middleware.

## Pendientes

No se ha ejecutado aún el censo CA-1 de S0–S3 ni CT sobre el nuevo artefacto Rust: se espera la versión corregida y estable, sus exportaciones reales y su identificación. No hay cierre experimental atribuido a estos controles de herramientas.
