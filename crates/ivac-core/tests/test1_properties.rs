//! TEST1 process-isolated worker. Invoked by scripts/test1/run.py with replay JSON.
#![cfg(all(feature = "cps", feature = "profiling"))]
mod common;
#[path = "test1/oracle.rs"]
mod oracle;
use common::*;
use ivac_core::drawing::{Binding, Drawing, Source, Transform};
use ivac_core::geometry::{BBox, Point2, Segment};
use ivac_core::pipeline::*;
use ivac_core::project::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::f64::consts::{PI, TAU};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Case {
    property: String,
    index: u32,
    seed: u64,
    complexity: u32,
    scale: f64,
}
fn save(name: &str, value: &impl Serialize) {
    let dir = std::env::var("TEST1_OUT").expect("TEST1_OUT");
    std::fs::write(
        std::path::Path::new(&dir).join(name),
        serde_json::to_vec_pretty(value).unwrap(),
    )
    .unwrap();
}
fn insist(ok: bool, label: &str) -> Result<(), String> {
    if ok {
        Ok(())
    } else {
        Err(label.into())
    }
}
fn request(c: &Case, shape: u32, v: bool) -> PipelineRequest {
    let w = 24. + f64::from((c.seed % 8) as u32) * 0.25 * c.scale;
    let h = 20.;
    let mut segments = closed_rectangle(10., 10., 10. + w, 10. + h);
    if shape == 1 {
        segments = closed_circle(24., 24., 10.);
    }
    if !v && shape >= 2 {
        segments = closed_rectangle(10., 10., 36. + 14. * c.scale, 32. + 8. * c.scale);
    }
    if (shape == 2 || shape == 3) && !(c.property == "B1" && shape == 2 && c.index % 8 == 2) {
        segments.extend(closed_rectangle(20., 18., 24., 22.));
    }
    if shape == 3 {
        segments.extend(closed_rectangle(29., 25., 32., 28.));
    }
    if shape == 4 {
        let angle = (30. + f64::from(c.index % 5) * 30.).to_radians();
        segments = oracle::ring(&[
            (10., 10.),
            (34., 10.),
            (10. + 18. * angle.cos(), 10. + 18. * angle.sin()),
        ]);
    }
    let depth = 0.5 + f64::from(c.index % 3) * 0.25;
    let kind = if v {
        OpKind::VCarve {
            carve: VCarveParams {
                full_medial_axis: c.index % 2 == 1,
                ..Default::default()
            },
        }
    } else if shape >= 2 {
        OpKind::Pocket {
            strategy: if c.index % 2 == 0 {
                PocketStrategy::Cascade
            } else {
                PocketStrategy::Zigzag { angle_deg: 0. }
            },
            contour: ContourParams::default(),
            pocket: PocketParams {
                pocket_islands: true,
                xy_overlap: 0.5,
                ..Default::default()
            },
        }
    } else {
        OpKind::Profile {
            offset: if (c.index / 4) % 2 == 0 {
                ToolOffset::Outside
            } else {
                ToolOffset::Inside
            },
            contour: ContourParams::default(),
            profile: ProfileParams::default(),
        }
    };
    let mut op = op_single_pass(1, "TEST1 SOFTWARE ONLY", kind, 1, -depth);
    op.params.step = Some(-depth / 2.);
    let tool = if v {
        vbit_tool(1, 6., if c.index % 4 < 2 { 60. } else { 90. }, 0.)
    } else {
        endmill_tool(1, 2.)
    };
    PipelineRequest {
        project: build_project(&stock_at_origin(100., 100., 18.), vec![tool], op, segments),
        post_processor: Some(PostProcessorKind::Cps),
        cps_post: Some(CpsPostSelection {
            source: CpsPostSource::Bundled {
                id: "shopbot".into(),
            },
            properties: Default::default(),
        }),
    }
}
fn run(req: PipelineRequest, tag: &str) -> Result<PipelineResponse, String> {
    save(&format!("{tag}.request.json"), &req);
    let r = run_pipeline(req, |_, _, _| {}).map_err(|e| format!("generation_refusal: {e}"))?;
    save(&format!("{tag}.response.json"), &r);
    if r.toolpath.len() > 20_000 {
        return Err(format!("move_budget:actual={}", r.toolpath.len()));
    }
    Ok(r)
}
fn a1(c: &Case) -> Result<(), String> {
    let n = c.complexity.clamp(1, 4);
    let mut sources = Vec::new();
    for id in 1..=n {
        let mut segs = closed_rectangle(0., 0., 10. + c.scale, 8.);
        if c.index % 2 == 0 {
            segs = closed_circle(0., 0., 5.);
        }
        if c.index % 3 == 0 {
            for s in &mut segs {
                std::mem::swap(&mut s.start, &mut s.end);
                s.bulge = -s.bulge;
            }
        }
        let len = segs.len();
        segs.rotate_left(c.index as usize % len);
        sources.push(Source {
            id,
            original: segs,
            legacy_objects: vec![],
            pivot: Point2::new(0., 0.),
            transform: Transform::default(),
            next_entity: 1,
        });
    }
    save("sources.json", &sources);
    let expected: std::collections::BTreeSet<_> = sources
        .iter()
        .flat_map(|s| (0..s.original.len()).map(move |i| (s.id, i)))
        .collect();
    let mut drawing = Drawing::from_sources(sources).map_err(|e| e.to_string())?;
    let actual: Vec<_> = drawing
        .contours
        .iter()
        .flat_map(|contour| {
            contour
                .edges
                .iter()
                .flat_map(|e| e.original_indices.iter().map(move |i| (e.id.import, *i)))
        })
        .collect();
    insist(
        actual.len() == expected.len()
            && actual
                .into_iter()
                .collect::<std::collections::BTreeSet<_>>()
                == expected,
        "source_membership",
    )?;
    let id = drawing.contours.remove(0).id;
    insist(
        drawing
            .evaluate()
            .map_err(|e| e.to_string())?
            .resolve(&Binding::Contours { ids: vec![id] })
            .is_err(),
        "deleted_binding",
    )
}
fn a2(c: &Case) -> Result<(), String> {
    let radius = 1. + f64::from((c.seed % 99) as u32);
    let start = f64::from(c.index) * 0.27;
    let sweep = (0.01 + (f64::from(c.index % 16) / 16.) * (TAU - 0.02))
        * if c.index % 2 == 0 { 1. } else { -1. };
    let center = Point2::new(12., -17.);
    let point = |a: f64| Point2::new(center.x + radius * a.cos(), center.y + radius * a.sin());
    let arc = Segment::arc(
        point(start),
        point(start + sweep),
        (sweep / 4.).tan(),
        Some(center),
        "0",
        7,
    );
    save("arc.json", &arc);
    let bounds = BBox::from_segments(&[arc]);
    for a in [start, start + sweep, 0., PI / 2., PI, 3. * PI / 2.] {
        let d = if sweep > 0. {
            (a - start).rem_euclid(TAU)
        } else {
            (start - a).rem_euclid(TAU)
        };
        if a == start || a == start + sweep || d <= sweep.abs() {
            let p = point(a);
            insist(
                p.x >= bounds.min_x - 1e-8
                    && p.x <= bounds.max_x + 1e-8
                    && p.y >= bounds.min_y - 1e-8
                    && p.y <= bounds.max_y + 1e-8,
                "analytic_arc_extrema",
            )?;
        }
    }
    let req = request(c, 0, false);
    let baseline = run(req.clone(), "base")?;
    let mut shifted = req;
    let dx = f64::from(c.index % 9);
    let dy = f64::from(c.index % 7);
    shifted.project.work_offset.x_mm = dx;
    shifted.project.work_offset.y_mm = dy;
    let r = run(shifted, "shifted")?;
    insist(
        r.toolpath.len() == baseline.toolpath.len(),
        "frame_path_count",
    )?;
    for (a, b) in baseline.toolpath.iter().zip(&r.toolpath) {
        for (x, y) in [(&a.from, &b.from), (&a.to, &b.to)] {
            insist(
                (x.x - dx - y.x).abs() < 0.00011
                    && (x.y - dy - y.y).abs() < 0.00011
                    && (x.z - y.z).abs() < 1e-8,
                "frame_roundtrip",
            )?;
        }
    }
    Ok(())
}
fn b1(c: &Case) -> Result<(), String> {
    let shape = c.index % 4;
    let req = request(c, shape, false);
    let depth = req.project.operations[0].params.depth;
    let w = req.project.segments[0].end.x;
    let h = req.project.segments[1].end.y;
    let r = run(req, "flat")?;
    let decoded = oracle::decode(&r.gcode)?;
    let mut witnessed = false;
    for m in &decoded {
        let Some(cut) = oracle::below_surface(m) else {
            continue;
        };
        let m = &cut;
        witnessed |= (m.b[2] - depth).abs() < 0.000051;
        if shape == 1 {
            let min = oracle::segment_distance([24., 24.], [m.a[0], m.a[1]], [m.b[0], m.b[1]]);
            let max = (m.a[0] - 24.)
                .hypot(m.a[1] - 24.)
                .max((m.b[0] - 24.).hypot(m.b[1] - 24.));
            if (c.index / 4) % 2 == 0 {
                insist(min >= 11. - 0.010001, "circle_outside_sweep")?;
            } else {
                insist(max <= 9. + 0.0001, "circle_inside_sweep")?;
            }
        } else if shape == 0 {
            if (c.index / 4) % 2 == 0 {
                oracle::clear_edges(
                    m,
                    &oracle::rect_edges([10., 10.], [w, 30.]),
                    1.,
                    1.,
                    0.010001,
                )?;
            }
            for p in [m.a, m.b] {
                if (c.index / 4) % 2 == 0 {
                    insist(
                        !(p[0] > 10. && p[0] < w && p[1] > 10. && p[1] < 30.),
                        "profile_outside_side",
                    )?;
                } else {
                    insist(
                        p[0] >= 10.9999 && p[0] <= w - 0.9999 && p[1] >= 10.9999 && p[1] <= 29.0001,
                        "profile_inside_side",
                    )?;
                }
            }
        } else {
            for p in [m.a, m.b] {
                insist(
                    p[0] >= 10.9999 && p[0] <= w - 0.9999 && p[1] >= 10.9999 && p[1] <= h - 0.9999,
                    "pocket_outer_clearance",
                )?;
            }
            for (lo, hi) in if shape == 3 {
                vec![([20., 18.], [24., 22.]), ([29., 25.], [32., 28.])]
            } else if c.index % 8 == 2 {
                vec![]
            } else {
                vec![([20., 18.], [24., 22.])]
            } {
                oracle::clear_edges(m, &oracle::rect_edges(lo, hi), 1., 1., 0.0002)?;
                for p in [m.a, m.b] {
                    insist(
                        !(p[0] > lo[0] && p[0] < hi[0] && p[1] > lo[1] && p[1] < hi[1]),
                        "pocket_island_interior",
                    )?;
                }
            }
        }
    }
    insist(witnessed, "flat_undercut")
}
fn b2(c: &Case) -> Result<(), String> {
    let shape = if c.index % 8 >= 4 {
        4
    } else if c.index % 8 >= 2 {
        2
    } else {
        0
    };
    let req = request(c, shape, true);
    let depth = req.project.operations[0].params.depth;
    let tangent = (req.project.tools[0].tip_angle_deg / 2.).to_radians().tan();
    let edges: Vec<_> = req
        .project
        .segments
        .iter()
        .map(|s| ([s.start.x, s.start.y], [s.end.x, s.end.y]))
        .collect();
    let r = run(req, "vcarve")?;
    let moves = oracle::decode(&r.gcode)?;
    let mut witnessed = false;
    let allowance = 0.0002 + 0.0000317 + 2_f64.sqrt() * 0.00005 + tangent * 0.00005 + 1e-8;
    for m in &moves {
        let Some(cut) = oracle::below_surface(m) else {
            continue;
        };
        let m = &cut;
        witnessed |= (m.b[2] - depth).abs() <= 0.00005011;
        for p in [m.a, m.b] {
            // Generated outer contours are convex and CCW. Signed half planes
            // prevent a distance-only oracle from accepting paths outside stock.
            let outer_count = if shape == 4 { 3 } else { 4 };
            for (a, b) in &edges[..outer_count] {
                insist(
                    (b[0] - a[0]) * (p[1] - a[1]) - (b[1] - a[1]) * (p[0] - a[0]) >= -1e-8,
                    "vcarve_wrong_side",
                )?;
            }
            if shape == 2 {
                insist(
                    !(p[0] > 20. && p[0] < 24. && p[1] > 18. && p[1] < 22.),
                    "vcarve_island_interior",
                )?;
            }
        }
        oracle::clear_edges(
            m,
            &edges,
            (-m.a[2] * tangent - allowance).max(0.),
            (-m.b[2] * tangent - allowance).max(0.),
            0.,
        )?;
    }
    insist(witnessed, "vcarve_undercut")
}
fn c1(c: &Case) -> Result<(), String> {
    let req = request(c, if c.index % 3 == 0 { 2 } else { 0 }, c.index % 3 == 1);
    clear_pipeline_cache();
    let cold = run(req.clone(), "cold")?;
    ivac_core::profiling::reset();
    let warm = run(req.clone(), "warm")?;
    insist(
        ivac_core::profiling::take()
            .iter()
            .any(|r| r.name == "post.cps_cache_hit" && r.count == Some(1)),
        "cache_hit_witness",
    )?;
    insist(
        serde_json::to_value(&cold).unwrap() == serde_json::to_value(&warm).unwrap(),
        "cache_full_response",
    )?;
    let mut changed = req.clone();
    match c.index % 10 {
        0 => {
            for s in &mut changed.project.segments {
                s.start.x += 1.;
                s.end.x += 1.;
                if let Some(center) = &mut s.center {
                    center.x += 1.;
                }
            }
        }
        1 => changed.project.work_offset.x_mm = 1.,
        2 => changed.project.tools[0].diameter += 0.5,
        3 => changed.project.operations[0].params.depth *= 0.8,
        4 => changed.project.operations[0].params.fast_move_z += 1.,
        5 => changed.project.operations[0].params.step = Some(-0.3),
        6 => changed.project.tools[0].feed_rate += 1,
        7 => changed.project.stock.as_mut().unwrap().top_z_mm = 18.,
        8 => changed.project.machine.max_feed_mm_min = Some(500),
        _ => changed.project.operations[0].params.start_depth = 0.1,
    }
    save("mutation.request.json", &changed);
    let cached = run_pipeline(changed.clone(), |_, _, _| {})
        .map(|r| serde_json::to_value(r).unwrap())
        .map_err(|e| e.to_string());
    clear_pipeline_cache();
    let fresh = run_pipeline(changed, |_, _, _| {})
        .map(|r| serde_json::to_value(r).unwrap())
        .map_err(|e| e.to_string());
    insist(cached == fresh, "cache_mutation_equivalence")?;
    let restored = run(req, "restored")?;
    insist(
        serde_json::to_value(restored).unwrap() == serde_json::to_value(cold).unwrap(),
        "cache_restore",
    )
}
fn d1(c: &Case) -> Result<(), String> {
    let mut req = request(c, if c.index % 4 == 0 { 2 } else { 0 }, c.index % 4 == 1);
    if c.index % 4 == 3 {
        req.project.operations[0].kind = OpKind::Drill {
            cycle: DrillCycle::Peck {
                peck_step_mm: 0.25,
                dwell_sec: 0.,
            },
            chamfer_after_width_mm: None,
            pattern: None,
            spot_first: None,
        };
        req.project.segments = closed_circle(24., 24., 1.);
    }
    if c.index % 8 >= 4 {
        let mut second = req.project.operations[0].clone();
        second.id = 2;
        second.params.fast_move_z = 8.;
        req.project.operations.push(second);
    }
    let r = run(req, "decoded")?;
    let moves = oracle::decode(&r.gcode)?;
    insist(moves.len() == r.toolpath.len(), "decoded_move_count")?;
    for (m, s) in moves.iter().zip(&r.toolpath) {
        for (a, b) in [
            (m.a, [s.from.x, s.from.y, s.from.z]),
            (m.b, [s.to.x, s.to.y, s.to.z]),
        ] {
            insist(
                a.iter().zip(b).all(|(x, y)| (x - y).abs() <= 1e-8),
                "decoded_preview_xyz",
            )?;
        }
        insist(m.line == s.gcode_line, "decoded_line_map")?;
        insist(m.op_id == s.op_id, "decoded_operation_map")?;
        insist(
            m.rapid == matches!(s.kind, ivac_core::gcode::preview::MoveKind::Rapid),
            "decoded_motion_kind",
        )?;
        if !m.rapid {
            insist(m.feed > 0. && m.feed <= 1200.0001, "decoded_feed_limit")?;
        }
    }
    Ok(())
}
fn d2(c: &Case) -> Result<(), String> {
    let mut req = request(c, 0, false);
    let top = f64::from(c.index % 5) * 3.;
    req.project.stock.as_mut().unwrap().top_z_mm = top;
    req.project.operations[0].params.start_depth = top;
    req.project.operations[0].params.depth = top - 0.5;
    let valid = c.index % 2 == 0;
    req.project.operations[0].params.fast_move_z = if valid { top + 5. } else { top };
    if c.index % 3 == 0 {
        req.project.machine.work_area.z = 0.;
    }
    save("safety.request.json", &req);
    let result = run_pipeline(req, |_, _, _| {});
    if !valid {
        return insist(
            matches!(result, Err(PipelineError::UnsafeClearance(..))),
            "unsafe_clearance_refusal",
        );
    }
    let r = result.map_err(|e| e.to_string())?;
    for m in oracle::decode(&r.gcode)? {
        if m.rapid {
            let vertical = m.a[0] == m.b[0] && m.a[1] == m.b[1];
            insist(
                (vertical && m.b[2] >= m.a[2] && m.b[2] > top) || (m.a[2] > top && m.b[2] > top),
                "rapid_stock_sweep",
            )?;
        }
    }
    Ok(())
}
#[test]
#[ignore = "Run via scripts/test1/run.py: process watchdog, seeds and evidence required"]
fn test1_case() {
    let c: Case =
        serde_json::from_slice(&std::fs::read(std::env::var("TEST1_CASE").unwrap()).unwrap())
            .unwrap();
    let result = match c.property.as_str() {
        "A1" => a1(&c),
        "A2" => a2(&c),
        "B1" => b1(&c),
        "B2" => b2(&c),
        "C1" => c1(&c),
        "D1" => d1(&c),
        "D2" => d2(&c),
        _ => Err("unknown_property".into()),
    };
    save("result.json", &json!({"case":c,"result":result}));
    assert!(result.is_ok(), "{}", result.unwrap_err());
}
