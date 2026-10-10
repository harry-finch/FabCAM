# M1 analytic SVG/DXF corpus

Coordinates are millimetres in CAM XY, stock top Z0. Every case has a static
SVG and ASCII DXF; expected dimensions are independent of these imported bytes
in `crates/ivac-core/tests/shopbot_m1_qualification.rs`.

| Case | Geometry |
| --- | --- |
| square_100 | Rectangle (10,10)–(110,110) |
| rectangle_100x50 | Rectangle (10,10)–(110,60) |
| circle_d50 | Circle center (60,60), radius 25 |
| concave_L_shape | Closed vertices (10,10),(110,10),(110,40),(40,40),(40,110),(10,110) |
| rectangle_with_hole | Outer (10,10)–(110,60), nested (45,25)–(75,45) |
| rectangle_with_island | Outer (10,10)–(110,110), nested (50,50)–(70,70) |
| multiple_closed_contours | Separate rectangles (10,10)–(50,40), (70,10)–(110,40) |
| open_polyline | Open vertices (10,10),(60,10),(60,40), length 80 |

SVGs declare width/height 140 mm, viewBox `0 0 140 140`, and group transform
`translate(0 140) scale(1 -1)` to compensate production SVG Y flip. Thus SVG
and DXF resolve to the same CAM coordinates. SVG paths have no fill and explicit
closure; containment supplies nested semantics. DXF declares AC1015 and
`$INSUNITS=4` (mm), with LINE polygon edges and CIRCLE circle. Open input has no
synthetic closing edge. These inputs are neither machine programs nor physical
tolerance claims. See `docs/shopbot-modernization/M1_QUALIFICATION.md`.
