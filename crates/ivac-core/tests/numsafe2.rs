//! NUMSAFE2 regressions: the 1000 depth-pass budget (`cam::MAX_DEPTH_PASSES`)
//! and the explicit depth-per-pass contract. Over-budget and invalid
//! requests are refused quickly, before any pass is emitted; valid schedules
//! are unchanged. No machine execution.
// Exact literal depths; Z levels are bucketed to 0.1 µm integers and pass
// counts are small whole numbers, so these casts are exact by construction.
#![allow(
    clippy::float_cmp,
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss
)]
mod common;
use common::{
    build_project, closed_circle, closed_rectangle, endmill_tool, op_single_pass, points,
    stock_at_origin, vbit_tool,
};
use ivac_core::cam::MAX_DEPTH_PASSES;
use ivac_core::pipeline::{
    run_pipeline, stream_gcode_to_writer, PipelineError, PipelineRequest, PipelineResponse,
    PostProcessorKind,
};
use ivac_core::project::{
    ContourParams, DrillCycle, OpKind, PocketParams, PocketStrategy, ProfileParams, Project,
    ToolOffset, VCarveParams,
};
use ivac_core::{Point2, Segment};
use std::collections::BTreeSet;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

const BUDGET: usize = 1000;
const _: () = assert!(MAX_DEPTH_PASSES == BUDGET);

fn profile(depth: f64, step: Option<f64>) -> Project {
    let mut op = op_single_pass(
        1,
        "numsafe2",
        OpKind::Profile {
            offset: ToolOffset::Outside,
            contour: ContourParams::default(),
            profile: ProfileParams::default(),
        },
        1,
        depth,
    );
    op.params.step = step;
    op.params.fast_move_z = 5.;
    build_project(
        &stock_at_origin(100., 100., 18.),
        vec![endmill_tool(1, 6.)],
        op,
        closed_rectangle(20., 20., 40., 40.),
    )
}

fn generate(p: Project) -> Result<PipelineResponse, PipelineError> {
    run_pipeline(
        crate::fixture_support::request(PipelineRequest {
            project: p,
            post_processor: Some(PostProcessorKind::Linuxcnc),
            cps_post: None,
        }),
        |_, _, _| {},
    )
}

/// Distinct Z levels of the cutting (non-plunge, non-rapid) toolpath.
fn cut_levels(r: &PipelineResponse) -> Vec<f64> {
    let set: BTreeSet<i64> = r
        .toolpath
        .iter()
        .filter(|s| matches!(format!("{:?}", s.kind).as_str(), "Cut" | "Arc"))
        .map(|s| (s.to.z * 1e4).round() as i64)
        .collect();
    set.into_iter().rev().map(|z| z as f64 / 1e4).collect()
}

fn assert_pass_limit(r: Result<PipelineResponse, PipelineError>, what: &str) {
    let err = r.expect_err(what);
    assert!(
        matches!(&err, PipelineError::PassLimit(1, d) if d.contains("1000")),
        "{what}: {err:?}"
    );
    let s = err.to_structured(None).expect("structured");
    assert!(s.message.contains("No program generated"), "{s:?}");
}

#[test]
fn shallow_and_deep_multi_pass_schedules_land_exactly() {
    // Shallow: 10 passes of 1 mm, last pass exactly at the final depth.
    let r = generate(profile(-10., Some(-1.))).unwrap();
    let levels = cut_levels(&r);
    assert_eq!(levels.len(), 10);
    assert_eq!(*levels.last().unwrap(), -10.);
    // Non-dividing step: 3 mm at 0.7 → 0.7, 1.4, 2.1, 2.8, 3.0.
    let r = generate(profile(-3., Some(-0.7))).unwrap();
    assert_eq!(cut_levels(&r), vec![-0.7, -1.4, -2.1, -2.8, -3.]);
    // Step larger than the total depth: one pass at the final depth.
    let r = generate(profile(-2., Some(-5.))).unwrap();
    assert_eq!(cut_levels(&r), vec![-2.]);
}

#[test]
fn pass_budget_boundary_is_exact() {
    // 999 and 1000 passes are generated in full; 1001 is refused.
    for (depth, ok) in [(-999., true), (-1000., true), (-1001., false)] {
        let r = generate(profile(depth, Some(-1.)));
        if ok {
            let levels = cut_levels(&r.unwrap());
            assert_eq!(levels.len(), (-depth) as usize, "depth {depth}");
            assert_eq!(*levels.last().unwrap(), depth);
        } else {
            assert_pass_limit(r, "1001 passes");
        }
    }
    // The finish pre-pass counts: 998 + pre-finish + final = 1000 passes.
    let mut p = profile(-999., Some(-1.));
    p.operations[0].params.finish_step = Some(-0.5);
    let levels = cut_levels(&generate(p).unwrap());
    assert_eq!(levels.len(), 1000);
    assert_eq!(&levels[997..], &[-998., -998.5, -999.]);
    let mut p = profile(-1000., Some(-1.));
    p.operations[0].params.finish_step = Some(-0.5);
    assert_pass_limit(generate(p), "1000 + pre-finish");
    // An explicit depth list is a schedule too.
    let mut p = profile(-1., Some(-1.));
    p.operations[0].params.depth_list = (1..=BUDGET + 1).map(|i| -(i as f64) * 0.01).collect();
    assert_pass_limit(generate(p), "1001-entry depth list");
}

#[test]
fn tiny_steps_and_huge_depths_are_refused_quickly_without_emitting() {
    for (depth, step) in [(-1e9, -1.), (-10., -1e-6), (-1e300, -1.), (-1., -0.0009)] {
        let t = Instant::now();
        assert_pass_limit(generate(profile(depth, Some(step))), "extreme");
        assert!(t.elapsed() < Duration::from_secs(5), "{depth}/{step}");
    }
    // A very small but sufficient step stays valid: 1 mm at 0.001 mm.
    let r = generate(profile(-1., Some(-0.001))).unwrap();
    assert_eq!(cut_levels(&r).len(), 1000);
}

#[test]
fn large_legitimate_job_completes() {
    // 1000 passes on a circle (50 k lines): well inside resources.
    let mut p = profile(-100., Some(-0.1));
    p.segments = closed_circle(50., 50., 30.);
    let t = Instant::now();
    let r = generate(p).unwrap();
    assert_eq!(cut_levels(&r).len(), 1000);
    assert!(t.elapsed() < Duration::from_secs(30));
}

#[test]
fn explicit_invalid_or_missing_step_refuses_without_tool_inheritance() {
    for step in [
        0.,
        -0.,
        1.,
        f64::NAN,
        f64::INFINITY,
        f64::NEG_INFINITY,
        -1e-12,
    ] {
        let err = generate(profile(-10., Some(step))).expect_err("invalid step");
        assert!(
            matches!(&err, PipelineError::AppliedCutting(1, d) if d.contains("Depth per pass must be finite and positive")),
            "{step}: {err:?}"
        );
    }
    // A tool default cannot satisfy a missing authored depth step…
    let mut p = profile(-10., None);
    p.tools[0].default_step = Some(0.);
    let err = generate(p).unwrap_err();
    assert!(
        matches!(&err, PipelineError::AppliedCutting(1, d) if d.contains("Set explicit depth per pass")),
        "{err:?}"
    );
    // …but unused when the op supplies its own valid step.
    let mut p = profile(-10., Some(-2.));
    p.tools[0].default_step = Some(0.);
    assert_eq!(cut_levels(&generate(p).unwrap()).len(), 5);
    // Omitted everywhere refuses; there is no whole-depth fallback.
    let err = generate(profile(-10., None)).unwrap_err();
    assert!(
        err.to_string().contains("Set explicit depth per pass"),
        "{err}"
    );
    // A library default cannot supply missing operation intent.
    let mut p = profile(-10., None);
    p.tools[0].default_step = Some(-2.5);
    let err = generate(p).unwrap_err();
    assert!(
        err.to_string().contains("Set explicit depth per pass"),
        "{err}"
    );
}

#[test]
fn refused_op_is_never_cached_and_regenerates_after_correction() {
    let valid = generate(profile(-5., Some(-1.))).unwrap().gcode;
    for _ in 0..2 {
        assert_pass_limit(generate(profile(-5000., Some(-1.))), "refused twice");
        assert!(matches!(
            generate(profile(-5., Some(0.))),
            Err(PipelineError::AppliedCutting(1, ref d)) if d.contains("Depth per pass")
        ));
    }
    assert_eq!(generate(profile(-5., Some(-1.))).unwrap().gcode, valid);
}

#[derive(Clone)]
struct Shared(Arc<Mutex<Vec<u8>>>);
impl std::io::Write for Shared {
    fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(b);
        Ok(b.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[test]
fn streamed_refusal_emits_no_cutting_move() {
    let buf = Arc::new(Mutex::new(Vec::new()));
    let r = stream_gcode_to_writer(
        crate::fixture_support::request(PipelineRequest {
            project: profile(-5000., Some(-1.)),
            post_processor: Some(PostProcessorKind::Grbl),
            cps_post: None,
        }),
        Box::new(Shared(buf.clone())),
    );
    assert!(r.is_err());
    let text = String::from_utf8(buf.lock().unwrap().clone()).unwrap();
    assert!(!text.lines().any(|l| l.starts_with("G1 ")), "{text}");
}

#[test]
fn pocket_and_drill_share_the_budget() {
    let mut p = profile(-1001., Some(-1.));
    p.operations[0].kind = OpKind::Pocket {
        strategy: PocketStrategy::Cascade,
        contour: ContourParams::default(),
        pocket: PocketParams::default(),
    };
    assert_pass_limit(generate(p.clone()), "pocket 1001");
    p.operations[0].params.depth = -3.;
    assert!(generate(p).is_ok());

    // Drill pecks per hole = ceil((R − Z) / Q), R = 0.5 mm above stock top.
    let drill = |depth: f64, q: f64| {
        let mut p = profile(depth, Some(-1.));
        p.segments = points(&[(50., 50.)]);
        p.operations[0].kind = OpKind::Drill {
            cycle: DrillCycle::Peck {
                peck_step_mm: q,
                dwell_sec: 0.,
            },
            chamfer_after_width_mm: None,
            pattern: None,
            spot_first: None,
        };
        p
    };
    assert!(generate(drill(-999.5, 1.)).is_ok(), "1000 pecks");
    assert_pass_limit(generate(drill(-1000.5, 1.)), "1001 pecks");
    assert_pass_limit(generate(drill(-10., 1e-6)), "micro peck");
    // A drill uses its explicit peck cycle and does not require a contour step.
    let mut p = drill(-5., 1.);
    p.operations[0].params.step = None;
    assert!(generate(p).is_ok());
}

#[test]
fn vcarve_depth_levels_share_the_budget() {
    let p = [(0., 0.), (20., 0.), (20., 10.), (0., 10.)];
    let segments: Vec<Segment> = (0..4)
        .map(|i| {
            Segment::line(
                Point2::new(p[i].0, p[i].1),
                Point2::new(p[(i + 1) % 4].0, p[(i + 1) % 4].1),
                "0",
                7,
            )
        })
        .collect();
    let project = |step: f64| {
        let mut op = op_single_pass(
            1,
            "vcarve",
            OpKind::VCarve {
                carve: VCarveParams::default(),
            },
            1,
            -1.,
        );
        op.params.step = Some(step);
        build_project(
            &stock_at_origin(30., 20., 10.),
            vec![vbit_tool(1, 6., 90., 0.)],
            op,
            segments.clone(),
        )
    };
    assert!(generate(project(-0.2)).is_ok());
    assert_pass_limit(generate(project(-0.0009)), "V-Carve 1112 levels");
    assert!(matches!(
        generate(project(f64::NAN)),
        Err(PipelineError::AppliedCutting(1, ref d)) if d.contains("Depth per pass")
    ));
}

#[path = "support/explicit.rs"]
mod fixture_support;
