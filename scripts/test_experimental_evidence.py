"""Regression tests for evidence instruments, not thesis experimental results."""
import contextlib
import copy
import hashlib
import io
import json
import os
from pathlib import Path
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import patch

from experimental_evidence import canonical_sources, compare, save, export, fault_observations, fault_primary_key, adaptation_run_status, adapt, RestorationFailure
from oracle_verifier import normalize_iso_datetime, safe_float, IndependentOracleModel, FhirOracleVerifier, run_mutation_tests
from run_all_cap4_experiments import consolidate, KINDS
from generate_data import generate_entities


class EvidenceRegressions(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.model = IndependentOracleModel()

    def test_equivalent_timezone_instants(self):
        self.assertEqual(normalize_iso_datetime('2026-10-01T09:00:00-06:00'),
                         normalize_iso_datetime('2026-10-01T15:00:00Z'))

    def test_different_timezone_instants_are_not_equal(self):
        self.assertNotEqual(normalize_iso_datetime('2026-10-01T09:00:00-06:00'),
                            normalize_iso_datetime('2026-10-01T09:00:00Z'))

    def test_fractional_precision_is_not_discarded(self):
        self.assertNotEqual(normalize_iso_datetime('2026-10-01T09:00:00.001Z'),
                            normalize_iso_datetime('2026-10-01T09:00:00.002Z'))

    def test_nanoseconds_are_not_discarded(self):
        self.assertNotEqual(normalize_iso_datetime('2026-10-01T09:00:00.123456001Z'),
                            normalize_iso_datetime('2026-10-01T09:00:00.123456002Z'))

    def test_historical_summer_offset(self):
        self.assertEqual(normalize_iso_datetime('2021-07-01T09:00:00'),
                         normalize_iso_datetime('2021-07-01T14:00:00Z'))

    def test_ambiguous_local_time_is_flagged(self):
        self.assertTrue(normalize_iso_datetime('2021-10-31T01:30:00').startswith('#ambiguous'))

    def test_nonexistent_local_time_is_flagged(self):
        self.assertTrue(normalize_iso_datetime('2021-04-04T02:30:00').startswith('#ambiguous'))

    def test_invalid_numbers_do_not_compare_equal_to_zero(self):
        self.assertNotEqual(safe_float('garbage'), safe_float(0))
        self.assertNotEqual(safe_float(None), safe_float(0))

    def test_network_failure_is_not_an_observed_http_response(self):
        counts=fault_observations([dict(injection_committed=True,response={'status':None,'error':'timeout'},
                     control_before={'status':200},control_after={'status':200})])
        self.assertEqual(counts['http_injection_request_attempts'],1)
        self.assertEqual(counts['observed_http_injections'],0)
        self.assertEqual(counts['nominal_controls_passed'],2)

    def test_s3_fault_keys_match_physical_schema(self):
        self.assertEqual({resource:fault_primary_key('s3',resource) for resource in
                         ('Patient','Encounter','Observation','Condition')},
                         {'Patient':'pac_id','Encounter':'con_id','Observation':'sig_id','Condition':'diag_id'})
        self.assertEqual(fault_primary_key('s0','Patient'),'id_paciente')

    def test_failed_fault_corpus_prevents_adaptation_pass(self):
        self.assertEqual(adaptation_run_status(0,0,0,1,True),'FAIL')
        self.assertEqual(adaptation_run_status(0,0,0,None,True),'INCOMPLETE')
        self.assertEqual(adaptation_run_status(0,0,0,0,True),'PASS')
        self.assertEqual(adaptation_run_status(0,0,0,None,False),'PASS')

    def test_restoration_failure_stops_next_schema_and_own_process(self):
        class InstrumentProcess:
            pid=123
            returncode=None
            def poll(self): return self.returncode
            def terminate(self): self.returncode=-15
            def wait(self,timeout=None): return self.returncode
        with tempfile.TemporaryDirectory() as tmp:
            root=Path(tmp);exe=root/'instrument-fixture.exe';exe.write_bytes(b'NOT A REAL EXECUTABLE')
            configs=root/'configs';configs.mkdir()
            for schema in ('s0','s1','s2','s3'):(configs/f'mapping_rules_{schema}.yaml').write_text('{}')
            args=SimpleNamespace(executable=exe,output=root/'run',config_dir=configs,database_env_prefix='TEST_DB_',
                 write_database_env_prefix='TEST_WRITE_',port_base=0,workers=1,lab_id='instrument-only',corpus='unused')
            def store_snapshot(a):save(a.output,{})
            def store_export(a):save(Path(a.output)/'export_report.json',{});return 0
            def store_report(a):save(a.output,{});return 0
            process=InstrumentProcess()
            with patch.dict(os.environ,{'TEST_DB_S0':'instrument-only'}), \
                 patch('experimental_evidence.snapshot',side_effect=store_snapshot) as snap, \
                 patch('experimental_evidence.subprocess.Popen',return_value=process), \
                 patch('experimental_evidence.fetch',return_value={'status':200}), \
                 patch('experimental_evidence.export',side_effect=store_export), \
                 patch('experimental_evidence.compare',side_effect=store_report), \
                 patch('experimental_evidence.mutations',side_effect=store_report), \
                 patch('experimental_evidence.faults',side_effect=RestorationFailure('instrument failure')):
                with self.assertRaises(RestorationFailure):adapt(args)
            self.assertEqual(snap.call_count,1)
            self.assertEqual(process.returncode,-15)
            self.assertFalse((root/'run/s1').exists())

    def test_previously_unchecked_text_field_detected(self):
        original = self.model.expected_resources['Encounter/1']
        candidate = copy.deepcopy(original)
        candidate['reasonCode'][0]['text'] = 'wrong source text'
        valid, errors = FhirOracleVerifier.verify_resource(candidate, original)
        self.assertFalse(valid)
        self.assertTrue(any('reasonCode' in e for e in errors))

    def test_twenty_original_mutations(self):
        with contextlib.redirect_stdout(io.StringIO()):
            detected, total, details = run_mutation_tests(self.model)
        self.assertEqual((detected, total), (20, 20))
        self.assertEqual([r['id'] for r in details], [f'MUT-{i:02}' for i in range(1, 21)])

    def test_every_patient_represented_in_flat_schema(self):
        patients, _, encounters, _, signs, _, conditions, _ = generate_entities()
        self.assertEqual({r['id_paciente'] for r in encounters}, set(patients))
        self.assertEqual(len(encounters), 2500)
        by_encounter = {r['id_consulta']:r['id_paciente'] for r in encounters}
        self.assertTrue(all(r['id_paciente'] == by_encounter[r['id_consulta']] for r in signs))
        self.assertTrue(all(r['id_paciente'] == by_encounter[r['id_consulta']] for r in conditions))

    def test_consolidation_rejects_missing_criteria(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / 'manifest.json'
            save(path, {})
            with self.assertRaisesRegex(ValueError, 'missing evidence'):
                consolidate(path)

    def test_consolidation_rejects_tampered_report(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / 'manifest.json'
            save(Path(tmp) / 'ca1.json', {'kind':'hl7_validator_execution', 'status':'PASS'})
            save(path, {'CA-1':[{'path':'ca1.json', 'sha256':'0'*64}]})
            with self.assertRaisesRegex(ValueError, 'SHA-256 mismatch'):
                consolidate(path)

    def synthetic_manifest(self, root, schemas, ct_status='PASS'):
        """Instrument fixtures only; never written to thesis evidence runs."""
        manifest = {}
        for criterion, kinds in KINDS.items():
            manifest[criterion] = []
            for index, kind in enumerate(kinds - {'CA-1'}):
                for schema in (schemas if criterion != 'CT' else ['s0']):
                    path = root / f'{criterion}-{index}-{schema}.json'
                    payload = dict(kind=kind, status=ct_status if criterion == 'CT' else 'PASS', schema=schema, schemas=schemas)
                    if kind == 'hl7_validator_execution':
                        payload.update(expected_count=16250,observed_count=16250,report_resource_count=16250,batches=[{'exit_code':0}])
                    elif kind == 'source_vs_rust':
                        payload.update(expected_resources=16250,observed_resources=16250,census_complete=True,discrepancies=[])
                    elif kind == 'oracle_sensitivity_only':
                        payload.update(detected=20,total=20,details=[{'id':f'MUT-{i:02}','detected':True} for i in range(1,21)])
                    elif kind == 'postgres_http_faults':
                        payload.update(cases=100,observed_http_injections=100,nominal_controls=200,all_restored=True,outcomes={'PASS':100})
                    save(path, payload)
                    manifest[criterion].append(dict(path=path.name, sha256=hashlib.sha256(path.read_bytes()).hexdigest()))
        save(root/'manifest.json', manifest)
        return root/'manifest.json'

    def test_one_schema_cannot_close_multischema_criterion(self):
        with tempfile.TemporaryDirectory() as tmp:
            report = consolidate(self.synthetic_manifest(Path(tmp), ['s0']))
            self.assertFalse(report['binding_criteria_evidence_complete'])
            self.assertEqual(report['criteria']['CA-2']['missing_schemas'], ['s1','s2','s3'])

    def test_ct_is_descriptive_not_binding(self):
        with tempfile.TemporaryDirectory() as tmp:
            report = consolidate(self.synthetic_manifest(Path(tmp), ['s0','s1','s2','s3'], ct_status='FAIL'))
            self.assertTrue(report['binding_criteria_all_passed'])
            self.assertTrue(report['characterization_evidence_complete'])
            self.assertFalse(report['characterization_is_binding'])

    def test_missing_export_is_not_pass(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            source = dict(kind='postgres_source_snapshot', schema='s0', tables={
                'tbl_pacientes':[dict(id_paciente=1, curp='ROMA900101HCSMNL09', primer_nombre='Alberto',
                    segundo_nombre=None, apellido_paterno='Ramos', apellido_materno='Gomez',
                    fecha_nacimiento='1990-01-01', sexo_biologico='M', telefono_contacto=None)],
                'tbl_consultas':[], 'tbl_signos_vitales':[], 'tbl_diagnosticos':[]})
            save(root / 'source.json', source)
            args = SimpleNamespace(source=root/'source.json', input_dir=root/'absent', output=root/'report.json')
            with contextlib.redirect_stdout(io.StringIO()):
                code = compare(args)
            self.assertEqual(code, 1)
            report = json.loads(args.output.read_text(encoding='utf-8'))
            self.assertEqual(report['expected_resources'], 1)
            self.assertEqual(report['observed_resources'], 0)
            self.assertEqual(report['discrepancies'][0]['error'], 'missing resource')

    def test_export_preserves_wire_bytes_instead_of_rounding_json(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            source = dict(kind='postgres_source_snapshot',schema='s0',tables={
                'tbl_pacientes':[dict(id_paciente=1,curp='ROMA900101HCSMNL09',primer_nombre='Alberto',
                    segundo_nombre=None,apellido_paterno='Ramos',apellido_materno='Gomez',
                    fecha_nacimiento='1990-01-01',sexo_biologico='M',telefono_contacto=None)],
                'tbl_consultas':[],'tbl_signos_vitales':[],'tbl_diagnosticos':[]})
            save(root/'source.json',source)
            wire='{ "resourceType":"Patient", "id":"1", "x":0.12345678901234567890 }'
            response=dict(status=200,headers={'Content-Type':'application/fhir+json'},body=wire,body_bytes=len(wire.encode()))
            args=SimpleNamespace(source=root/'source.json',base_url='http://fixture.invalid',output=root/'export',workers=1)
            with patch('experimental_evidence.fetch',return_value=response),contextlib.redirect_stdout(io.StringIO()):
                code=export(args)
            self.assertEqual((root/'export/resources/Patient_1.json').read_bytes(),wire.encode())
            self.assertEqual(code,1)  # A one-resource fixture is never a complete census.


if __name__ == '__main__':
    unittest.main(verbosity=2)
