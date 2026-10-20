#!/usr/bin/env python3
"""Compare `ivac import` output with expected.json (see generate.py).

Usage: check.py <ivac binary> <output.json>
Prints one PASS/FAIL line per case and writes the detailed results.
"""
import json
import math
import pathlib
import subprocess
import sys

HERE = pathlib.Path(__file__).resolve().parent
TOL = 1e-6  # mm: comparison tolerance for analytically computed coordinates.
CURVE_TOL = 0.0101  # mm: the importer's 0.01 mm chord budget for sampled curves.


def close(a, b, tol=TOL):
    return math.hypot(a[0] - b[0], a[1] - b[1]) <= tol


def pt(p):
    return [p["x"], p["y"]]


def match(chains, segs):
    """Consume actual segments in order against the expected primitives."""
    i = 0
    for prim in chains:
        if i >= len(segs):
            return f"missing output for {prim}"
        if "line" in prim:
            a, b = prim["line"]
            s = segs[i]
            if s["type"] != "LINE" or not close(pt(s["start"]), a) or not close(pt(s["end"]), b):
                return f"segment {i}: expected line {a}->{b}, got {s['type']} {pt(s['start'])}->{pt(s['end'])}"
            i += 1
        elif "arc" in prim:
            e = prim["arc"]
            r = math.hypot(e["start"][0] - e["center"][0], e["start"][1] - e["center"][1])
            if not close(pt(segs[i]["start"]), e["start"]):
                return f"segment {i}: arc start {pt(segs[i]['start'])} != {e['start']}"
            while True:
                s = segs[i]
                if s["type"] != "ARC" or s.get("center") is None or not close(pt(s["center"]), e["center"]):
                    return f"segment {i}: expected arc about {e['center']}, got {s['type']} {s.get('center')}"
                if abs(math.hypot(s["end"]["x"] - e["center"][0], s["end"]["y"] - e["center"][1]) - r) > TOL:
                    return f"segment {i}: endpoint off radius"
                if (s["bulge"] > 0) != e["ccw"]:
                    return f"segment {i}: arc direction {'ccw' if s['bulge'] > 0 else 'cw'} != expected"
                i += 1
                if close(pt(s["end"]), e["end"]):
                    break
                if i >= len(segs):
                    return "arc never reached its expected end"
        elif "circle" in prim:
            e = prim["circle"]
            for _ in range(2):
                s = segs[i]
                if s["type"] != "CIRCLE" or not close(pt(s["center"]), e["center"]):
                    return f"segment {i}: expected circle about {e['center']}, got {s['type']} {s.get('center')}"
                if abs(math.hypot(s["start"]["x"] - e["center"][0], s["start"]["y"] - e["center"][1]) - e["r"]) > TOL:
                    return f"segment {i}: circle radius mismatch"
                i += 1
        elif "curve" in prim:
            e = prim["curve"]
            c, u, v = e["center"], e["u"], e["v"]
            det = u[0] * v[1] - u[1] * v[0]
            if not close(pt(segs[i]["start"]), e["start"], CURVE_TOL):
                return f"segment {i}: curve start {pt(segs[i]['start'])} != {e['start']}"
            while True:
                s = segs[i]
                for p in (pt(s["start"]), pt(s["end"])):
                    dx, dy = p[0] - c[0], p[1] - c[1]
                    a = (dx * v[1] - dy * v[0]) / det
                    b = (u[0] * dy - u[1] * dx) / det
                    if abs(math.hypot(a, b) - 1) > 1e-6:
                        return f"segment {i}: point {p} not on the expected ellipse"
                i += 1
                if close(pt(s["end"]), e["end"], CURVE_TOL):
                    break
                if i >= len(segs):
                    return "curve never reached its expected end"
    if i != len(segs):
        return f"{len(segs) - i} unexpected extra segment(s)"
    return None


def main():
    binary, out = sys.argv[1], sys.argv[2]
    cases = json.loads((HERE / "expected.json").read_text())
    results = {}
    failures = 0
    for c in cases:
        r = subprocess.run([binary, "import", str(HERE / c["file"])], capture_output=True, text=True)
        res = {"exit": r.returncode}
        if r.returncode != 0:
            res["error"] = (r.stderr or r.stdout).strip()[:400]
            ok = "error" in c and c["error"].lower() in res["error"].lower()
            why = None if ok else f"import failed: {res['error'][:160]}"
        else:
            d = json.loads(r.stdout)
            segs = d["segments"]
            res.update(segments=len(segs), bbox=d["bbox"], warnings=d["warnings"], unit_scale=d["unit_scale"])
            if "error" in c:
                why = "expected refusal, import succeeded"
            elif "chains" in c:
                why = match(c["chains"], segs)
            else:
                width = d["bbox"]["max_x"] - d["bbox"]["min_x"]
                why = None if abs(width - c["width_mm"]) <= 1e-3 else f"width {width:.4f} mm != {c['width_mm']}"
                res["width_mm"] = width
            if why is None and "warning" in c and not any(c["warning"] in w.lower() for w in d["warnings"]):
                why = f"no '{c['warning']}' warning (warnings: {d['warnings']})"
        res["status"] = "PASS" if why is None else "FAIL"
        res["reason"] = why
        failures += why is not None
        results[c["name"]] = res
        print(f"{res['status']}  {c['name']:32} {why or ''}")
    pathlib.Path(out).write_text(json.dumps(results, indent=1) + "\n")
    print(f"{len(cases) - failures}/{len(cases)} pass")
    sys.exit(1 if failures else 0)


if __name__ == "__main__":
    main()
