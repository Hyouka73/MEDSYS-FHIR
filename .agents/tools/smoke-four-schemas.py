"""Capture small, explicitly non-exhaustive HTTP controls for the frozen binary."""
import json
import os
from pathlib import Path
import subprocess
import time
import urllib.error
import urllib.request

root = Path(__file__).resolve().parents[2]
out = root / 'auditoria/evidencias/2026-10-27-run1/smoke-four'
out.mkdir(exist_ok=False)
exe = root / 'output/evaluation/20261027/artifact-v1/medsys-server.exe'
paths = ['health', 'fhir/r4/Patient/1', 'fhir/r4/Encounter/1', 'fhir/r4/Observation/bp-1',
         'fhir/r4/Observation/temp-1', 'fhir/r4/Observation/hr-1', 'fhir/r4/Observation/resp-1',
         'fhir/r4/Condition/1', 'fhir/r4/Patient?limit=1']
results = []
for n in range(4):
    schema, port = f's{n}', 3340+n
    env = dict(os.environ, DATABASE_URL=f'postgres://medsys_ro@127.0.0.1:55432/medsys_eval_v2_{schema}',
               SERVER_HOST='127.0.0.1', SERVER_PORT=str(port), RUST_LOG='warn',
               MAPPING_RULES_PATH=str(root/f'config/mapping_rules_{schema}.yaml'))
    with (out/f'{schema}-stdout.log').open('wb') as stdout, (out/f'{schema}-stderr.log').open('wb') as stderr:
        server = subprocess.Popen([str(exe)], cwd=root, env=env, stdout=stdout, stderr=stderr,
                                  creationflags=getattr(subprocess, 'CREATE_NO_WINDOW', 0))
        try:
            for ix, path in enumerate(paths):
                url = f'http://127.0.0.1:{port}/{path}'
                for attempt in range(20):
                    try:
                        with urllib.request.urlopen(url, timeout=20) as response:
                            body, status, content_type = response.read(), response.status, response.headers.get('content-type')
                        break
                    except urllib.error.HTTPError as error:
                        body, status, content_type = error.read(), error.code, error.headers.get('content-type')
                        break
                    except OSError as error:
                        if server.poll() is not None or attempt == 19:
                            body, status, content_type = str(error).encode(), None, None
                            break
                        time.sleep(0.5)
                (out/f'{schema}-{ix:02d}.body').write_bytes(body)
                record = {'schema': schema, 'url': url, 'status': status, 'content_type': content_type}
                if path.startswith('fhir/r4/Patient?') and status == 200:
                    record['bundle_total'] = json.loads(body).get('total')
                results.append(record)
        finally:
            server.terminate()
            server.wait(timeout=10)
(out/'results.json').write_text(json.dumps(results, indent=2)+'\n', encoding='utf-8')
print(json.dumps(results, ensure_ascii=False))
raise SystemExit(0 if len(results) == 36 and all(r['status'] == 200 for r in results) else 1)
