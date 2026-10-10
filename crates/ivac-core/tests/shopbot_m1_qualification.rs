//! M1 machining-intent qualification. No ShopBot dialect is emitted here.
//! Fixtures are imported through the production SVG/DXF dispatch; assertions
//! use independent analytic dimensions, distances and raster coverage.
mod common;

use common::{
    build_heightmap, build_project, drill_tool, endmill_tool, op_single_pass, run, sim_carve,
    stock_at_origin,
};
use ivac_core::cam::chaining::{classify_containment, segments_to_objects};
use ivac_core::gcode::preview::{MoveKind, ToolpathSegment};
use ivac_core::geometry::{BBox, Point2};
use ivac_core::input::{import_bytes, ImportOptions, ImportOutput};
use ivac_core::pipeline::PipelineResponse;
use ivac_core::project::{
    ContourParams, DrillCycle, OpKind, PocketParams, PocketStrategy, ProfileParams, ToolOffset,
};
use ivac_core::schema::PostProcessorKind;

// Output comparison tolerance (mm), not a topology or machining tolerance.
const XY_OUTPUT_MM: f64 = 0.02;
const Z_OUTPUT_MM: f64 = 0.001;
const CUTTER_R_MM: f64 = 3.0;
const FORMATS: [&str; 2] = ["svg", "dxf"];
const CASES: [(&str, f64, f64, usize, bool); 8] = [
    ("square_100", 100.0, 100.0, 1, true),
    ("rectangle_100x50", 100.0, 50.0, 1, true),
    ("circle_d50", 50.0, 50.0, 1, true),
    ("concave_L_shape", 100.0, 100.0, 1, true),
    ("rectangle_with_hole", 100.0, 50.0, 2, true),
    ("rectangle_with_island", 100.0, 100.0, 2, true),
    ("multiple_closed_contours", 100.0, 30.0, 2, true),
    ("open_polyline", 50.0, 30.0, 1, false),
];

fn fixture(name: &str, format: &str) -> ImportOutput {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/shopbot-m1")
        .join(format!("{name}.{format}"));
    let bytes = std::fs::read(&path).expect("required M1 fixture exists");
    import_bytes(
        &format!("{name}.{format}"),
        &bytes,
        &ImportOptions::default(),
    )
    .unwrap_or_else(|e| panic!("{name}.{format}: {e}"))
}

fn near(actual: f64, expected: f64, tolerance: f64, context: &str) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "{context}: expected {expected}, got {actual}, tolerance {tolerance} mm"
    );
}

fn job(name: &str, format: &str, kind: OpKind) -> PipelineResponse {
    let mut op = op_single_pass(1, name, kind, 1, -5.0);
    op.params.step = Some(-2.0);
    run(
        build_project(
            &stock_at_origin(140.0, 140.0, 10.0),
            vec![endmill_tool(1, 6.0)],
            op,
            fixture(name, format).segments,
        ),
        PostProcessorKind::Linuxcnc,
    )
}

fn profile(offset: ToolOffset) -> OpKind {
    OpKind::Profile {
        offset,
        contour: ContourParams::default(),
        profile: ProfileParams::default(),
    }
}

fn pocket(overlap: f64, allowance: Option<f64>) -> OpKind {
    OpKind::Pocket {
        strategy: PocketStrategy::Cascade,
        contour: ContourParams::default(),
        pocket: PocketParams {
            xy_overlap: overlap,
            pocket_islands: true,
            finish_xy_allowance_mm: allowance,
            ..Default::default()
        },
    }
}

fn floor_moves(resp: &PipelineResponse) -> Vec<&ToolpathSegment> {
    resp.toolpath
        .iter()
        .filter(|s| {
            s.kind != MoveKind::Rapid
                && (s.from.z + 5.0).abs() < Z_OUTPUT_MM
                && (s.to.z + 5.0).abs() < Z_OUTPUT_MM
                && (s.to.x - s.from.x).hypot(s.to.y - s.from.y) > 1e-8
        })
        .collect()
}

// Preview chords retain analytic arc descriptors. Sample those arcs rather than
// using chords to assess compensated boundaries.
fn at(s: &ToolpathSegment, t: f64) -> Point2 {
    if let Some(a) = s.arc {
        let start = (s.from.y - a.cy).atan2(s.from.x - a.cx);
        let end = (s.to.y - a.cy).atan2(s.to.x - a.cx);
        let tau = std::f64::consts::TAU;
        let sweep = if a.ccw {
            (end - start).rem_euclid(tau)
        } else {
            -(start - end).rem_euclid(tau)
        };
        let r = (s.from.x - a.cx).hypot(s.from.y - a.cy);
        let angle = start + t * sweep;
        Point2::new(a.cx + r * angle.cos(), a.cy + r * angle.sin())
    } else {
        Point2::new(
            s.from.x + t * (s.to.x - s.from.x),
            s.from.y + t * (s.to.y - s.from.y),
        )
    }
}

fn assert_passes(resp: &PipelineResponse, label: &str) {
    let mut levels = Vec::new();
    for s in &resp.toolpath {
        for p in [s.from, s.to] {
            assert!(p.z >= -5.0 - Z_OUTPUT_MM, "{label}: over-depth {p:?}");
        }
        if s.kind != MoveKind::Rapid
            && s.to.z < -Z_OUTPUT_MM
            && (s.to.z - s.from.z).abs() < Z_OUTPUT_MM
            && (s.to.x - s.from.x).hypot(s.to.y - s.from.y) > 1e-8
            && !levels
                .iter()
                .any(|z: &f64| (*z - s.to.z).abs() < Z_OUTPUT_MM)
        {
            levels.push(s.to.z);
        }
        // Positive-Z approach motion is excluded; ordinary descent below the
        // stock top must respect stepdown, including re-entry after retracts.
        if s.to.z < s.from.z && s.to.z < 0.0 {
            assert!(
                s.from.z.min(0.0) - s.to.z <= 2.0 + Z_OUTPUT_MM,
                "{label}: excessive cutting descent {s:?}"
            );
        }
    }
    levels.sort_by(f64::total_cmp);
    assert_eq!(levels.len(), 3, "{label}: depth levels {levels:?}");
    for (actual, expected) in levels.iter().zip([-5.0, -4.0, -2.0]) {
        near(*actual, expected, Z_OUTPUT_MM, label);
    }
}

#[test]
fn imports_preserve_analytic_dimensions_and_open_closed_topology() {
    for (name, width, height, count, closed) in CASES {
        for format in FORMATS {
            let imported = fixture(name, format);
            let label = format!("{name}.{format}");
            assert!(
                imported.warnings.is_empty(),
                "{label}: {:?}",
                imported.warnings
            );
            near(imported.bbox.width(), width, XY_OUTPUT_MM, &label);
            near(imported.bbox.height(), height, XY_OUTPUT_MM, &label);
            let mut objects = segments_to_objects(&imported.segments);
            classify_containment(&mut objects);
            assert_eq!(objects.len(), count, "{label}");
            assert!(objects.iter().all(|o| o.closed == closed), "{label}");
            if name == "rectangle_with_hole" || name == "rectangle_with_island" {
                assert_eq!(
                    objects
                        .iter()
                        .filter(|o| !o.inner_objects.is_empty())
                        .count(),
                    1,
                    "{label}"
                );
                assert_eq!(
                    objects
                        .iter()
                        .filter(|o| !o.outer_objects.is_empty())
                        .count(),
                    1,
                    "{label}"
                );
            }
        }
    }
}

#[test]
fn square_and_rectangle_profile_compensation_and_depth_passes() {
    for (name, width, height) in [
        ("square_100", 100.0, 100.0),
        ("rectangle_100x50", 100.0, 50.0),
    ] {
        for format in FORMATS {
            for (offset, expansion) in [
                (ToolOffset::Outside, 3.0),
                (ToolOffset::Inside, -3.0),
                (ToolOffset::On, 0.0),
            ] {
                let resp = job(name, format, profile(offset));
                let label = format!("{name}.{format}/{offset:?}");
                assert_passes(&resp, &label);
                let moves = floor_moves(&resp);
                assert!(!moves.is_empty(), "{label}");
                let mut bbox = BBox::EMPTY;
                for s in moves {
                    for t in [0.0, 0.25, 0.5, 0.75, 1.0] {
                        bbox.extend_point(at(s, t));
                    }
                }
                near(bbox.min_x, 10.0 - expansion, XY_OUTPUT_MM, &label);
                near(bbox.min_y, 10.0 - expansion, XY_OUTPUT_MM, &label);
                near(bbox.max_x, 10.0 + width + expansion, XY_OUTPUT_MM, &label);
                near(bbox.max_y, 10.0 + height + expansion, XY_OUTPUT_MM, &label);
            }
        }
    }
}

#[test]
fn circular_profiles_preserve_radius_at_each_compensation_side() {
    for format in FORMATS {
        for (offset, radius) in [
            (ToolOffset::Outside, 28.0),
            (ToolOffset::Inside, 22.0),
            (ToolOffset::On, 25.0),
        ] {
            let resp = job("circle_d50", format, profile(offset));
            let label = format!("circle.{format}/{offset:?}");
            assert_passes(&resp, &label);
            let moves = floor_moves(&resp);
            assert!(!moves.is_empty());
            let mut length = 0.0;
            for s in moves {
                for t in [0.0, 0.5, 1.0] {
                    let p = at(s, t);
                    near((p.x - 60.0).hypot(p.y - 60.0), radius, XY_OUTPUT_MM, &label);
                }
                length += if let Some(a) = s.arc {
                    let radius = (s.from.x - a.cx).hypot(s.from.y - a.cy);
                    let start = (s.from.y - a.cy).atan2(s.from.x - a.cx);
                    let end = (s.to.y - a.cy).atan2(s.to.x - a.cx);
                    let sweep = if a.ccw {
                        (end - start).rem_euclid(std::f64::consts::TAU)
                    } else {
                        (start - end).rem_euclid(std::f64::consts::TAU)
                    };
                    radius * sweep
                } else {
                    (s.to.x - s.from.x).hypot(s.to.y - s.from.y)
                };
            }
            // Reject missing sectors / duplicate full laps, independently of winding.
            near(length, std::f64::consts::TAU * radius, 0.15, &label);
        }
    }
}

#[test]
fn open_on_line_profile_has_no_synthetic_closing_edge() {
    for format in FORMATS {
        let resp = job("open_polyline", format, profile(ToolOffset::On));
        assert_passes(&resp, format);
        let moves = floor_moves(&resp);
        assert!(!moves.is_empty());
        let mut length = 0.0;
        for s in moves {
            for t in [0.0, 0.5, 1.0] {
                let p = at(s, t);
                let on_horizontal =
                    (p.y - 10.0).abs() < XY_OUTPUT_MM && (10.0..=60.0).contains(&p.x);
                let on_vertical = (p.x - 60.0).abs() < XY_OUTPUT_MM && (10.0..=40.0).contains(&p.y);
                assert!(
                    on_horizontal || on_vertical,
                    "{format}: unintended open-path cut {p:?}"
                );
            }
            length += (s.to.x - s.from.x).hypot(s.to.y - s.from.y);
        }
        near(length, 80.0, XY_OUTPUT_MM, format);
    }
}

fn polygon(name: &str) -> Vec<Vec<Point2>> {
    let outer = match name {
        "concave_L_shape" => vec![
            (10., 10.),
            (110., 10.),
            (110., 40.),
            (40., 40.),
            (40., 110.),
            (10., 110.),
        ],
        "rectangle_with_island" => vec![(10., 10.), (110., 10.), (110., 110.), (10., 110.)],
        _ => vec![(10., 10.), (110., 10.), (110., 60.), (10., 60.)],
    };
    let mut contours = vec![outer];
    if name == "rectangle_with_island" {
        contours.push(vec![(50., 50.), (70., 50.), (70., 70.), (50., 70.)]);
    }
    if name == "rectangle_with_hole" {
        contours.push(vec![(45., 25.), (75., 25.), (75., 45.), (45., 45.)]);
    }
    contours
        .into_iter()
        .map(|ps| ps.into_iter().map(|(x, y)| Point2::new(x, y)).collect())
        .collect()
}

fn inside(p: Point2, poly: &[Point2]) -> bool {
    let mut hit = false;
    for (a, b) in poly
        .iter()
        .zip(poly.iter().cycle().skip(1))
        .take(poly.len())
    {
        if (a.y > p.y) != (b.y > p.y) && p.x < (b.x - a.x) * (p.y - a.y) / (b.y - a.y) + a.x {
            hit = !hit;
        }
    }
    hit
}

fn clearance(p: Point2, contours: &[Vec<Point2>]) -> f64 {
    contours
        .iter()
        .flat_map(|poly| {
            poly.iter()
                .zip(poly.iter().cycle().skip(1))
                .take(poly.len())
        })
        .map(|(a, b)| {
            let dx = b.x - a.x;
            let dy = b.y - a.y;
            let t = (((p.x - a.x) * dx + (p.y - a.y) * dy) / (dx * dx + dy * dy)).clamp(0.0, 1.0);
            (p.x - a.x - t * dx).hypot(p.y - a.y - t * dy)
        })
        .fold(f64::INFINITY, f64::min)
}

fn machinable(p: Point2, contours: &[Vec<Point2>]) -> bool {
    inside(p, &contours[0]) && !contours[1..].iter().any(|hole| inside(p, hole))
}

fn assert_pocket(name: &str, format: &str, overlap: f64, allowance: Option<f64>) {
    let resp = job(name, format, pocket(overlap, allowance));
    let label = format!("{name}.{format}/overlap={overlap}/allowance={allowance:?}");
    assert_passes(&resp, &label);
    let contours = polygon(name);
    assert!(!floor_moves(&resp).is_empty(), "{label}");
    for s in resp
        .toolpath
        .iter()
        .filter(|s| s.kind != MoveKind::Rapid && s.from.z < 0.0 && s.to.z < 0.0)
    {
        for t in [0.0, 0.25, 0.5, 0.75, 1.0] {
            let p = at(s, t);
            assert!(
                machinable(p, &contours),
                "{label}: cutter center outside pocket {p:?}"
            );
            assert!(
                clearance(p, &contours) >= CUTTER_R_MM - XY_OUTPUT_MM,
                "{label}: cutter edge crosses boundary/island at {p:?}, clearance {}",
                clearance(p, &contours)
            );
        }
    }
    let tool = endmill_tool(1, 6.0);
    let mut hm = build_heightmap(&stock_at_origin(140.0, 140.0, 10.0), 0.5);
    sim_carve(&mut hm, &resp.toolpath, &tool);
    let mut tested_floor = 0;
    let mut tested_preserved = 0;
    // 1 mm lattice, deliberately avoiding discretized boundary cells. Verify
    // coverage and forbidden cuts, rather than merely trusting a plausible path.
    for x in 0..=130 {
        for y in 0..=130 {
            let p = Point2::new(f64::from(x), f64::from(y));
            let dist = clearance(p, &contours);
            let z = f64::from(hm.sample(p.x, p.y));
            if machinable(p, &contours) && dist > CUTTER_R_MM + 0.5 {
                near(z, -5.0, Z_OUTPUT_MM, &format!("{label}/coverage {p:?}"));
                tested_floor += 1;
            } else if !machinable(p, &contours) && dist > 1.0 {
                near(z, 0.0, Z_OUTPUT_MM, &format!("{label}/preservation {p:?}"));
                tested_preserved += 1;
            }
        }
    }
    assert!(
        tested_floor > 100 && tested_preserved > 100,
        "{label}: vacuous raster assertions"
    );
}

#[test]
fn pockets_preserve_concavity_holes_and_islands_and_reach_floor() {
    for name in [
        "rectangle_100x50",
        "concave_L_shape",
        "rectangle_with_hole",
        "rectangle_with_island",
    ] {
        for format in FORMATS {
            assert_pocket(name, format, 0.5, None);
        }
    }
}

#[test]
fn pocket_stepover_variants_have_no_coverage_gaps() {
    for overlap in [0.25, 0.75] {
        for format in FORMATS {
            assert_pocket("rectangle_100x50", format, overlap, None);
        }
    }
}

#[test]
fn pocket_finish_allowance_is_consumed_by_the_finishing_ring() {
    for format in FORMATS {
        assert_pocket("rectangle_100x50", format, 0.5, Some(1.0));
        let mut tool = endmill_tool(1, 6.0);
        tool.feed_rate_finish = Some(600);
        let op = op_single_pass(1, "allowance", pocket(0.5, Some(1.0)), 1, -5.0);
        let resp = run(
            build_project(
                &stock_at_origin(140., 140., 10.),
                vec![tool],
                op,
                fixture("rectangle_100x50", format).segments,
            ),
            PostProcessorKind::Linuxcnc,
        );
        let mut feed = 0.0;
        let feeds: Vec<f64> = resp
            .gcode
            .lines()
            .map(|line| {
                for word in line.split_whitespace() {
                    if let Some(number) = word.strip_prefix('F').and_then(|s| s.parse::<f64>().ok())
                    {
                        feed = number;
                    }
                }
                feed
            })
            .collect();
        let boundary = polygon("rectangle_100x50");
        let mut rough = 0;
        let mut finish = 0;
        for s in floor_moves(&resp) {
            let f = feeds[s.gcode_line as usize - 1];
            for t in [0.0, 0.5, 1.0] {
                let dist = clearance(at(s, t), &boundary);
                if (f - 1200.0).abs() < 0.1 {
                    assert!(
                        dist >= 4.0 - XY_OUTPUT_MM,
                        "{format}: roughing consumed allowance"
                    );
                    rough += 1;
                } else if (f - 600.0).abs() < 0.1 {
                    near(dist, 3.0, XY_OUTPUT_MM, "finishing ring radius");
                    finish += 1;
                }
            }
        }
        assert!(
            rough > 0 && finish > 0,
            "{format}: missing rough/finish phases"
        );
    }
}

#[test]
fn island_inflation_chords_keep_cutter_clearance_at_multiple_scales() {
    let contours = vec![vec![
        Point2::new(10., 10.),
        Point2::new(30., 10.),
        Point2::new(30., 30.),
        Point2::new(10., 30.),
    ]];
    for radius in [0.1, 3.0, 10.0] {
        let rings = ivac_core::cam::offsets::inflate_islands_by_tool_radius(&contours, radius);
        assert_eq!(rings.len(), 1);
        let ring = &rings[0];
        for (a, b) in ring
            .iter()
            .zip(ring.iter().cycle().skip(1))
            .take(ring.len())
        {
            for t in [0.0, 0.25, 0.5, 0.75, 1.0] {
                let p = Point2::new(a.x + t * (b.x - a.x), a.y + t * (b.y - a.y));
                let dist = clearance(p, &contours);
                assert!(
                    dist >= radius - 0.00015,
                    "radius {radius}: inscribed chord undercuts island at {p:?}, distance {dist}"
                );
                // Bound conservative stock left by the sagitta/grid padding.
                assert!(
                    dist <= radius + 0.0013,
                    "radius {radius}: excessive inflation {dist}"
                );
            }
        }
    }
}

#[test]
fn disconnected_profiles_do_not_cut_across_the_gap() {
    for format in FORMATS {
        let resp = job("multiple_closed_contours", format, profile(ToolOffset::On));
        assert_passes(&resp, format);
        let mut left = 0.0;
        let mut right = 0.0;
        for s in floor_moves(&resp) {
            let mut bounds = BBox::EMPTY;
            bounds.extend_point(at(s, 0.0));
            bounds.extend_point(at(s, 1.0));
            let length = (s.to.x - s.from.x).hypot(s.to.y - s.from.y);
            if bounds.max_x <= 50.0 + XY_OUTPUT_MM {
                left += length;
            } else if bounds.min_x >= 70.0 - XY_OUTPUT_MM {
                right += length;
            } else {
                panic!("{format}: cutting connection between disconnected contours {s:?}");
            }
        }
        near(left, 140.0, XY_OUTPUT_MM, "left perimeter");
        near(right, 140.0, XY_OUTPUT_MM, "right perimeter");
    }
}

#[test]
fn imported_drill_centers_depth_and_peck_semantics() {
    for format in FORMATS {
        for name in ["circle_d50", "multiple_closed_contours"] {
            let expected: Vec<(f64, f64)> = if name == "circle_d50" {
                vec![(60., 60.)]
            } else {
                vec![(30., 25.), (90., 25.)]
            };
            for cycle in [
                DrillCycle::Simple { dwell_sec: 0.0 },
                DrillCycle::Peck {
                    peck_step_mm: 2.0,
                    dwell_sec: 0.0,
                },
            ] {
                // Use GRBL's explicit motion expansion to inspect peck semantics.
                let mut op = op_single_pass(
                    1,
                    name,
                    OpKind::Drill {
                        cycle,
                        chamfer_after_width_mm: None,
                        pattern: None,
                        spot_first: None,
                    },
                    1,
                    -5.0,
                );
                // Explicit cycle clearance plane, independent of travel Z.
                op.params.start_depth = 1.5;
                let tool = drill_tool(1, 6.0, 118.0);
                let resp = run(
                    build_project(
                        &stock_at_origin(140., 140., 10.),
                        vec![tool],
                        op,
                        fixture(name, format).segments,
                    ),
                    PostProcessorKind::Grbl,
                );
                let tip_z = -5.0 - 3.0 / (59.0_f64.to_radians().tan());
                let label = format!("drill {name}.{format}/{cycle:?}");
                let min_z = resp
                    .toolpath
                    .iter()
                    .map(|s| s.to.z)
                    .fold(f64::INFINITY, f64::min);
                near(min_z, tip_z, Z_OUTPUT_MM, &label);
                let mut reached = vec![false; expected.len()];
                let mut new_bottoms = vec![0.0; expected.len()];
                let mut pecks = vec![0; expected.len()];
                let mut retracted = vec![false; expected.len()];
                for s in &resp.toolpath {
                    if s.to.z >= 0.0 {
                        if s.to.z >= 1.5 - Z_OUTPUT_MM {
                            if let Some(i) = expected.iter().position(|&(x, y)| {
                                (s.to.x - x).abs() < XY_OUTPUT_MM
                                    && (s.to.y - y).abs() < XY_OUTPUT_MM
                            }) {
                                retracted[i] = true;
                            }
                        }
                        continue;
                    }
                    let i = expected
                        .iter()
                        .position(|&(x, y)| {
                            (s.to.x - x).abs() < XY_OUTPUT_MM && (s.to.y - y).abs() < XY_OUTPUT_MM
                        })
                        .unwrap_or_else(|| panic!("{label}: unexpected hole/motion {:?}", s.to));
                    assert!(
                        (s.from.x - s.to.x).abs() < XY_OUTPUT_MM
                            && (s.from.y - s.to.y).abs() < XY_OUTPUT_MM,
                        "{label}: lateral motion below stock"
                    );
                    assert!(s.to.z >= tip_z - Z_OUTPUT_MM, "{label}: over-depth");
                    if s.kind == MoveKind::Rapid {
                        assert!(
                            s.to.z >= new_bottoms[i] - Z_OUTPUT_MM,
                            "{label}: rapid enters uncut material"
                        );
                    }
                    if s.to.z < new_bottoms[i] - Z_OUTPUT_MM {
                        if matches!(cycle, DrillCycle::Peck { .. }) {
                            assert!(
                                retracted[i],
                                "{label}: no full clearance retract before peck"
                            );
                            assert!(
                                new_bottoms[i] - s.to.z <= 2.0 + Z_OUTPUT_MM,
                                "{label}: peck too deep"
                            );
                        }
                        new_bottoms[i] = s.to.z;
                        pecks[i] += 1;
                        retracted[i] = false;
                    }
                    if (s.to.z - tip_z).abs() < Z_OUTPUT_MM {
                        reached[i] = true;
                    }
                }
                assert!(reached.iter().all(|b| *b), "{label}: missing holes");
                if matches!(cycle, DrillCycle::Peck { .. }) {
                    assert!(pecks.iter().all(|n| *n >= 4), "{label}: missing pecks");
                }
                let last = resp.toolpath.last().expect("nonempty drill path");
                assert!(last.to.z >= 0.0, "{label}: program ends below stock");
            }
        }
    }
}

#[test]
fn qualification_outputs_are_deterministic() {
    for format in FORMATS {
        for (name, kind) in [
            ("square_100", profile(ToolOffset::Outside)),
            ("rectangle_with_island", pocket(0.5, None)),
        ] {
            let a = job(name, format, kind.clone());
            let b = job(name, format, kind);
            assert_eq!(a.gcode, b.gcode, "{name}.{format}");
            assert_eq!(
                serde_json::to_value(&a.toolpath).unwrap(),
                serde_json::to_value(&b.toolpath).unwrap()
            );
        }
    }
}
