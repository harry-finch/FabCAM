import hashlib
import json
import pathlib
import tempfile
import unittest
from perf1_evidence import compare_responses, series_evidence, verify_request

class EvidenceTests(unittest.TestCase):
    def test_mixed_and_missing_samples_never_stable(self):
        ok = {"success": True, "identical_to_first": True}
        bad = {"success": False, "error": "refused"}
        for rows in [[], [ok,bad], [bad,ok], [ok,{"success":True}], [bad,{"success":False,"error":"other"}]]:
            self.assertFalse(series_evidence(rows)["series_consistent"])
        self.assertTrue(series_evidence([ok,ok])["warm_output_stable"])
        self.assertTrue(series_evidence([bad,bad])["refusals_stable"])
        self.assertIsNone(series_evidence([bad,bad])["warm_output_stable"])

    def test_hashes_missing_and_stale_artifacts(self):
        with tempfile.TemporaryDirectory() as folder:
            p = pathlib.Path(folder)/'response.json'
            self.assertEqual(compare_responses(p,p,{},{} )["comparison_status"], "not_verified")
            p.write_text(json.dumps({"gcode":"same", "toolpath":[]}))
            h = hashlib.sha256(p.read_bytes()).hexdigest()
            self.assertEqual(verify_request(p,h),h)
            with self.assertRaises(ValueError): verify_request(p,'stale')
            self.assertEqual(compare_responses(p,p,{},{} )["comparison_status"], "not_verified")
            record = {"response_sha256":h,"request_sha256":"request"}
            self.assertTrue(compare_responses(p,p,record,record)["full_structure_equal"])
            with self.assertRaises(ValueError): compare_responses(p,p,record,dict(record,response_sha256='stale'))
            with self.assertRaises(ValueError): compare_responses(p,p,record,dict(record,request_sha256='other'))

    def test_different_path_lengths_do_not_report_a_truncated_delta(self):
        with tempfile.TemporaryDirectory() as folder:
            a = pathlib.Path(folder)/'a.json'; b = pathlib.Path(folder)/'b.json'
            a.write_text(json.dumps({"gcode":"x","toolpath":[]}))
            b.write_text(json.dumps({"gcode":"x","toolpath":[{}]}))
            def record(p): return {"response_sha256":hashlib.sha256(p.read_bytes()).hexdigest(),"request_sha256":"same"}
            result = compare_responses(a,b,record(a),record(b))
            self.assertFalse(result["moves_equal"])
            self.assertIsNone(result["max_preview_coordinate_delta_mm"])

if __name__ == '__main__': unittest.main()
