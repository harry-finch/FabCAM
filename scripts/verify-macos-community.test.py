import importlib.util
from pathlib import Path
import unittest

spec = importlib.util.spec_from_file_location('verify', Path(__file__).with_name('verify-macos-community.py'))
verify = importlib.util.module_from_spec(spec)
spec.loader.exec_module(verify)

class IdentityTests(unittest.TestCase):
    def setUp(self):
        self.info = dict(CFBundleIdentifier='org.opcenter.ivacam', CFBundleName='FabCAM',
                         CFBundleDisplayName='FabCAM', CFBundleExecutable='ivac-desktop',
                         CFBundleShortVersionString='0.9.0-beta.1')

    def test_exact_identity_and_universal_order(self):
        verify.validate_identity(self.info, '0.9.0-beta.1', 'arm64', 'arm64')
        verify.validate_identity(self.info, '0.9.0-beta.1', 'x86_64 arm64', 'arm64,x86_64')

    def test_each_identity_mismatch_rejected(self):
        for key in self.info:
            with self.subTest(key=key), self.assertRaises(ValueError):
                verify.validate_identity({**self.info, key: 'wrong'}, '0.9.0-beta.1', 'arm64', 'arm64')

    def test_missing_architecture_rejected(self):
        with self.assertRaises(ValueError):
            verify.validate_identity(self.info, '0.9.0-beta.1', 'arm64', 'arm64,x86_64')

    def test_additional_architecture_rejected(self):
        with self.assertRaises(ValueError):
            verify.validate_identity(self.info, '0.9.0-beta.1', 'arm64 x86_64', 'arm64')

if __name__ == '__main__':
    unittest.main()
