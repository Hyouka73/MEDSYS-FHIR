"""Create a review checkpoint, excluding credentials and installed dependencies."""
from pathlib import Path
import hashlib
import json
import re
import subprocess
import zipfile

root = Path(__file__).resolve().parents[2]
out = root / 'auditoria' / 'revision'
out.mkdir(exist_ok=True)
stamp = __import__('datetime').datetime.now(__import__('datetime').timezone.utc).strftime('%Y%m%dT%H%M%SZ')
archive = out / f'medsys-review-{stamp}.zip'
diff = subprocess.check_output(['git', 'diff', '--', 'crates', 'Cargo.lock', 'scripts', '.agents/rules/project-profile.md'], cwd=root)
files = [root / p for p in ['STATE.md', 'BACKLOG.md', 'DECISIONS.md', 'auditoria/REVERIFICACION_2026-10-25.md']]
new_paths = subprocess.check_output(['git', 'ls-files', '--others', '--exclude-standard', '--', 'crates', 'scripts', '.agents/tools', 'tools/experimental'], cwd=root, text=True).splitlines()
files += [root / p for p in new_paths if Path(p).suffix.lower() in {'.rs', '.py', '.ps1', '.md', '.txt', '.yaml', '.yml', '.sh'}]
files += [p for p in (root / 'auditoria/evidencias').rglob('*') if p.is_file() and p.suffix.lower() in {'.md', '.txt', '.log', '.json'} and p.stat().st_size < 10_000_000 and '2026-10-20-reverificacion' not in p.parts and 'pending' not in p.parts]
credential = re.compile(rb'(?:postgres(?:ql)?://[^\s/@:]+:[^\s/@]+@|(?:access_token|refresh_token|client_secret)"\s*:\s*"[^"\s]+)', re.I)
manifest, skipped = {}, []
with zipfile.ZipFile(archive, 'x', zipfile.ZIP_DEFLATED) as z:
    if not credential.search(diff):
        z.writestr('code.diff', diff)
        manifest['code.diff'] = hashlib.sha256(diff).hexdigest()
    else:
        skipped.append('code.diff: credential-like text; inspect locally')
    for p in sorted(set(files)):
        data = p.read_bytes()
        name = p.relative_to(root).as_posix()
        if credential.search(data) or p.name == 'stderr.log':
            skipped.append(name)
            continue
        z.writestr(name, data)
        manifest[name] = hashlib.sha256(data).hexdigest()
    z.writestr('MANIFEST.json', json.dumps({'kind': 'review_checkpoint_not_experimental_approval', 'commit': subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=root, text=True).strip(), 'files_sha256': manifest, 'excluded': skipped, 'note': 'Installed binaries, dependencies, build output and credentials excluded; code.diff covers tracked modifications only.'}, indent=2))
print(json.dumps({'archive': str(archive), 'sha256': hashlib.sha256(archive.read_bytes()).hexdigest(), 'files': len(manifest), 'excluded': len(skipped)}))
