#!/usr/bin/env python3
"""ADR-0025 follow-up reproduction matrix (2026-10-19).

Replays one TEST1 request (base.request.json) through a running ivac-server
(`POST /generate`) under controlled variations and records, per case, the
provenance authority, diagnostics, binding count and the emitted tool-related
lines. Machine commands are summarized separately from comments so before/after
runs can be compared semantically.

Usage: repro_matrix.py <port> <output.json>
"""
import copy
import json
import pathlib
import sys
import urllib.request

HERE = pathlib.Path(__file__).resolve().parent
BASE = json.loads((HERE / "base.request.json").read_text())
INCH = 25.4


def request(post, cps=None):
    d = copy.deepcopy(BASE)
    d["post_processor"] = post
    d["cps_post"] = cps
    return d


SHOPBOT = {"source": {"kind": "bundled", "id": "shopbot"}, "properties": {}}


def two_tools(d, name="Fraise 1mm", dia=1.0, strategy="manual_m0_pause"):
    p = d["project"]
    t2 = copy.deepcopy(p["tools"][0])
    t2.update({"id": 2, "name": name, "diameter": dia})
    p["tools"].append(t2)
    p["operations"][0]["finish_tool_id"] = 2
    p["machine"]["tool_change"] = strategy
    return d


def program_only(d, kind):
    op = copy.deepcopy(d["project"]["operations"][0])
    op.update({"id": 2, "name": "program-only", "kind": kind})
    d["project"]["operations"].append(op)
    return d


def cases():
    out = []
    # A. ShopBot single-tool diameter matrix (mm and imperial fractions).
    for label, dia in [
        ("6mm", 6.0), ("3.175mm", 3.175), ("0.8mm", 0.8),
        ("1/64in", INCH / 64), ("1/32in", INCH / 32), ("3/64in", 3 * INCH / 64),
        ("1/16in", INCH / 16), ("3/32in", 3 * INCH / 32), ("1/8in", INCH / 8),
        ("5/32in", 5 * INCH / 32), ("3/16in", 3 * INCH / 16), ("7/32in", 7 * INCH / 32),
        ("1/4in", INCH / 4), ("9/32in", 9 * INCH / 32), ("1/2in", INCH / 2),
    ]:
        d = request("cps", SHOPBOT)
        d["project"]["tools"][0]["diameter"] = dia
        out.append((f"A.shopbot.single.{label}", d))
    # A. ShopBot rough 6 mm + finish 3/32 in (multi-section).
    out.append(("A.shopbot.rough6_finish3_32", two_tools(request("cps", SHOPBOT), dia=3 * INCH / 32)))
    # B. Native manual M0 tool names (LinuxCNC + GRBL) and ATC.
    names = [
        "Fraise 1mm", "Fraise filetage M6", "Taraud M06", "T2 special", "outil t3",
        "Fraise (6mm)", "Fraise ø3 ünïcødé", "x) T5 (y", "M6", "a;b M6",
    ]
    for post in ("linuxcnc", "grbl"):
        for name in names:
            out.append((f"B.{post}.m0.{name}", two_tools(request(post), name=name)))
        for name in ("Fraise 1mm", "Fraise filetage M6"):
            out.append((f"B.{post}.atc.{name}", two_tools(request(post), name=name, strategy="atc")))
    # Ignore must stay refused for multi-tool.
    out.append(("B.linuxcnc.ignore.multi", two_tools(request("linuxcnc"), strategy="ignore")))
    # C. Single-tool programs with optional configurations.
    out.append(("C.linuxcnc.single", request("linuxcnc")))
    out.append(("C.linuxcnc.single.homing", program_only(request("linuxcnc"), {"type": "homing", "retract_to_safe_z": True})))
    out.append(("C.linuxcnc.single.homing_noretract", program_only(request("linuxcnc"), {"type": "homing", "retract_to_safe_z": False})))
    d = request("linuxcnc")
    d["project"]["machine"]["post_profile"] = {}
    out.append(("C.linuxcnc.single.empty_profile", d))
    d = request("linuxcnc")
    d["project"]["machine"]["comments"] = False
    out.append(("C.linuxcnc.single.comments_false", d))
    d = two_tools(request("linuxcnc"))
    d["project"]["machine"]["comments"] = False
    out.append(("C.linuxcnc.multi.comments_false", d))
    d = request("linuxcnc")
    d["project"]["machine"]["optional_stop"] = True
    out.append(("C.linuxcnc.multi.optional_stop", two_tools(d)))
    return out


def classify(gcode):
    tool_lines, commands = [], []
    for line in gcode.splitlines():
        s = line.strip()
        if "FABTOOL/1" in s or "tool" in s.lower() or s.startswith(("T", "M0", "M1", "M6")):
            tool_lines.append(s)
        if s and not s.startswith(("(", ";", "'")):
            commands.append(s)
    return tool_lines, commands


def main():
    port, target = sys.argv[1], sys.argv[2]
    results = {}
    for name, body in cases():
        req = urllib.request.Request(
            f"http://127.0.0.1:{port}/generate",
            data=json.dumps(body).encode(),
            headers={"content-type": "application/json"},
        )
        try:
            with urllib.request.urlopen(req, timeout=600) as resp:
                r = json.loads(resp.read())
        except urllib.error.HTTPError as e:
            results[name] = {"http_error": e.code, "body": json.loads(e.read() or b"null")}
            continue
        tp = r.get("tool_provenance") or {}
        tool_lines, commands = classify(r["gcode"])
        results[name] = {
            "authority": tp.get("authority"),
            "diagnostics": tp.get("diagnostics"),
            "bindings": len(tp.get("bindings", [])),
            "refs": sorted({s.get("tool_ref") for s in r["toolpath"]}, key=lambda v: (v is None, v)),
            "tool_lines": tool_lines,
            "command_line_count": len(commands),
            "command_digest": __import__("hashlib").sha256("\n".join(commands).encode()).hexdigest(),
        }
    pathlib.Path(target).write_text(json.dumps(results, indent=1, ensure_ascii=False) + "\n")
    for name, v in results.items():
        print(f"{name:45} {v.get('authority', v.get('http_error'))} {v.get('diagnostics', '')}")


if __name__ == "__main__":
    main()
