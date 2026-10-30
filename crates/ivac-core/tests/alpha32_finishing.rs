//! ALPHA-3.2 source-bound synthetic regressions (no private geometry).
use ivac_core::pipeline::{run_pipeline, PipelineRequest};
use serde_json::{json, Value};

fn request(scope: &str) -> Value {
    let mut v: Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/alpha31/convex-pocket.request.json"
    ))
    .unwrap();
    v["project"]["tools"][0]["flute_length_mm"] = json!(12.);
    let op = &mut v["project"]["operations"][0];
    op["params"]["stages"]["contract"] = json!("pocket_finishing_v2");
    let f = &mut op["params"]["stages"]["finishing"];
    f["scope"] = json!(scope);
    f["lateral_allowance_mm"] = json!(if scope == "floor_only" { 0.0 } else { 0.1 });
    f["floor_allowance_mm"] = json!(if scope == "walls_only" { 0.0 } else { 0.1 });
    v
}
#[test]
fn explicit_scopes_generate_with_two_authored_stages() {
    for scope in ["walls_only", "floor_only", "walls_and_floor"] {
        let req: PipelineRequest = serde_json::from_value(request(scope)).unwrap();
        let result = run_pipeline(req, |_, _, _| {}).unwrap();
        assert_eq!(result.cutting_stages.len(), 2, "{scope}");
    }
}
#[test]
fn v2_round_trip_requires_scope_and_rejects_unknown_scope() {
    let v = request("walls_only");
    let req: PipelineRequest = serde_json::from_value(v.clone()).unwrap();
    assert_eq!(
        serde_json::to_value(&req).unwrap()["project"]["operations"][0]["params"]["stages"],
        v["project"]["operations"][0]["params"]["stages"]
    );
    for scope in [Value::Null, json!("automatic")] {
        let mut bad = v.clone();
        bad["project"]["operations"][0]["params"]["stages"]["finishing"]["scope"] = scope;
        assert!(serde_json::from_value::<PipelineRequest>(bad).is_err());
    }
    let mut bad = v;
    bad["project"]["operations"][0]["params"]["stages"]["finishing"]
        .as_object_mut()
        .unwrap()
        .remove("scope");
    assert!(serde_json::from_value::<PipelineRequest>(bad).is_err());
}

fn polygon(v: &mut Value, name: &str) {
    let shapes: Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/alpha31/polygons.json"
    ))
    .unwrap();
    let points = shapes[name].as_array().unwrap();
    v["project"]["segments"]=Value::Array((0..points.len()).map(|i|{let a=&points[i];let b=&points[(i+1)%points.len()];json!({"type":"LINE","start":{"x":a[0],"y":a[1]},"end":{"x":b[0],"y":b[1]},"layer":"0","color":7})}).collect());
}
#[test]
fn polygon_scope_strategy_matrix() {
    let mut failures = Vec::new();
    for shape in [
        "rectangle",
        "rotated",
        "triangle",
        "convex",
        "concave",
        "irregular_concave",
    ] {
        for strategy in ["cascade", "zigzag", "spiral"] {
            for scope in ["walls_only", "floor_only", "walls_and_floor"] {
                let mut v = request(scope);
                polygon(&mut v, shape);
                v["project"]["operations"][0]["kind"]["strategy"] = json!(strategy);
                let req: PipelineRequest = serde_json::from_value(v).unwrap();
                let result = run_pipeline(req, |_, _, _| {});
                if strategy == "spiral" && matches!(shape, "concave" | "irregular_concave") {
                    // Source-bound investigation retains this explicit decoded
                    // 5000-path refusal; no fallback or enlarged proof tolerance.
                    assert!(result
                        .unwrap_err()
                        .to_string()
                        .contains("decoded planar proof budget"));
                } else if let Err(e) = result {
                    failures.push(format!("{shape}/{strategy}/{scope}: {e}"));
                }
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
fn loop_segments(points: &[[f64; 2]]) -> Vec<Value> {
    (0..points.len()).map(|i|{let a=points[i];let b=points[(i+1)%points.len()];json!({"type":"LINE","start":{"x":a[0],"y":a[1]},"end":{"x":b[0],"y":b[1]},"layer":"0","color":7})}).collect()
}
fn circle(cx: f64, cy: f64, r: f64) -> Vec<Value> {
    let pts = [[cx + r, cy], [cx, cy + r], [cx - r, cy], [cx, cy - r]];
    (0..4).map(|i|{let a=pts[i];let b=pts[(i+1)%4];json!({"type":"ARC","start":{"x":a[0],"y":a[1]},"end":{"x":b[0],"y":b[1]},"bulge":(std::f64::consts::FRAC_PI_2/4.).tan(),"center":{"x":cx,"y":cy},"layer":"0","color":7})}).collect()
}
#[test]
fn curved_and_island_scope_matrix() {
    let mut failures = Vec::new();
    for shape in ["circle", "rounded", "island", "annulus"] {
        for scope in ["walls_only", "floor_only", "walls_and_floor"] {
            let mut v = request(scope);
            v["project"]["tools"][0]["diameter"] = json!(6.);
            let mut segs = match shape {
                "circle" => circle(25., 25., 8.),
                "annulus" => circle(25., 25., 15.),
                "island" => loop_segments(&[[10., 10.], [50., 10.], [50., 50.], [10., 50.]]),
                _ => {
                    let mut segs = loop_segments(&[
                        [15., 10.],
                        [35., 10.],
                        [40., 15.],
                        [40., 35.],
                        [35., 40.],
                        [15., 40.],
                        [10., 35.],
                        [10., 15.],
                    ]);
                    for i in [1, 3, 5, 7] {
                        segs[i]["type"] = json!("ARC");
                        segs[i]["bulge"] = json!((std::f64::consts::FRAC_PI_2 / 4.).tan());
                    }
                    segs
                }
            };
            if shape == "island" {
                segs.extend(loop_segments(&[
                    [24., 24.],
                    [36., 24.],
                    [36., 36.],
                    [24., 36.],
                ]));
            }
            if shape == "annulus" {
                segs.extend(circle(25., 25., 5.));
            }
            v["project"]["segments"] = json!(segs);
            let req: PipelineRequest = serde_json::from_value(v).unwrap();
            if let Err(e) = run_pipeline(req, |_, _, _| {}) {
                failures.push(format!("{shape}/{scope}: {e}"));
            }
        }
    }
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
fn scope_validation_and_machining_refusals_are_explicit() {
    for case in [
        "scope_floor",
        "scope_wall",
        "profile",
        "missing_flute",
        "small_allowance",
        "ramp",
        "tabs",
        "trochoidal",
        "curve_corner",
        "micro_gap",
        "crossing",
        "unreachable",
        "different_single_surface",
        "rpm",
    ] {
        let mut v = request("walls_only");
        match case {
            "scope_floor" => {
                v["project"]["operations"][0]["params"]["stages"]["finishing"]
                    ["floor_allowance_mm"] = json!(0.2);
            }
            "scope_wall" => {
                v["project"]["operations"][0]["params"]["stages"]["finishing"]["scope"] =
                    json!("floor_only");
            }
            "profile" => {
                let op = &mut v["project"]["operations"][0];
                op["kind"] =
                    json!({"type":"profile","offset":"inside","contour":op["kind"]["contour"]});
            }
            "missing_flute" => {
                v["project"]["tools"][0]
                    .as_object_mut()
                    .unwrap()
                    .remove("flute_length_mm");
            }
            "small_allowance" => {
                v["project"]["operations"][0]["params"]["stages"]["finishing"]
                    ["lateral_allowance_mm"] = json!(0.049);
            }
            "ramp" => {
                v["project"]["operations"][0]["params"]["plunge"] =
                    json!({"kind":"ramp","angle_deg":2.});
            }
            "tabs" => {
                v["project"]["operations"][0]["kind"]["contour"]["tabs"]["active"] = json!(true);
            }
            "trochoidal" => v["project"]["operations"][0]["kind"]["strategy"] = json!("trochoidal"),
            "curve_corner" => {
                v["project"]["segments"][0]["type"] = json!("ARC");
                v["project"]["segments"][0]["bulge"] = json!(0.1);
            }
            "micro_gap" => v["project"]["segments"][4]["end"]["y"] = json!(10.000_001),
            "crossing" => {
                v["project"]["segments"] = json!(loop_segments(&[
                    [10., 10.],
                    [40., 40.],
                    [10., 40.],
                    [40., 10.]
                ]));
            }
            "unreachable" => v["project"]["tools"][0]["diameter"] = json!(100.),
            "different_single_surface" => {
                let mut tool = v["project"]["tools"][0].clone();
                tool["id"] = json!(2);
                tool["diameter"] = json!(1.5);
                v["project"]["tools"].as_array_mut().unwrap().push(tool);
                v["project"]["operations"][0]["finish_tool_id"] = json!(2);
                v["project"]["operations"][0]["params"]["stages"]["finishing"]["cutting"]
                    ["tool_id"] = json!(2);
            }
            "rpm" => {
                v["post_processor"] = json!("cps");
                v["cps_post"] = json!({"source":{"kind":"bundled","id":"shopbot"}});
                v["project"]["operations"][0]["params"]["stages"]["finishing"]["cutting"]["rpm"] =
                    json!(12000);
            }
            _ => unreachable!(),
        }
        let result = serde_json::from_value::<PipelineRequest>(v)
            .map_err(|e| e.to_string())
            .and_then(|r| run_pipeline(r, |_, _, _| {}).map_err(|e| e.to_string()));
        assert!(result.is_err(), "{case} unexpectedly accepted");
    }
}
#[test]
fn combined_different_cutters_and_zero_or_minimum_reserves() {
    for (l, f) in [(0., 0.), (0.05, 0.05), (0.1, 0.2)] {
        let mut v = request("walls_and_floor");
        let mut tool = v["project"]["tools"][0].clone();
        tool["id"] = json!(2);
        tool["diameter"] = json!(1.5);
        v["project"]["tools"].as_array_mut().unwrap().push(tool);
        let op = &mut v["project"]["operations"][0];
        op["finish_tool_id"] = json!(2);
        let finish = &mut op["params"]["stages"]["finishing"];
        finish["cutting"]["tool_id"] = json!(2);
        finish["lateral_allowance_mm"] = json!(l);
        finish["floor_allowance_mm"] = json!(f);
        let result = run_pipeline(serde_json::from_value(v).unwrap(), |_, _, _| {}).unwrap();
        assert_eq!(result.cutting_stages[1].parameters.tool_id, 2);
    }
}

// Independent sampled oracle over actual decoded line/arc motions. It does not
// call production offsets, chord flattening, interval unions or tile coverage.
fn motion_distance(p: [f64; 2], m: &ivac_core::gcode::preview::ToolpathSegment) -> f64 {
    let a = [m.from.x, m.from.y];
    let b = [m.to.x, m.to.y];
    if let Some(arc) = m.arc {
        let angle = |q: [f64; 2]| (q[1] - arc.cy).atan2(q[0] - arc.cx);
        let start = angle(a);
        let end = angle(b);
        let at = angle(p);
        let turn = std::f64::consts::TAU;
        let span = if arc.ccw {
            (end - start).rem_euclid(turn)
        } else {
            (start - end).rem_euclid(turn)
        };
        let delta = if arc.ccw {
            (at - start).rem_euclid(turn)
        } else {
            (start - at).rem_euclid(turn)
        };
        if delta <= span + 1e-10 {
            return ((p[0] - arc.cx).hypot(p[1] - arc.cy) - (a[0] - arc.cx).hypot(a[1] - arc.cy))
                .abs();
        }
        return (p[0] - a[0])
            .hypot(p[1] - a[1])
            .min((p[0] - b[0]).hypot(p[1] - b[1]));
    }
    let d = [b[0] - a[0], b[1] - a[1]];
    let n = d[0] * d[0] + d[1] * d[1];
    let t = if n == 0. {
        0.
    } else {
        (((p[0] - a[0]) * d[0] + (p[1] - a[1]) * d[1]) / n).clamp(0., 1.)
    };
    (p[0] - a[0] - t * d[0]).hypot(p[1] - a[1] - t * d[1])
}
#[test]
fn independent_decoded_circular_sweep_and_boundary_only_oracle() {
    use ivac_core::gcode::preview::MoveKind;
    for post in ["linuxcnc", "grbl", "cps"] {
        for scope in ["walls_only", "floor_only", "walls_and_floor"] {
            let mut v = request(scope);
            v["project"]["segments"] = json!(circle(25., 25., 8.));
            v["project"]["tools"][0]["diameter"] = json!(6.);
            v["post_processor"] = json!(post);
            if post == "cps" {
                v["cps_post"] = json!({"source":{"kind":"bundled","id":"shopbot"}});
            }
            let r = run_pipeline(serde_json::from_value(v).unwrap(), |_, _, _| {}).unwrap();
            let f = &r.cutting_stages[1];
            let finish = &r.toolpath[f.first_segment..f.end_segment_exclusive];
            let at_floor: Vec<_> = r
                .toolpath
                .iter()
                .filter(|m| {
                    m.kind != MoveKind::Rapid
                        && (m.from.z + 0.5).abs() < 0.0001
                        && (m.to.z + 0.5).abs() < 0.0001
                })
                .collect();
            for ix in -32..=32 {
                for iy in -32..=32 {
                    let p = [25. + f64::from(ix) * 0.25, 25. + f64::from(iy) * 0.25];
                    if (p[0] - 25.).hypot(p[1] - 25.) <= 7.98 {
                        assert!(
                            at_floor.iter().any(|m| motion_distance(p, m) <= 3.02),
                            "{post}/{scope}: uncovered {p:?}"
                        );
                    }
                }
            }
            for m in finish
                .iter()
                .filter(|m| m.kind != MoveKind::Rapid && m.from.z < 0. && m.to.z < 0.)
            {
                for q in [m.from, m.to] {
                    let r = (q.x - 25.).hypot(q.y - 25.);
                    assert!(r <= 5.02, "gouge");
                    if scope == "walls_only" {
                        assert!(
                            (r - 5.).abs() <= 0.02,
                            "interior clearing in wall-only stage"
                        );
                    }
                }
            }
            // The oracle independently catches a complete missing final-floor
            // group; points inside the pocket cannot be credited to air motion.
            let absent: Vec<_> = r
                .toolpath
                .iter()
                .filter(|m| m.from.z < -0.6 && m.to.z < -0.6)
                .collect();
            assert!(!absent
                .iter()
                .any(|m| motion_distance([25., 25.], m) <= 3.02));
        }
    }
}
#[test]
fn translated_scaled_and_rotated_contours_retain_scope_semantics() {
    for factor in [0.7, 1., 1.5] {
        let mut v = request("walls_and_floor");
        polygon(&mut v, "convex");
        for s in v["project"]["segments"].as_array_mut().unwrap() {
            for field in ["start", "end"] {
                let x = s[field]["x"].as_f64().unwrap() * factor;
                let y = s[field]["y"].as_f64().unwrap() * factor;
                s[field] = json!({"x":20.+x*0.8-y*0.6,"y":10.+x*0.6+y*0.8});
            }
        }
        run_pipeline(serde_json::from_value(v).unwrap(), |_, _, _| {}).unwrap();
    }
}
#[test]
fn preset_scope_is_part_of_exact_context_and_portable_ancestry() {
    use ivac_core::presets::*;
    let mut req: PipelineRequest = serde_json::from_value(request("walls_and_floor")).unwrap();
    req.project.machine.physical_id = Some("synthetic-router".into());
    req.project.tools[0].physical_id = Some("synthetic-endmill".into());
    req.project.material = Some(ivac_core::project::MaterialIdentity {
        id: "synthetic-plywood".into(),
        revision: "1".into(),
    });
    let op = &req.project.operations[0];
    let collection = PresetCollection {
        format: "fabcam-preset-collection".into(),
        version: 2,
        id: "synthetic".into(),
        name: "Software only".into(),
        presets: vec![TestedPreset {
            id: "test".into(),
            name: "Synthetic attestation fixture, not physical evidence".into(),
            context: context(&req.project, op).unwrap(),
            parameters: parameters(op).unwrap(),
            tested: true,
            test_date: None,
            notes: Some("PRIVATE NOT PORTABLE".into()),
        }],
    };
    assert_eq!(
        parse_collection(&serde_json::to_string(&collection).unwrap()).unwrap(),
        collection
    );
    let loaded = load(&req.project, op, &collection, "test").unwrap();
    assert_eq!(loaded.params.stages, op.params.stages);
    assert!(!serde_json::to_string(&loaded).unwrap().contains("PRIVATE"));
    let mut changed = op.clone();
    if let Some(ivac_core::cutting_stages::MainFinishing::PocketV2 {
        finishing: Some(f), ..
    }) = &mut changed.params.stages
    {
        f.scope = ivac_core::cutting_stages::PocketFinishingScope::WallsOnly;
        f.floor_allowance_mm = 0.;
    }
    assert_ne!(
        context(&req.project, &changed).unwrap().signature,
        collection.presets[0].context.signature
    );
    assert!(load(&req.project, &changed, &collection, "test").is_err());
}

#[derive(Clone)]
struct Sink(std::sync::Arc<std::sync::Mutex<Vec<u8>>>);
impl std::io::Write for Sink {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
#[test]
fn scoped_streams_publish_only_verified_bytes_and_cancel_without_partial_output() {
    use ivac_core::pipeline::{
        generate_streaming, stream_gcode_to_writer, stream_gcode_with_preview, CancelToken,
    };
    for scope in ["walls_only", "floor_only", "walls_and_floor"] {
        let req: PipelineRequest = serde_json::from_value(request(scope)).unwrap();
        let expected = run_pipeline(req.clone(), |_, _, _| {}).unwrap();
        for preview in [false, true] {
            let sink = Sink(std::sync::Arc::default());
            let handle = sink.clone();
            if preview {
                assert_eq!(
                    stream_gcode_with_preview(req.clone(), Box::new(sink))
                        .unwrap()
                        .toolpath,
                    expected.toolpath
                );
            } else {
                stream_gcode_to_writer(req.clone(), Box::new(sink)).unwrap();
            }
            assert_eq!(*handle.0.lock().unwrap(), expected.gcode.as_bytes());
        }
        let cancel = CancelToken::new();
        cancel.cancel();
        assert!(generate_streaming(req, &cancel, &mut |_| {}).is_err());
    }
    let mut v = request("walls_only");
    polygon(&mut v, "concave");
    v["project"]["operations"][0]["kind"]["strategy"] = json!("spiral");
    for preview in [false, true] {
        let sink = Sink(std::sync::Arc::default());
        let handle = sink.clone();
        let req = serde_json::from_value(v.clone()).unwrap();
        if preview {
            assert!(stream_gcode_with_preview(req, Box::new(sink)).is_err());
        } else {
            assert!(stream_gcode_to_writer(req, Box::new(sink)).is_err());
        }
        assert!(handle.0.lock().unwrap().is_empty());
    }
}

#[test]
fn axial_budget_refuses_before_offset_or_motion_construction() {
    let mut v = request("walls_and_floor");
    v["project"]["operations"][0]["params"]["stages"]["main"]["depth_per_pass_mm"] = json!(0.0001);
    let req: PipelineRequest = serde_json::from_value(v).unwrap();
    let error = run_pipeline(req, |_, _, _| {}).unwrap_err().to_string();
    assert!(error.contains("1000 levels"), "{error}");
}
