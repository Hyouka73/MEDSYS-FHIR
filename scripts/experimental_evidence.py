#!/usr/bin/env python3
"""Primary evidence collector. Never treats a model self-test as a Rust result.

Database connection strings are read from environment variables, never recorded.
Snapshots use a repeatable-read transaction. Exports retain every HTTP response,
including failures, separately from resources intended for the HL7 validator.
"""
import argparse
import base64
from collections import Counter
from concurrent.futures import ThreadPoolExecutor
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import sys
import subprocess
import socket
from types import SimpleNamespace
import time
import urllib.error
import urllib.request

from oracle_verifier import IndependentOracleModel, FhirOracleVerifier, run_mutation_tests

TABLES = {
    's0': ['tbl_pacientes', 'tbl_consultas', 'tbl_signos_vitales', 'tbl_diagnosticos'],
    's1': ['tbl_sabana_consulta_externa', 'tbl_sabana_diagnosticos'],
    's2': ['tbl_pacientes_s2', 'tbl_consultas_s2', 'tbl_signos_s2', 'tbl_diagnosticos_s2',
           'cat_sexo_biologico', 'cat_estado_consulta', 'cat_cie10', 'cat_tipo_diagnostico'],
    's3': ['tbl_pacientes_s3', 'tbl_consultas_s3', 'tbl_signos_s3', 'tbl_diagnosticos_s3'],
}
FULL_CENSUS = {'Patient': 1000, 'Encounter': 2500, 'Observation': 9750, 'Condition': 3000}


def full_census(model):
    return dict(Counter(key.split('/')[0] for key in model.expected_resources)) == FULL_CENSUS


def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def save(path, value):
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(value, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')


def load(path):
    return json.loads(Path(path).read_text(encoding='utf-8'))


def stamp():
    return datetime.now(timezone.utc).isoformat()


def provenance():
    root = Path(__file__).resolve().parent.parent
    revision = subprocess.run(['git','rev-parse','HEAD'], cwd=root, capture_output=True, text=True, check=False)
    diff = subprocess.run(['git','diff','--binary','HEAD'], cwd=root, capture_output=True, check=False)
    status = subprocess.run(['git','status','--porcelain'], cwd=root, capture_output=True, text=True, check=False)
    return dict(command=[sys.executable] + sys.argv, python=sys.version,
                git_commit=revision.stdout.strip(), git_status=status.stdout,
                tracked_diff_sha256=hashlib.sha256(diff.stdout).hexdigest(),
                scripts_sha256={name: digest(Path(__file__).parent/name)
                                for name in ('experimental_evidence.py','oracle_verifier.py','generate_data.py')})


def fetch(url):
    start = time.perf_counter()
    record = {'url': url, 'started_utc': stamp()}
    try:
        request = urllib.request.Request(url, headers={'Accept': 'application/fhir+json'})
        try:
            response = urllib.request.urlopen(request, timeout=30)
        except urllib.error.HTTPError as error:
            response = error
        with response:
            raw = response.read()
            record.update(status=response.code, headers=dict(response.headers),
                          body=raw.decode('utf-8', errors='replace'),
                          body_bytes=len(raw),
                          body_sha256=hashlib.sha256(raw).hexdigest())
            try:
                raw.decode('utf-8')
            except UnicodeDecodeError:
                record.update(invalid_utf8=True,raw_body_base64=base64.b64encode(raw).decode('ascii'))
    except (OSError, urllib.error.URLError) as error:
        record.update(status=None, error=str(error))
    record['elapsed_seconds'] = time.perf_counter() - start
    return record


def snapshot(args):
    import psycopg
    from psycopg import sql
    result = {'kind': 'postgres_source_snapshot', 'schema': args.schema, 'started_utc': stamp(),
              'tables': {}, 'isolation': 'REPEATABLE READ READ ONLY', 'queries': {}, 'provenance':provenance()}
    with psycopg.connect(os.environ[args.database_env],connect_timeout=5) as conn:
        conn.execute('SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY')
        result['postgres_version'] = conn.execute('SELECT version()').fetchone()[0]
        result['database_name'] = conn.execute('SELECT current_database()').fetchone()[0]
        markers = conn.execute('SELECT run_id FROM experimental_lab_guard').fetchall()
        if len(markers) != 1 or not markers[0][0]:
            raise ValueError('Source database is not marked as one isolated synthetic laboratory')
        result['lab_id'] = markers[0][0]
        for table in TABLES[args.schema]:
            query = sql.SQL('SELECT row_to_json(t) FROM {} t').format(sql.Identifier(table))
            result['queries'][table] = query.as_string(conn)
            rows = [row[0] for row in conn.execute(query)]
            result['tables'][table] = sorted(rows, key=lambda row: json.dumps(row, sort_keys=True))
    result['finished_utc'] = stamp()
    result['table_statistics'] = {
        table: dict(row_count=len(rows), sha256=hashlib.sha256(json.dumps(rows,sort_keys=True,ensure_ascii=False).encode()).hexdigest())
        for table,rows in result['tables'].items()}
    save(args.output, result)
    return 0


def canonical_sources(snapshot_data):
    """Independent, explicit source projections; does not read middleware YAML."""
    if snapshot_data.get('kind') != 'postgres_source_snapshot':
        raise ValueError('Expected a PostgreSQL source snapshot, not generated expected FHIR resources')
    schema, tables = snapshot_data['schema'], snapshot_data['tables']
    if schema == 's0':
        patients, encounters, signs, conditions = [tables[t] for t in TABLES[schema]]
    elif schema == 's1':
        rows = tables['tbl_sabana_consulta_externa']
        grouped = {}
        for row in rows:
            pid = row['id_paciente']
            projected = {k: row.get(k) for k in ('id_paciente', 'curp', 'primer_nombre',
                'segundo_nombre', 'apellido_paterno', 'apellido_materno', 'fecha_nacimiento',
                'sexo_biologico', 'telefono_contacto')}
            if pid in grouped and projected != grouped[pid]:
                raise ValueError(f'Inconsistent demographics across S1 rows for Patient/{pid}')
            grouped[pid] = projected
        patients, encounters, signs, conditions = list(grouped.values()), rows, rows, tables['tbl_sabana_diagnosticos']
    elif schema == 's2':
        lookup = lambda table, key: {r[key]: r for r in tables[table]}
        sexes = lookup('cat_sexo_biologico', 'id_sexo')
        states = lookup('cat_estado_consulta', 'id_estado_consulta')
        codes = lookup('cat_cie10', 'id_cie10')
        kinds = lookup('cat_tipo_diagnostico', 'id_tipo_diag')
        patients = [dict(r, sexo_biologico=sexes[r['id_sexo']]['clave']) for r in tables['tbl_pacientes_s2']]
        encounters = [dict(r, estado_consulta=states[r['id_estado_consulta']]['clave']) for r in tables['tbl_consultas_s2']]
        signs = tables['tbl_signos_s2']
        conditions = [dict(r, codigo_cie10=codes[r['id_cie10']]['clave'],
                          tipo_diagnostico=kinds[r['id_tipo_diag']]['clave'] if r['id_tipo_diag'] is not None else None)
                      for r in tables['tbl_diagnosticos_s2']]
    else:
        def rename(row, mapping):
            return {target: row.get(source) for target, source in mapping.items()}
        patients = [rename(r, dict(id_paciente='pac_id', curp='pac_curp', primer_nombre='pac_nom1',
                    segundo_nombre='pac_nom2', apellido_paterno='pac_ap_pat', apellido_materno='pac_ap_mat',
                    fecha_nacimiento='fech_nac_txt', sexo_biologico='sexo', telefono_contacto='tel_contacto'))
                    for r in tables['tbl_pacientes_s3']]
        encounters = [rename(r, dict(id_consulta='con_id', id_paciente='pac_id', estado_consulta='est_consulta',
                      motivo_consulta='motivo', fecha_hora_inicio='fech_ini_txt', fecha_hora_fin='fech_fin_txt',
                      cedula_medico_tratante='ced_medico', nombre_medico='nom_medico', unidad_medica='unidad_medica'))
                      for r in tables['tbl_consultas_s3']]
        signs = []
        for r in tables['tbl_signos_s3']:
            blood = r['presion_arterial_txt'].split('/')
            if len(blood) != 2:
                raise ValueError('Invalid source blood pressure')
            signs.append(dict(id_signo=r['sig_id'], id_consulta=r['con_id'], id_paciente=r['pac_id'],
                         presion_sistolica=float(blood[0]), presion_diastolica=float(blood[1]),
                         temperatura_celsius=r['temp_celsius'], frecuencia_cardiaca=r['fc'],
                         frecuencia_respiratoria=r['fr'], fecha_registro=r['fech_reg_txt']))
        conditions = [rename(r, dict(id_diagnostico='diag_id', id_consulta='con_id', id_paciente='pac_id',
                      codigo_cie10='cie10', descripcion_diagnostico='desc_diag',
                      tipo_diagnostico='tipo_diag', fecha_diagnostico='fech_diag_txt'))
                      for r in tables['tbl_diagnosticos_s3']]
    def date(value):
        if value is None:
            return None
        for pattern in ('%d/%m/%Y %H:%M:%S', '%d/%m/%Y %H:%M', '%d/%m/%Y'):
            try:
                parsed = datetime.strptime(value, pattern)
                return parsed.isoformat(sep=' ') if ' ' in value else parsed.date().isoformat()
            except ValueError:
                pass
        return value
    model = IndependentOracleModel.__new__(IndependentOracleModel)
    model.pacientes = {r['id_paciente']: dict(r, fecha_nacimiento=date(r['fecha_nacimiento'])) for r in patients}
    model.consultas = [dict(r, estado=r['estado_consulta'], motivo=r['motivo_consulta'],
                       inicio=date(r['fecha_hora_inicio']), fin=date(r['fecha_hora_fin']),
                       cedula=r['cedula_medico_tratante'], medico=r['nombre_medico'], unidad=r.get('unidad_medica'))
                       for r in encounters]
    model.signos = [dict(r, sistolica=r['presion_sistolica'], diastolica=r['presion_diastolica'],
                    temperatura=r['temperatura_celsius'], registro=date(r['fecha_registro'])) for r in signs]
    model.diagnosticos = [dict(r, descripcion=r['descripcion_diagnostico'],
                         fecha=date(r['fecha_diagnostico']), tipo=r['tipo_diagnostico']) for r in conditions]
    model.expected_resources = {}
    model.expected_absent_observations = set()
    model.expected_omitted_condition_ver_status = set()
    model._build_all_expected()
    return model


def export(args):
    source = load(args.source)
    model = canonical_sources(source)
    targets = [(key, 200) for key in sorted(model.expected_resources)]
    targets += [(f'Observation/resp-{rid}', 404) for rid in sorted(model.expected_absent_observations)]
    root = Path(args.output)
    root.mkdir(parents=True, exist_ok=False)
    def collect(target):
        key, expected_status = target
        response = fetch(args.base_url.rstrip('/') + '/fhir/r4/' + key)
        response.update(resource=key, expected_status=expected_status)
        if response['status'] is not None:
            content_type = next((v for k,v in response.get('headers', {}).items() if k.lower() == 'content-type'), '')
            response['invalid_content_type'] = 'application/fhir+json' not in content_type
        name = key.replace('/', '_')
        save(root / 'http' / (name + '.json'), response)
        if response['status'] == 200:
            try:
                resource = json.loads(response['body'])
                if resource.get('resourceType') != key.split('/')[0] or str(resource.get('id')) != key.split('/')[1]:
                    response['identity_error'] = True
                resource_path = root / 'resources' / (name + '.json')
                resource_path.parent.mkdir(parents=True,exist_ok=True)
                # Preserve the actual HTTP payload, not a JSON reserialization
                # that could round decimals or discard duplicate object keys.
                payload = base64.b64decode(response['raw_body_base64']) if response.get('invalid_utf8') else response['body'].encode('utf-8')
                resource_path.write_bytes(payload)
            except (ValueError, AttributeError):
                response['invalid_json'] = True
        elif expected_status == 404 and response['status'] == 404:
            try:
                outcome = json.loads(response['body'])
                response['omission_outcome_valid'] = (outcome.get('resourceType') == 'OperationOutcome'
                    and any(i.get('code') == 'not-found' for i in outcome.get('issue', [])))
            except ValueError:
                response['omission_outcome_valid'] = False
        save(root / 'http' / (name + '.json'), response)
        return response
    with ThreadPoolExecutor(max_workers=args.workers) as pool:
        records = list(pool.map(collect, targets))
    failures = [r['resource'] for r in records if r['status'] != r['expected_status'] or r.get('identity_error')
                or r.get('invalid_json') or r.get('invalid_content_type') or r.get('invalid_utf8')
                or (r['expected_status'] == 404 and not r.get('omission_outcome_valid'))]
    summary = dict(kind='rust_http_export', schema=source['schema'], source_sha256=digest(args.source),
                   provenance=provenance(),
                   expected_resources=len(model.expected_resources), requests=len(records),
                   status_counts=dict(Counter(str(r['status']) for r in records)),
                   expected_omissions=len(model.expected_absent_observations), failures=failures,
                   actual_payload_bytes=sum(r.get('body_bytes',0) for r in records if r['status'] == 200),
                   actual_payload_mean_kib=(sum(r.get('body_bytes',0) for r in records if r['status'] == 200)
                                            / max(1,sum(r['status'] == 200 for r in records)) / 1024),
                   census_complete=full_census(model),
                   status='FAIL' if failures else 'PASS' if full_census(model) else 'INCOMPLETE', finished_utc=stamp())
    save(root / 'export_report.json', summary)
    print(json.dumps(summary))
    return int(summary['status'] != 'PASS')


def compare(args):
    source = load(args.source)
    model = canonical_sources(source)
    resources = Path(args.input_dir) / 'resources'
    discrepancies, observed, by_type, fields = [], set(), Counter(), Counter()
    for path in sorted(resources.glob('*.json')):
        candidate = load(path)
        key = f"{candidate.get('resourceType')}/{candidate.get('id')}"
        if key in observed:
            discrepancies.append(dict(resource=key, field='$identity', error='duplicate resource'))
        observed.add(key)
        expected = model.expected_resources.get(key)
        if expected is None:
            discrepancies.append(dict(resource=key, field='$identity', error='unexpected resource'))
            continue
        by_type[candidate['resourceType']] += 1
        try:
            _, errors = FhirOracleVerifier.verify_resource(candidate, expected)
        except (ValueError, TypeError, KeyError, IndexError, AttributeError) as error:
            errors = [f'$malformed: verifier could not inspect resource: {error}']
        for error in errors:
            discrepancies.append(dict(resource=key, field=error.split(' mismatch')[0], error=error))
        # Census of leaf positions projected by the independent source model.
        def visit(value, prefix='$'):
            if isinstance(value, dict):
                for k, v in value.items():
                    visit(v, prefix + '.' + k)
            elif isinstance(value, list):
                for i, v in enumerate(value):
                    visit(v, f'{prefix}[{i}]')
            else:
                fields[candidate['resourceType'] + ':' + prefix] += 1
        visit(expected)
    for key in sorted(set(model.expected_resources) - observed):
        discrepancies.append(dict(resource=key, field='$identity', error='missing resource'))
    report = dict(kind='source_vs_rust', schema=source['schema'], source_sha256=digest(args.source),
                  provenance=provenance(),
                  expected_resources=len(model.expected_resources), observed_resources=len(observed),
                  resource_denominators=dict(Counter(k.split('/')[0] for k in model.expected_resources)),
                  observed_by_type=dict(by_type), field_positions=dict(fields), discrepancies=discrepancies,
                  comparison_scope='All independently projected leaf positions plus domain and omission constraints; extra non-projected FHIR fields are not compared',
                  census_complete=full_census(model),
                  status='FAIL' if discrepancies else 'PASS' if full_census(model) else 'INCOMPLETE', finished_utc=stamp())
    save(args.output, report)
    print(json.dumps({k:v for k,v in report.items() if k not in ('discrepancies','field_positions')}))
    return int(report['status'] != 'PASS')


def mutations(args):
    model = canonical_sources(load(args.source))
    detected, total, details = run_mutation_tests(model)
    report = dict(kind='oracle_sensitivity_only', schema=load(args.source)['schema'], source_sha256=digest(args.source), detected=detected,
                  provenance=provenance(),
                  total=total, details=details, status='PASS' if detected == total == 20 else 'FAIL',
                  limitation='Mutated independent expectations test oracle sensitivity, not middleware correctness')
    save(args.output, report)
    return int(detected != total or total != 20)


def fault_observations(records):
    """Count received HTTP responses separately from attempted requests."""
    return dict(
        postgres_injections_committed=sum(r.get('injection_committed',False) for r in records),
        http_injection_request_attempts=sum('response' in r for r in records),
        observed_http_injections=sum(r.get('response',{}).get('status') is not None for r in records),
        nominal_control_attempts=sum('control_before' in r for r in records)*2,
        nominal_controls=sum(r.get(which,{}).get('status') is not None for r in records for which in ('control_before','control_after')),
        nominal_controls_passed=sum(r.get(which,{}).get('status') == 200 for r in records for which in ('control_before','control_after')))


def fault_primary_key(schema, resource_type):
    keys = {
        's0': {'Patient':'id_paciente','Encounter':'id_consulta','Observation':'id_signo','Condition':'id_diagnostico'},
        's3': {'Patient':'pac_id','Encounter':'con_id','Observation':'sig_id','Condition':'diag_id'},
    }
    try:
        return keys[schema][resource_type]
    except KeyError as error:
        raise ValueError(f'Unsupported fault schema/resource: {schema}/{resource_type}') from error


def adaptation_run_status(exported, compared, mutated, faults_exit_code=None, faults_required=False):
    if faults_required and faults_exit_code is None:
        return 'INCOMPLETE'
    return 'FAIL' if any(code for code in (exported,compared,mutated,faults_exit_code)) else 'PASS'


class RestorationFailure(RuntimeError):
    """Fatal laboratory contamination risk; never continue another schema."""


def faults(args):
    """Commit one lab fixture, observe HTTP, then delete and verify restoration.

    Database constraints that prevent injection are reported as BLOCKED, never
    as successful middleware rejection. Only previously absent IDs >= 9000
    can be inserted. A run-specific database marker is mandatory.
    """
    import psycopg
    from psycopg import sql
    cases = load(args.corpus)
    root = Path(args.output)
    root.mkdir(parents=True, exist_ok=False)
    results = []
    with psycopg.connect(os.environ[args.database_env], autocommit=True,connect_timeout=5) as conn:
        guard = conn.execute('SELECT run_id FROM experimental_lab_guard WHERE run_id = %s', (args.lab_id,)).fetchone()
        if not guard:
            raise ValueError('Missing matching experimental_lab_guard marker; refusing database writes')
        for case in cases:
            if case['archetype'] != args.schema:
                continue
            record = dict(id=case['id'], category=case['category'], schema=args.schema,
                          started_utc=stamp(), fixture=case, status='BLOCKED')
            table, values = case['table'], dict(case['data'])
            if table not in TABLES[args.schema]:
                raise ValueError(f'Table outside schema allowlist: {table}')
            if table == 'tbl_signos_s3' and 'temp_c' in values:
                values['temp_celsius'] = values.pop('temp_c')
                record['fixture_column_correction'] = 'temp_c -> temp_celsius per schema_s3_legacy.sql'
            pk = fault_primary_key(args.schema,case['resource_type'])
            rid = values[pk]
            if not isinstance(rid, int) or rid < 9000:
                raise ValueError('Fault IDs must be integers >= 9000')
            exists = sql.SQL('SELECT count(*) FROM {} WHERE {} = %s').format(sql.Identifier(table), sql.Identifier(pk))
            if conn.execute(exists, (rid,)).fetchone()[0]:
                raise ValueError(f'Refusing to overwrite existing {table}/{rid}')
            prefix = {'blood_pressure_panel': 'bp-', 'body_temperature': 'temp-',
                      'heart_rate': 'hr-', 'respiratory_rate': 'resp-'}.get(case.get('observation_type'), '')
            path = case['resource_type'] + '/' + prefix
            control_url = args.base_url.rstrip('/') + '/fhir/r4/' + path + '1'
            record['control_before'] = fetch(control_url)
            inserted = False
            try:
                query = sql.SQL('INSERT INTO {} ({}) VALUES ({})').format(sql.Identifier(table),
                    sql.SQL(',').join(map(sql.Identifier, values)),
                    sql.SQL(',').join(sql.Placeholder() for _ in values))
                record['injection_sql'] = query.as_string(conn)
                record['injection_values'] = list(values.values())
                conn.execute(query, tuple(values.values()))
                inserted = True
                record['injection_committed'] = True
                response = fetch(args.base_url.rstrip('/') + '/fhir/r4/' + path + str(rid))
                record['response'] = response
                try:
                    outcome = json.loads(response.get('body', ''))
                    correct = (response['status'] == case['expected_status']
                        and outcome.get('resourceType') == 'OperationOutcome'
                        and any(issue.get('code') == case['expected_issue_code']
                                and issue.get('severity') == 'error' for issue in outcome.get('issue', [])))
                except (ValueError, AttributeError):
                    correct = False
                record['status'] = 'PASS' if correct else 'FAIL'
            except psycopg.Error as error:
                record['injection_error'] = dict(sqlstate=error.sqlstate, message=str(error))
                record['reason'] = 'injection_blocked_by_database'
            finally:
                try:
                    if inserted:
                        deletion = sql.SQL('DELETE FROM {} WHERE {} = %s').format(sql.Identifier(table), sql.Identifier(pk))
                        conn.execute(deletion, (rid,))
                        record['restoration_sql'] = deletion.as_string(conn)
                    record['restored'] = conn.execute(exists, (rid,)).fetchone()[0] == 0
                except psycopg.Error as error:
                    record.update(restored=False, restoration_error=str(error))
                record['control_after'] = fetch(control_url)
                if not record['restored'] or record['control_before']['status'] != 200 or record['control_after']['status'] != 200:
                    record['status'] = 'FAIL'
                    record['control_or_restoration_failure'] = True
                record['finished_utc'] = stamp()
                save(root / (case['id'] + '.json'), record)
                results.append(record)
                if not record['restored']:
                    raise RestorationFailure('Restoration failed; stopped all further injections and schema runs')
    summary = dict(kind='postgres_http_faults', schema=args.schema, lab_id=args.lab_id,
                   provenance=provenance(),
                   corpus_sha256=digest(args.corpus), cases=len(results),
                   outcomes=dict(Counter(r['status'] for r in results)),
                   categories={cat: dict(Counter(r['status'] for r in results if r['category'] == cat))
                               for cat in ('T1','T2','T3','T4','T5')},
                   **fault_observations(results),
                   all_restored=all(r['restored'] for r in results),
                   status='PASS' if results and all(r['status'] == 'PASS' for r in results) else 'FAIL')
    save(root / 'fault_report.json', summary)
    print(json.dumps(summary))
    return int(summary['status'] != 'PASS')


def adapt(args):
    """Evaluate the same already-built binary against four isolated lab DBs.

    Only processes launched here are terminated. No rebuild, schema write,
    container mutation, tag, commit, or push is performed by this collector.
    """
    executable = Path(args.executable).resolve(strict=True)
    root = Path(args.output).resolve()
    root.mkdir(parents=True, exist_ok=False)
    before = digest(executable)
    runs = []
    project = Path(__file__).resolve().parent.parent
    for number, schema in enumerate(TABLES):
        folder = root/schema
        folder.mkdir()
        config = (Path(args.config_dir)/f'mapping_rules_{schema}.yaml').resolve(strict=True)
        frozen_config = folder/'configuration.yaml'
        frozen_config.write_bytes(config.read_bytes())
        database_env = args.database_env_prefix + schema.upper()
        # Validate the environment reference before launching any process.
        database_url = os.environ[database_env]
        record = dict(schema=schema, executable_sha256_before=digest(executable),
                      configuration_original=str(config), configuration=str(frozen_config),
                      configuration_sha256=digest(frozen_config),
                      port=args.port_base+number, started_utc=stamp(), status='INCOMPLETE')
        runs.append(record)
        process = None
        try:
            with socket.socket() as port_check:
                port_check.bind(('127.0.0.1',args.port_base+number))
            source = folder/'source.json'
            snapshot(SimpleNamespace(schema=schema, database_env=database_env, output=source))
            record['source_sha256'] = digest(source)
            environment = dict(os.environ, DATABASE_URL=database_url, MAPPING_RULES_PATH=str(frozen_config),
                               SERVER_HOST='127.0.0.1', SERVER_PORT=str(args.port_base+number), RUST_LOG='warn')
            with (folder/'server-stdout.log').open('w',encoding='utf-8') as stdout, (folder/'server-stderr.log').open('w',encoding='utf-8') as stderr:
                flags = getattr(subprocess, 'CREATE_NO_WINDOW', 0)
                process = subprocess.Popen([str(executable)], cwd=project, env=environment,
                                           stdout=stdout, stderr=stderr, creationflags=flags)
                record['pid'] = process.pid
                record['process_command'] = [str(executable)]
                base = f'http://127.0.0.1:{args.port_base+number}'
                deadline = time.monotonic()+90
                while True:
                    record['health'] = fetch(base+'/health')
                    if record['health']['status'] == 200 and process.poll() is None:
                        break
                    if process.poll() is not None or time.monotonic() >= deadline:
                        raise ValueError(f'{schema} server did not become healthy')
                    time.sleep(0.25)
                exported = export(SimpleNamespace(source=source, base_url=base, output=folder/'export',workers=args.workers))
                compared = compare(SimpleNamespace(source=source, input_dir=folder/'export',output=folder/'oracle.json'))
                mutated = mutations(SimpleNamespace(source=source, output=folder/'mutations.json'))
                if args.lab_id and schema in {'s0','s3'}:
                    record['faults_exit_code'] = faults(SimpleNamespace(schema=schema,database_env=args.write_database_env_prefix+schema.upper(),
                        lab_id=args.lab_id,base_url=base,corpus=args.corpus,output=folder/'faults'))
                record.update(export_exit_code=exported, compare_exit_code=compared, mutations_exit_code=mutated,
                              oracle_report_sha256=digest(folder/'oracle.json'),
                              export_report_sha256=digest(folder/'export/export_report.json'),
                              mutation_report_sha256=digest(folder/'mutations.json'),
                              status=adaptation_run_status(exported,compared,mutated,record.get('faults_exit_code'),
                                  faults_required=bool(args.lab_id and schema in {'s0','s3'})))
        except (OSError, ValueError, KeyError) as error:
            record.update(status='INCOMPLETE', error=str(error))
        except Exception as error:
            if type(error).__module__.startswith('psycopg'):
                record.update(status='BLOCKED', error=str(error))
            else:
                raise
        finally:
            if process is not None:
                if process.poll() is None:
                    process.terminate()
                    try:
                        process.wait(timeout=15)
                    except subprocess.TimeoutExpired:
                        process.kill()
                        process.wait(timeout=15)
                record['process_exit_code_after_operator_stop'] = process.returncode
            record.update(executable_sha256_after=digest(executable), finished_utc=stamp())
            save(folder/'transition.json', record)
            save(root/'schema-runs.json',runs)
    after = digest(executable)
    invariant = before == after and all(r['executable_sha256_before'] == before == r['executable_sha256_after'] for r in runs)
    executed = len(runs) == 4 and all('export_exit_code' in r for r in runs)
    transition = dict(kind='binary_configuration_transition', schemas=list(TABLES), provenance=provenance(),
                      artifact_scope='New locally corrected binary; no claim of historical v1.2.0-eval identity',
                      executable=str(executable), executable_sha256_before=before, executable_sha256_after=after,
                      invariant_binary=invariant, schema_runs=runs,
                      status='PASS' if invariant and executed else 'INCOMPLETE')
    contracts = dict(kind='adaptation_contract_comparison', schemas=list(TABLES), provenance=provenance(),
                     fault_corpus_requested=bool(args.lab_id),
                     fault_corpus_scope='Original97 S0+3 S3; S1/S2 fault coverage not demonstrated' if args.lab_id else 'Not requested in this adaptation run',
                     binary_transition_report_sha256=None,
                     resource_scope='Base census 16,250 per schema; S3 body_weight excluded and requires separate evaluation',
                     schemas_comparison={r['schema']:dict(status=r['status'], oracle_report=f"{r['schema']}/oracle.json",
                         oracle_report_sha256=r.get('oracle_report_sha256')) for r in runs},
                     status='PASS' if invariant and executed and all(r['status']=='PASS' for r in runs) else 'FAIL' if executed else 'INCOMPLETE')
    save(root/'ca4-transition.json',transition)
    contracts['binary_transition_report_sha256'] = digest(root/'ca4-transition.json')
    save(root/'ca5-contract.json',contracts)
    return 0 if transition['status'] == contracts['status'] == 'PASS' else 1


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    subs = parser.add_subparsers(dest='command', required=True)
    p = subs.add_parser('snapshot')
    p.add_argument('--schema', required=True, choices=TABLES)
    p.add_argument('--database-env', default='EXPERIMENT_DATABASE_URL')
    p.add_argument('--output', required=True)
    p.set_defaults(func=snapshot)
    p = subs.add_parser('export')
    p.add_argument('--source', required=True)
    p.add_argument('--base-url', required=True)
    p.add_argument('--workers', type=int, default=10)
    p.add_argument('--output', required=True, help='New directory; existing directories are refused')
    p.set_defaults(func=export)
    p = subs.add_parser('compare')
    p.add_argument('--source', required=True)
    p.add_argument('--input-dir', required=True)
    p.add_argument('--output', required=True)
    p.set_defaults(func=compare)
    p = subs.add_parser('mutations')
    p.add_argument('--source', required=True)
    p.add_argument('--output', required=True)
    p.set_defaults(func=mutations)
    p = subs.add_parser('faults')
    p.add_argument('--schema', required=True, choices=['s0', 's3'])
    p.add_argument('--database-env', default='EXPERIMENT_DATABASE_URL')
    p.add_argument('--lab-id', required=True)
    p.add_argument('--base-url', required=True)
    p.add_argument('--corpus', default='tests/fault_corpus/fault_corpus_100.json')
    p.add_argument('--output', required=True)
    p.set_defaults(func=faults)
    p = subs.add_parser('adapt')
    p.add_argument('--executable', required=True)
    p.add_argument('--config-dir', default='config')
    p.add_argument('--database-env-prefix', default='EXPERIMENT_DATABASE_URL_')
    p.add_argument('--write-database-env-prefix', default='EXPERIMENT_WRITE_DATABASE_URL_',
                   help='Separate lab writer environment prefix for fault injections; server should use medsys_ro')
    p.add_argument('--port-base', type=int, default=3310)
    p.add_argument('--workers', type=int, default=8)
    p.add_argument('--lab-id', help='If supplied, execute the original97 S0+3 S3 fault fixtures while their servers run')
    p.add_argument('--corpus', default='tests/fault_corpus/fault_corpus_100.json')
    p.add_argument('--output', required=True)
    p.set_defaults(func=adapt)
    args = parser.parse_args()
    try:
        return args.func(args)
    except (OSError, ValueError, KeyError) as error:
        print(f'INCOMPLETE EVIDENCE: {error}', file=sys.stderr)
        return 2
    except ImportError as error:
        print(f'MISSING EXPERIMENT DEPENDENCY: {error}; install scripts/requirements-experiments.txt', file=sys.stderr)
        return 2
    except Exception as error:
        if type(error).__module__.startswith('psycopg'):
            print(f'BLOCKED POSTGRESQL EVIDENCE: {error}', file=sys.stderr)
            return 2
        raise


if __name__ == '__main__':
    sys.exit(main())
