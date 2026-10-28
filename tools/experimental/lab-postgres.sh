#!/usr/bin/env bash
set -euo pipefail
# Isolated synthetic cluster, never use the distribution's main cluster.
LAB_ROOT=/var/lib/postgresql/medsys-eval-2933964
LAB_PORT=55432
LAB_ID=reanudacion-2933964
REPO='/mnt/d/Proyectos X/MedSys-FHIR'
LOGS="$REPO/auditoria/evidencias/lab-development/$(date -u +%Y%m%dT%H%M%SZ)"
PG=/usr/lib/postgresql/16/bin
mkdir -p "$LOGS"
if ! sudo -n -u postgres test -f "$LAB_ROOT/PG_VERSION"; then
  if [[ -e "$LAB_ROOT" ]]; then
    echo 'Refusing init: laboratory path exists without PG_VERSION' >&2
    exit 1
  fi
  sudo -n install -d -o postgres -g postgres -m 700 "$LAB_ROOT"
  sudo -n -u postgres "$PG/initdb" -D "$LAB_ROOT" --auth-local=trust --auth-host=trust > "$LOGS/initdb.log" 2>&1
  printf "\nport = %s\nlisten_addresses = '127.0.0.1'\nunix_socket_directories = '%s'\n" "$LAB_PORT" "$LAB_ROOT" | sudo -n -u postgres tee -a "$LAB_ROOT/postgresql.conf" > /dev/null
fi
if ! sudo -n -u postgres "$PG/pg_ctl" -D "$LAB_ROOT" status > "$LOGS/pg-status.log" 2>&1; then
  sudo -n -u postgres "$PG/pg_ctl" -D "$LAB_ROOT" -l "$LAB_ROOT/server.log" -w start > "$LOGS/pg-start.log" 2>&1
fi
export PGHOST=127.0.0.1 PGPORT="$LAB_PORT" PGUSER=postgres
for schema in s0 s1 s2 s3; do
  db="medsys_eval_$schema"
  existing=$("$PG/psql" -d postgres -Atqc "SELECT count(*) FROM pg_database WHERE datname='$db'")
  if [[ "$existing" != 0 ]]; then
    marker=$("$PG/psql" -d "$db" -v ON_ERROR_STOP=1 -Atqc 'SELECT run_id FROM experimental_lab_guard')
    [[ "$marker" == "$LAB_ID" ]]
    echo "Preserving existing laboratory database $db with verified marker" >&2
    continue
  fi
  "$PG/createdb" "$db"
  case "$schema" in
    s0) ddl=schema_s0_base.sql; bulk=02_bulk_data.sql;;
    s1) ddl=schema_s1_flat.sql; bulk=bulk_s1.sql;;
    s2) ddl=schema_s2_lookup.sql; bulk=bulk_s2.sql;;
    s3) ddl=schema_s3_legacy.sql; bulk=bulk_s3.sql;;
  esac
  "$PG/psql" -d "$db" -v ON_ERROR_STOP=1 --single-transaction -f "$REPO/docker/sql/$ddl" > "$LOGS/$schema-ddl.log" 2>&1
  "$PG/psql" -d "$db" -v ON_ERROR_STOP=1 --single-transaction -f "$REPO/docker/sql/$bulk" > "$LOGS/$schema-bulk.log" 2>&1
  "$PG/psql" -d "$db" -v ON_ERROR_STOP=1 -c "CREATE TABLE experimental_lab_guard(run_id text PRIMARY KEY); INSERT INTO experimental_lab_guard VALUES ('$LAB_ID'); GRANT SELECT ON experimental_lab_guard TO medsys_ro;" > "$LOGS/$schema-guard.log" 2>&1
  "$PG/psql" -d "$db" -v ON_ERROR_STOP=1 -Atqc "SELECT version(); SELECT current_database(); SELECT run_id FROM experimental_lab_guard; SELECT rolname,rolsuper,rolcreatedb,rolcreaterole FROM pg_roles WHERE rolname='medsys_ro';" > "$LOGS/$schema-environment.log" 2>&1
done
"$PG/psql" -d medsys_eval_s0 -v ON_ERROR_STOP=1 -Atqc "SELECT 'Patient',count(*) FROM tbl_pacientes UNION ALL SELECT 'Encounter',count(*) FROM tbl_consultas UNION ALL SELECT 'signs',count(*) FROM tbl_signos_vitales UNION ALL SELECT 'Condition',count(*) FROM tbl_diagnosticos UNION ALL SELECT 'omitted_resp',count(*) FROM tbl_signos_vitales WHERE frecuencia_respiratoria IS NULL;" > "$LOGS/s0-census.txt"
echo "Laboratory cluster ready: localhost:$LAB_PORT; databases medsys_eval_s0..s3; marker $LAB_ID"
