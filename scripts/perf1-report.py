#!/usr/bin/env python3
"""Consolidate PERF1-A raw observations; do not turn refusals into speedups."""
import collections
import hashlib
import json
import pathlib
import statistics
import argparse
from perf1_evidence import series_evidence, compare_responses

parser = argparse.ArgumentParser()
parser.add_argument("--root", type=pathlib.Path, default=pathlib.Path(__file__).resolve().parents[1] / "docs/shopbot-modernization/perf1")
parser.add_argument("--responses", type=pathlib.Path, default=pathlib.Path("/private/tmp/fabcam-perf1"))
parser.add_argument("--expected-runs", type=int, default=7)
args = parser.parse_args()

root = args.root
def read(path):
    return json.loads(path.read_text())
def distribution(values):
    return {"n": len(values), "median": statistics.median(values), "min": min(values), "max": max(values)} if values else None
cases = []
for path in sorted((root / "native-control").glob("*.json")):
    control = read(path)
    profiled = read(root / "native-profile" / path.name)
    if any(len(record["runs"]) != args.expected_runs for record in [control, profiled]):
        raise ValueError(f"missing or unexpected samples: {path}")
    if not series_evidence(control["runs"] + profiled["runs"])["series_consistent"]:
        raise ValueError(f"inconsistent outcomes: {path}")
    first = control["runs"][0]
    classification = "refused" if not first["success"] else "empty-source-diagnostic" if "op_source_empty" in first["warnings"] else "accepted"
    stages = collections.defaultdict(list)
    for run in profiled["runs"][2:]:
        totals = collections.defaultdict(float)
        for record in run["spans"]:
            if record.get("count") is None:
                totals[record["name"]] += record["elapsed_ns"] / 1e6
        for name, value in totals.items():
            stages[name].append(value)
    counts = collections.defaultdict(int)
    for record in profiled["runs"][0]["spans"]:
        if record.get("count") is not None:
            counts[record["name"]] += record["count"]
    cases.append({
        "name": path.stem, "classification": classification,
        "cold_ms": first["pipeline_ms"],
        "warm_ms": distribution([x["pipeline_ms"] for x in control["runs"][2:]]),
        "profiled_warm_ms": distribution([x["pipeline_ms"] for x in profiled["runs"][2:]]),
        "profiled_inclusive_stage_median_ms": {name: statistics.median(values) for name, values in stages.items()},
        "cold_workload_counts": dict(counts), "moves": first.get("moves"),
        "program_bytes": first.get("program_bytes"), "response_bytes": first.get("response_bytes"),
        "resources": control["process_resources"],
        **series_evidence(control["runs"] + profiled["runs"]),
        "instrumentation_response_equal": bool(control.get("response_sha256")) and control.get("response_sha256") == profiled.get("response_sha256") if first["success"] else None,
        "refusal_equal": first.get("error") == profiled["runs"][0].get("error") if not first["success"] else None,
        "error": first.get("error"),
        "warm_op_cache_hit_observed": any("cached" in p["message"] for run in control["runs"][2:] for p in run["progress"]),
    })

wasm = []
for path in sorted((root / "wasm").glob("*-linuxcnc.json")):
    data = read(path)
    item = {"name": path.stem, "init_ms": data["init_ms"], "cold_binding_ms": data["runs"][0]["generate_ms"], "warm_binding_ms": distribution([x["generate_ms"] for x in data["runs"][2:]]), "success": data["runs"][0]["success"]}
    w = args.responses / "wasm" / (path.name + ".response.json")
    n = args.responses / "control" / (path.name + ".response.json")
    item.update(compare_responses(n, w, read(root / "native-control" / path.name), data))
    wasm.append(item)

events = [json.loads(row["message"][row["message"].index("{"):]) for row in read(root / "browser-http.json")]
groups = collections.defaultdict(dict)
for event in events:
    groups[event["id"]][event["stage"]] = event["elapsed_ms"]
quiet = [2, 3, 7, 8, 9]
browser = {"viewport": [1366, 768], "cold_id": 1, "quiet_warm_ids": quiet, "excluded_exploratory_ids": [4, 5, 6], "reason": "4–6 were burst runs overlapping standalone WASM measurement; not quiet baseline", "all_runs": dict(groups), "warm_ms": {stage: distribution([groups[i][stage] for i in quiet if stage in groups[i]]) for stage in ["context_ready", "request_sent", "response_received", "state_applied", "wire_buffers_ready", "first_3d_frame", "simulation_replayed"]}}
cancel = {str(delay): [read(p) for p in sorted((root / "cancellation").glob(f"native-cps-{delay}ms-*.json"))] for delay in [1, 50]}
summary = {"native": cases, "wasm_v8_binding": wasm, "browser_http": browser, "cancellation": {key: {"latency_ms": distribution([r["latency_ms"] for r in rows]), "all_cancelled": all(r["cancelled"] for r in rows), "any_valid_partial_result": any(r["returned_valid_result"] for r in rows)} for key, rows in cancel.items()}, "imports": {ext: read(root / "imports" / (ext + ".json")) for ext in ["svg", "dxf"]}}
(root / "SUMMARY.json").write_text(json.dumps(summary, indent=2) + "\n")
lines = ["# PERF1-A raw-data summary", "", "Native control release; cold process first call, warm n=5 after one warmup. Timings in ms.", "", "| Case/post | Classification | Cold | Warm median [min, max] | Moves | Peak RSS MiB |", "| --- | --- | ---: | ---: | ---: | ---: |"]
for item in cases:
    w = item["warm_ms"]
    lines.append(f"| {item['name']} | {item['classification']} | {item['cold_ms']:.2f} | {w['median']:.2f} [{w['min']:.2f}, {w['max']:.2f}] | {item['moves']} | {item['resources']['peak_rss_bytes']/1048576:.1f} |")
lines += ["", "Nested/inclusive native stages, exact counts, WASM binding boundaries and browser endpoint distributions are in SUMMARY.json. Do not sum child spans into parent elapsed time."]
(root / "SUMMARY.md").write_text("\n".join(lines) + "\n")
print("Consolidated", len(cases), "native cases,", len(wasm), "WASM cases,", len(groups), "browser generations.")
