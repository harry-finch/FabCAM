#!/usr/bin/env python3
"""Read-only DMG validation; no signing, credentials, quarantine edits or publication."""
import argparse
import hashlib
import json
import plistlib
import subprocess
import tempfile
from pathlib import Path


def run(*args):
    return subprocess.check_output(args, text=True).strip()


def validate_identity(info, version, architectures, expected_architectures):
    expected = {'CFBundleIdentifier': 'org.opcenter.ivacam', 'CFBundleName': 'FabCAM',
                'CFBundleDisplayName': 'FabCAM', 'CFBundleExecutable': 'ivac-desktop',
                'CFBundleShortVersionString': version}
    for key, value in expected.items():
        if info.get(key) != value:
            raise ValueError(f'Unexpected {key}: {info.get(key)!r}; expected {value!r}')
    if set(architectures.split()) != set(expected_architectures.split(',')):
        raise ValueError(f'Unexpected architectures: {architectures}')


def verify_bundle(app, version, arch):
    if app.is_symlink() or not app.is_dir():
        raise ValueError('Expected a real FabCAM.app bundle')
    subprocess.run(['codesign', '--verify', '--deep', '--strict', '--verbose=2', str(app)], check=True)
    info = plistlib.loads((app / 'Contents/Info.plist').read_bytes())
    binary = app / 'Contents/MacOS/ivac-desktop'
    architectures = run('lipo', '-archs', str(binary))
    validate_identity(info, version, architectures, arch)
    signature = subprocess.run(['codesign', '-dv', '--verbose=4', str(app)],
                               check=True, capture_output=True, text=True).stderr
    if 'Signature=adhoc' not in signature or 'Sealed Resources version=' not in signature:
        raise ValueError('Expected assembled-bundle ad-hoc signature with sealed resources')
    return {'identity': {k: info[k] for k in ('CFBundleIdentifier', 'CFBundleName',
            'CFBundleDisplayName', 'CFBundleExecutable', 'CFBundleShortVersionString')},
            'architectures': architectures.split(), 'signature': 'ad-hoc; strict verification passed',
            'binary_sha256': hashlib.sha256(binary.read_bytes()).hexdigest()}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('dmg', type=Path)
    parser.add_argument('--version', required=True)
    parser.add_argument('--arch', default='arm64')
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    if args.output.exists():
        raise ValueError('Evidence already exists; choose a new path')
    subprocess.run(['hdiutil', 'verify', str(args.dmg)], check=True)
    with tempfile.TemporaryDirectory(prefix='fabcam-dmg-') as temp:
        mount = Path(temp) / 'mounted'
        mount.mkdir()
        subprocess.run(['hdiutil', 'attach', '-readonly', '-nobrowse', '-mountpoint',
                        str(mount), str(args.dmg)], check=True)
        try:
            apps = list(mount.glob('*.app'))
            if [p.name for p in apps] != ['FabCAM.app']:
                raise ValueError('DMG must contain exactly FabCAM.app')
            link = mount / 'Applications'
            if not link.is_symlink() or str(link.readlink()) != '/Applications':
                raise ValueError('Missing Applications installation shortcut')
            evidence = verify_bundle(apps[0], args.version, args.arch)
        finally:
            subprocess.run(['hdiutil', 'detach', str(mount)], check=True)
    evidence.update({'artifact': args.dmg.name,
                     'sha256': hashlib.sha256(args.dmg.read_bytes()).hexdigest(),
                     'notarization': 'not notarized by Apple',
                     'clean_install_qualified': False, 'runtime_qualified': False})
    args.output.parent.mkdir(parents=True, exist_ok=True)
    with args.output.open('x') as handle:
        json.dump(evidence, handle, indent=2)
        handle.write('\n')


if __name__ == '__main__':
    main()
