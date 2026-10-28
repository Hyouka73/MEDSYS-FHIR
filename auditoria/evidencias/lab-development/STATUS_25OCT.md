# Laboratorio PostgreSQL — 25 de octubre de 2026

El cluster anterior bajo `/tmp/medsys-reverificacion-2933964` desapareció entre sesiones. Sus logs originales se preservaron; no se tomaron como comprobación de una base todavía disponible.

Se recrearon las cuatro bases sintéticas usando los DDL/bulk actuales, con logs nuevos en `20261026T033909Z` (UTC; 25 de octubre local). Después de verificar los cuatro marcadores se detuvo exclusivamente ese cluster, se movió su directorio al destino dedicado previamente ausente `/var/lib/postgresql/medsys-eval-2933964`, se ajustó su socket y se reinició. No se usó ni modificó `main`.

La primera ejecución de carga terminó con `syntax error near unexpected token done` después de cargar las cuatro bases: se había editado el script mientras Bash aún lo leía. Se corrigió el procedimiento; no se repitió la carga sobre datos existentes. La primera tentativa de migración se negó por la comprobación de `PG_VERSION` sin privilegios para atravesar el directorio PostgreSQL de modo 0700; se cambió esa comprobación a `sudo -u postgres test`. La migración posterior terminó con código 0. Se conservan los directorios `migration-*` de los intentos y `bash -n` verificó ambos scripts finales con código 0.

`WINDOWS_VERIFICATION_25OCT.json` procede de conexiones reales psycopg desde Windows a `127.0.0.1:55432`. Acredita cuatro bases `medsys_eval_s0` a `medsys_eval_s3`, marcador `reanudacion-2933964`, ruta persistente y censos físicos. En S0/S2/S3: 1,000 pacientes, 2,500 consultas, 2,500 signos y 3,000 diagnósticos. En S1: 2,500 filas de sábana y 3,000 diagnósticos. Para `medsys_ro` se comprobó SELECT permitido y ninguno de INSERT/UPDATE/DELETE permitido en las tablas fuente.

El servicio permanece activo. El script futuro preserva bases existentes sólo si su marcador coincide y falla frente a cualquier directorio o base no reconocidos. La autenticación trust está restringida a loopback/sockets de este laboratorio; no se usa en bases ajenas.

La configuración Docker `tools/experimental/lab-compose.yaml` fue validada con `docker compose ... config --quiet` (código 0): proyecto/red/volumen dedicados, PostgreSQL16, puerto loopback55433, cuota1CPU y512MiB. **No se ejecutó compose up** ni se construyó Rust Linux. El motor Docker volvió a responder durante esta sesión, pero no se atribuye una ejecución cgroup al laboratorio WSL.
