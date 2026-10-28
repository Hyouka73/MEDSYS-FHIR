#!/usr/bin/env bash
set -euo pipefail
SOURCE=/tmp/medsys-reverificacion-2933964
TARGET=/var/lib/postgresql/medsys-eval-2933964
PG=/usr/lib/postgresql/16/bin
REPO='/mnt/d/Proyectos X/MedSys-FHIR'
LOGS="$REPO/auditoria/evidencias/lab-development/migration-$(date -u +%Y%m%dT%H%M%SZ)"
mkdir -p "$LOGS"
[[ "$(readlink -f "$SOURCE")" == /tmp/medsys-reverificacion-2933964 ]]
[[ "$(readlink -m "$TARGET")" == /var/lib/postgresql/medsys-eval-2933964 ]]
sudo -n -u postgres test -f "$SOURCE/PG_VERSION"
[[ ! -e "$TARGET" ]]
export PGHOST=127.0.0.1 PGPORT=55432 PGUSER=postgres
for schema in s0 s1 s2 s3; do
  marker=$("$PG/psql" -d "medsys_eval_$schema" -Atqc 'SELECT run_id FROM experimental_lab_guard')
  [[ "$marker" == reanudacion-2933964 ]]
done
sudo -n -u postgres "$PG/pg_ctl" -D "$SOURCE" -w stop -m fast > "$LOGS/stop.log" 2>&1
# Only the explicitly checked, self-created laboratory directory is moved.
sudo -n mv -- "$SOURCE" "$TARGET"
sudo -n -u postgres sed -i "s@unix_socket_directories = '/tmp/medsys-reverificacion-2933964'@unix_socket_directories = '/var/lib/postgresql/medsys-eval-2933964'@" "$TARGET/postgresql.conf"
sudo -n -u postgres "$PG/pg_ctl" -D "$TARGET" -l "$TARGET/server.log" -w start > "$LOGS/start.log" 2>&1
for schema in s0 s1 s2 s3; do
  "$PG/psql" -d "medsys_eval_$schema" -v ON_ERROR_STOP=1 -Atqc "SELECT current_database(); SELECT run_id FROM experimental_lab_guard; SHOW data_directory; SHOW port; SET ROLE medsys_ro; SELECT current_user; SELECT has_table_privilege(current_user,'experimental_lab_guard','SELECT'),has_table_privilege(current_user,'experimental_lab_guard','INSERT');" > "$LOGS/$schema-verification.txt"
done
echo "Isolated laboratory cluster persisted at $TARGET; localhost:55432"
