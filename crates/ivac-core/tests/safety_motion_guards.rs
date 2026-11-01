//! Synthetic source-bound regressions for SAFETY-FIX-1, never machine acceptance.
use ivac_core::pipeline::{
    generate_streaming, run_pipeline, run_pipeline_two_sided, stream_gcode_to_writer, CancelToken,
    PipelineError, PipelineRequest,
};
use serde_json::{json, Value};
use std::sync::{Arc, Mutex};
fn fixture(name: &str) -> Value {
    serde_json::from_str(match name {
        "island" => include_str!("../../../tests/fixtures/safety-fix1/island-helix.json"),
        "profile" => include_str!("../../../tests/fixtures/safety-fix1/profile-helix.json"),
        "overcut" => include_str!("../../../tests/fixtures/safety-fix1/profile-overcut.json"),
        _ => include_str!("../../../tests/fixtures/safety-fix1/two-sided.json"),
    })
    .unwrap()
}
fn req(v: &Value) -> PipelineRequest {
    serde_json::from_value(v.clone()).unwrap()
}
#[derive(Clone)]
struct Capture(Arc<Mutex<Vec<u8>>>);
impl std::io::Write for Capture {
    fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(b);
        Ok(b.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
fn rejects_every_single_boundary(v: &Value, reason: &str) {
    let error = run_pipeline(req(v), |_, _, _| {}).unwrap_err();
    let structured = error.to_structured(Some(&req(v).project)).unwrap();
    let payload = serde_json::to_value(structured).unwrap();
    assert_eq!(payload["params"]["safety_guard"], reason, "{payload}");
    let cap = Capture(Arc::default());
    let bytes = cap.0.clone();
    assert!(stream_gcode_to_writer(req(v), Box::new(cap)).is_err());
    assert!(
        bytes.lock().unwrap().is_empty(),
        "unsafe bytes escaped before guard"
    );
    let mut events = Vec::new();
    assert!(generate_streaming(req(v), &CancelToken::default(), &mut |e| events.push(e)).is_err());
    assert!(
        events
            .iter()
            .all(|e| !serde_json::to_string(e).unwrap().contains("gcode")),
        "unsafe worker payload: {events:?}"
    );
}
#[test]
fn helix_islands_simple_narrow_profiles_and_multiple_passes_refuse_before_output() {
    for name in ["island", "profile"] {
        for passes in [1., 3.] {
            let mut v = fixture(name);
            v["project"]["operations"][0]["params"]["stages"]["main"]["depth_per_pass_mm"] =
                json!(passes);
            rejects_every_single_boundary(&v, "helical_entry");
            if name == "profile" {
                v["project"]["operations"][0]["kind"]["offset"] = json!("inside");
                rejects_every_single_boundary(&v, "helical_entry");
            } else {
                v["project"]["segments"].as_array_mut().unwrap().truncate(4);
                rejects_every_single_boundary(&v, "helical_entry");
            }
        }
    }
}
#[test]
fn implicit_trochoidal_entry_and_legacy_profile_descent_cannot_bypass_guard() {
    let mut v = fixture("island");
    v["project"]["operations"][0]["params"]["plunge"] = json!({"kind":"direct"});
    v["project"]["operations"][0]["kind"]["strategy"] =
        json!({"kind":"trochoidal","engagement_angle_deg":30.,"loop_radius_factor":1.});
    rejects_every_single_boundary(&v, "helical_entry");
    let mut v = fixture("profile");
    v["project"]["operations"][0]["params"]["plunge"] = json!({"kind":"direct"});
    v["project"]["operations"][0]["kind"]["profile"]["helix"] = json!(true);
    rejects_every_single_boundary(&v, "helical_entry");
}
#[test]
fn overcut_cw_ccw_inside_outside_refuse_and_ordinary_profiles_still_cut() {
    for side in ["inside", "outside", "on"] {
        for reverse in [false, true] {
            let mut v = fixture("overcut");
            v["project"]["operations"][0]["kind"]["offset"] = json!(side);
            if reverse {
                for s in v["project"]["segments"].as_array_mut().unwrap() {
                    let a = s["start"].clone();
                    s["start"] = s["end"].clone();
                    s["end"] = a;
                }
            }
            rejects_every_single_boundary(&v, "legacy_overcut");
            v["project"]["operations"][0]["kind"]["profile"]["overcut"] = json!(false);
            assert!(!run_pipeline(req(&v), |_, _, _| {})
                .unwrap()
                .gcode
                .is_empty());
        }
    }
}
#[test]
fn ineffective_front_holding_modes_and_degenerate_placements_refuse_pair() {
    for mode in [
        json!({"kind":"manual"}),
        json!({"kind":"auto","count":0}),
        json!({"kind":"mixed","auto_count":0}),
    ] {
        let mut v = fixture("two");
        v["project"]["operations"][0]["kind"]["contour"]["tab_mode"] = mode;
        assert!(matches!(
            run_pipeline_two_sided(req(&v), |_, _, _| {}),
            Err(PipelineError::TwoSidedThrough { .. })
        ));
    }
    let mut v = fixture("two");
    v["project"]["operations"][0]["kind"]["contour"]["tab_placements"] =
        json!([{"object_id":999_999,"t":0.5}]);
    assert!(run_pipeline_two_sided(req(&v), |_, _, _| {}).is_err());
}
fn holding() -> Value {
    let mut v = fixture("two");
    let c = &mut v["project"]["operations"][0]["kind"]["contour"];
    c["tab_mode"] = json!({"kind":"auto","count":4});
    c["tabs"]["width_mode"] = json!("material_v1");
    c["tabs"]["width"] = json!(6.);
    c["tabs"]["height"] = json!(1.);
    v["project"]["operations"][1]["params"]["depth"] = json!(-0.25);
    v
}
#[test]
fn resolved_material_tabs_preserve_real_holding_and_do_not_puncture_the_far_face() {
    let v = holding();
    let response = run_pipeline_two_sided(req(&v), |_, _, _| {}).unwrap();
    assert!(response.back.is_some());
    let moves = &response.front.toolpath;
    assert!(moves.iter().any(|m| m.to.z <= -2.));
    assert!(moves
        .iter()
        .any(|m| m.from.z < 0. && m.to.z < 0. && m.to.z >= -1.));
    for field in ["width", "height"] {
        let mut bad = v.clone();
        bad["project"]["operations"][0]["kind"]["contour"]["tabs"][field] = json!(0.01);
        assert!(
            run_pipeline_two_sided(req(&bad), |_, _, _| {}).is_err(),
            "unproven small holding {field}"
        );
    }
}
#[test]
fn back_text_approach_and_pattern_are_refused_without_partial_pair() {
    for reason in ["back_text", "back_approach", "back_pattern"] {
        let mut v = holding();
        if reason == "back_text" {
            v["project"]["text_layers"] = json!([{"id":1,"kind":"TEXT","name":"synthetic F","text":"F","font_bytes":ivac_core::input::svg_font::bundled_fonts()[0].1,"size_mm":8.,"origin":[20.,20.]}]);
            v["project"]["operations"][1]["source"] =
                json!({"kind":"layers","layers":["__text_1"]});
        } else if reason == "back_approach" {
            v["project"]["operations"][1]["kind"]["contour"]["approach_point"] = json!([10., 10.]);
        } else {
            v["project"]["operations"][1]["kind"] = json!({"type":"drill","cycle":{"kind":"simple"},"pattern":{"kind":"grid","count_x":2,"count_y":1,"dx":10.,"dy":10.}});
        }
        let error = run_pipeline_two_sided(req(&v), |_, _, _| {}).unwrap_err();
        let wire = serde_json::to_value(error.to_structured(None)).unwrap();
        assert_eq!(wire["params"]["safety_guard"], reason, "{wire}");
    }
}
#[test]
fn shallow_supported_two_sided_segments_keep_the_actual_mirror() {
    let mut v = fixture("two");
    v["project"]["operations"][0]["params"]["depth"] = json!(-0.25);
    v["project"]["operations"][1]["params"]["depth"] = json!(-0.25);
    let response = run_pipeline_two_sided(req(&v), |_, _, _| {}).unwrap();
    let back = response.back.unwrap();
    let xs: Vec<f64> = back
        .toolpath
        .iter()
        .filter(|m| m.to.z < 0.)
        .map(|m| m.to.x)
        .collect();
    assert!(
        xs.iter().all(|x| *x > 40.),
        "actual mirrored back bounds: {xs:?}"
    );
}
#[cfg(feature = "cps")]
#[test]
fn bundled_shopbot_restrictions_remain_fail_closed() {
    for name in ["island", "profile", "overcut", "two"] {
        let mut v = fixture(name);
        v["post_processor"] = json!("cps");
        v["cps_post"] = json!({"source":{"kind":"bundled","id":"shopbot"}});
        assert!(run_pipeline(req(&v), |_, _, _| {}).is_err());
        if name == "two" {
            assert!(run_pipeline_two_sided(req(&v), |_, _, _| {}).is_err());
        }
    }
}

#[test]
fn reviewer_front_text_and_cumulative_holding_gaps_fail_closed() {
    let mut v = holding();
    v["project"]["text_layers"] = json!([{"id":1,"kind":"TEXT","name":"synthetic F","text":"F","font_bytes":ivac_core::input::svg_font::bundled_fonts()[0].1,"size_mm":8.,"origin":[20.,20.]}]);
    v["project"]["operations"][1]["source"] = json!({"kind":"layers","layers":["0"]});
    assert!(matches!(
        run_pipeline_two_sided(req(&v), |_, _, _| {}),
        Err(PipelineError::TwoSidedThrough { .. })
    ));
    let mut v = holding();
    let mut extra = v["project"]["operations"][0].clone();
    extra["id"] = json!(3);
    // Even independently valid tab sets cannot prove a common surviving bridge.
    extra["kind"]["contour"]["tabs"]["width"] = json!(7.);
    v["project"]["operations"]
        .as_array_mut()
        .unwrap()
        .push(extra);
    let error = run_pipeline_two_sided(req(&v), |_, _, _| {}).unwrap_err();
    assert!(matches!(
        error,
        PipelineError::MotionGuard(_, "two_sided_holding", _)
    ));
}

#[test]
fn displaced_island_narrow_entry_and_disabled_unsafe_op() {
    let mut v = fixture("island");
    for segment in v["project"]["segments"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .skip(4)
    {
        for end in ["start", "end"] {
            let x = segment[end]["x"].as_f64().unwrap();
            segment[end]["x"] = json!(x + 6.);
        }
    }
    rejects_every_single_boundary(&v, "helical_entry");
    v["project"]["segments"].as_array_mut().unwrap().truncate(4);
    for segment in v["project"]["segments"].as_array_mut().unwrap() {
        for end in ["start", "end"] {
            segment[end]["x"] = json!(segment[end]["x"].as_f64().unwrap() * 0.04);
        }
    }
    rejects_every_single_boundary(&v, "helical_entry");
    let mut v = fixture("overcut");
    v["project"]["operations"][0]["enabled"] = json!(false);
    let mut safe = v["project"]["operations"][0].clone();
    safe["id"] = json!(2);
    safe["enabled"] = json!(true);
    safe["kind"]["profile"]["overcut"] = json!(false);
    v["project"]["operations"]
        .as_array_mut()
        .unwrap()
        .push(safe);
    assert!(!run_pipeline(req(&v), |_, _, _| {})
        .unwrap()
        .gcode
        .is_empty());
}

#[test]
fn specialized_actual_depths_cannot_hide_front_severing() {
    for kind in [
        json!({"type":"chamfer","width_mm":2.,"finish_pass":false}),
        json!({"type":"drill","cycle":{"kind":"simple"},"spot_first":{"spot_tool_id":1,"spot_depth_mm":-3.}}),
        json!({"type":"pocket","strategy":{"kind":"halfpipe","profile":{"kind":"v_bottom","included_angle_deg":90.}}}),
    ] {
        let mut v = holding();
        v["project"]["operations"][0]["params"]["depth"] = json!(-5e-10);
        v["project"]["operations"][0]["kind"] = kind;
        if v["project"]["operations"][0]["kind"]["type"] == "chamfer" {
            v["project"]["tools"][0]["kind"] = json!("v_bit");
            v["project"]["tools"][0]["diameter"] = json!(6.);
        }
        let mut request = req(&v);
        let op = &mut request.project.operations[0];
        op.params.stages = None;
        op.params.step = Some(-0.25);
        op.params.cutting = Some(ivac_core::cutting_parameters::seed(
            op,
            &request.project.tools,
        ));
        let error = run_pipeline_two_sided(request, |_, _, _| {}).unwrap_err();
        assert!(
            matches!(
                error,
                PipelineError::TwoSidedThrough { .. }
                    | PipelineError::MotionGuard(_, "two_sided_depth", _)
            ),
            "{error:?}"
        );
    }
}

fn disjoint_front() -> Value {
    let mut v = holding();
    v["project"]["operations"][0]["source"] = json!({"kind":"layers","layers":["0"]});
    let mut extra = v["project"]["operations"][0].clone();
    extra["id"] = json!(3);
    extra["source"] = json!({"kind":"layers","layers":["far"]});
    extra["params"]["depth"] = json!(-1.99);
    extra["kind"]["contour"]["tab_mode"] = json!({"kind":"off"});
    extra["kind"]["contour"]["tabs"]["active"] = json!(false);
    v["project"]["operations"]
        .as_array_mut()
        .unwrap()
        .push(extra);
    let segments = v["project"]["segments"].as_array().unwrap().clone();
    for mut segment in segments {
        segment["layer"] = json!("far");
        for end in ["start", "end"] {
            segment[end]["x"] = json!(segment[end]["x"].as_f64().unwrap() + 50.);
        }
        v["project"]["segments"]
            .as_array_mut()
            .unwrap()
            .push(segment);
    }
    v
}

#[test]
fn disjoint_cuts_preserved_but_unbounded_leads_and_secondary_envelopes_refuse() {
    let v = disjoint_front();
    assert!(run_pipeline_two_sided(req(&v), |_, _, _| {}).is_ok());
    for field in ["in", "out"] {
        let mut bad = v.clone();
        bad["project"]["operations"][2]["kind"]["contour"]["leads"][field] = json!("straight");
        bad["project"]["operations"][2]["kind"]["contour"]["leads"][format!("{field}_lenght")] =
            json!(100.);
        assert!(matches!(
            run_pipeline_two_sided(req(&bad), |_, _, _| {}),
            Err(PipelineError::MotionGuard(_, "two_sided_holding", _))
        ));
    }
    for kind in [
        json!({"type":"thread","internal":false}),
        json!({"type":"drill","cycle":{"kind":"simple"},"spot_first":{"spot_tool_id":1,"spot_depth_mm":-1.99}}),
    ] {
        let mut bad = v.clone();
        bad["project"]["operations"][2]["kind"] = kind;
        let mut request = req(&bad);
        let op = &mut request.project.operations[2];
        op.params.stages = None;
        op.params.step = Some(-0.25);
        op.params.cutting = Some(ivac_core::cutting_parameters::seed(
            op,
            &request.project.tools,
        ));
        assert!(matches!(
            run_pipeline_two_sided(request, |_, _, _| {}),
            Err(PipelineError::MotionGuard(_, "two_sided_holding", _))
        ));
    }
}

#[test]
fn ascending_thread_start_depth_is_part_of_actual_front_reach() {
    let mut v = holding();
    v["project"]["operations"][0]["kind"] = json!({"type":"thread","internal":true});
    v["project"]["operations"][0]["params"]["depth"] = json!(-0.5);
    v["project"]["operations"][0]["params"]["start_depth"] = json!(-3.);
    let mut request = req(&v);
    let op = &mut request.project.operations[0];
    op.params.stages = None;
    op.params.step = Some(-0.25);
    op.params.cutting = Some(ivac_core::cutting_parameters::seed(
        op,
        &request.project.tools,
    ));
    assert!(matches!(
        run_pipeline_two_sided(request, |_, _, _| {}),
        Err(PipelineError::TwoSidedThrough { removal_mm: 3., .. })
    ));
}
