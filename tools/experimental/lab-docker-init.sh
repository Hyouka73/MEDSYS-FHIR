#!/bin/sh
set -eu
for schema in s0 s1 s2 s3; do
  db="medsys_eval_$schema"
  createdb --username "$POSTGRES_USER" "$db"
  case "$schema" in
    s0) ddl=schema_s0_base.sql; bulk=02_bulk_data.sql;;
    s1) ddl=schema_s1_flat.sql; bulk=bulk_s1.sql;;
    s2) ddl=schema_s2_lookup.sql; bulk=bulk_s2.sql;;
    s3) ddl=schema_s3_legacy.sql; bulk=bulk_s3.sql;;
  esac
  psql --username "$POSTGRES_USER" --dbname "$db" --set ON_ERROR_STOP=1 --single-transaction --file "/lab-source/sql/$ddl"
  psql --username "$POSTGRES_USER" --dbname "$db" --set ON_ERROR_STOP=1 --single-transaction --file "/lab-source/sql/$bulk"
  psql --username "$POSTGRES_USER" --dbname "$db" --set ON_ERROR_STOP=1 --command "CREATE TABLE experimental_lab_guard(run_id text PRIMARY KEY); INSERT INTO experimental_lab_guard VALUES ('reanudacion-2933964-docker'); GRANT SELECT ON experimental_lab_guard TO medsys_ro;"
done
