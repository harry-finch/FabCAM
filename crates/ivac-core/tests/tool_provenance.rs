//! ADR-0025 semantic identity regressions through actual generation.
// Test fixtures assert exact literal values and bounded indices.
#![allow(clippy::float_cmp)]
mod common;
use common::*;
use ivac_core::pipeline::provenance::Authority;
use ivac_core::pipeline::{run_pipeline, PipelineRequest, PostProcessorKind};
use ivac_core::project::{ContourParams, OpKind, PocketParams, PocketStrategy, ToolChangeStrategy};
fn pocket() -> ivac_core::project::Project {
    let mut op = op_single_pass(
        17,
        "rough then finish",
        OpKind::Pocket {
            strategy: PocketStrategy::Cascade,
            contour: ContourParams::default(),
            pocket: PocketParams::default(),
        },
        41,
        -2.,
    );
    op.finish_tool_id = Some(7);
    let mut p = build_project(
        &stock_at_origin(60., 60., 10.),
        vec![endmill_tool(41, 8.), endmill_tool(7, 2.)],
        op,
        closed_rectangle(5., 5., 45., 45.),
    );
    p.machine.tool_change = ToolChangeStrategy::ManualM0Pause;
    p
}
#[test]
fn native_rough_finish_carries_two_bodies_under_one_operation() {
    let result = run_pipeline(
        PipelineRequest {
            project: pocket(),
            post_processor: Some(PostProcessorKind::Linuxcnc),
            cps_post: None,
        },
        |_, _, _| {},
    )
    .unwrap();
    assert_eq!(result.tool_provenance.authority, Authority::Resolved);
    assert_eq!(result.tool_provenance.bindings.len(), 2);
    assert!(result.toolpath.iter().all(|s| s.tool_ref.is_some()));
    let refs: std::collections::HashSet<_> = result
        .toolpath
        .iter()
        .filter(|s| s.op_id == 17)
        .map(|s| s.tool_ref)
        .collect();
    assert_eq!(refs.len(), 2);
}
#[test]
fn ignored_multi_tool_is_not_resolved() {
    let mut p = pocket();
    p.machine.tool_change = ToolChangeStrategy::Ignore;
    let r = run_pipeline(
        PipelineRequest {
            project: p,
            post_processor: Some(PostProcessorKind::Linuxcnc),
            cps_post: None,
        },
        |_, _, _| {},
    )
    .unwrap();
    assert_ne!(r.tool_provenance.authority, Authority::Resolved);
}
#[cfg(feature = "cps")]
#[test]
fn shopbot_resolved_moves_use_finish_body_and_keep_guarded_subset() {
    use ivac_core::pipeline::{CpsPostSelection, CpsPostSource};
    let p = pocket();
    let r = run_pipeline(
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
    )
    .unwrap();
    assert_eq!(
        r.tool_provenance.authority,
        Authority::Resolved,
        "{:?}",
        r.tool_provenance.diagnostics
    );
    assert!(r.toolpath.iter().all(|s| s.tool_ref.is_some()));
    assert_eq!(r.tool_provenance.bindings.len(), 2);
    assert!(!r.gcode.contains("\nC9"));
    let finish = r
        .tool_provenance
        .bindings
        .iter()
        .find(|b| b.project_tool_id == 7)
        .unwrap()
        .reference;
    let transition = r
        .tool_provenance
        .transitions
        .iter()
        .find(|t| t.after_ref == finish)
        .unwrap();
    assert!(r
        .toolpath
        .iter()
        .filter(|s| s.gcode_line > transition.output_line)
        .all(|s| s.tool_ref == Some(finish)));
    // Before activation, the new section's JZ still belongs to the outgoing tool.
    assert!(r
        .toolpath
        .iter()
        .filter(|s| s.gcode_line < transition.output_line)
        .all(|s| s.tool_ref != Some(finish)));
}
#[test]
fn spot_drill_and_chamfer_keep_three_tool_geometries() {
    use ivac_core::project::{DrillCycle, SpotConfig};
    let mut op = op_single_pass(
        9,
        "three tools",
        OpKind::Drill {
            cycle: DrillCycle::Simple { dwell_sec: 0. },
            chamfer_after_width_mm: Some(0.3),
            pattern: None,
            spot_first: Some(SpotConfig {
                spot_tool_id: 2,
                spot_depth_mm: -0.5,
            }),
        },
        1,
        -3.,
    );
    op.finish_tool_id = Some(3);
    let mut p = build_project(
        &stock_at_origin(60., 60., 10.),
        vec![
            drill_tool(1, 3., 118.),
            vbit_tool(2, 6., 90., 0.),
            vbit_tool(3, 8., 60., 0.),
        ],
        op,
        closed_circle(15., 15., 1.5),
    );
    p.machine.tool_change = ToolChangeStrategy::ManualM0Pause;
    let r = run_pipeline(
        PipelineRequest {
            project: p,
            post_processor: Some(PostProcessorKind::Linuxcnc),
            cps_post: None,
        },
        |_, _, _| {},
    )
    .unwrap();
    assert_eq!(
        r.tool_provenance.authority,
        Authority::Resolved,
        "{:?}",
        r.tool_provenance.diagnostics
    );
    let ids: std::collections::HashSet<_> = r
        .toolpath
        .iter()
        .filter(|s| s.kind != ivac_core::gcode::preview::MoveKind::Rapid)
        .map(|s| r.tool_provenance.bindings[s.tool_ref.unwrap().get() as usize - 1].project_tool_id)
        .collect();
    assert_eq!(ids, std::collections::HashSet::from([1, 2, 3]));
    for s in &r.toolpath {
        let t = &r.tool_provenance.bindings[s.tool_ref.unwrap().get() as usize - 1].tool;
        if t.id == 2 && s.to.z < 0. {
            // Existing Drill contract adds the 90° cone length (radius 3 mm)
            // below the nominal -0.5 mm bore floor. Provenance must not change it.
            assert!(s.to.z >= -3.5001);
            assert_eq!(t.tip_angle_deg, 90.);
        }
        if t.id == 1 {
            assert_eq!(t.kind, ivac_core::project::ToolKind::Drill);
        }
        if t.id == 3 {
            assert_eq!(t.tip_angle_deg, 60.);
        }
    }
}
#[test]
fn leading_tool_less_pause_and_duplicate_ids_do_not_guess() {
    let mut p = pocket();
    let mut pause = p.operations[0].clone();
    pause.id = 88;
    pause.tool_id = 0;
    pause.finish_tool_id = None;
    pause.kind = ivac_core::project::OpKind::Pause {
        message: "Check setup".into(),
    };
    p.operations.insert(0, pause);
    let r = run_pipeline(
        PipelineRequest {
            project: p.clone(),
            post_processor: Some(PostProcessorKind::Linuxcnc),
            cps_post: None,
        },
        |_, _, _| {},
    )
    .unwrap();
    assert_eq!(r.tool_provenance.authority, Authority::Resolved);
    p.tools.push(p.tools[0].clone());
    let r = run_pipeline(
        PipelineRequest {
            project: p,
            post_processor: Some(PostProcessorKind::Linuxcnc),
            cps_post: None,
        },
        |_, _, _| {},
    )
    .unwrap();
    assert_eq!(r.tool_provenance.authority, Authority::Unavailable);
}

#[test]
fn numbered_native_output_keeps_physical_line_bindings() {
    let mut p = pocket();
    p.machine.line_number_start = Some(10);
    let r = run_pipeline(
        PipelineRequest {
            project: p,
            post_processor: Some(PostProcessorKind::Linuxcnc),
            cps_post: None,
        },
        |_, _, _| {},
    )
    .unwrap();
    assert_eq!(r.tool_provenance.authority, Authority::Resolved);
    assert_eq!(r.tool_provenance.bindings.len(), 2);
    assert!(r.toolpath.iter().all(|s| s.tool_ref.is_some()));
}

#[test]
fn native_change_strategies_and_tool_reuse_keep_stable_bindings() {
    for strategy in [
        ToolChangeStrategy::Atc,
        ToolChangeStrategy::ManualM6Prompt,
        ToolChangeStrategy::ManualM0Pause,
    ] {
        let mut p = pocket();
        p.machine.tool_change = strategy;
        let mut second = p.operations[0].clone();
        second.id = 18;
        p.operations.push(second);
        let r = run_pipeline(
            PipelineRequest {
                project: p,
                post_processor: Some(PostProcessorKind::Linuxcnc),
                cps_post: None,
            },
            |_, _, _| {},
        )
        .unwrap();
        assert_eq!(
            r.tool_provenance.authority,
            Authority::Resolved,
            "{strategy:?}"
        );
        assert_eq!(r.tool_provenance.bindings.len(), 2);
        for id in [17, 18] {
            let refs: std::collections::HashSet<_> = r
                .toolpath
                .iter()
                .filter(|s| s.op_id == id)
                .map(|s| s.tool_ref)
                .collect();
            assert_eq!(refs.len(), 2);
        }
    }
}

#[test]
fn injected_reserved_marker_cannot_certify_native_output() {
    let mut p = pocket();
    p.operations[0].name = "FABTOOL/1 ACTIVE 7".into();
    let r = run_pipeline(
        PipelineRequest {
            project: p,
            post_processor: Some(PostProcessorKind::Linuxcnc),
            cps_post: None,
        },
        |_, _, _| {},
    )
    .unwrap();
    assert_eq!(r.tool_provenance.authority, Authority::Unavailable);
}

fn generate(
    p: ivac_core::project::Project,
    post: PostProcessorKind,
) -> ivac_core::pipeline::PipelineResponse {
    #[cfg(feature = "cps")]
    let cps_post =
        (post == PostProcessorKind::Cps).then(|| ivac_core::pipeline::CpsPostSelection {
            source: ivac_core::pipeline::CpsPostSource::Bundled {
                id: "shopbot".into(),
            },
            properties: std::collections::BTreeMap::new(),
        });
    #[cfg(not(feature = "cps"))]
    let cps_post = None;
    run_pipeline(
        PipelineRequest {
            project: p,
            post_processor: Some(post),
            cps_post,
        },
        |_, _, _| {},
    )
    .unwrap()
}

/// ADR-0025 follow-up A: the bundled post rounds load-instruction diameters
/// half away from zero; imperial fractions whose 5th decimal is a tie must
/// still resolve, with each finish move bound to the finish body.
#[cfg(feature = "cps")]
#[test]
fn shopbot_imperial_diameters_resolve_with_their_own_bodies() {
    for (fraction, emitted) in [
        (1., "0.7938"),
        (3., "2.3813"),
        (5., "3.9688"),
        (7., "5.5563"),
        (9., "7.1438"),
    ] {
        let mut p = pocket();
        p.tools[1].diameter = fraction * 25.4 / 32.;
        let r = generate(p, PostProcessorKind::Cps);
        assert_eq!(
            r.tool_provenance.authority,
            Authority::Resolved,
            "{fraction}/32 in: {:?}",
            r.tool_provenance.diagnostics
        );
        assert!(r.gcode.contains(&format!("Load tool 7 D={emitted};")));
        let finish = r
            .tool_provenance
            .bindings
            .iter()
            .find(|b| b.project_tool_id == 7)
            .unwrap();
        assert!((finish.tool.diameter - fraction * 25.4 / 32.).abs() < 1e-12);
        assert!(r
            .toolpath
            .iter()
            .any(|s| s.tool_ref == Some(finish.reference)));
    }
}

/// ADR-0025 follow-up B: tool names reach only operator comments; they must
/// neither refuse a valid manual change nor hide a real one.
#[test]
fn native_manual_change_ignores_tool_names_in_comments() {
    for post in [PostProcessorKind::Linuxcnc, PostProcessorKind::Grbl] {
        for name in [
            "Fraise filetage M6",
            "Taraud M06",
            "T2 spécial",
            "Fraise (6mm)",
            "ø3 ünïcødé",
            "a;b M6",
        ] {
            let mut p = pocket();
            p.tools[1].name = name.into();
            let r = generate(p, post);
            assert_eq!(
                r.tool_provenance.authority,
                Authority::Resolved,
                "{post:?} {name}: {:?}",
                r.tool_provenance.diagnostics
            );
            assert!(r.toolpath.iter().all(|s| s.tool_ref.is_some()));
        }
    }
}

/// `comments: false` keeps the structured provenance markers: they are
/// required metadata of owned native output, not optional commentary.
#[test]
fn provenance_markers_are_emitted_even_without_comments() {
    let mut p = pocket();
    p.machine.comments = false;
    let r = generate(p, PostProcessorKind::Linuxcnc);
    assert_eq!(r.tool_provenance.authority, Authority::Resolved);
    for marker in ["PRELOAD 41", "REQUEST 7", "ACTIVE 7"] {
        assert!(
            r.gcode.contains(&format!("(FABTOOL/1 {marker})")),
            "{marker}"
        );
    }
}

/// ADR-0025 follow-up C: one-tool programs with unmodelled machine semantics
/// stay planned-only (export policy unchanged) and say why — the preview does
/// not model G28, and post profiles may template arbitrary text.
#[test]
fn unmodelled_native_features_stay_planned_with_a_specific_reason() {
    let single = || {
        let mut p = pocket();
        p.operations[0].finish_tool_id = None;
        p
    };
    let mut homing = single();
    let mut home = homing.operations[0].clone();
    home.id = 99;
    home.kind = OpKind::Homing {
        retract_to_safe_z: true,
    };
    homing.operations.push(home);
    let mut profile = single();
    profile.machine.post_profile = Some(ivac_core::gcode::post_profile::PostProfile::default());
    let mut optional_stop = pocket();
    optional_stop.machine.optional_stop = true;
    for (label, p, reason) in [
        ("homing", homing, "Homing (G28)"),
        ("profile", profile, "Custom post profiles"),
        ("optional stop", optional_stop, "lacks a standalone M0 stop"),
    ] {
        let r = generate(p, PostProcessorKind::Linuxcnc);
        assert_eq!(r.tool_provenance.authority, Authority::Planned, "{label}");
        assert!(
            r.tool_provenance
                .diagnostics
                .iter()
                .any(|d| d.contains(reason)),
            "{label}: {:?}",
            r.tool_provenance.diagnostics
        );
    }
    assert_eq!(
        generate(single(), PostProcessorKind::Linuxcnc)
            .tool_provenance
            .authority,
        Authority::Resolved
    );
}
