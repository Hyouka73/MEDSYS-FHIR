"""Audit official folder-source control without claiming middleware conformance."""
from collections import Counter
import json
from pathlib import Path
import sys
from evidence_runners import digest, OO_FILE, stamp, write

root=Path(sys.argv[1]).resolve()
batch=root/'batch-0001'
mapping=json.loads((batch/'input-mapping.json').read_text(encoding='utf-8'))
execution=json.loads((batch/'execution.json').read_text(encoding='utf-8'))
report=json.loads((batch/'outcomes.json').read_text(encoding='utf-8'))
assert report['resourceType']=='Bundle'
seen=set()
observed={}
for entry in report['entry']:
    outcome=entry['resource']
    assert outcome['resourceType']=='OperationOutcome'
    source=next(e['valueString'] for e in outcome['extension'] if e['url']==OO_FILE)
    source=str(Path(source).resolve())
    assert source in mapping and source not in seen
    seen.add(source)
    original=mapping[source]
    assert digest(source)==original['sha256']==digest(original['path'])
    observed[original['resource']]=dict(Counter(i['severity'] for i in outcome.get('issue',[])))
assert seen==set(mapping) and len(seen)==2
assert observed['Patient/invalid'].get('error')==2
assert observed['Patient/nominal'].get('error',0)==0
result={'kind':'validator_directory_instrument_control','status':'PASS','checked_utc':stamp(),
        'java_exit_code':execution['exit_code'],'duration_seconds':execution['duration_seconds'],
        'structured_outcomes':len(seen),'per_resource_severity_counts':observed,
        'mapping_one_to_one_and_bytes_unchanged':True,
        'limitation':'PASS means the instrument recognizes an invalid fixture and accepts folder sources, not CA-1 of Rust.'}
write(root/'directory-control-verification.json',result)
print(json.dumps(result))
