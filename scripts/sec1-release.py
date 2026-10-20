#!/usr/bin/env python3
"""Fail-closed software receipt and explicit-run candidate verification. No publication."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess

MANDATORY = {'rust', 'wasm', 'frontend', 'docs', 'cargo-deny', 'test1', 'strict-software'}
TARGETS = {'macos': 'aarch64-apple-darwin', 'windows': 'x86_64-pc-windows-msvc', 'ubuntu': 'x86_64-unknown-linux-gnu'}
WORKFLOWS = {'.github/workflows/desktop-candidates.yml', '.github/workflows/release-desktop.yml'}


def require(condition, message):
    if not condition:
        raise ValueError(message)


def qualified(results, sha, actual_sha, run_id):
    require(re.fullmatch(r'[0-9a-f]{40}', sha or ''), 'invalid source SHA')
    require(sha == actual_sha, 'checkout SHA differs from workflow SHA')
    require(set(results) == MANDATORY, 'mandatory job inventory differs')
    require(all(v.get('result') == 'success' for v in results.values()), 'mandatory gate failed, skipped or cancelled')
    require(str(run_id).isdigit(), 'invalid qualification run ID')
    return {'format': 1, 'source_sha': sha, 'run_id': str(run_id), 'results': results,
            'software_qualified': True, 'installed_platform_acceptance': False,
            'physical_qualification': False, 'publication_approval': False}


def protection_verified(environment):
    rules = environment.get('protection_rules', [])
    require(any(r.get('type') == 'required_reviewers' and r.get('prevent_self_review') is True
                and r.get('reviewers') for r in rules), 'required human reviewers with self-review prevention are missing')
    require(environment.get('can_admins_bypass') is False, 'environment administrator bypass must be disabled')


def verified_run(run, sha, run_id, repository):
    require(str(run.get('id')) == str(run_id), 'wrong run ID')
    require(run.get('repository', {}).get('full_name') == repository, 'wrong repository')
    require(run.get('head_sha') == sha, 'wrong run SHA')
    require(run.get('path') in WORKFLOWS, 'unapproved build workflow')
    require(run.get('event') in {'push', 'workflow_dispatch'}, 'unapproved build event')
    require(run.get('status') == 'completed' and run.get('conclusion') == 'success', 'build run is not successful')


def checked_files(directory):
    require(directory.is_dir() and not directory.is_symlink(), 'invalid artifact directory')
    entries = list(directory.iterdir())
    require(all(p.is_file() and not p.is_symlink() for p in entries), 'artifact contains nonregular entries')
    sums = directory / 'SHA256SUMS.txt'
    recorded = {}
    for line in sums.read_text().splitlines():
        match = re.fullmatch(r'([0-9a-f]{64})  ([^/\\]+)', line)
        require(match is not None, 'invalid checksum entry')
        digest, name = match.groups()
        require(name not in {'.', '..', 'SHA256SUMS.txt'} and name not in recorded, 'invalid or duplicate checksum path')
        recorded[name] = digest
    require(set(recorded) == {p.name for p in entries} - {'SHA256SUMS.txt'}, 'unchecked or missing artifact file')
    for name, digest in recorded.items():
        require(hashlib.sha256((directory / name).read_bytes()).hexdigest() == digest, 'artifact hash mismatch: ' + name)
    return recorded


def verify_artifacts(root, sha, run_id, version):
    receipt_path = root / 'software-qualification' / 'software-qualification.json'
    require(receipt_path.is_file() and not receipt_path.is_symlink(), 'missing qualification receipt')
    receipt = json.loads(receipt_path.read_text())
    require(receipt.get('software_qualified') is True, 'receipt does not qualify software')
    qualified(receipt['results'], receipt['source_sha'], sha, receipt['run_id'])
    require(str(receipt['run_id']) == str(run_id), 'receipt belongs to another run')
    installers = []
    for platform, target in TARGETS.items():
        directory = root / f'FabCAM-candidate-{platform}-{target}'
        checked_files(directory)
        manifest = json.loads((directory / 'candidate.json').read_text())
        require(manifest.get('format') == 1 and manifest.get('product') == 'FabCAM', 'invalid candidate format/product')
        require(manifest.get('version') == version, 'wrong candidate version')
        require(manifest.get('platform') == platform and manifest.get('declared_build_target') == target, 'wrong candidate target')
        provenance = manifest.get('provenance', {})
        require(provenance.get('commit') == sha and provenance.get('dirty') is False, 'candidate source is not exact clean SHA')
        require(str(provenance.get('qualification_run_id')) == str(run_id), 'candidate qualification run differs')
        artifact = manifest['artifact']
        name = artifact['file']
        require(isinstance(name, str) and Path(name).name == name and '\\' not in name, 'unsafe installer path')
        suffix = {'macos': '.dmg', 'windows': '-setup.exe', 'ubuntu': '.deb'}[platform]
        require(name.endswith(suffix) and f'_{version}_' in name, 'unexpected installer filename')
        installer = directory / name
        data = installer.read_bytes()
        require(len(data) > 0 and len(data) == artifact['bytes'] and hashlib.sha256(data).hexdigest() == artifact['sha256'], 'manifest installer identity mismatch')
        installers.append(installer)
    return installers


def gh_json(path):
    return json.loads(subprocess.check_output(['gh', 'api', path], text=True))


def fetch_and_verify(args):
    require(re.fullmatch(r'[0-9a-f]{40}', args.sha), 'invalid SHA')
    require(args.run.isdigit(), 'invalid run ID')
    repo = os.environ['GITHUB_REPOSITORY']
    require(repo == 'harry-finch/FabCAM', 'promotion is restricted to the authoritative repository')
    protection_verified(gh_json(f'repos/{repo}/environments/release-promotion'))
    run = gh_json(f'repos/{repo}/actions/runs/{args.run}')
    verified_run(run, args.sha, args.run, repo)
    require(subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip() == args.sha, 'promotion checkout differs')
    version = json.loads(Path('crates/ivac-tauri/tauri.conf.json').read_text())['version']
    listing = gh_json(f'repos/{repo}/actions/runs/{args.run}/artifacts?per_page=100')
    require(listing['total_count'] <= 100, 'ambiguous paginated artifact inventory')
    wanted = ['software-qualification'] + [f'FabCAM-candidate-{p}-{t}' for p, t in TARGETS.items()]
    root = Path(args.directory)
    require(not root.exists(), 'download destination already exists')
    for name in wanted:
        matches = [a for a in listing['artifacts'] if a['name'] == name]
        require(len(matches) == 1 and not matches[0]['expired'], 'missing, ambiguous or expired artifact: ' + name)
        subprocess.run(['gh', 'run', 'download', args.run, '--repo', repo, '--name', name, '--dir', str(root / name)], check=True)
    installers = verify_artifacts(root, args.sha, args.run, version)
    # Assemble verified bytes only; never rebuild, infer latest run, or create a tag.
    out = Path('verified-promotion')
    require(not out.exists(), 'promotion output already exists')
    out.mkdir()
    for installer in installers:
        require(not (out / installer.name).exists(), 'installer filename collision')
        shutil.copyfile(installer, out / installer.name)
        shutil.copyfile(installer.parent / 'candidate.json', out / (installer.parent.name + '.json'))
    shutil.copyfile(root / 'software-qualification/software-qualification.json', out / 'software-qualification.json')
    (out / 'SHA256SUMS.txt').write_text(''.join(hashlib.sha256(p.read_bytes()).hexdigest() + '  ' + p.name + '\n' for p in sorted(out.iterdir())))


def main():
    parser = argparse.ArgumentParser()
    sub = parser.add_subparsers(dest='command', required=True)
    q = sub.add_parser('qualify'); q.add_argument('--output', required=True)
    f = sub.add_parser('fetch'); f.add_argument('--run', required=True); f.add_argument('--sha', required=True); f.add_argument('--directory', required=True)
    args = parser.parse_args()
    if args.command == 'qualify':
        head = subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip()
        require(not subprocess.check_output(['git', 'status', '--porcelain', '--untracked-files=no'], text=True).strip(), 'qualification checkout is dirty')
        receipt = qualified(json.loads(os.environ['QUALIFICATION_RESULTS']), os.environ['GITHUB_SHA'], head, os.environ['GITHUB_RUN_ID'])
        Path(args.output).write_text(json.dumps(receipt, indent=2) + '\n')
    else:
        fetch_and_verify(args)


if __name__ == '__main__':
    main()
