"""Real-process CA-1 and CT runners. Never infer a passing result from missing output."""
from __future__ import annotations
import argparse
from collections import Counter
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import sys
import threading
import time

ROOT = Path(__file__).resolve().parents[2]
OO_FILE = 'http://hl7.org/fhir/StructureDefinition/operationoutcome-file'


def stamp():
    return datetime.now(timezone.utc).isoformat()


def digest(path):
    with Path(path).open('rb') as stream:
        return hashlib.file_digest(stream, 'sha256').hexdigest()


def write(path, value):
    Path(path).write_text(json.dumps(value, ensure_ascii=False, indent=2), encoding='utf-8')


def command(args, timeout=None):
    result = subprocess.run(args, capture_output=True, text=True, encoding='utf-8', errors='replace', timeout=timeout)
    return {'command': args, 'exit_code': result.returncode, 'stdout': result.stdout, 'stderr': result.stderr}


def windows_command_length(command):
    return len(subprocess.list2cmdline(command).encode('utf-16-le')) // 2 + 1


def validator_batches(inputs, batch_size, prefix, suffix, max_chars=24000):
    """Keep full CreateProcess command lines below the Windows 32767-char limit."""
    batch = []
    for item in inputs:
        candidate = batch + [item]
        length = windows_command_length(prefix + [i['path'] for i in candidate] + suffix)
        if batch and (len(candidate) > batch_size or length > max_chars):
            yield batch
            batch = []
        batch.append(item)
        if windows_command_length(prefix + [i['path'] for i in batch] + suffix) > max_chars:
            raise ValueError('A single validator input path exceeds the command-line safety limit')
    if batch:
        yield batch


def stage_validation_inputs(batch, folder):
    """Create byte-identical short-name inputs and preserve their original identity."""
    folder.mkdir()
    mapping = {}
    for index, original in enumerate(batch):
        staged = folder / f'{index:05d}.json'
        try:
            os.link(original['path'], staged)
            method = 'hardlink'
        except OSError:
            shutil.copyfile(original['path'], staged)
            method = 'byte_copy'
        if digest(staged) != original['sha256']:
            raise ValueError('Staged input differs from recorded original SHA-256')
        mapping[str(staged.resolve())] = dict(original, validation_input_path=str(staged.resolve()), staging_method=method)
    return mapping


def packages(loaded_ids):
    cache = Path.home() / '.fhir' / 'packages'
    result = []
    recorded = set()
    canonical = [cache/identity/'package'/'package.json' for identity in sorted(loaded_ids)]
    manifests = [p for p in canonical if p.is_file()]
    manifests.extend(p for p in sorted(cache.glob('*/package/package.json')) if p not in manifests)
    for manifest in manifests:
        package = json.loads(manifest.read_text(encoding='utf-8'))
        identity = str(package.get('name')) + '#' + str(package.get('version'))
        if identity in loaded_ids and identity not in recorded:
            files = sorted(manifest.parent.rglob('*'))
            h = hashlib.sha256()
            count = 0
            for file in files:
                if file.is_file():
                    h.update(str(file.relative_to(manifest.parent)).replace('\\', '/').encode())
                    h.update(bytes.fromhex(digest(file)))
                    count += 1
            result.append({'name': package.get('name'), 'version': package.get('version'), 'directory': str(manifest.parent), 'file_count': count, 'tree_sha256': h.hexdigest(), 'hash_definition': 'sorted relative POSIX path bytes followed by binary SHA256 for each file'})
            recorded.add(identity)
    missing = set(loaded_ids) - recorded
    if missing:
        raise ValueError('Missing cache manifest for actually loaded packages: ' + ', '.join(sorted(missing)))
    return result


def validate(args):
    out = Path(args.output).resolve()
    out.mkdir(parents=True, exist_ok=False)
    meta = {'kind': 'CA-1', 'started_utc': stamp(), 'status': 'BLOCKED', 'fhir_version': '4.0.1', 'terminology': args.tx,
            'terminology_limit': 'tx=n/a disables terminology server checks; structural conformance is not complete terminology validation' if args.tx == 'n/a' else 'Terminology checks depend on server availability and packages.', 'batches': []}
    meta['runner_sha256'] = digest(__file__)
    if args.schema:
        meta['schema'] = args.schema
    try:
        jar = Path(args.jar).resolve(strict=True)
        source = Path(args.input).resolve(strict=True)
        if not jar.is_file() or not source.is_dir():
            raise ValueError('Expected existing JAR file and resource directory')
        inputs = []
        for path in sorted(source.rglob('*.json')):
            resource = json.loads(path.read_text(encoding='utf-8'))
            if resource.get('resourceType') in {'Patient', 'Encounter', 'Observation', 'Condition'}:
                inputs.append({'path': str(path), 'resource': resource.get('resourceType') + '/' + str(resource.get('id')), 'sha256': digest(path)})
        if not inputs:
            raise ValueError('No actual FHIR resources found in input directory')
        write(out / 'inputs.json', inputs)
        census = Counter(i['resource'].split('/')[0] for i in inputs)
        identities = Counter(i['resource'] for i in inputs)
        duplicates = [identity for identity, count in identities.items() if count != 1]
        meta.update(jar=str(jar), jar_sha256=digest(jar), java=command([args.java, '-version']), expected_count=args.expected_count, observed_count=len(inputs), resource_census=dict(census), unique_resource_count=len(identities), duplicate_resource_ids=duplicates)
        failed = len(inputs) != args.expected_count or bool(duplicates)
        if args.expected_count == 16250:
            expected_census = {'Patient':1000, 'Encounter':2500, 'Observation':9750, 'Condition':3000}
            meta['expected_resource_census'] = expected_census
            failed |= dict(census) != expected_census
        infra = False
        counts = Counter()
        records = []
        loaded_ids = set()
        prefix = [args.java, '-Xmx' + args.heap, '-jar', str(jar)]
        suffix = ['-version', '4.0.1', '-tx', args.tx, '-output', str(out / 'batch-999999' / 'outcomes.json'), '-output-style', 'json']
        batches = (inputs[i:i+args.batch_size] for i in range(0,len(inputs),args.batch_size)) if args.stage_batches else validator_batches(inputs,args.batch_size,prefix,suffix)
        meta['input_mode'] = 'staged_batch_directories' if args.stage_batches else 'explicit_original_paths'
        for number, batch in enumerate(batches, 1):
            folder = out / f'batch-{number:04d}'
            folder.mkdir()
            report = folder / 'outcomes.json'
            expected = stage_validation_inputs(batch,folder/'inputs') if args.stage_batches else {str(Path(x['path']).resolve()):x for x in batch}
            if args.stage_batches:
                write(folder/'input-mapping.json',expected)
            sources = [str(folder/'inputs')] if args.stage_batches else [x['path'] for x in batch]
            cmd = prefix + sources + ['-version', '4.0.1', '-tx', args.tx, '-output', str(report), '-output-style', 'json']
            started = time.monotonic()
            item = {'command': cmd, 'exit_code': None, 'started_utc': stamp(), 'input_count': len(batch)}
            item['windows_command_line_utf16_units_including_nul'] = windows_command_length(cmd)
            meta['batches'].append(item)
            write(folder / 'execution.json', item)
            write(out / 'summary.json', meta)
            with (folder / 'stdout.log').open('w', encoding='utf-8') as stdout, (folder / 'stderr.log').open('w', encoding='utf-8') as stderr:
                proc = subprocess.run(cmd, stdout=stdout, stderr=stderr, timeout=args.timeout)
            item.update(exit_code=proc.returncode, duration_seconds=time.monotonic()-started, finished_utc=stamp())
            log_text = (folder / 'stdout.log').read_text(encoding='utf-8', errors='replace')
            for listed in re.findall(r'Package Summary:\s*\[([^\]]+)\]', log_text):
                loaded_ids.update(p.strip() for p in listed.split(',') if p.strip())
            write(folder / 'execution.json', item)
            if not report.exists():
                infra = True
                item['error'] = 'Validator did not produce structured report'
                continue
            outcome = json.loads(report.read_text(encoding='utf-8-sig'))
            outcomes = [e['resource'] for e in outcome.get('entry', [])] if outcome.get('resourceType') == 'Bundle' else [outcome]
            found = set()
            for oo in outcomes:
                if oo.get('resourceType') != 'OperationOutcome':
                    infra = True
                    continue
                file = next((e.get('valueString') for e in oo.get('extension', []) if e.get('url') == OO_FILE), None)
                if file is None and len(batch) == 1:
                    file = next(iter(expected))
                resolved = str(Path(file).resolve()) if file else None
                if resolved not in expected or resolved in found:
                    infra = True
                    continue
                found.add(resolved)
                sev = Counter(issue.get('severity', 'MISSING') for issue in oo.get('issue', []))
                counts.update(sev)
                if sev['fatal'] or sev['error']:
                    failed = True
                records.append({**expected[resolved], 'batch': number, 'severity_counts': dict(sev), 'issues': oo.get('issue', [])})
            if found != set(expected):
                infra = True
                item['missing_outcomes'] = sorted(set(expected) - found)
            if proc.returncode != 0:
                failed = True
            write(out / 'resources.json', records)
            write(out / 'summary.json', meta)
        if not loaded_ids:
            raise ValueError('No observed Package Summary in validator logs; package provenance is incomplete')
        meta.update(status='BLOCKED' if infra else ('FAIL' if failed else 'PASS'), severity_counts=dict(counts), report_resource_count=len(records), loaded_package_ids=sorted(loaded_ids), packages=packages(loaded_ids))
    except (OSError, ValueError, subprocess.SubprocessError, KeyError) as error:
        meta['error'] = str(error)
    meta['finished_utc'] = stamp()
    write(out / 'summary.json', meta)
    print(json.dumps(meta, ensure_ascii=False))
    return 0 if meta['status'] == 'PASS' else 1


def telemetry(stop, containers, path):
    with path.open('w', encoding='utf-8') as log:
        while not stop.is_set():
            start = time.monotonic()
            sample = {'started_utc': stamp(), 'containers': {}, 'alignment': 'one docker stats call for all containers; cgroup/proc samples sequential within recorded start/end'}
            try:
                sample['docker_stats'] = command(['docker', 'stats', '--no-stream', '--format', '{{json .}}'] + containers, 15)
                for container in containers:
                    sample['containers'][container] = command(['docker', 'exec', container, 'sh', '-c', 'cat /sys/fs/cgroup/cpu.stat; cat /sys/fs/cgroup/memory.current; grep "^VmRSS:" /proc/1/status'], 15)
            except (OSError, subprocess.SubprocessError) as error:
                sample['error'] = str(error)
            sample.update(finished_utc=stamp(), duration_seconds=time.monotonic()-start)
            log.write(json.dumps(sample) + '\n')
            log.flush()
            stop.wait(max(0, 2-(time.monotonic()-start)))


def native_telemetry(stop, server_pid, path):
    """Alternative host measurements, explicitly not cgroup protocol evidence."""
    with path.open('w', encoding='utf-8') as log:
        while not stop.is_set():
            start = time.monotonic()
            sample = {'started_utc': stamp(), 'mode': 'Windows Rust process + WSL laboratory PostgreSQL processes', 'server_pid': server_pid}
            try:
                query = f'Get-Process -Id {int(server_pid)} -ErrorAction Stop | Select-Object Id,CPU,WorkingSet64,PrivateMemorySize64 | ConvertTo-Json -Compress'
                sample['rust_windows'] = command(['powershell.exe', '-NoProfile', '-NonInteractive', '-Command', query], 20)
                # Only this laboratory cluster and its direct worker children, never all host processes.
                query = 'p=$(sudo -n -u postgres head -1 /var/lib/postgresql/medsys-eval-2933964/postmaster.pid); ps -p "$p" --ppid "$p" -o pid=,ppid=,cputimes=,rss=,comm='
                sample['postgres_wsl'] = command(['wsl', '-d', 'Ubuntu', '--', 'bash', '-lc', query], 20)
            except (OSError, subprocess.SubprocessError) as error:
                sample['error'] = str(error)
            sample.update(finished_utc=stamp(), duration_seconds=time.monotonic()-start)
            log.write(json.dumps(sample) + '\n')
            log.flush()
            stop.wait(max(0, 2-(time.monotonic()-start)))


def ct(args):
    out = Path(args.output).resolve()
    out.mkdir(parents=True, exist_ok=False)
    meta = {'kind': 'CT', 'started_utc': stamp(), 'status': 'BLOCKED', 'replicas': [], 'protocol': '90s warmup; 300s plateau at 50 VUs; 30s cooldown; sleep 0.1-0.3s; original script thresholds retained', 'definitions': {'CPU': 'docker stats CPU %; 100% corresponds to one CPU core; raw cgroup cpu.stat cumulative microseconds retained', 'memory': 'docker stats working-set value, raw cgroup memory.current bytes and /proc/1/status VmRSS KiB retained separately', 'joint_percentiles': 'must be computed from temporally aligned per-container samples; never sum individual percentiles'}, 'base_url': args.base_url}
    try:
        k6 = Path(args.k6).resolve(strict=True)
        script = ROOT / 'tests/k6/load_test.js'
        meta.update(k6_version=command([str(k6), 'version']), k6_sha256=digest(k6), script_sha256=digest(script), omitted_ids_sha256=digest(script.parent / 'omitted_resp_ids.json'))
        if args.native_pid:
            meta.update(environment_mode='ALTERNATIVE_NATIVE_WINDOWS_WSL', protocol_limit='No Docker/cgroups quotas. CT-3/CT-4 original protocol NOT VERIFIED. CPU uses cumulative process time deltas; Windows WorkingSet64 and PostgreSQL per-process RSS are different measurements. PostgreSQL summed RSS double-counts shared pages; do not call it physical joint RAM.')
        else:
            if not args.container:
                raise ValueError('Specify laboratory containers or explicit --native-pid alternative')
            inspection = command(['docker', 'inspect'] + args.container, 20)
            if inspection['exit_code']:
                raise ValueError('Container inspection failed: ' + inspection['stderr'])
            write(out / 'containers.json', json.loads(inspection['stdout']))
        failed = False
        for replica in range(1, 4):
            folder = out / f'replica-{replica}'
            folder.mkdir()
            stop = threading.Event()
            sample_target = native_telemetry if args.native_pid else telemetry
            sample_identity = args.native_pid if args.native_pid else args.container
            sampler = threading.Thread(target=sample_target, args=(stop, sample_identity, folder / 'telemetry.jsonl'), daemon=True)
            cmd = [str(k6), 'run', '--env', 'BASE_URL=' + args.base_url, '--summary-export', str(folder / 'k6-summary.json'), '--out', 'json=' + str(folder / 'k6-raw.json.gz'), str(script)]
            execution = {'command': cmd, 'started_utc': stamp()}
            sampler.start()
            started = time.monotonic()
            try:
                with (folder / 'stdout.log').open('w', encoding='utf-8') as stdout, (folder / 'stderr.log').open('w', encoding='utf-8') as stderr:
                    proc = subprocess.run(cmd, stdout=stdout, stderr=stderr, timeout=510)
                execution.update(exit_code=proc.returncode, duration_seconds=time.monotonic()-started)
                failed |= proc.returncode != 0 or not (folder / 'k6-summary.json').exists() or not (folder / 'k6-raw.json.gz').exists()
            finally:
                stop.set()
                sampler.join(timeout=60)
            execution['finished_utc'] = stamp()
            write(folder / 'execution.json', execution)
            meta['replicas'].append(execution)
            write(out / 'execution.json', meta)
        meta['status'] = 'FAIL' if failed else 'EXECUTED_REQUIRES_AGGREGATION'
    except (OSError, ValueError, subprocess.SubprocessError) as error:
        meta['error'] = str(error)
    meta['finished_utc'] = stamp()
    write(out / 'execution.json', meta)
    print(json.dumps(meta))
    return 0 if meta['status'] == 'EXECUTED_REQUIRES_AGGREGATION' else 1


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest='action', required=True)
    ca = sub.add_parser('validate')
    ca.add_argument('--jar', default=str(ROOT / 'tools/experimental/bin/validator_cli.jar'))
    ca.add_argument('--input', required=True)
    ca.add_argument('--output', required=True)
    ca.add_argument('--expected-count', type=int, default=16250)
    ca.add_argument('--schema', choices=['s0', 's1', 's2', 's3'], help='Evaluated relational archetype (provide for manuscript evidence)')
    ca.add_argument('--stage-batches', action='store_true', help='Validate byte-identical short-name batch directories; retain original path/hash mapping')
    ca.add_argument('--batch-size', type=int, default=500)
    ca.add_argument('--timeout', type=int, default=1800)
    ca.add_argument('--java', default='java')
    ca.add_argument('--heap', default='768m', help='JVM heap ceiling; tune explicitly for the laboratory host')
    ca.add_argument('--tx', default='n/a')
    ca.set_defaults(run=validate)
    bench = sub.add_parser('ct')
    bench.add_argument('--k6', required=True)
    bench.add_argument('--base-url', required=True)
    bench.add_argument('--container', action='append')
    bench.add_argument('--native-pid', type=int, help='Explicit alternative native-host CT; does not satisfy cgroup CPU/RAM protocol')
    bench.add_argument('--output', required=True)
    bench.set_defaults(run=ct)
    args = parser.parse_args()
    if getattr(args, 'batch_size', 1) < 1 or getattr(args, 'expected_count', 1) < 1:
        parser.error('Counts must be positive')
    return args.run(args)


if __name__ == '__main__':
    sys.exit(main())
