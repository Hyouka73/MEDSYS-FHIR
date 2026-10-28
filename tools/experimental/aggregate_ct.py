"""Aggregate real k6 points and aligned Docker samples; do not add percentiles."""
import argparse
from collections import Counter, defaultdict
from datetime import datetime
import gzip
import json
import math
from pathlib import Path
import re
import sys


def percentile(values, quantile=.95):
    values = sorted(values)
    if not values:
        return None
    location = (len(values)-1)*quantile
    lo = math.floor(location)
    hi = math.ceil(location)
    return values[lo] + (values[hi]-values[lo])*(location-lo)


def stats(values):
    return {'count': len(values), 'mean': sum(values)/len(values) if values else None,
            'p95': percentile(values), 'max': max(values) if values else None}


def memory_bytes(text):
    match = re.fullmatch(r'([\d.]+)\s*(B|KiB|MiB|GiB|kB|MB|GB)', text.strip())
    if not match:
        raise ValueError('Unrecognized Docker memory: ' + text)
    factors = {'B':1,'KiB':1024,'MiB':1024**2,'GiB':1024**3,'kB':1000,'MB':1000**2,'GB':1000**3}
    return float(match[1])*factors[match[2]]


def aggregate(folder):
    execution = json.loads((folder / 'execution.json').read_text(encoding='utf-8'))
    start = datetime.fromisoformat(execution['started_utc'])
    latency = defaultdict(list)
    http = Counter()
    failure = Counter()
    with gzip.open(folder / 'k6-raw.json.gz', 'rt', encoding='utf-8') as stream:
        for line in stream:
            point = json.loads(line)
            if point.get('type') != 'Point':
                continue
            data = point['data']
            tags = data.get('tags', {})
            phase = tags.get('phase', 'unspecified')
            if point.get('metric') == 'http_req_duration':
                latency[phase].append(data['value'])
                # Preserve each concrete route and status for attribution, not just endpoint family.
                http[(phase, tags.get('status', 'missing'), tags.get('url', tags.get('name', 'missing')))] += 1
            if point.get('metric') == 'http_req_failed':
                failure[(phase, str(data['value']))] += 1
    series = defaultdict(lambda: defaultdict(list))
    bad_samples = []
    times = []
    native_samples = []
    for line in (folder / 'telemetry.jsonl').read_text(encoding='utf-8').splitlines():
        sample = json.loads(line)
        seconds = (datetime.fromisoformat(sample['started_utc']) - start).total_seconds()
        phase = 'warmup' if seconds < 90 else ('plateau' if seconds < 390 else 'cooldown')
        if sample.get('mode'):
            native_samples.append((seconds, phase, sample))
            continue
        if sample.get('error') or sample.get('docker_stats', {}).get('exit_code') != 0:
            bad_samples.append(sample)
            continue
        rows = [json.loads(x) for x in sample['docker_stats']['stdout'].splitlines() if x.strip()]
        if len(rows) != len(sample['containers']):
            bad_samples.append(sample)
            continue
        joint = {'cpu_percent':0, 'docker_memory_bytes':0, 'vmrss_bytes':0, 'cgroup_memory_bytes':0}
        okay = True
        for row in rows:
            name = row.get('Name', '')
            values = {'cpu_percent':float(row['CPUPerc'].rstrip('%')), 'docker_memory_bytes':memory_bytes(row['MemUsage'].split('/')[0])}
            raw = sample['containers'].get(name)
            if raw is None or raw['exit_code'] != 0:
                okay = False
                break
            lines = raw['stdout'].splitlines()
            rss = next((x for x in lines if x.startswith('VmRSS:')), None)
            memory = next((x for x in lines if re.fullmatch(r'\d+', x.strip())), None)
            if rss is None or memory is None:
                okay = False
                break
            values.update(vmrss_bytes=int(rss.split()[1])*1024, cgroup_memory_bytes=int(memory))
            for metric, value in values.items():
                series[(phase, name)][metric].append(value)
                joint[metric] += value
        if okay:
            times.append(seconds)
            for metric, value in joint.items():
                series[(phase, 'joint')][metric].append(value)
        else:
            bad_samples.append(sample)
    if not latency.get('plateau'):
        raise ValueError('No plateau k6 HTTP points')
    if not native_samples and not series.get(('plateau', 'joint')):
        raise ValueError('No aligned joint plateau telemetry')
    native_series = defaultdict(lambda: defaultdict(list))
    previous = None
    for seconds, phase, sample in native_samples:
        if sample.get('error') or sample['rust_windows']['exit_code'] != 0 or sample['postgres_wsl']['exit_code'] != 0:
            bad_samples.append(sample)
            continue
        rust = json.loads(sample['rust_windows']['stdout'])
        postgres = {}
        for row in sample['postgres_wsl']['stdout'].splitlines():
            columns = row.split()
            if len(columns) >= 5:
                postgres[int(columns[0])] = {'cpu_seconds':float(columns[2]), 'rss_bytes':int(columns[3])*1024}
        native_series[(phase,'Rust_Windows')]['working_set_bytes'].append(rust['WorkingSet64'])
        native_series[(phase,'Rust_Windows')]['private_bytes'].append(rust['PrivateMemorySize64'])
        native_series[(phase,'PostgreSQL_WSL')]['summed_process_rss_bytes_shared_pages_duplicated'].append(sum(p['rss_bytes'] for p in postgres.values()))
        if previous:
            before_seconds, before_rust, before_postgres = previous
            dt = seconds-before_seconds
            if dt > 0:
                cpu = (rust['CPU']-before_rust['CPU'])/dt*100
                native_series[(phase,'Rust_Windows')]['cpu_percent_one_core'].append(cpu)
                shared = set(postgres) & set(before_postgres)
                pg_cpu = sum(max(0, postgres[p]['cpu_seconds']-before_postgres[p]['cpu_seconds']) for p in shared)/dt*100
                native_series[(phase,'PostgreSQL_WSL')]['cpu_percent_existing_processes_one_core'].append(pg_cpu)
                native_series[(phase,'joint')]['cpu_percent_existing_processes_one_core'].append(cpu+pg_cpu)
        previous = (seconds, rust, postgres)
    if native_samples and not native_series.get(('plateau','Rust_Windows')):
        raise ValueError('No valid native plateau telemetry')
    summary = {'execution_exit_code':execution['exit_code'], 'duration_seconds':execution['duration_seconds'], 'latency_ms':{phase:stats(points) for phase,points in latency.items()},
               'http_by_phase_status_route':[{'phase':p,'status':s,'url':url,'count':count} for (p,s,url),count in sorted(http.items())],
               'http_req_failed_callback_points':[{'phase':p,'value':v,'count':n} for (p,v),n in sorted(failure.items())],
               'telemetry':[{'phase':phase,'container':name, **{metric:stats(values) for metric,values in metrics.items()}} for (phase,name),metrics in sorted(series.items())],
               'telemetry_bad_sample_count':len(bad_samples), 'sample_elapsed_seconds':times,
               'native_telemetry':[{'phase':phase,'process_group':name, **{metric:stats(values) for metric,values in metrics.items()}} for (phase,name),metrics in sorted(native_series.items())],
               'native_protocol_limit':'Alternative Windows/WSL has no cgroup quotas; original CT3/4 NOT VERIFIED. CPU one-core denominator = delta CPU seconds / wall seconds *100. PostgreSQL CPU counts processes present in adjacent samples, omitting exited/new short-lived processes. Summed PostgreSQL RSS duplicates shared pages; not physical combined memory.',
               'percentile_method':'linear interpolation at (n-1)*0.95; joint values summed within each aligned sample before percentile',
               'limitations':'VmRSS reads PID1 only (PostgreSQL other processes not included); docker stats memory and cgroup memory.current are separate definitions. k6 latency tags identify exact phases; telemetry phases use process launch UTC. Samples have sequential container read skew in raw timestamps.'}
    (folder / 'aggregated.json').write_text(json.dumps(summary,ensure_ascii=False,indent=2),encoding='utf-8')
    return summary


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument('run_directory')
    args=parser.parse_args()
    root=Path(args.run_directory)
    try:
        outputs=[aggregate(root / f'replica-{n}') for n in range(1,4)]
        native = any(r.get('native_telemetry') for r in outputs)
        invalid = any(r['execution_exit_code'] != 0 for r in outputs)
        missing = any(r['telemetry_bad_sample_count'] for r in outputs)
        summary={'kind':'k6_resource_measurement', 'replicas':outputs,
                 'status':'FAIL' if invalid else ('INCOMPLETE' if native or missing else 'PASS'),
                 'protocol_original_complete':not (native or missing or invalid),
                 'environment_mode':'ALTERNATIVE_NATIVE_WINDOWS_WSL' if native else 'DOCKER_CGROUP',
                 'pooled_percentile': 'Not computed; each replica retained separately; no summed percentiles'}
        (root / 'aggregated.json').write_text(json.dumps(summary,ensure_ascii=False,indent=2),encoding='utf-8')
    except (OSError,ValueError,KeyError) as error:
        print('Missing or invalid CT evidence: '+str(error),file=sys.stderr)
        return 1
    return 0 if summary['status']=='PASS' else 1


if __name__=='__main__':
    sys.exit(main())
