#!/usr/bin/env python3
"""Consolidate recorded experiments; absent evidence is an error, never a pass.

Historical assigned values were removed. This command does not run experiments
or apply the thesis decision rule. --manifest must contain CA-1..CA-5 and CT;
each is a nonempty list of {path, sha256}, relative to its directory. Reports
must identify their kind and status (PASS/FAIL/BLOCKED/INCOMPLETE).
"""
import argparse
import hashlib
import json
from pathlib import Path
import sys

KINDS = {
    'CA-1': {'hl7_validator_execution', 'CA-1'},
    'CA-2': {'source_vs_rust', 'oracle_sensitivity_only'},
    'CA-3': {'postgres_http_faults'},
    'CA-4': {'binary_configuration_transition'},
    'CA-5': {'adaptation_contract_comparison'},
    'CT': {'k6_resource_measurement'},
}
REQUIRED_SCHEMAS = {'s0', 's1', 's2', 's3'}


def validate_pass_report(report, kind):
    """Reject incomplete summaries even when someone set status=PASS."""
    if report.get('status') != 'PASS':
        return
    if kind in {'hl7_validator_execution', 'CA-1'}:
        if not all(report.get(k) == 16250 for k in ('expected_count','observed_count','report_resource_count')):
            raise ValueError('CA-1 PASS requires a complete 16,250-resource validation report')
        if not report.get('batches'):
            raise ValueError('CA-1 PASS requires validator batch execution records')
        if any(batch.get('exit_code') != 0 for batch in report['batches']):
            raise ValueError('CA-1 PASS cannot contain failed or unobserved validator executions')
        if any(report.get('severity_counts',{}).get(level,0) for level in ('fatal','error')):
            raise ValueError('CA-1 PASS cannot contain Fatal/Error outcomes')
    elif kind == 'source_vs_rust':
        if (report.get('expected_resources') != 16250 or report.get('observed_resources') != 16250
                or not report.get('census_complete') or report.get('discrepancies') != []):
            raise ValueError('CA-2 PASS requires complete census and zero observed discrepancies')
    elif kind == 'oracle_sensitivity_only':
        details = report.get('details', [])
        if (report.get('detected') != 20 or report.get('total') != 20 or len(details) != 20
                or {d.get('id') for d in details} != {f'MUT-{i:02}' for i in range(1,21)}
                or not all(d.get('detected') is True for d in details)):
            raise ValueError('CA-2 sensitivity PASS requires all twenty original mutation results')
    elif kind == 'postgres_http_faults':
        cases = report.get('cases', 0)
        if (not cases or report.get('observed_http_injections') != cases or report.get('nominal_controls') != cases*2
                or report.get('all_restored') is not True or report.get('outcomes') != {'PASS':cases}):
            raise ValueError('CA-3 PASS requires observed HTTP, nominal controls and complete restoration')


def consolidate(manifest_path):
    root = manifest_path.resolve().parent
    manifest = json.loads(manifest_path.read_text(encoding='utf-8'))
    results = {}
    for criterion, required_kinds in KINDS.items():
        entries = manifest.get(criterion)
        if not isinstance(entries, list) or not entries:
            raise ValueError(f'{criterion}: missing evidence list')
        kinds, reports, schemas = set(), [], set()
        for entry in entries:
            path = (root / entry['path']).resolve()
            if not path.is_relative_to(root):
                raise ValueError(f'{criterion}: evidence path escapes run directory')
            raw = path.read_bytes()
            sha = hashlib.sha256(raw).hexdigest()
            if sha != entry['sha256']:
                raise ValueError(f'{criterion}: SHA-256 mismatch: {entry["path"]}')
            report = json.loads(raw)
            kind, status = report.get('kind'), report.get('status')
            if kind not in required_kinds or status not in {'PASS','FAIL','BLOCKED','INCOMPLETE'}:
                raise ValueError(f'{criterion}: unrecognized report kind/status: {kind}/{status}')
            validate_pass_report(report, kind)
            kinds.add(kind)
            if criterion != 'CT':
                if kind in {'hl7_validator_execution','CA-1','source_vs_rust','oracle_sensitivity_only','postgres_http_faults'}:
                    covered = [report.get('schema', entry.get('schema'))]
                else:
                    covered = report.get('schemas', [report.get('schema', entry.get('schema'))])
                if not isinstance(covered, list) or any(s not in REQUIRED_SCHEMAS for s in covered):
                    raise ValueError(f'{criterion}: missing or invalid S0-S3 scope: {entry["path"]}')
                schemas.update(covered)
            reports.append(dict(path=entry['path'], sha256=sha, kind=kind, status=status))
        normalized_kinds = {'hl7_validator_execution' if k == 'CA-1' else k for k in kinds}
        normalized_required = {'hl7_validator_execution' if k == 'CA-1' else k for k in required_kinds}
        if normalized_kinds != normalized_required:
            raise ValueError(f'{criterion}: missing evidence types {required_kinds - kinds}')
        statuses = [r['status'] for r in reports]
        status = ('FAIL' if 'FAIL' in statuses else 'BLOCKED' if 'BLOCKED' in statuses
                  else 'INCOMPLETE' if 'INCOMPLETE' in statuses else 'PASS')
        gaps = sorted(REQUIRED_SCHEMAS - schemas) if criterion != 'CT' else []
        if gaps and status == 'PASS':
            status = 'INCOMPLETE'
        results[criterion] = dict(status=status, reports=reports, missing_schemas=gaps,
                                 evidence_complete=not gaps and all(s in {'PASS','FAIL'} for s in statuses))
    return dict(kind='evidence_consolidation', criteria=results,
                binding_criteria_evidence_complete=all(results[c]['evidence_complete'] for c in KINDS if c != 'CT'),
                binding_criteria_all_passed=all(results[c]['status'] == 'PASS' for c in KINDS if c != 'CT'),
                characterization_evidence_complete=results['CT']['evidence_complete'],
                characterization_is_binding=False,
                thesis_verdict='Requires original manuscript decision rule; no inferred viability verdict')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--manifest', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    try:
        report = consolidate(args.manifest)
    except (ValueError, KeyError, OSError, TypeError) as error:
        print(f'INCOMPLETE EVIDENCE: {error}', file=sys.stderr)
        return 2
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(report, ensure_ascii=False, indent=2) + '\n', encoding='utf-8')
    print(json.dumps(report, ensure_ascii=False))
    return 0 if report['binding_criteria_all_passed'] and report['characterization_evidence_complete'] else 1


if __name__ == '__main__':
    sys.exit(main())
