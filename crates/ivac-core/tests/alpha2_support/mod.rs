#![allow(dead_code)]
#[path = "../common/mod.rs"]
pub mod common;
use ivac_core::{
    cancel::CancelToken,
    drawing::{
        repair::Repair,
        scoped::{RepairContext, ScopedPreview, ScopedRequest, ScopedResponse},
        Drawing,
    },
    project::{OpKind, ProfileParams, ToolOffset},
};
pub fn input(d: Drawing, r: Repair) -> ScopedPreview {
    let op = common::op_single_pass(
        1,
        "test",
        OpKind::Profile {
            offset: ToolOffset::Outside,
            contour: Default::default(),
            profile: ProfileParams::default(),
        },
        1,
        -1.,
    );
    let mut project = common::build_project(
        &common::stock_at_origin(200., 200., 10.),
        vec![],
        op,
        d.evaluate().unwrap().segments,
    );
    project.operations.clear();
    ScopedPreview {
        context: RepairContext {
            drawing: d,
            project,
            bindings: vec![],
            document: serde_json::json!({"revision":1}),
        },
        repairs: vec![r],
    }
}
pub fn preview(input: ScopedPreview) -> Result<ScopedResponse, String> {
    ivac_core::drawing::scoped::evaluate(
        ScopedRequest::Preview {
            version: 2,
            input: Box::new(input),
        },
        &CancelToken::new(),
    )
}
