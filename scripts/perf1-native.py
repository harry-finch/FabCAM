#!/usr/bin/env python3
"""Run each fixture in its own process; retain raw timings and output hashes.

macOS time -l: RSS is bytes. First pipeline call is process-cold; second is
warmup; remaining calls are warm. Full responses stay in the temporary work dir.
"""
import argparse
import hashlib
import json
import pathlib
import re
import subprocess
import time
import os
import signal
from perf1_evidence import verify_request

p = argparse.ArgumentParser()
p.add_argument("--binary", type=pathlib.Path, required=True)
p.add_argument("--requests", type=pathlib.Path, required=True)
p.add_argument("--output", type=pathlib.Path, required=True)
p.add_argument("--work", type=pathlib.Path, required=True)
p.add_argument("--repeats", type=int, default=7)
p.add_argument("--case", help="Exact case-post name; omitted runs the whole manifest")
args = p.parse_args()
if args.repeats < 3:
    p.error("repeats must include cold, warmup and at least one warm sample")
args.output.mkdir(parents=True, exist_ok=True)
args.work.mkdir(parents=True, exist_ok=True)
manifest = json.loads((args.requests / "manifest.json").read_text())
for item in manifest:
    name = pathlib.Path(item["name"]).stem
    if args.case and name != args.case:
        continue
    request_hash = verify_request(args.requests / item["name"], item["sha256"])
    result = args.work / f"{name}.json"
    cmd = ["/usr/bin/time", "-l", str(args.binary.resolve()),
           str((args.requests / item["name"]).resolve()), str(result.resolve()), str(args.repeats)]
    launched_at = time.time()
    started = time.perf_counter()
    process = subprocess.Popen(cmd, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                               text=True, start_new_session=True)
    try:
        stdout, stderr = process.communicate(timeout=90)
    except subprocess.TimeoutExpired:
        os.killpg(process.pid, signal.SIGTERM)
        stdout, stderr = process.communicate()
        (args.output / f"{name}.timeout.json").write_text(json.dumps({
            "status": "unavailable", "timeout_s": 90, "command": cmd,
            "request_sha256": item["sha256"], "note": "Not a measured speedup or completed sample."}, indent=2) + "\n")
        raise
    (args.output / f"{name}.resource.txt").write_text(stderr)
    if process.returncode:
        raise RuntimeError(f"{name}: {stderr}")
    data = json.loads(result.read_text())
    data["process_wall_ms"] = (time.perf_counter() - started) * 1000
    resource_time = re.search(r"([\d.]+) real\s+([\d.]+) user\s+([\d.]+) sys", stderr)
    rss = re.search(r"(\d+)\s+maximum resident set size", stderr)
    data["process_resources"] = {
        "real_s": float(resource_time[1]) if resource_time else None,
        "user_s": float(resource_time[2]) if resource_time else None,
        "sys_s": float(resource_time[3]) if resource_time else None,
        "peak_rss_bytes": int(rss[1]) if rss else None,
    }
    response = pathlib.Path(str(result) + ".response.json")
    if data["runs"][0]["success"]:
        if not response.exists() or response.stat().st_mtime < launched_at:
            raise RuntimeError(f"{name}: missing or stale response artifact")
        payload = response.read_bytes()
        data["response_sha256"] = hashlib.sha256(payload).hexdigest()
        full = json.loads(payload)
        data["program_sha256"] = hashlib.sha256(full["gcode"].encode()).hexdigest()
    data["request_sha256"] = request_hash
    data["binary_sha256"] = hashlib.sha256(args.binary.read_bytes()).hexdigest()
    data["command"] = cmd
    (args.output / f"{name}.json").write_text(json.dumps(data, indent=2) + "\n")
    first = data["runs"][0]
    print(name, first["success"], round(first["pipeline_ms"], 2), "ms cold", flush=True)
