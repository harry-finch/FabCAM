//! Diagnostic observations only: no install, machine execution or safety verdict.
//! Run with `cargo run --offline -p ivac-core --features cps --example consolidation_probe`.
#[path = "../tests/common/mod.rs"]
mod common;

fn main() {
    use ivac_core::pipeline::{
        CpsPostSelection, CpsPostSource, PipelineRequest, PostProcessorKind,
    };
    use ivac_core::project::{ContourParams, OpKind, ProfileParams, ToolOffset};
    let dxf =
        include_bytes!("../../../docs/releases/consolidation-2026-10-18-evidence/block-hatch.dxf");
    let imported = ivac_core::input::import_bytes(
        "block-hatch.dxf",
        dxf,
        &ivac_core::input::ImportOptions::default(),
    );
    let mut cases = Vec::new();
    for (name, top, origin_z, start, depth, fast_z) in [
        ("top_zero", 0., 0., 0., -1., 5.),
        ("top_zero_rapid_minus5", 0., 0., 0., -1., -5.),
        ("top18_rapid5", 18., 0., 18., 17., 5.),
        ("top18_rapid5_unknown_workarea", 18., 0., 18., 17., 5.),
        ("top18_rapid23", 18., 0., 18., 17., 23.),
        ("translated_z_refusal", 18., 18., 0., -1., 5.),
    ] {
        let mut stock = common::stock_at_origin(100., 100., 18.);
        stock.top_z_mm = top;
        let mut op = common::op_single_pass(
            1,
            name,
            OpKind::Profile {
                offset: ToolOffset::Outside,
                contour: ContourParams::default(),
                profile: ProfileParams::default(),
            },
            1,
            depth,
        );
        op.params.start_depth = start;
        op.params.step = Some(-1.);
        op.params.fast_move_z = fast_z;
        let mut project = common::build_project(
            &stock,
            vec![common::endmill_tool(1, 6.)],
            op,
            common::closed_rectangle(20., 20., 40., 40.),
        );
        project.work_offset.z_mm = origin_z;
        if name.ends_with("unknown_workarea") {
            project.machine.work_area.z = 0.;
        }
        let request = PipelineRequest {
            project,
            post_processor: Some(PostProcessorKind::Cps),
            cps_post: Some(CpsPostSelection {
                source: CpsPostSource::Bundled {
                    id: "shopbot".into(),
                },
                properties: std::collections::BTreeMap::default(),
            }),
        };
        let response = ivac_core::pipeline::run_pipeline(request.clone(), |_, _, _| {});
        cases.push(serde_json::json!({"name":name, "request":request, "response":response.map_err(|e|e.to_string())}));
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "hatch":imported.map_err(|e|e.to_string()), "cases":cases
        }))
        .unwrap()
    );
}
