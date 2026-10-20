//! NUMSAFE regressions: values that must never reach a machine program
//! (non-finite numbers, zero or missing cutting feeds), bounded STL surface
//! grids, and unchanged output for valid programs. No machine execution.
#![allow(clippy::float_cmp)]
mod common;
use common::{build_project, closed_rectangle, endmill_tool, op_single_pass, stock_at_origin};
use ivac_core::cam::surface::{SurfaceField, MAX_SURFACE_CELLS};
use ivac_core::pipeline::{
    run_pipeline, stream_gcode_to_writer, PipelineError, PipelineRequest, PipelineResponse,
    PostProcessorKind, StreamGcodeError,
};
use ivac_core::project::{ContourParams, OpKind, ProfileParams, Project, ToolOffset};
use ivac_core::sim::stl::StlError;
use std::sync::{Arc, Mutex};

// A 4000 × 4000 grid (the largest square `max_dim`) stays inside the budget.
const _: () = assert!(4_000 * 4_000 <= MAX_SURFACE_CELLS);

const MILLING_POSTS: [PostProcessorKind; 2] =
    [PostProcessorKind::Linuxcnc, PostProcessorKind::Grbl];

fn project() -> Project {
    let stock = stock_at_origin(100., 100., 18.);
    let mut op = op_single_pass(
        1,
        "numsafe",
        OpKind::Profile {
            offset: ToolOffset::Outside,
            contour: ContourParams::default(),
            profile: ProfileParams::default(),
        },
        1,
        -2.,
    );
    op.params.step = Some(-1.);
    op.params.fast_move_z = 5.;
    build_project(
        &stock,
        vec![endmill_tool(1, 6.)],
        op,
        closed_rectangle(20., 20., 40., 40.),
    )
}

fn generate(p: Project, post: PostProcessorKind) -> Result<PipelineResponse, PipelineError> {
    run_pipeline(
        PipelineRequest {
            project: p,
            post_processor: Some(post),
            cps_post: None,
        },
        |_, _, _| {},
    )
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

fn stream(p: Project, post: PostProcessorKind) -> (Result<(), StreamGcodeError>, String) {
    let buf = Arc::new(Mutex::new(Vec::new()));
    let r = stream_gcode_to_writer(
        PipelineRequest {
            project: p,
            post_processor: Some(post),
            cps_post: None,
        },
        Box::new(Shared(buf.clone())),
    )
    .map(|_| ());
    let text = String::from_utf8(buf.lock().unwrap().clone()).unwrap();
    (r, text)
}

/// Every numeric word of every program line parses as a finite number.
fn assert_all_words_finite(gcode: &str) {
    for line in gcode
        .lines()
        .filter(|l| !l.starts_with('(') && !l.starts_with(';'))
    {
        for word in line.split_whitespace() {
            assert!(!word.contains("NaN") && !word.contains("inf"), "{line}");
            if let Some(v) = word.get(1..).and_then(|n| n.parse::<f64>().ok()) {
                assert!(v.is_finite(), "{line}");
            }
        }
    }
}

#[test]
fn valid_program_is_finite_fed_and_stream_identical() {
    for post in MILLING_POSTS {
        let out = generate(project(), post).expect("valid program");
        assert_all_words_finite(&out.gcode);
        // The feed contract: the first cutting block runs under an explicit F.
        let cut = out.gcode.find("\nG1 ").expect("cuts");
        assert!(out.gcode[..cut].contains("\nF"), "{}", out.gcode);
        let (r, streamed) = stream(project(), post);
        r.expect("valid stream");
        assert_eq!(streamed.trim_end(), out.gcode.trim_end(), "{post:?}");
    }
}

/// Before NUMSAFE a NaN clearance without a stock model formatted as `Z0`:
/// a rapid down to the work surface. It is now refused for both milling
/// posts, buffered and streamed, and the stream never writes a faulty line.
#[test]
fn nan_clearance_is_refused_never_zeroed() {
    for post in MILLING_POSTS {
        let mut p = project();
        p.stock = None;
        p.operations[0].params.fast_move_z = f64::NAN;
        let err = generate(p.clone(), post).expect_err("NaN clearance must not generate");
        assert!(
            matches!(&err, PipelineError::RefusedOutput(0, d) if d.contains("non-finite")),
            "{post:?}: {err:?}"
        );
        let (r, streamed) = stream(p, post);
        assert!(r.is_err(), "{post:?}");
        assert!(!streamed.contains("Z0\n"), "{streamed}");
        assert_all_words_finite(&streamed);
    }
}

#[test]
fn non_finite_parameters_are_refused_before_planning() {
    type Edit = fn(&mut Project);
    // A NaN `step` is not listed: the existing policy already treats any
    // non-finite or zero step as unspecified (`step_unspecified` warning).
    let edits: [(&str, Edit); 4] = [
        // Used to loop forever in the Z schedule (process killed, OOM).
        ("depth", |p| p.operations[0].params.depth = f64::NAN),
        ("start depth", |p| {
            p.operations[0].params.start_depth = f64::INFINITY;
        }),
        // Used to offset silently by a near-zero radius, without a warning.
        ("tool diameter", |p| p.tools[0].diameter = f64::NAN),
        ("depth list entry", |p| {
            p.operations[0].params.depth_list = vec![-1., f64::NEG_INFINITY];
        }),
    ];
    for (name, edit) in edits {
        let mut p = project();
        edit(&mut p);
        let err = generate(p, PostProcessorKind::Linuxcnc).expect_err(name);
        assert!(
            matches!(&err, PipelineError::NonFiniteParameter(1, d) if d.starts_with(name)),
            "{name}: {err:?}"
        );
        let structured = err.to_structured(None).expect("structured");
        assert!(
            structured.message.contains("No program generated"),
            "{structured:?}"
        );
    }
}

/// Before NUMSAFE a zero cutting feed was skipped and the cut inherited the
/// modal plunge `F` (a critical warning only, bypassable by preference and
/// absent on CLI/server). Now it is refused on every path, and a refused op
/// is never served from the op cache on a second attempt.
#[test]
fn zero_cutting_or_plunge_feed_is_refused() {
    type Edit = fn(&mut Project);
    let edits: [(&str, Edit); 3] = [
        ("tool feed", |p| p.tools[0].feed_rate = 0),
        ("tool plunge", |p| p.tools[0].plunge_rate = 0),
        ("op feed override", |p| {
            p.operations[0].params.feed_rate_override = Some(0);
        }),
    ];
    for post in MILLING_POSTS {
        for (name, edit) in edits {
            let mut p = project();
            edit(&mut p);
            for attempt in 0..2 {
                let err = generate(p.clone(), post).expect_err(name);
                assert!(
                    matches!(&err, PipelineError::RefusedOutput(1, d) if d.contains("zero feed")),
                    "{post:?} {name} attempt {attempt}: {err:?}"
                );
            }
            let (r, streamed) = stream(p, post);
            assert!(r.is_err(), "{post:?} {name}");
            assert!(
                !streamed.lines().any(|l| l.starts_with("G1 ")),
                "{post:?} {name}: {streamed}"
            );
        }
    }
}

#[test]
fn stl_grid_is_bounded_before_allocation() {
    let quad = |s: f64| {
        format!(
            "solid t\nfacet normal 0 0 1\nouter loop\nvertex 0 0 1\nvertex {s} 0 1\nvertex {s} {s} 1\nendloop\nendfacet\n\
             facet normal 0 0 1\nouter loop\nvertex 0 0 1\nvertex {s} {s} 1\nvertex 0 {s} 1\nendloop\nendfacet\nendsolid t\n"
        )
    };
    let stl = quad(10.);
    // The UI budget (256) still rasterizes exactly as before.
    let f = SurfaceField::from_stl_capped(stl.as_bytes(), 256)
        .unwrap()
        .unwrap();
    assert_eq!((f.cols, f.rows), (256, 256));
    // u32::MAX used to wrap `cols * rows` on 32-bit WASM (len 1, then an
    // out-of-bounds panic = trap) and request ~1.8e19 cells natively.
    // Refused before any allocation; just over the budget is refused too.
    for max_dim in [u32::MAX, 4_001] {
        let err = SurfaceField::from_stl_capped(stl.as_bytes(), max_dim).unwrap_err();
        assert!(
            matches!(&err, StlError::Grid(m) if m.contains("cell limit")),
            "{err}"
        );
    }
    for cell in [0., -1., f64::NAN, f64::INFINITY, 1e-9] {
        assert!(
            matches!(
                SurfaceField::from_stl(stl.as_bytes(), cell),
                Err(StlError::Grid(_))
            ),
            "cell {cell}"
        );
    }
    // Non-finite vertices are refused instead of sizing or sampling the grid.
    for bad in ["nan", "inf", "-inf"] {
        let stl = quad(10.).replacen("vertex 0 0 1", &format!("vertex {bad} 0 1"), 1);
        assert_eq!(
            SurfaceField::from_stl_capped(stl.as_bytes(), 64).unwrap_err(),
            StlError::NonFinite(0)
        );
    }
}

/// `ShopBot` is audit-only in NUMSAFE: its `OpenSBP` post already refuses zero
/// or non-finite feeds and coordinates. Pin that it still fails closed.
#[cfg(feature = "cps")]
#[test]
fn shopbot_post_still_fails_closed_on_invalid_feed_and_clearance() {
    use ivac_core::pipeline::{CpsPostSelection, CpsPostSource};
    let shopbot = |p: Project| PipelineRequest {
        project: p,
        post_processor: Some(PostProcessorKind::Cps),
        cps_post: Some(CpsPostSelection {
            source: CpsPostSource::Bundled {
                id: "shopbot".into(),
            },
            properties: std::collections::BTreeMap::default(),
        }),
    };
    let valid = run_pipeline(shopbot(project()), |_, _, _| {}).expect("valid ShopBot program");
    assert!(valid.gcode.contains("MS,"), "{}", valid.gcode);
    let mut zero_feed = project();
    zero_feed.tools[0].feed_rate = 0;
    let err = run_pipeline(shopbot(zero_feed), |_, _, _| {}).unwrap_err();
    assert!(err.to_string().contains("feed"), "{err}");
    let mut nan_clearance = project();
    nan_clearance.stock = None;
    nan_clearance.operations[0].params.fast_move_z = f64::NAN;
    assert!(run_pipeline(shopbot(nan_clearance), |_, _, _| {}).is_err());
}

/// The CPS route records into IR instead of formatting text, so it needs
/// the same guard: the bundled `grbl.cps` rendered a zero feed as `F0`.
#[cfg(feature = "cps")]
#[test]
fn cps_route_refuses_zero_feed_before_any_post_runs() {
    use ivac_core::pipeline::{CpsPostSelection, CpsPostSource};
    for id in ["grbl", "shopbot"] {
        let request = |p: Project| PipelineRequest {
            project: p,
            post_processor: Some(PostProcessorKind::Cps),
            cps_post: Some(CpsPostSelection {
                source: CpsPostSource::Bundled { id: id.into() },
                properties: std::collections::BTreeMap::default(),
            }),
        };
        assert!(
            run_pipeline(request(project()), |_, _, _| {}).is_ok(),
            "{id}"
        );
        let mut p = project();
        p.operations[0].params.feed_rate_override = Some(0);
        let err = run_pipeline(request(p), |_, _, _| {}).unwrap_err();
        assert!(
            matches!(&err, PipelineError::RefusedOutput(1, d) if d.contains("zero feed")),
            "{id}: {err:?}"
        );
    }
}
