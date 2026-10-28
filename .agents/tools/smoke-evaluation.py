"""Exercise a frozen binary against the isolated synthetic S0 database."""
import json
import os
from pathlib import Path
import subprocess
import time
import urllib.error
import urllib.request

root = Path(__file__).resolve().parents[2]
out = root / 'auditoria/evidencias/2026-10-27-run1'
exe = root / 'output/evaluation/20261027/artifact-v1/medsys-server.exe'
env = dict(os.environ, DATABASE_URL='postgres://medsys_ro@127.0.0.1:55432/medsys_eval_v2_s0',
           SERVER_HOST='127.0.0.1', SERVER_PORT='3330', RUST_LOG='warn',
           MAPPING_RULES_PATH=str(root / 'config/mapping_rules_s0.yaml'))
results = []
with (out/'smoke-server.out.log').open('wb') as stdout, (out/'smoke-server.err.log').open('wb') as stderr:
    server = subprocess.Popen([str(exe)], cwd=root, env=env, stdout=stdout, stderr=stderr,
                              creationflags=getattr(subprocess, 'CREATE_NO_WINDOW', 0))
    try:
        for suffix, name in [('health', 'health'), ('fhir/r4/Patient/1', 'patient-1')]:
            url = 'http://127.0.0.1:3330/' + suffix
            for attempt in range(20):
                try:
                    with urllib.request.urlopen(url, timeout=10) as response:
                        body = response.read()
                        (out/f'smoke-{name}.json').write_bytes(body)
                        results.append({'url': url, 'status': response.status, 'content_type': response.headers.get('content-type')})
                    break
                except urllib.error.HTTPError as error:
                    (out/f'smoke-{name}.json').write_bytes(error.read())
                    results.append({'url': url, 'status': error.code, 'content_type': error.headers.get('content-type')})
                    break
                except OSError as error:
                    if server.poll() is not None or attempt == 19:
                        results.append({'url': url, 'error': str(error)})
                        break
                    time.sleep(0.5)
    finally:
        server.terminate()
        server.wait(timeout=10)
(out/'smoke-results.json').write_text(json.dumps(results, indent=2)+'\n', encoding='utf-8')
print(json.dumps(results))
raise SystemExit(0 if len(results) == 2 and all(r.get('status') == 200 for r in results) else 1)
