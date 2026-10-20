#!/usr/bin/env python3
"""Offline adversarial tests: no GitHub calls, packaging or publication."""
import copy
import hashlib
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parent.parent
spec = importlib.util.spec_from_file_location('release', ROOT / 'scripts/sec1-release.py')
r = importlib.util.module_from_spec(spec); spec.loader.exec_module(r)
SHA = 'a' * 40
RESULTS = {name: {'result': 'success'} for name in r.MANDATORY}


def sums(directory):
    files = sorted(p for p in directory.iterdir() if p.name != 'SHA256SUMS.txt')
    (directory / 'SHA256SUMS.txt').write_text(''.join(hashlib.sha256(p.read_bytes()).hexdigest() + '  ' + p.name + '\n' for p in files))


class ReleaseTests(unittest.TestCase):
    def test_every_mandatory_result_and_exact_revision(self):
        self.assertTrue(r.qualified(RESULTS, SHA, SHA, '123')['software_qualified'])
        for name in r.MANDATORY:
            for status in ['failure', 'skipped', 'cancelled', 'neutral', None]:
                results = copy.deepcopy(RESULTS); results[name]['result'] = status
                with self.assertRaises(ValueError): r.qualified(results, SHA, SHA, '123')
            results = copy.deepcopy(RESULTS); del results[name]
            with self.assertRaises(ValueError): r.qualified(results, SHA, SHA, '123')
        with self.assertRaises(ValueError): r.qualified(RESULTS, SHA, 'b' * 40, '123')

    def test_approval_protection_is_not_an_environment_name(self):
        good = {'can_admins_bypass': False, 'protection_rules': [{'type': 'required_reviewers', 'prevent_self_review': True, 'reviewers': [{'type': 'User', 'reviewer': {'id': 1}}]}]}
        r.protection_verified(good)
        for bad in [{}, {'protection_rules': []}, {**good, 'can_admins_bypass': True}]:
            with self.assertRaises(ValueError): r.protection_verified(bad)
        for key, value in [('prevent_self_review', False), ('reviewers', [])]:
            bad = copy.deepcopy(good); bad['protection_rules'][0][key] = value
            with self.assertRaises(ValueError): r.protection_verified(bad)

    def test_run_selection_never_uses_latest_or_unrelated_success(self):
        run = {'id': 123, 'repository': {'full_name': 'harry-finch/FabCAM'}, 'head_sha': SHA, 'path': '.github/workflows/desktop-candidates.yml', 'event': 'workflow_dispatch', 'status': 'completed', 'conclusion': 'success'}
        r.verified_run(run, SHA, '123', 'harry-finch/FabCAM')
        for key, value in [('id', 124), ('head_sha', 'b' * 40), ('path', '.github/workflows/other.yml'), ('status', 'in_progress'), ('conclusion', 'skipped'), ('event', 'pull_request'), ('repository', {'full_name': 'other/FabCAM'})]:
            with self.assertRaises(ValueError): r.verified_run({**run, key: value}, SHA, '123', 'harry-finch/FabCAM')

    def fixture(self, root):
        q = root / 'software-qualification'; q.mkdir()
        (q / 'software-qualification.json').write_text(json.dumps(r.qualified(RESULTS, SHA, SHA, '123')))
        for platform, target in r.TARGETS.items():
            d = root / f'FabCAM-candidate-{platform}-{target}'; d.mkdir()
            name = 'FabCAM_0.9.0-beta.3_x64' + {'macos': '.dmg', 'windows': '-setup.exe', 'ubuntu': '.deb'}[platform]
            data = b'non-executable test artifact'; (d / name).write_bytes(data)
            m = {'format': 1, 'product': 'FabCAM', 'version': '0.9.0-beta.3', 'platform': platform, 'declared_build_target': target, 'provenance': {'commit': SHA, 'dirty': False, 'qualification_run_id': '123'}, 'artifact': {'file': name, 'bytes': len(data), 'sha256': hashlib.sha256(data).hexdigest()}}
            (d / 'candidate.json').write_text(json.dumps(m)); sums(d)
        return d

    def test_artifact_hash_manifest_and_receipt_are_all_required(self):
        mutations = [('version', 'wrong'), ('declared_build_target', 'wrong'), ('platform', 'wrong'), ('format', 2), ('product', 'other')]
        for key, value in mutations + [('provenance', {'commit': 'b' * 40, 'dirty': False, 'qualification_run_id': '123'}), ('provenance', {'commit': SHA, 'dirty': True, 'qualification_run_id': '123'}), ('provenance', {'commit': SHA, 'dirty': False, 'qualification_run_id': '124'})]:
            with tempfile.TemporaryDirectory() as tmp:
                root = Path(tmp); d = self.fixture(root)
                self.assertEqual(len(r.verify_artifacts(root, SHA, '123', '0.9.0-beta.3')), 3)
                p = d / 'candidate.json'; m = json.loads(p.read_text()); m[key] = value; p.write_text(json.dumps(m)); sums(d)
                with self.assertRaises(ValueError): r.verify_artifacts(root, SHA, '123', '0.9.0-beta.3')
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp); d = self.fixture(root)
            installer = next(d.glob('*.deb')); installer.write_bytes(b'changed')
            with self.assertRaises(ValueError): r.verify_artifacts(root, SHA, '123', '0.9.0-beta.3')
            sums(d)  # Even forged checksums do not override manifest identity.
            with self.assertRaises(ValueError): r.verify_artifacts(root, SHA, '123', '0.9.0-beta.3')
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp); d = self.fixture(root)
            p = root / 'software-qualification/software-qualification.json'; m = json.loads(p.read_text()); m['run_id'] = '124'; p.write_text(json.dumps(m))
            with self.assertRaises(ValueError): r.verify_artifacts(root, SHA, '123', '0.9.0-beta.3')

    def test_unsafe_and_unchecked_files(self):
        with tempfile.TemporaryDirectory() as tmp:
            d = Path(tmp); (d / 'SHA256SUMS.txt').write_text('a' * 64 + '  ../outside\n')
            with self.assertRaises(ValueError): r.checked_files(d)
            (d / 'SHA256SUMS.txt').write_text(''); (d / 'extra').write_bytes(b'extra')
            with self.assertRaises(ValueError): r.checked_files(d)

    def gate(self, fail_codegen='', missing=False):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp); (root / 'scripts').mkdir(); (root / 'frontend').mkdir(); binpath = root / 'bin'; binpath.mkdir()
            shutil.copyfile(ROOT / 'scripts/pre-release.sh', root / 'scripts/pre-release.sh')
            for tool in ['bash', 'dirname']:
                (binpath / tool).symlink_to(shutil.which(tool))
            for tool in ['cargo', 'pnpm', 'git', 'wasm-pack', 'cargo-deny', 'google-chrome', 'python3', 'node']:
                if missing and tool == 'cargo-deny': continue
                p = binpath / tool
                p.write_text('#!/bin/bash\n' + ('if [[ "$2" == "'+ fail_codegen +'" ]]; then exit 9; fi\n' if tool == 'pnpm' and fail_codegen else '') + 'exit 0\n'); p.chmod(0o755)
            for script in ['check-readme-parity.sh', 'wasm-size-guard.sh', 'ui-smoke.sh']:
                p = root / 'scripts' / script; p.write_text('#!/bin/bash\nexit 0\n'); p.chmod(0o755)
            return subprocess.run(['/bin/bash', str(root / 'scripts/pre-release.sh')], env={**os.environ, 'PATH': str(binpath)}, capture_output=True, text=True)

    def test_local_gate_propagates_codegen_and_missing_tools(self):
        self.assertEqual(self.gate().returncode, 0)
        for codegen in ['codegen', 'i18n:codegen']:
            self.assertNotEqual(self.gate(codegen).returncode, 0)
        self.assertNotEqual(self.gate(missing=True).returncode, 0)

    def test_portable_size_gate_and_invalid_budget(self):
        import sys
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp); (root / 'scripts').mkdir(); binpath = root / 'bin'; binpath.mkdir()
            script = root / 'scripts/wasm-size-guard.sh'
            shutil.copyfile(ROOT / 'scripts/wasm-size-guard.sh', script)
            stub = binpath / 'wasm-pack'
            stub.write_text('#!' + sys.executable + '\nimport sys\nfrom pathlib import Path\np=Path(sys.argv[sys.argv.index("--out-dir")+1].removeprefix("../../"))\np.mkdir(parents=True,exist_ok=True)\n(p/"test_bg.wasm").write_bytes(bytes(range(256)) if "cps" in sys.argv else b"A"*100)\n')
            stub.chmod(0o755)
            env = {**os.environ, 'PATH': str(binpath) + ':' + os.environ['PATH']}
            for budget, expected in [('1700', 0), ('0', 1), ('-1', 1), ('junk', 1), ('999999999999999999', 1)]:
                result = subprocess.run(['/bin/bash', str(script)], env={**env, 'WASM_CPS_BUDGET_KB': budget}, capture_output=True, text=True)
                self.assertEqual(result.returncode, expected, result.stdout + result.stderr)

    def test_no_publication_path_enabled(self):
        promotion = (ROOT / '.github/workflows/promote-desktop-draft.yml').read_text()
        self.assertIn('if: ${{ false }}', promotion)
        for path in (ROOT / '.github/workflows').glob('*.yml'):
            self.assertNotIn('softprops/action-gh-release', path.read_text())
            self.assertNotIn('gh release create', path.read_text())


if __name__ == '__main__':
    unittest.main()
