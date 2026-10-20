#!/usr/bin/env python3
"""Generate DXF/SVG import-correctness fixtures with analytically known results.

Expected values are computed here from the DXF reference semantics, independently
of the Rust importer:

* OCS entities (ARC, CIRCLE, LWPOLYLINE, 2D POLYLINE, TEXT, INSERT) store points
  in their Object Coordinate System; the Arbitrary Axis Algorithm maps OCS to WCS.
  LINE, POINT and ELLIPSE/SPLINE points are WCS; an ELLIPSE's minor axis is
  ``normal x major * ratio``.
* INSERT: ``WCS = OCS_insert(location + R(rotation) * S(sx, sy) * (p - base))``.
* Only planes parallel to XY (normal +-Z) are representable in 2D CAM; any other
  normal must be refused.

Run from the repository root:  python3 tests/fixtures/import-correctness/generate.py
"""
import json
import math
import pathlib

OUT = pathlib.Path(__file__).resolve().parent


def ocs_axes(n):
    """Arbitrary Axis Algorithm (DXF reference): OCS X/Y axes in WCS."""
    l = math.sqrt(sum(c * c for c in n))
    n = [c / l for c in n]
    w = [0.0, 1.0, 0.0] if abs(n[0]) < 1 / 64 and abs(n[1]) < 1 / 64 else [0.0, 0.0, 1.0]
    ax = cross(w, n)
    la = math.sqrt(sum(c * c for c in ax))
    ax = [c / la for c in ax]
    ay = cross(n, ax)
    return ax, ay


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def ocs_to_wcs(n, p):
    ax, ay = ocs_axes(n)
    return [ax[0] * p[0] + ay[0] * p[1], ax[1] * p[0] + ay[1] * p[1]]


# ── DXF writing ──────────────────────────────────────────────────────────


def header(insunits=4):
    h = ["0", "SECTION", "2", "HEADER", "9", "$ACADVER", "1", "AC1015"]
    if insunits is not None:
        h += ["9", "$INSUNITS", "70", str(insunits)]
    return h + ["0", "ENDSEC"]


def normal(n):
    return [] if n is None else ["210", fmt(n[0]), "220", fmt(n[1]), "230", fmt(n[2])]


def fmt(v):
    return repr(float(v))


def arc(c, r, a0, a1, n=None, layer="0"):
    return ["0", "ARC", "8", layer, "10", fmt(c[0]), "20", fmt(c[1]), "30", "0.0", "40", fmt(r),
            "50", fmt(a0), "51", fmt(a1)] + normal(n)


def circle(c, r, n=None):
    return ["0", "CIRCLE", "8", "0", "10", fmt(c[0]), "20", fmt(c[1]), "30", "0.0", "40", fmt(r)] + normal(n)


def line(a, b, n=None):
    return ["0", "LINE", "8", "0", "10", fmt(a[0]), "20", fmt(a[1]), "30", "0.0",
            "11", fmt(b[0]), "21", fmt(b[1]), "31", "0.0"] + normal(n)


def lwpoly(verts, closed, n=None):
    out = ["0", "LWPOLYLINE", "8", "0", "90", str(len(verts)), "70", "1" if closed else "0"] + normal(n)
    for x, y, b in verts:
        out += ["10", fmt(x), "20", fmt(y), "42", fmt(b)]
    return out


def polyline(verts, closed, n=None):
    out = ["0", "POLYLINE", "8", "0", "66", "1", "10", "0.0", "20", "0.0", "30", "0.0",
           "70", "1" if closed else "0"] + normal(n)
    for x, y, b in verts:
        out += ["0", "VERTEX", "8", "0", "10", fmt(x), "20", fmt(y), "30", "0.0", "42", fmt(b)]
    return out + ["0", "SEQEND", "8", "0"]


def ellipse(c, major, ratio, t0, t1, n=None):
    return ["0", "ELLIPSE", "8", "0", "10", fmt(c[0]), "20", fmt(c[1]), "30", "0.0",
            "11", fmt(major[0]), "21", fmt(major[1]), "31", "0.0"] + normal(n) + [
            "40", fmt(ratio), "41", fmt(t0), "42", fmt(t1)]


def insert(name, loc, sx=1.0, sy=1.0, rot=0.0, n=None, cols=None):
    out = ["0", "INSERT", "8", "0", "2", name, "10", fmt(loc[0]), "20", fmt(loc[1]), "30", "0.0",
           "41", fmt(sx), "42", fmt(sy), "43", "1.0", "50", fmt(rot)]
    if cols:
        out += ["70", str(cols), "44", "20.0"]
    return out + normal(n)


def block(name, base, entities):
    return (["0", "BLOCK", "8", "0", "2", name, "70", "0", "10", fmt(base[0]), "20", fmt(base[1]), "30", "0.0",
             "3", name] + entities + ["0", "ENDBLK", "8", "0"])


def dxf(entities, blocks=(), insunits=4):
    out = header(insunits)
    if blocks:
        out += ["0", "SECTION", "2", "BLOCKS"] + [x for b in blocks for x in b] + ["0", "ENDSEC"]
    out += ["0", "SECTION", "2", "ENTITIES"] + entities + ["0", "ENDSEC", "0", "EOF"]
    return "\n".join(out) + "\n"


# ── Expected primitives (WCS, mm) ────────────────────────────────────────


def p_line(a, b):
    return {"line": [list(a), list(b)]}


def p_arc(c, s, e, ccw):
    return {"arc": {"center": list(c), "start": list(s), "end": list(e), "ccw": ccw}}


def p_circle(c, r):
    return {"circle": {"center": list(c), "r": r}}


def p_curve(c, u, v, t0, sweep):
    pt = lambda t: [c[0] + u[0] * math.cos(t) + v[0] * math.sin(t), c[1] + u[1] * math.cos(t) + v[1] * math.sin(t)]
    return {"curve": {"center": list(c), "u": list(u), "v": list(v), "start": pt(t0), "end": pt(t0 + sweep)}}


def ocs_arc_expect(c, r, a0, a1, n):
    """WCS arc from an OCS ARC, by mapping points (independent of bulge rules)."""
    s = ocs_to_wcs(n, [c[0] + r * math.cos(math.radians(a0)), c[1] + r * math.sin(math.radians(a0))])
    e = ocs_to_wcs(n, [c[0] + r * math.cos(math.radians(a1)), c[1] + r * math.sin(math.radians(a1))])
    ax, ay = ocs_axes(n)
    det = ax[0] * ay[1] - ax[1] * ay[0]
    return p_arc(ocs_to_wcs(n, c), s, e, det > 0)


def bulge_poly_expect(verts, closed, n, map_pt=None):
    """WCS primitives of a bulged polyline whose vertices are in OCS ``n``."""
    m = map_pt or (lambda p: ocs_to_wcs(n, p))
    o, ex, ey = m([0, 0]), m([1, 0]), m([0, 1])
    det = (ex[0] - o[0]) * (ey[1] - o[1]) - (ex[1] - o[1]) * (ey[0] - o[0])
    out = []
    k = len(verts) if closed else len(verts) - 1
    for i in range(k):
        (x0, y0, b), (x1, y1, _) = verts[i], verts[(i + 1) % len(verts)]
        a, bb = m([x0, y0]), m([x1, y1])
        if b == 0:
            out.append(p_line(a, bb))
            continue
        # OCS centre of the bulge arc.
        chord = math.hypot(x1 - x0, y1 - y0)
        theta = 4 * math.atan(b)
        r = chord / (2 * math.sin(theta / 2))
        mx, my = (x0 + x1) / 2, (y0 + y1) / 2
        d = r * math.cos(theta / 2)
        ux, uy = (x1 - x0) / chord, (y1 - y0) / chord
        cx, cy = mx - uy * d, my + ux * d
        out.append(p_arc(m([cx, cy]), a, bb, (b > 0) == (det > 0)))
    return out


def insert_map(loc, sx, sy, rot, base, n=None):
    c, s = math.cos(math.radians(rot)), math.sin(math.radians(rot))

    def f(p):
        x, y = (p[0] - base[0]) * sx, (p[1] - base[1]) * sy
        q = [loc[0] + c * x - s * y, loc[1] + s * x + c * y]
        return ocs_to_wcs(n, q) if n else q
    return f


CASES = []


def case(name, text, expect, ext="dxf"):
    (OUT / f"{name}.{ext}").write_text(text)
    CASES.append({"name": name, "file": f"{name}.{ext}", **expect})


def main():
    PZ, NZ = [0, 0, 1], [0, 0, -1]
    # ── OCS / extrusion normals ──
    case("ocs_arc_pz", dxf(arc([10, 0], 5, 0, 90, PZ)), {"chains": [ocs_arc_expect([10, 0], 5, 0, 90, PZ)]})
    case("ocs_arc_nz", dxf(arc([10, 0], 5, 0, 90, NZ)), {"chains": [ocs_arc_expect([10, 0], 5, 0, 90, NZ)]})
    case("ocs_arc_nz_scaled_normal", dxf(arc([10, 0], 5, 0, 90, [0, 0, -2.5])),
         {"chains": [ocs_arc_expect([10, 0], 5, 0, 90, NZ)]})
    case("ocs_circle_nz", dxf(circle([20, 10], 3, NZ)), {"chains": [p_circle(ocs_to_wcs(NZ, [20, 10]), 3)]})
    sq = [(0, 0, 0), (10, 0, 1), (10, 10, 0), (0, 10, -0.5)]
    case("ocs_lwpoly_pz", dxf(lwpoly(sq, True, PZ)), {"chains": bulge_poly_expect(sq, True, PZ)})
    case("ocs_lwpoly_nz", dxf(lwpoly(sq, True, NZ)), {"chains": bulge_poly_expect(sq, True, NZ)})
    case("ocs_polyline2d_nz", dxf(polyline(sq, True, NZ)), {"chains": bulge_poly_expect(sq, True, NZ)})
    case("line_nz_is_wcs", dxf(line([1, 2], [5, 2], NZ)), {"chains": [p_line([1, 2], [5, 2])]})
    for nm, n in (("pz", PZ), ("nz", NZ)):
        minor = cross(n, [10, 0, 0])
        case(f"ellipse_quarter_{nm}", dxf(ellipse([0, 0], [10, 0], 0.5, 0, math.pi / 2, n)),
             {"chains": [p_curve([0, 0], [10, 0], [minor[0] * 0.5, minor[1] * 0.5], 0, math.pi / 2)]})
    for nm, n in (("y", [0, 1, 0]), ("tilted", [0.6, 0, 0.8]), ("zero", [0, 0, 0])):
        case(f"ocs_reject_{nm}", dxf(arc([10, 0], 5, 0, 90, n)), {"error": "extrusion"})
    case("ocs_reject_lwpoly_tilted", dxf(lwpoly(sq, True, [0, 0.6, 0.8])), {"error": "extrusion"})

    # ── BLOCK base points and INSERT transforms ──
    case("block_base_point", dxf(insert("B", [0, 0]), [block("B", [100, 100], line([100, 100], [110, 100]))]),
         {"chains": [p_line([0, 0], [10, 0])]})
    f = insert_map([50, 20], 2, 2, 90, [5, 5])
    case("block_base_rot_scale",
         dxf(insert("B", [50, 20], 2, 2, 90), [block("B", [5, 5], line([5, 5], [15, 5]) + arc([5, 5], 2, 0, 90))]),
         {"chains": [p_line(f([5, 5]), f([15, 5])), p_arc(f([5, 5]), f([7, 5]), f([5, 7]), True)]})
    f = insert_map([0, 0], -1, 1, 0, [0, 0])
    case("block_mirror_x", dxf(insert("B", [0, 0], -1, 1), [block("B", [0, 0], arc([10, 0], 5, 0, 90))]),
         {"chains": [p_arc(f([10, 0]), f([15, 0]), f([10, 5]), False)]})
    f = insert_map([30, 0], 1, -1, 30, [2, 0])
    case("block_mirror_y_rotated",
         dxf(insert("B", [30, 0], 1, -1, 30), [block("B", [2, 0], lwpoly(sq, True))]),
         {"chains": bulge_poly_expect(sq, True, PZ, map_pt=f)})
    inner, outer = insert_map([10, 0], 1, 1, 90, [1, 1]), insert_map([100, 100], 2, 2, 0, [0, 0])
    case("block_nested",
         dxf(insert("OUTER", [100, 100], 2, 2), [block("INNER", [1, 1], line([1, 1], [3, 1])),
                                                 block("OUTER", [0, 0], insert("INNER", [10, 0], 1, 1, 90))]),
         {"chains": [p_line(outer(inner([1, 1])), outer(inner([3, 1])))]})
    case("block_multiple_occurrences",
         dxf(insert("B", [0, 0]) + insert("B", [50, 0]), [block("B", [5, 5], line([5, 5], [10, 5]))]),
         {"chains": [p_line([0, 0], [5, 0]), p_line([50, 0], [55, 0])]})
    f = insert_map([10, 0], 1, 1, 0, [0, 0], NZ)
    case("block_insert_ocs_nz", dxf(insert("B", [10, 0], n=NZ), [block("B", [0, 0], line([0, 0], [5, 0]))]),
         {"chains": [p_line(f([0, 0]), f([5, 0]))]})
    f = insert_map([100, 0], 1, 1, 0, [0, 0])
    a = ocs_arc_expect([10, 0], 5, 0, 90, NZ)["arc"]
    case("block_entity_ocs_inside", dxf(insert("B", [100, 0]), [block("B", [0, 0], arc([10, 0], 5, 0, 90, NZ))]),
         {"chains": [p_arc(f(a["center"]), f(a["start"]), f(a["end"]), a["ccw"])]})
    f = insert_map([10, 10], 1, 1, 0, [3, 0], NZ)
    a = ocs_arc_expect([10, 0], 5, 0, 90, NZ)["arc"]
    case("block_combined_ocs",
         dxf(insert("B", [10, 10], n=NZ), [block("B", [3, 0], arc([10, 0], 5, 0, 90, NZ))]),
         {"chains": [p_arc(f(a["center"]), f(a["start"]), f(a["end"]), not a["ccw"])]})
    case("block_minsert_refused", dxf(insert("B", [0, 0], cols=3), [block("B", [0, 0], line([0, 0], [5, 0]))]),
         {"error": "MINSERT"})

    # ── DXF units ──
    case("dxf_units_inch", dxf(line([0, 0], [1, 0]), insunits=1), {"chains": [p_line([0, 0], [25.4, 0])]})
    case("dxf_units_absent", dxf(line([0, 0], [1, 0]), insunits=None),
         {"chains": [p_line([0, 0], [1, 0])], "warning": "unit"})
    case("dxf_units_unitless", dxf(line([0, 0], [1, 0]), insunits=0),
         {"chains": [p_line([0, 0], [1, 0])], "warning": "unit"})

    # ── SVG dimensions (a 0,0 → W,0 top edge of one rectangle) ──
    def svg(attrs, body):
        return f'<svg xmlns="http://www.w3.org/2000/svg" {attrs}>{body}</svg>\n'

    rect = '<rect x="0" y="0" width="{w}" height="{h}" fill="none" stroke="black"/>'
    svg_cases = [
        ("svg_mm", 'width="100mm" height="50mm" viewBox="0 0 100 50"', rect.format(w=100, h=50), 100, None),
        ("svg_mm_px_viewbox", 'width="100mm" height="50mm" viewBox="0 0 377.952756 188.976378"',
         rect.format(w=377.952756, h=188.976378), 100, None),
        ("svg_cm", 'width="10cm" height="5cm" viewBox="0 0 10 5"', rect.format(w=10, h=5), 100, None),
        ("svg_in", 'width="2in" height="1in" viewBox="0 0 2 1"', rect.format(w=2, h=1), 50.8, None),
        ("svg_pt", 'width="72pt" height="72pt" viewBox="0 0 72 72"', rect.format(w=72, h=72), 25.4, None),
        ("svg_px", 'width="96px" height="96px" viewBox="0 0 96 96"', rect.format(w=96, h=96), 96, "unit"),
        ("svg_unitless", 'width="96" height="96"', rect.format(w=96, h=96), 96, "unit"),
        ("svg_viewbox_only", 'viewBox="0 0 100 100"', rect.format(w=100, h=100), 100, "unit"),
        ("svg_group_transform", 'width="100mm" height="100mm" viewBox="0 0 100 100"',
         '<g transform="translate(10,0)"><g transform="scale(2)">' + rect.format(w=5, h=5) + "</g></g>", 10, None),
        ("svg_inkscape_a4", 'width="210mm" height="297mm" viewBox="0 0 210 297" version="1.1"',
         rect.format(w=210, h=297), 210, None),
        ("svg_illustrator_pt_as_px", 'version="1.1" x="0px" y="0px" width="595.28px" height="841.89px" '
         'viewBox="0 0 595.28 841.89"', rect.format(w=595.28, h=841.89), 595.28, "unit"),
    ]
    for name, attrs, body, width_mm, warn in svg_cases:
        exp = {"width_mm": width_mm}
        if warn:
            exp["warning"] = warn
        case(name, svg(attrs, body), exp, ext="svg")

    (OUT / "expected.json").write_text(json.dumps(CASES, indent=1) + "\n")
    print(len(CASES), "cases")


if __name__ == "__main__":
    main()
