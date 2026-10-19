//! User-controlled text (tool/operation/group names, pause messages, cycle
//! labels) only ever reaches comments: hostile strings must leave every
//! executable line identical to a benign project and keep provenance resolved.
mod common;
use common::*;
use ivac_core::pipeline::provenance::Authority;
use ivac_core::pipeline::{run_pipeline, PipelineRequest, PipelineResponse, PostProcessorKind};
use ivac_core::project::{
    ContourParams, OpKind, PocketParams, PocketStrategy, Project, ToolChangeStrategy,
};

fn project(text: &str) -> Project {
    let mut op = op_single_pass(
        1,
        text,
        OpKind::Pocket {
            strategy: PocketStrategy::Cascade,
            contour: ContourParams::default(),
            pocket: PocketParams::default(),
        },
        1,
        -2.,
    );
    op.finish_tool_id = Some(2);
    op.group = Some(text.into());
    let mut pause = op.clone();
    pause.id = 2;
    pause.kind = OpKind::Pause {
        message: text.into(),
    };
    let mut marker = op.clone();
    marker.id = 3;
    marker.kind = OpKind::CycleMarker { label: text.into() };
    let mut p = build_project(
        &stock_at_origin(60., 60., 10.),
        vec![endmill_tool(1, 8.), endmill_tool(2, 2.)],
        op,
        closed_rectangle(5., 5., 45., 45.),
    );
    p.tools[1].name = text.into();
    p.operations.extend([pause, marker]);
    p.machine.tool_change = ToolChangeStrategy::ManualM0Pause;
    p
}

fn generate(p: Project, post: PostProcessorKind) -> PipelineResponse {
    run_pipeline(
        PipelineRequest {
            project: p,
            post_processor: Some(post),
            cps_post: None,
        },
        |_, _, _| {},
    )
    .unwrap()
}

/// RS274 view of a line: `(` comments end at the first `)`, `;` ends the line.
fn executable(line: &str) -> String {
    let mut out = String::new();
    let mut comment = false;
    for c in line.chars() {
        match c {
            ')' if comment => comment = false,
            _ if comment => {}
            '(' => comment = true,
            ';' => break,
            c if c.is_whitespace() => {}
            c => out.push(c.to_ascii_uppercase()),
        }
    }
    out
}

#[test]
fn hostile_names_never_reach_executable_gcode() {
    for post in [PostProcessorKind::Linuxcnc, PostProcessorKind::Grbl] {
        let control = generate(project("benign"), post);
        let commands =
            |r: &PipelineResponse| -> Vec<String> { r.gcode.lines().map(executable).collect() };
        for hostile in [
            "Fraise filetage M6",
            "x) T5 (y",
            "Fraise (M6) ((nested))",
            "line\nG0 Z-50\nT9 M6",
            "a;b\rM6\tT7",
            ")\n(MSG,boom",
            "MSG,boom",
            "py,import os",
            "\u{0}\u{1b}[31m T7 ø3 ünïcødé",
        ] {
            let r = generate(project(hostile), post);
            assert_eq!(commands(&r), commands(&control), "{post:?} {hostile:?}");
            assert_eq!(
                r.tool_provenance.authority,
                Authority::Resolved,
                "{post:?} {hostile:?}: {:?}",
                r.tool_provenance.diagnostics
            );
            for line in r.gcode.lines() {
                assert!(!line.chars().any(char::is_control), "{line:?}");
                if post == PostProcessorKind::Linuxcnc && line.starts_with('(') {
                    // One balanced, non-nested comment per line.
                    assert_eq!(line.matches('(').count(), 1, "{line}");
                    assert!(
                        line.ends_with(')') && line.matches(')').count() == 1,
                        "{line}"
                    );
                    assert!(!line.to_ascii_lowercase().starts_with("(msg,"), "{line}");
                }
            }
        }
    }
}
