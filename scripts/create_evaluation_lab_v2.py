#!/usr/bin/env python3
"""Create ONLY new medsys_eval_v2_s0..s3 synthetic DBs; never drop or overwrite.

Administrative DSN is read from EXPERIMENT_ADMIN_DATABASE_URL. The supplied
cluster must be loopback-local. Old evaluation DBs and their data are preserved.
The original schema files remain unchanged; the initial cluster-role management
block is omitted because roles already exist in this isolated laboratory.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import sys
import time

import psycopg
from psycopg import sql

from experimental_evidence import save, stamp, digest, provenance

FILES = {
    's0':('schema_s0_base.sql','02_bulk_data.sql'),
    's1':('schema_s1_flat.sql','bulk_s1.sql'),
    's2':('schema_s2_lookup.sql','bulk_s2.sql'),
    's3':('schema_s3_legacy.sql','bulk_s3.sql'),
}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output',required=True)
    args = parser.parse_args()
    folder = Path(args.output)
    folder.mkdir(parents=True,exist_ok=False)
    root = Path(__file__).resolve().parent.parent
    report = dict(kind='synthetic_laboratory_v2_creation', started_utc=stamp(),
                  lab_id='reeval20261025-v2',provenance=provenance(),databases=[])
    try:
        dsn = os.environ['EXPERIMENT_ADMIN_DATABASE_URL']
        params = psycopg.conninfo.conninfo_to_dict(dsn)
        if params.get('host') not in {'127.0.0.1','localhost','::1'}:
            raise ValueError('Refusing administration outside a loopback laboratory cluster')
        save(folder/'creation.json',report)
        with psycopg.connect(dsn,autocommit=True,connect_timeout=5) as admin:
            # Check all destinations before making the first change.
            for schema in FILES:
                name = 'medsys_eval_v2_'+schema
                if admin.execute('SELECT 1 FROM pg_database WHERE datname=%s',(name,)).fetchone():
                    raise ValueError(f'Refusing existing database {name}; no overwrite or DROP is allowed')
            for schema,(ddl_name,bulk_name) in FILES.items():
                name = 'medsys_eval_v2_'+schema
                admin.execute(sql.SQL('CREATE DATABASE {}').format(sql.Identifier(name)))
                record = dict(schema=schema,database=name,started_utc=stamp(),status='CREATED_NOT_LOADED')
                report['databases'].append(record)
                save(folder/'creation.json',report)
                ddl_path,bulk_path = root/'docker/sql'/ddl_name,root/'docker/sql'/bulk_name
                ddl = ddl_path.read_text(encoding='utf-8')
                ddl = ddl[ddl.index('CREATE TABLE IF NOT EXISTS '):]
                # Preserve exactly the executed DDL and its difference in scope.
                save(folder/f'{schema}-inputs.json',dict(original_ddl=str(ddl_path),original_ddl_sha256=digest(ddl_path),
                    bulk=str(bulk_path),bulk_sha256=digest(bulk_path),
                    omitted_scope='Initial DO block managing existing cluster roles; table DDL and seed inserts unchanged'))
                (folder/f'{schema}-applied-ddl.sql').write_text(ddl,encoding='utf-8')
                started = time.monotonic()
                connection = dict(params,dbname=name)
                with psycopg.connect(**connection,connect_timeout=5) as conn:
                    conn.execute(ddl,prepare=False)
                    conn.execute(bulk_path.read_text(encoding='utf-8'),prepare=False)
                    conn.execute('CREATE TABLE experimental_lab_guard(run_id text PRIMARY KEY)')
                    conn.execute('INSERT INTO experimental_lab_guard VALUES (%s)',(report['lab_id'],))
                    conn.execute('GRANT SELECT ON experimental_lab_guard TO medsys_ro')
                record.update(status='LOADED',duration_seconds=time.monotonic()-started,
                              bulk_sha256=digest(bulk_path),applied_ddl_sha256=digest(folder/f'{schema}-applied-ddl.sql'),finished_utc=stamp())
                save(folder/'creation.json',report)
                print(name,'LOADED',flush=True)
        report.update(status='PASS',finished_utc=stamp())
    except (OSError,ValueError,KeyError,psycopg.Error) as error:
        report.update(status='BLOCKED',error=str(error),finished_utc=stamp())
    save(folder/'creation.json',report)
    return 0 if report.get('status') == 'PASS' else 1


if __name__ == '__main__':
    sys.exit(main())
