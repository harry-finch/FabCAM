// Test fixtures assert exact literal values and bounded indices.
#![allow(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]
mod common;
use common::{build_project, endmill_tool, op_single_pass, run, stock_at_origin};
use ivac_core::gcode::preview::MoveKind;
use ivac_core::geometry::{Point2, Segment};
use ivac_core::project::{ContourParams, OpKind, PocketParams, PocketStrategy};
use ivac_core::schema::PostProcessorKind;

fn square(lo: f64, hi: f64) -> Vec<Segment> {
    let p = [(lo, lo), (hi, lo), (hi, hi), (lo, hi)];
    (0..4)
        .map(|i| {
            Segment::line(
                Point2::new(p[i].0, p[i].1),
                Point2::new(p[(i + 1) % 4].0, p[(i + 1) % 4].1),
                "nested",
                7,
            )
        })
        .collect()
}
#[test]
fn nested_auto_pocket_cuts_even_depth_region_and_preserves_odd_depth_island() {
    let segments = [square(0., 60.), square(15., 45.), square(23., 37.)].concat();
    let op = op_single_pass(
        1,
        "nested",
        OpKind::Pocket {
            strategy: PocketStrategy::Cascade,
            contour: ContourParams::default(),
            pocket: PocketParams::default(),
        },
        1,
        -1.,
    );
    let output = run(
        build_project(
            &stock_at_origin(60., 60., 10.),
            vec![endmill_tool(1, 2.)],
            op,
            segments,
        ),
        PostProcessorKind::Linuxcnc,
    );
    let mut inner_cut = false;
    let mut outer_cut = false;
    for s in output
        .toolpath
        .iter()
        .filter(|s| matches!(s.kind, MoveKind::Cut) && s.to.z < -0.9)
    {
        // Independent sampling of actual generated moves, not source regions.
        for i in 0..=20 {
            let t = f64::from(i) / 20.;
            let x = s.from.x + (s.to.x - s.from.x) * t;
            let y = s.from.y + (s.to.y - s.from.y) * t;
            if x > 24. && x < 36. && y > 24. && y < 36. {
                inner_cut = true;
            }
            if x > 1. && x < 14. && y > 1. && y < 14. {
                outer_cut = true;
            }
            let in_island = x > 15. && x < 45. && y > 15. && y < 45.;
            let in_inner =
                (24. - 1e-4..=36. + 1e-4).contains(&x) && (24. - 1e-4..=36. + 1e-4).contains(&y);
            assert!(
                !in_island || in_inner,
                "tool center entered retained island: {x},{y}"
            );
        }
    }
    assert!(outer_cut, "outer region must be cut");
    assert!(
        inner_cut,
        "depth-two region shown by Auto preview must also be cut"
    );
}

#[test]
fn nested_pocket_matrix_preserves_cutter_clearance_and_every_even_region() {
    use ivac_core::project::{OpSource, SourceCombine};
    for strategy in [
        PocketStrategy::Cascade,
        PocketStrategy::Zigzag { angle_deg: 0. },
        PocketStrategy::Spiral,
    ] {
        for selected in [vec![0, 1, 2], vec![0, 1, 2, 3], vec![0, 2], vec![1, 2, 3]] {
            let bounds = [(0., 60.), (15., 45.), (23., 37.), (28., 32.)];
            let segments = bounds.iter().flat_map(|&(lo, hi)| square(lo, hi)).collect();
            let mut op = op_single_pass(
                1,
                "nested matrix",
                OpKind::Pocket {
                    strategy,
                    contour: ContourParams::default(),
                    pocket: PocketParams::default(),
                },
                1,
                -1.,
            );
            op.source = OpSource::Objects {
                ids: selected.iter().map(|&i| i as u32 + 1).collect(),
                combine: SourceCombine::Auto,
            };
            let output = run(
                build_project(
                    &stock_at_origin(60., 60., 10.),
                    vec![endmill_tool(1, 2.)],
                    op,
                    segments,
                ),
                PostProcessorKind::Linuxcnc,
            );
            let mut visited = std::collections::HashSet::new();
            for s in output
                .toolpath
                .iter()
                .filter(|s| matches!(s.kind, MoveKind::Cut) && s.from.z < -0.9 && s.to.z < -0.9)
            {
                let n =
                    (((s.to.x - s.from.x).hypot(s.to.y - s.from.y) / 0.2).ceil() as usize).max(1);
                for i in 0..=n {
                    let t = i as f64 / n as f64;
                    let (x, y) = (
                        s.from.x + (s.to.x - s.from.x) * t,
                        s.from.y + (s.to.y - s.from.y) * t,
                    );
                    let containing: Vec<_> = selected
                        .iter()
                        .filter(|&&j| {
                            let (lo, hi) = bounds[j];
                            x > lo && x < hi && y > lo && y < hi
                        })
                        .collect();
                    assert_eq!(
                        containing.len() % 2,
                        1,
                        "{strategy:?} {selected:?}: overcut at {x},{y}"
                    );
                    visited.insert(**containing.last().unwrap());
                    for &j in &selected {
                        let (lo, hi) = bounds[j];
                        let distance = if x >= lo && x <= hi && y >= lo && y <= hi {
                            (x - lo).min(hi - x).min(y - lo).min(hi - y)
                        } else {
                            (lo - x)
                                .max(0.)
                                .max(x - hi)
                                .hypot((lo - y).max(0.).max(y - hi))
                        };
                        assert!(distance>=1.-0.002,"{strategy:?} {selected:?}: cutter crosses boundary {j} at {x},{y} (distance {distance})");
                    }
                }
            }
            let expected: std::collections::HashSet<_> =
                selected.iter().step_by(2).copied().collect();
            assert_eq!(
                visited, expected,
                "{strategy:?} {selected:?}: omitted region"
            );
        }
    }
}

#[cfg(feature = "cps")]
#[test]
fn deep_pocket_opensbp_decodes_to_the_same_preview_moves() {
    use ivac_core::pipeline::{run_pipeline, CpsPostSelection, CpsPostSource, PipelineRequest};
    let op = op_single_pass(
        1,
        "deep pocket",
        OpKind::Pocket {
            strategy: PocketStrategy::Cascade,
            contour: ContourParams::default(),
            pocket: PocketParams::default(),
        },
        1,
        -1.,
    );
    let p = build_project(
        &stock_at_origin(60., 60., 10.),
        vec![endmill_tool(1, 2.)],
        op,
        [square(0., 60.), square(15., 45.), square(23., 37.)].concat(),
    );
    let out = run_pipeline(
        crate::fixture_support::request(PipelineRequest {
            project: p,
            post_processor: Some(PostProcessorKind::Cps),
            cps_post: Some(CpsPostSelection {
                source: CpsPostSource::Bundled {
                    id: "shopbot".into(),
                },
                properties: std::collections::BTreeMap::default(),
            }),
        }),
        |_, _, _| {},
    )
    .unwrap();
    // Independent text decoder: no preview parser / planner helpers.
    let mut pos: [Option<f64>; 3] = [None, None, None];
    let mut decoded = vec![];
    for (line, text) in out.gcode.lines().enumerate() {
        let mut fields = text.split(',');
        let cmd = fields.next().unwrap_or("");
        if !["M3", "J2", "JZ"].contains(&cmd) {
            continue;
        }
        let v: Vec<f64> = fields.map(|s| s.parse().unwrap()).collect();
        let old = pos;
        match cmd {
            "M3" => pos = [Some(v[0]), Some(v[1]), Some(v[2])],
            "J2" => {
                pos[0] = Some(v[0]);
                pos[1] = Some(v[1]);
            }
            "JZ" => pos[2] = Some(v[0]),
            _ => unreachable!(),
        }
        if let ([Some(x), Some(y), Some(z)], [Some(a), Some(b), Some(c)]) = (old, pos) {
            if (x - a).abs() + (y - b).abs() + (z - c).abs() < 1e-10 {
                continue;
            }
            decoded.push(([x, y, z], [a, b, c], cmd == "M3", line + 1));
        }
    }
    assert_eq!(decoded.len(), out.toolpath.len());
    for ((a, b, cut, line), s) in decoded.iter().zip(&out.toolpath) {
        assert_eq!(*line, s.gcode_line as usize);
        assert_eq!(*cut, !matches!(s.kind, MoveKind::Rapid));
        for (actual, expected) in a
            .iter()
            .zip([s.from.x, s.from.y, s.from.z])
            .chain(b.iter().zip([s.to.x, s.to.y, s.to.z]))
        {
            assert!((actual - expected).abs() < 1e-9);
        }
    }
}

#[path = "support/explicit.rs"]
mod fixture_support;
