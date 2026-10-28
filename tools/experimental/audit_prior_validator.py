"""Read and audit the existing control validation; this does not execute Java."""
import json
from pathlib import Path
import re
from evidence_runners import digest, packages, stamp, write

ROOT = Path(__file__).resolve().parents[2]
CONTROL = ROOT / 'auditoria/evidencias/validator-development/controlled-invalid'
report = json.loads((CONTROL / 'summary.json').read_text(encoding='utf-8'))
log = (CONTROL / 'batch-0001/stdout.log').read_text(encoding='utf-8')
loaded = set()
for listed in re.findall(r'Package Summary:\s*\[([^\]]+)\]', log):
    loaded.update(p.strip() for p in listed.split(',') if p.strip())
resources = json.loads((CONTROL / 'resources.json').read_text(encoding='utf-8'))
audit = {'kind':'prior_validator_control_reinspection', 'inspected_utc':stamp(),
         'original_summary_sha256':digest(CONTROL/'summary.json'),
         'original_outcomes_sha256':digest(CONTROL/'batch-0001/outcomes.json'),
         'validator_process_exit_code':report['batches'][0]['exit_code'],
         'runner_status':report['status'],
         'resources':[{'resource':r['resource'],'severity_counts':r['severity_counts']} for r in resources],
         'loaded_package_ids':sorted(loaded), 'package_hashes_now':packages(loaded),
         'limitation':'Reinspection of a historical control, not a new middleware conformance execution. Package hashes are read now from the retained cache and do not replace the original run timestamp.'}
assert audit['validator_process_exit_code'] == 0
assert audit['runner_status'] == 'FAIL'
assert sum(r['severity_counts'].get('error',0) for r in resources) == 2
assert len(resources) == 2
write(CONTROL.parent/'REINSPECTION_25OCT.json', audit)
print(json.dumps({k:v for k,v in audit.items() if k!='package_hashes_now'}))
