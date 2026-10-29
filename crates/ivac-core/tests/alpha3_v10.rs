//! New authored v10 requests. Historical preflight evidence remains separate.
// Exact decoded Z endpoints are part of this serialization regression.
#![allow(clippy::float_cmp)]
use ivac_core::pipeline::{run_pipeline, PipelineRequest, PipelineResponse};
use serde_json::{json, Value};
fn profile(offset: &str) -> Value {
    let mut v: Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/cli-v9/profile.request.json"
    ))
    .unwrap();
    v["project"]["cutting_version"] = json!(10);
    v["post_processor"] = json!("linuxcnc");
    v.as_object_mut().unwrap().remove("cps_post");
    let op = &mut v["project"]["operations"][0];
    op["kind"]["offset"] = json!(offset);
    op["params"].as_object_mut().unwrap().remove("cutting");
    op["params"].as_object_mut().unwrap().remove("step");
    op["params"]["stages"] = json!({"contract":"main_finishing_v1","main":{
        "tool_id":1,"rpm":11000,"feed_mm_min":811,"plunge_mm_min":111,"depth_per_pass_mm":0.25},"finishing":null});
    v
}
fn pocket(strategy: &str) -> Value {
    let mut v = profile("inside");
    let op = &mut v["project"]["operations"][0];
    op["kind"] = json!({"type":"pocket","strategy":strategy,"contour":op["kind"]["contour"],"pocket":{"xy_overlap":0.5}});
    v
}
fn finish(v: &mut Value, wall: f64, floor: f64) {
    let op = &mut v["project"]["operations"][0];
    op["finish_tool_id"] = json!(1);
    op["params"]["stages"]["finishing"] = json!({"cutting":{"tool_id":1,"rpm":11000,"feed_mm_min":413,"plunge_mm_min":113,"depth_per_pass_mm":0.2},"lateral_allowance_mm":wall,"floor_allowance_mm":floor});
}
fn run(v: Value) -> Result<PipelineResponse, String> {
    let req: PipelineRequest = serde_json::from_value(v).map_err(|e| e.to_string())?;
    run_pipeline(req, |_, _, _| {}).map_err(|e| e.to_string())
}
#[test]
fn main_only_uses_main_not_unused_finish_for_every_ordinary_strategy() {
    for v in [
        profile("inside"),
        profile("outside"),
        profile("on"),
        pocket("cascade"),
        pocket("zigzag"),
        pocket("spiral"),
    ] {
        let r = run(v).unwrap();
        assert!(r.gcode.contains("F811"));
        assert!(r.gcode.contains("F111"));
        assert!(r.gcode.contains("S11000"));
        assert!(!r.gcode.contains(" finishing"));
        assert!(r.warnings.iter().all(|w| !w.message.contains("Finish:")));
        assert!(r.toolpath.iter().any(|m| m.to.z == -0.5));
    }
}

#[test]
fn single_selected_rectangle_finishes_inside_an_unselected_stock_outline() {
    let mut v = pocket("cascade");
    finish(&mut v, 0.1, 0.1);
    let mut frame = v["project"]["segments"].clone();
    for s in frame.as_array_mut().unwrap() {
        s["layer"] = json!("stock-outline");
        for end in ["start", "end"] {
            for axis in ["x", "y"] {
                s[end][axis] = json!(s[end][axis].as_f64().unwrap() * 2.0 - 20.0);
            }
        }
    }
    v["project"]["segments"]
        .as_array_mut()
        .unwrap()
        .extend(frame.as_array().unwrap().clone());
    v["project"]["operations"][0]["source"] = json!({"kind":"layers","layers":["0"]});
    assert_eq!(run(v).unwrap().cutting_stages.len(), 2);
}
#[test]
fn rectangle_finishing_reaches_floor_and_emits_separate_authored_rates() {
    for strategy in ["cascade", "zigzag", "spiral"] {
        for (wall, floor) in [(0.1, 0.0), (0.0, 0.1), (0.1, 0.1)] {
            let mut v = pocket(strategy);
            finish(&mut v, wall, floor);
            let r = run(v).unwrap_or_else(|e| panic!("{strategy} {wall} {floor}: {e}"));
            assert!(r.gcode.contains("F811"));
            assert!(r.gcode.contains("F413"));
            assert!(r.gcode.contains("FABSTAGE_1 1 finishing"));
            assert!(r.toolpath.iter().any(|m| m.to.z == -0.5));
            if strategy == "cascade" {
                let rest = &r.remaining_material[0];
                assert_eq!(rest.status, "ready");
                // A round cutter cannot remove the four sharp target corners.
                // Independent analytical residual area, allowing the planar
                // diagnostic's documented 0.005 mm sweep approximation.
                let radius = 1.0;
                let corner_area = (4.0 - std::f64::consts::PI) * radius * radius;
                assert!((rest.area_mm2 - corner_area).abs() < 0.05);
            }
        }
    }
}
#[test]
fn profile_finishing_inside_and_refusals_are_explicit() {
    let mut v = profile("inside");
    finish(&mut v, 0.1, 0.0);
    assert!(run(v.clone()).is_ok());
    v["project"]["operations"][0]["params"]["stages"]["finishing"]["floor_allowance_mm"] =
        json!(0.1);
    assert!(run(v).unwrap_err().contains("floor"));
    let mut v = profile("on");
    finish(&mut v, 0.1, 0.0);
    assert!(run(v).unwrap_err().contains("lateral"));
}
#[test]
fn old_mixed_missing_and_contradictory_contracts_refuse() {
    for mode in ["v9", "mixed", "missing", "tool"] {
        let mut v = profile("inside");
        let op = &mut v["project"]["operations"][0];
        match mode {
            "v9" => {}
            "mixed" => op["params"]["cutting"] = json!({"rates":[]}),
            "missing" => {
                op["params"].as_object_mut().unwrap().remove("stages");
            }
            _ => op["params"]["stages"]["main"]["tool_id"] = json!(2),
        }
        if mode == "v9" {
            v["project"]["cutting_version"] = json!(9);
        }
        assert!(run(v).is_err(), "{mode}");
    }
}
#[test]
#[cfg(feature = "cps")]
fn same_tool_shopbot_rpm_transition_stays_refused() {
    let mut v = pocket("cascade");
    finish(&mut v, 0.1, 0.1);
    v["project"]["operations"][0]["params"]["stages"]["finishing"]["cutting"]["rpm"] = json!(13000);
    v["post_processor"] = json!("cps");
    v["cps_post"] = json!({"source":{"kind":"bundled","id":"shopbot"}});
    let e = run(v).unwrap_err();
    assert!(e.contains("spindleSpeed"), "{e}");
}

#[test]
fn unresolvable_finishing_allowances_refuse_instead_of_claiming_removal() {
    for (wall, floor) in [(0.000_001, 0.0), (0.0, 0.000_001)] {
        let mut v = pocket("cascade");
        finish(&mut v, wall, floor);
        let e = run(v).unwrap_err();
        assert!(e.contains("output precision"), "{e}");
    }
    let mut v = pocket("cascade");
    finish(&mut v, 0.05, 0.05);
    assert!(run(v).is_ok());
}

// Independent swept-disc oracle on decoded output (does not call the production
// offset or coverage functions). Interior samples are separated from boundaries
// by 0.001 mm; exact boundary/strip proofs are separately exercised in core.
fn cut_distance(r: &PipelineResponse, stage: &str, x: f64, y: f64, z: f64) -> f64 {
    use ivac_core::gcode::preview::MoveKind;
    let binding = r.cutting_stages.iter().find(|b| b.stage == stage).unwrap();
    r.toolpath[binding.first_segment..binding.end_segment_exclusive]
        .iter()
        .filter(|m| {
            matches!(m.kind, MoveKind::Cut | MoveKind::Plunge)
                && m.from.z <= z + 1e-6
                && m.to.z <= z + 1e-6
        })
        .map(|m| {
            let dx = m.to.x - m.from.x;
            let dy = m.to.y - m.from.y;
            let q = dx * dx + dy * dy;
            let t = if q == 0.0 {
                0.0
            } else {
                ((x - m.from.x) * dx + (y - m.from.y) * dy) / q
            }
            .clamp(0.0, 1.0);
            (x - m.from.x - t * dx).hypot(y - m.from.y - t * dy)
        })
        .fold(f64::INFINITY, f64::min)
}
#[test]
fn decoded_main_and_finish_sweeps_match_separate_wall_and_floor_envelopes() {
    // Fixture is the public 20 x 20 square at (5,5), queried from source, never
    // inferred from the generated paths being tested.
    for strategy in ["cascade", "zigzag", "spiral"] {
        for (wall, floor) in [(0.1, 0.0), (0.0, 0.1), (0.1, 0.1)] {
            let mut v = pocket(strategy);
            finish(&mut v, wall, floor);
            let segments = v["project"]["segments"].as_array().unwrap();
            let xs: Vec<_> = segments
                .iter()
                .map(|s| s["start"]["x"].as_f64().unwrap())
                .collect();
            let ys: Vec<_> = segments
                .iter()
                .map(|s| s["start"]["y"].as_f64().unwrap())
                .collect();
            let x0 = xs.iter().copied().fold(f64::INFINITY, f64::min);
            let x1 = xs.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            let y0 = ys.iter().copied().fold(f64::INFINITY, f64::min);
            let y1 = ys.iter().copied().fold(f64::NEG_INFINITY, f64::max);
            let radius = v["project"]["tools"][0]["diameter"].as_f64().unwrap() * 0.5;
            let r = run(v).unwrap();
            for i in 0..=30 {
                for j in 0..=30 {
                    let x = x0 + radius + (x1 - x0 - 2.0 * radius) * f64::from(i) / 30.;
                    let y = y0 + radius + (y1 - y0 - 2.0 * radius) * f64::from(j) / 30.;
                    assert!(
                        cut_distance(&r, "finishing", x, y, -0.5) <= radius + 0.001,
                        "{strategy}: uncleared floor at {x},{y}"
                    );
                }
            }
            if floor > 0.0 {
                assert!(
                    cut_distance(&r, "main", (x0 + x1) / 2., (y0 + y1) / 2., -0.5).is_infinite()
                );
            }
            if wall > 0.0 {
                let x = x0 + wall / 2.;
                let y = (y0 + y1) / 2.;
                assert!(cut_distance(&r, "main", x, y, -0.5 + floor) > radius);
                assert!(cut_distance(&r, "finishing", x, y, -0.5) < radius);
            }
            for b in &r.cutting_stages {
                let mut f = 0.;
                let mut rpm = 0;
                for (i, line) in r.gcode.lines().enumerate() {
                    for token in line.split_whitespace() {
                        if let Some(value) =
                            token.strip_prefix('F').and_then(|s| s.parse::<f64>().ok())
                        {
                            f = value;
                        }
                        if let Some(value) =
                            token.strip_prefix('S').and_then(|s| s.parse::<u32>().ok())
                        {
                            rpm = value;
                        }
                    }
                    for m in r.toolpath[b.first_segment..b.end_segment_exclusive]
                        .iter()
                        .filter(|m| m.gcode_line as usize == i + 1)
                    {
                        use ivac_core::gcode::preview::MoveKind;
                        if matches!(m.kind, MoveKind::Cut | MoveKind::Plunge | MoveKind::Arc) {
                            assert_eq!(rpm, b.parameters.rpm.unwrap());
                            let expected = if matches!(m.kind, MoveKind::Plunge) {
                                b.parameters.plunge_mm_min
                            } else {
                                b.parameters.feed_mm_min
                            };
                            assert!(
                                (f - f64::from(expected.unwrap())).abs() < 0.0001,
                                "{} {line}",
                                b.stage
                            );
                        }
                    }
                }
            }
        }
    }
}
#[test]
fn outside_rectangle_finishing_and_distinct_pocket_cutter_have_actual_stages() {
    let mut v = profile("outside");
    finish(&mut v, 0.1, 0.0);
    assert_eq!(run(v).unwrap().cutting_stages.len(), 2);
    let mut v = pocket("cascade");
    finish(&mut v, 0.1, 0.1);
    let mut tool = v["project"]["tools"][0].clone();
    tool["id"] = json!(2);
    tool["diameter"] = json!(2.0);
    v["project"]["tools"].as_array_mut().unwrap().push(tool);
    let op = &mut v["project"]["operations"][0];
    op["finish_tool_id"] = json!(2);
    op["params"]["stages"]["finishing"]["cutting"]["tool_id"] = json!(2);
    let r = run(v).unwrap();
    assert_eq!(r.cutting_stages[1].parameters.tool_id, 2);
    assert!(r
        .tool_provenance
        .bindings
        .iter()
        .any(|b| b.project_tool_id == 2));
}

#[test]
fn finishing_refuses_unproven_geometry_and_entries_without_partial_success() {
    for case in [
        "triangle",
        "island",
        "approach",
        "tabs",
        "whirl",
        "retained-on",
        "floor-too-deep",
    ] {
        let mut v = pocket("cascade");
        finish(&mut v, 0.1, 0.1);
        match case {
            "triangle" => {
                v["project"]["segments"].as_array_mut().unwrap().remove(1);
            }
            "island" => {
                let mut inner = v["project"]["segments"].as_array().unwrap().clone();
                for s in &mut inner {
                    for e in ["start", "end"] {
                        s[e]["x"] = json!(20.0 + (s[e]["x"].as_f64().unwrap() - 20.0) * 0.1);
                        s[e]["y"] = json!(20.0 + (s[e]["y"].as_f64().unwrap() - 20.0) * 0.1);
                    }
                }
                v["project"]["segments"]
                    .as_array_mut()
                    .unwrap()
                    .extend(inner);
            }
            "approach" => {
                v["project"]["operations"][0]["kind"]["contour"]["approach_point"] =
                    json!({"x":15.0,"y":15.0});
            }
            "tabs" => {
                v["project"]["operations"][0]["kind"]["contour"]["tabs"]["active"] = true.into();
            }
            "whirl" => v["project"]["tools"][0]["whirl"] = true.into(),
            "retained-on" => {
                v = profile("on");
                finish(&mut v, 0.0, 0.0);
            }
            _ => {
                v["project"]["operations"][0]["params"]["stages"]["finishing"]
                    ["floor_allowance_mm"] = json!(0.5);
            }
        }
        assert!(run(v).is_err(), "{case}");
    }
}

#[test]
fn retained_stock_remains_after_distinct_tool_finishing_and_output_rounding() {
    for post in ["linuxcnc", "grbl", "cps"] {
        if post == "cps" && !cfg!(feature = "cps") {
            continue;
        }
        let mut v = pocket("cascade");
        finish(&mut v, 0.101, 0.103);
        v["post_processor"] = json!(post);
        if post == "cps" {
            v["cps_post"] = json!({"source":{"kind":"bundled","id":"shopbot"}});
        }
        v["project"]["operations"][0]["params"]["stock_to_leave_mm"] = json!(0.2);
        let mut tool = v["project"]["tools"][0].clone();
        tool["id"] = 2.into();
        tool["diameter"] = json!(1.0);
        v["project"]["tools"].as_array_mut().unwrap().push(tool);
        v["project"]["operations"][0]["finish_tool_id"] = 2.into();
        v["project"]["operations"][0]["params"]["stages"]["finishing"]["cutting"]["tool_id"] =
            2.into();
        // Non-round input values exercise output precision without moving work zero.
        for s in v["project"]["segments"].as_array_mut().unwrap() {
            for e in ["start", "end"] {
                s[e]["x"] = json!(s[e]["x"].as_f64().unwrap() + 0.00037);
            }
        }
        let r = run(v).unwrap();
        assert_eq!(r.cutting_stages.len(), 2);
        for b in &r.cutting_stages {
            assert_eq!(
                b.authority,
                ivac_core::pipeline::provenance::Authority::Resolved
            );
            let radius = if b.stage == "main" { 1.0 } else { 0.5 };
            let allowance = if b.stage == "main" { 0.101 } else { 0.0 };
            let min = r.toolpath[b.first_segment..b.end_segment_exclusive]
                .iter()
                .filter(|m| m.to.z < 0.0)
                .flat_map(|m| [m.from.x, m.to.x])
                .fold(f64::INFINITY, f64::min);
            assert!(
                (min - (10.00037 + 0.2 + allowance + radius)).abs() < 0.001,
                "{post} {}: {min}",
                b.stage
            );
        }
        assert!(cut_distance(&r, "finishing", 20., 20., -0.5) < 0.5);
    }
}

#[test]
fn stage_presets_v2_keep_context_and_private_data_boundaries() {
    use ivac_core::presets::*;
    let mut req: PipelineRequest = serde_json::from_value(pocket("cascade")).unwrap();
    req.project.machine.physical_id = Some("synthetic-router".into());
    req.project.material = Some(ivac_core::project::MaterialIdentity {
        id: "synthetic-plywood".into(),
        revision: "1".into(),
    });
    req.project.tools[0].physical_id = Some("synthetic-cutter".into());
    let op = &req.project.operations[0];
    let preset = TestedPreset {
        id: "synthetic".into(),
        name: "Synthetic only".into(),
        context: context(&req.project, op).unwrap(),
        parameters: parameters(op).unwrap(),
        tested: true,
        test_date: None,
        notes: Some("PRIVATE NOTE NOT PORTABLE".into()),
    };
    let c = PresetCollection {
        format: "fabcam-preset-collection".into(),
        version: 2,
        id: "synthetic".into(),
        name: "Synthetic".into(),
        presets: vec![preset],
    };
    assert_eq!(
        parse_collection(&serde_json::to_string(&c).unwrap()).unwrap(),
        c
    );
    let loaded = load(&req.project, op, &c, "synthetic").unwrap();
    assert_eq!(loaded.params.stages, op.params.stages);
    assert!(!serde_json::to_string(&loaded)
        .unwrap()
        .contains("PRIVATE NOTE"));
    let mut old = c.clone();
    old.version = 1;
    assert!(validate_collection(&old).is_err());
    let mut mismatch = req.project.clone();
    mismatch.operations[0].kind = ivac_core::project::OpKind::Profile {
        offset: ivac_core::project::ToolOffset::Inside,
        contour: ivac_core::project::ContourParams::default(),
        profile: ivac_core::project::ProfileParams::default(),
    };
    assert!(load(&mismatch, &mismatch.operations[0], &c, "synthetic").is_err());
    let mut bad = c.clone();
    bad.presets[0].tested = false;
    assert!(load(&req.project, op, &bad, "synthetic").is_err());
}
