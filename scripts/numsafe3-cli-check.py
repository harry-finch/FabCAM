#!/usr/bin/env python3
"""Real CLI export regressions. First build: cargo build -p ivac-cli.

All destinations live in a private temporary directory. No machine execution.
"""
import copy
import json
from pathlib import Path
import subprocess
import tempfile

repo = Path(__file__).resolve().parents[1]
binary = repo / "target/debug/ivac"
request = json.loads((repo / "tests/fixtures/test1/profile.request.json").read_text())
request["post_processor"] = "linuxcnc"
request["cps_post"] = None

with tempfile.TemporaryDirectory(prefix="fabcam-numsafe3-export-") as directory:
    root = Path(directory)

    def save(name, value):
        path = root / name
        path.write_text(json.dumps(value))
        return path

    def run(path, *options):
        return subprocess.run(
            [str(binary), "stream-gcode", str(path), *map(str, options)],
            capture_output=True, timeout=30, check=False,
        )

    valid = save("valid.json", request)
    target = root / "job.ngc"
    target.write_bytes(b"previous")
    bad = copy.deepcopy(request)
    operation = copy.deepcopy(bad["project"]["operations"][0])
    operation["id"] = 2
    operation["params"]["step"] = -1e-8
    bad["project"]["operations"].append(operation)
    failed = run(save("bad.json", bad), "--output", target)
    assert failed.returncode and b"1000" in failed.stderr, failed.stderr
    assert target.read_bytes() == b"previous"
    assert not list(root.glob(".fabcam-export-*"))

    streamed = run(valid)
    published = run(valid, "--output", target)
    assert streamed.returncode == published.returncode == 0
    assert target.read_bytes() == streamed.stdout

    pair = copy.deepcopy(request)
    pair["project"]["stock"]["flip"] = {"axis": "x", "dowels": None}
    operation = copy.deepcopy(pair["project"]["operations"][0])
    operation.update(id=2, side="back")
    pair["project"]["operations"].append(operation)
    pair_request = save("pair.json", pair)
    front = root / "pair.front.ngc"
    back = root / "pair.back.ngc"
    front.write_bytes(b"old-front")
    back.write_bytes(b"old-back")
    refused = run(pair_request, "--two-sided", "--output", root / "pair.ngc")
    assert refused.returncode and b"new output names" in refused.stderr
    assert front.read_bytes() == b"old-front" and back.read_bytes() == b"old-back"
    fresh = run(pair_request, "--two-sided", "--output", root / "fresh.ngc")
    assert fresh.returncode == 0, fresh.stderr
    assert (root / "fresh.front.ngc").stat().st_size
    assert (root / "fresh.back.ngc").stat().st_size

    pair["project"]["operations"][1]["params"]["step"] = -1e-8
    failed_pair = run(save("bad-pair.json", pair), "--two-sided", "--output", root / "failed.ngc")
    assert failed_pair.returncode and b"1000" in failed_pair.stderr
    assert not (root / "failed.front.ngc").exists()
    assert not (root / "failed.back.ngc").exists()
    assert not list(root.glob(".fabcam-export-*"))

print("PASS: target preservation, cleanup, byte identity, new-only pairs, both-side staging.")
