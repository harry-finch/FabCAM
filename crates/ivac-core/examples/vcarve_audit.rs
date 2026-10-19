#[path = "../tests/common/mod.rs"]
mod common;
use common::{build_project, op_single_pass, stock_at_origin, vbit_tool};
use ivac_core::pipeline::{
    run_pipeline, CpsPostSelection, CpsPostSource, PipelineRequest, PostProcessorKind,
};
use ivac_core::project::{OpKind, VCarveParams};
use ivac_core::{Point2, Segment};
fn main() {
    let pts = [(0., 0.), (20., 0.), (20., 10.), (0., 10.)];
    let segments = (0..4)
        .map(|i| {
            Segment::line(
                Point2::new(pts[i].0, pts[i].1),
                Point2::new(pts[(i + 1) % 4].0, pts[(i + 1) % 4].1),
                "0",
                7,
            )
        })
        .collect::<Vec<_>>();
    for medial in [false, true] {
        let mut op = op_single_pass(
            1,
            "AUDIT SOFTWARE ONLY",
            OpKind::VCarve {
                carve: VCarveParams {
                    full_medial_axis: medial,
                    ..Default::default()
                },
            },
            1,
            -1.,
        );
        op.params.step = Some(-0.2);
        let p = build_project(
            &stock_at_origin(30., 20., 10.),
            vec![vbit_tool(1, 6., 90., 0.)],
            op,
            segments.clone(),
        );
        let r = run_pipeline(
            PipelineRequest {
                project: p.clone(),
                post_processor: None,
                cps_post: None,
            },
            |_, _, _| {},
        )
        .unwrap();
        println!(
            "mode medial={medial}, segments={}, min_z={}, warnings={:?}",
            r.toolpath.len(),
            r.toolpath.iter().map(|s| s.to.z).fold(0., f64::min),
            r.warnings.iter().map(|w| &w.kind).collect::<Vec<_>>()
        );
        for s in r.toolpath.iter().filter(|s| s.to.z < 0.).take(5) {
            println!("cut {:?} -> {:?}, {:?}", s.from, s.to, s.kind);
        }
        let sbp = run_pipeline(
            PipelineRequest {
                project: p,
                post_processor: Some(PostProcessorKind::Cps),
                cps_post: Some(CpsPostSelection {
                    source: CpsPostSource::Bundled {
                        id: "shopbot".into(),
                    },
                    properties: std::collections::BTreeMap::default(),
                }),
            },
            |_, _, _| {},
        );
        println!(
            "ShopBot={:?}",
            sbp.map(|r| r.toolpath.len()).map_err(|e| e.to_string())
        );
    }
    let axis = [(0., 0., -1., 1.), (1., 0., -1., 1.)];
    let paths = ivac_core::cam::vcarve_emit::ratchet_emit(&axis, 0.5);
    println!(
        "short chain target=-1, dpp=0.5, actual_min={}, paths={}",
        paths.iter().flatten().map(|p| p.2).fold(0., f64::min),
        paths.len()
    );
    let axis = [
        (0., 0., -1., 1.),
        (1., 0., -0.01, 0.01),
        (10., 0., -0.01, 0.01),
    ];
    let paths = ivac_core::cam::vcarve_emit::ratchet_emit(&axis, 0.5);
    let gouges = paths
        .iter()
        .flatten()
        .filter(|p| p.0 > 1. && p.2 < -0.010_001)
        .collect::<Vec<_>>();
    println!("ramp in shallow target zone (x>1, target=-0.01): {gouges:?}");
}
