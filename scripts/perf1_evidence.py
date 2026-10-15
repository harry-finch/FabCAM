"""Fail-closed helpers shared by PERF1 evidence consolidation and tests."""
import hashlib
import json


def series_evidence(runs):
    if not runs:
        return {"series_consistent": False, "warm_output_stable": False, "reason": "missing samples"}
    success = runs[0].get("success")
    if success not in (True, False) or any(r.get("success") != success for r in runs):
        return {"series_consistent": False, "warm_output_stable": False, "reason": "mixed or missing statuses"}
    if success:
        stable = all(r.get("identical_to_first") is True for r in runs)
        return {"series_consistent": stable, "warm_output_stable": stable}
    stable = all("error" in r and r["error"] == runs[0].get("error") for r in runs)
    return {"series_consistent": stable, "warm_output_stable": None, "refusals_stable": stable}


def verify_request(path, expected):
    actual = hashlib.sha256(path.read_bytes()).hexdigest()
    if actual != expected:
        raise ValueError(f"request hash mismatch: {path}")
    return actual


def compare_responses(native_path, wasm_path, native_record, wasm_record):
    if not native_path.exists() or not wasm_path.exists():
        return {"comparison_status": "not_verified", "reason": "response artifact missing"}
    for path, record in [(native_path, native_record), (wasm_path, wasm_record)]:
        expected = record.get("response_sha256")
        if not expected:
            return {"comparison_status": "not_verified", "reason": "response hash not recorded"}
        if hashlib.sha256(path.read_bytes()).hexdigest() != expected:
            raise ValueError(f"response hash mismatch: {path}")
    if native_record["request_sha256"] != wasm_record["request_sha256"]:
        raise ValueError("comparison request mismatch")
    a, b = json.loads(native_path.read_text()), json.loads(wasm_path.read_text())
    equal_count = len(a["toolpath"]) == len(b["toolpath"])
    delta = None
    if equal_count:
        delta = max((abs(x[p][axis] - y[p][axis])
                     for x, y in zip(a["toolpath"], b["toolpath"])
                     for p in ["from", "to"] for axis in ["x", "y", "z"]), default=0.)
    return {"comparison_status": "verified", "program_equal": a["gcode"] == b["gcode"],
            "full_structure_equal": a == b, "moves_equal": equal_count,
            "max_preview_coordinate_delta_mm": delta,
            "program_sha256": hashlib.sha256(a["gcode"].encode()).hexdigest()}
