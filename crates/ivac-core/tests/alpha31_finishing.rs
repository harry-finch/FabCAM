//! Synthetic ALPHA-3.1 regression matrix; the accepted ALPHA-3 baseline refuses
//! nonrectangles. Independent decoded-sweep assertions follow the same requests.
use ivac_core::pipeline::{run_pipeline, PipelineRequest, PipelineResponse};
use serde_json::{json, Value};

fn request(shape: &str, strategy: &str, finish: bool) -> Value {
    let shapes: Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/alpha31/polygons.json"
    ))
    .unwrap();
    let points = shapes[shape].as_array().unwrap();
    let mut v: Value = serde_json::from_str(include_str!(
        "../../../tests/fixtures/alpha3-v10/profile.request.json"
    ))
    .unwrap();
    v["post_processor"] = json!("linuxcnc");
    v.as_object_mut().unwrap().remove("cps_post");
    v["project"]["segments"] = Value::Array((0..points.len()).map(|i| {
        let a = &points[i]; let b = &points[(i+1)%points.len()];
        json!({"type":"LINE", "start":{"x":a[0],"y":a[1]},"end":{"x":b[0],"y":b[1]},"layer":"0","color":7})
    }).collect());
    let op = &mut v["project"]["operations"][0];
    if strategy == "inside" || strategy == "outside" {
        op["kind"]["offset"] = json!(strategy);
    } else {
        op["kind"] = json!({"type":"pocket","strategy":strategy,"contour":op["kind"]["contour"],"pocket":{"xy_overlap":0.5}});
    }
    if finish {
        op["finish_tool_id"] = json!(1);
        op["params"]["stages"]["finishing"] = json!({"cutting":{"tool_id":1,"rpm":18000,"feed_mm_min":413,"plunge_mm_min":113,"depth_per_pass_mm":0.2},"lateral_allowance_mm":0.1,"floor_allowance_mm":if strategy=="inside"||strategy=="outside" {0.} else {0.1}});
    }
    v
}
fn run(v: Value) -> Result<PipelineResponse, String> {
    let r: PipelineRequest = serde_json::from_value(v).map_err(|e| e.to_string())?;
    run_pipeline(r, |_, _, _| {}).map_err(|e| e.to_string())
}
#[test]
fn nonrectangular_convex_and_concave_profile_finishing() {
    for shape in [
        "rotated",
        "triangle",
        "convex",
        "concave",
        "irregular_concave",
    ] {
        for side in ["inside", "outside"] {
            let r =
                run(request(shape, side, true)).unwrap_or_else(|e| panic!("{shape}/{side}: {e}"));
            assert_eq!(r.cutting_stages.len(), 2);
        }
    }
}
#[test]
fn nonrectangular_convex_and_concave_pocket_finishing() {
    for shape in [
        "rotated",
        "triangle",
        "convex",
        "concave",
        "irregular_concave",
    ] {
        for strategy in ["cascade", "zigzag", "spiral"] {
            let outcome = run(request(shape, strategy, true));
            let refused = (strategy == "zigzag" && shape != "concave")
                || (strategy == "spiral" && matches!(shape, "concave" | "irregular_concave"));
            if refused {
                let error = outcome.unwrap_err();
                assert!(
                    error.contains(if strategy == "zigzag" {
                        "floor coverage"
                    } else {
                        "proof budget"
                    }),
                    "{shape}/{strategy}: {error}"
                );
            } else {
                let r = outcome.unwrap_or_else(|e| panic!("{shape}/{strategy}: {e}"));
                assert_eq!(r.cutting_stages.len(), 2);
            }
        }
    }
}

// Separate sampled oracle: directly queries exact line/arc decoded motions and
// point-to-original-edge distances. Does not invoke production offsets, chord
// conversion, interval/tile proofs or their geometry helpers.
fn line_distance(p: [f64; 2], a: [f64; 2], b: [f64; 2]) -> f64 {
    let d = [b[0] - a[0], b[1] - a[1]];
    let q = d[0] * d[0] + d[1] * d[1];
    let t = if q == 0. {
        0.
    } else {
        (((p[0] - a[0]) * d[0] + (p[1] - a[1]) * d[1]) / q).clamp(0., 1.)
    };
    (p[0] - a[0] - t * d[0]).hypot(p[1] - a[1] - t * d[1])
}
fn boundary_distance(v: &Value, p: [f64; 2]) -> f64 {
    v["project"]["segments"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| {
            line_distance(
                p,
                [
                    s["start"]["x"].as_f64().unwrap(),
                    s["start"]["y"].as_f64().unwrap(),
                ],
                [
                    s["end"]["x"].as_f64().unwrap(),
                    s["end"]["y"].as_f64().unwrap(),
                ],
            )
        })
        .fold(f64::INFINITY, f64::min)
}
fn inside(v: &Value, p: [f64; 2]) -> bool {
    let mut winding = 0;
    for s in v["project"]["segments"].as_array().unwrap() {
        let ax = s["start"]["x"].as_f64().unwrap();
        let ay = s["start"]["y"].as_f64().unwrap();
        let bx = s["end"]["x"].as_f64().unwrap();
        let by = s["end"]["y"].as_f64().unwrap();
        let cross = (bx - ax) * (p[1] - ay) - (by - ay) * (p[0] - ax);
        if ay <= p[1] && by > p[1] && cross > 0. {
            winding += 1;
        }
        if ay > p[1] && by <= p[1] && cross < 0. {
            winding -= 1;
        }
    }
    winding != 0
}
fn motion_distance(m: &ivac_core::gcode::preview::ToolpathSegment, p: [f64; 2]) -> f64 {
    if let Some(arc) = m.arc {
        let a = (m.from.y - arc.cy).atan2(m.from.x - arc.cx);
        let b = (m.to.y - arc.cy).atan2(m.to.x - arc.cx);
        let t = (p[1] - arc.cy).atan2(p[0] - arc.cx);
        let tau = std::f64::consts::TAU;
        let span = if arc.ccw {
            (b - a).rem_euclid(tau)
        } else {
            (a - b).rem_euclid(tau)
        };
        let query = if arc.ccw {
            (t - a).rem_euclid(tau)
        } else {
            (a - t).rem_euclid(tau)
        };
        if query <= span {
            return ((p[0] - arc.cx).hypot(p[1] - arc.cy)
                - (m.from.x - arc.cx).hypot(m.from.y - arc.cy))
            .abs();
        }
        return (p[0] - m.from.x)
            .hypot(p[1] - m.from.y)
            .min((p[0] - m.to.x).hypot(p[1] - m.to.y));
    }
    line_distance(p, [m.from.x, m.from.y], [m.to.x, m.to.y])
}
fn oracle(v: &Value, r: &PipelineResponse) {
    use ivac_core::gcode::preview::MoveKind;
    let op = &v["project"]["operations"][0];
    let pocket = op["kind"]["type"] == "pocket";
    let outside = op["kind"]["offset"] == "outside";
    let retained = op["params"]["stock_to_leave_mm"].as_f64().unwrap_or(0.);
    for binding in &r.cutting_stages {
        let main = binding.stage == "main";
        let f = &op["params"]["stages"]["finishing"];
        let depth = op["params"]["depth"].as_f64().unwrap()
            + if main {
                f["floor_allowance_mm"].as_f64().unwrap()
            } else {
                0.
            };
        let tool = v["project"]["tools"]
            .as_array()
            .unwrap()
            .iter()
            .find(|t| t["id"] == binding.parameters.tool_id)
            .unwrap();
        let radius = tool["diameter"].as_f64().unwrap() / 2.;
        let inset = radius
            + retained
            + if main {
                f["lateral_allowance_mm"].as_f64().unwrap()
            } else {
                0.
            };
        let moves = &r.toolpath[binding.first_segment..binding.end_segment_exclusive];
        let floor: Vec<_> = moves
            .iter()
            .filter(|m| {
                m.kind != MoveKind::Rapid && m.from.z <= depth + 1e-4 && m.to.z <= depth + 1e-4
            })
            .collect();
        assert!(!floor.is_empty(), "{} reaches final depth", binding.stage);
        for m in moves
            .iter()
            .filter(|m| m.kind != MoveKind::Rapid && m.from.z.min(m.to.z) < 0.)
        {
            assert!(m.from.z.min(m.to.z) >= depth - 1e-4);
            for k in 0..=30 {
                let t = f64::from(k) / 30.;
                let p = if let Some(a) = m.arc {
                    let start = (m.from.y - a.cy).atan2(m.from.x - a.cx);
                    let end = (m.to.y - a.cy).atan2(m.to.x - a.cx);
                    let sweep = if a.ccw {
                        (end - start).rem_euclid(std::f64::consts::TAU)
                    } else {
                        -(start - end).rem_euclid(std::f64::consts::TAU)
                    };
                    let rad = (m.from.x - a.cx).hypot(m.from.y - a.cy);
                    let angle = start + t * sweep;
                    [a.cx + rad * angle.cos(), a.cy + rad * angle.sin()]
                } else {
                    [
                        m.from.x + t * (m.to.x - m.from.x),
                        m.from.y + t * (m.to.y - m.from.y),
                    ]
                };
                assert_ne!(inside(v, p), outside, "wrong compensation side at {p:?}");
                assert!(
                    boundary_distance(v, p) >= inset - 0.02,
                    "gouge at {p:?}, stage {}",
                    binding.stage
                );
            }
        }
        if pocket {
            for ix in 0..=100 {
                for iy in 0..=100 {
                    let p = [5. + 0.5 * f64::from(ix), 5. + 0.5 * f64::from(iy)];
                    if inside(v, p) && boundary_distance(v, p) >= inset + 0.001 {
                        let d = floor
                            .iter()
                            .map(|m| motion_distance(m, p))
                            .fold(f64::INFINITY, f64::min);
                        assert!(
                            d <= radius + 0.02,
                            "unremoved reachable floor at {p:?}: {d}, {}",
                            binding.stage
                        );
                    }
                }
            }
        }
    }
}
#[test]
fn independent_decoded_removal_oracle_for_convex_concave_and_retention() {
    for shape in ["triangle", "convex", "concave", "irregular_concave"] {
        for strategy in ["inside", "outside", "cascade"] {
            let mut v = request(shape, strategy, true);
            v["project"]["operations"][0]["params"]["stock_to_leave_mm"] = json!(0.2);
            let r = run(v.clone()).unwrap_or_else(|e| panic!("{shape}/{strategy}: {e}"));
            oracle(&v, &r);
        }
    }
}
#[test]
fn windings_rotations_translations_scales_and_distinct_pocket_cutter() {
    for shape in ["convex", "concave"] {
        for scale in [0.7, 1.5] {
            for reverse in [false, true] {
                let mut v = request(shape, "cascade", true);
                let segments = v["project"]["segments"].as_array_mut().unwrap();
                for s in &mut *segments {
                    for key in ["start", "end"] {
                        let x = s[key]["x"].as_f64().unwrap() * scale;
                        let y = s[key]["y"].as_f64().unwrap() * scale;
                        s[key] = json!({"x":20.+x*0.8-y*0.6,"y":7.+x*0.6+y*0.8});
                    }
                    if reverse {
                        let a = s["start"].clone();
                        s["start"] = s["end"].clone();
                        s["end"] = a;
                    }
                }
                if reverse {
                    segments.reverse();
                }
                let mut tool = v["project"]["tools"][0].clone();
                tool["id"] = json!(2);
                tool["diameter"] = json!(1.5);
                v["project"]["tools"].as_array_mut().unwrap().push(tool);
                v["project"]["operations"][0]["finish_tool_id"] = json!(2);
                v["project"]["operations"][0]["params"]["stages"]["finishing"]["cutting"]
                    ["tool_id"] = json!(2);
                let result = run(v.clone());
                if shape == "concave" && scale > 1.0 {
                    assert!(result.unwrap_err().contains("proof budget"));
                    continue;
                }
                let r = result.unwrap_or_else(|e| panic!("{shape}/{scale}/{reverse}: {e}"));
                oracle(&v, &r);
                assert_eq!(r.cutting_stages[1].parameters.tool_id, 2);
            }
        }
    }
}

#[test]
fn zero_wall_floor_and_minimum_allowances_remain_distinct() {
    for shape in ["convex", "concave"] {
        for (wall, floor) in [(0., 0.), (0.05, 0.), (0., 0.05), (0.1, 0.1)] {
            let mut v = request(shape, "cascade", true);
            let f = &mut v["project"]["operations"][0]["params"]["stages"]["finishing"];
            f["lateral_allowance_mm"] = json!(wall);
            f["floor_allowance_mm"] = json!(floor);
            let r = run(v.clone()).unwrap();
            oracle(&v, &r);
        }
    }
}
#[test]
fn invalid_topology_holding_curves_and_collapse_are_refused() {
    for change in [
        "gap",
        "cross",
        "island",
        "curve",
        "tabs",
        "approach",
        "collapse",
        "tool",
        "small_allowance",
    ] {
        let mut v = request("convex", "inside", true);
        let segments = v["project"]["segments"].as_array_mut().unwrap();
        match change {
            "gap" => segments[0]["end"]["x"] = json!(36.000_001),
            "cross" => {
                let end = segments[0]["end"].clone();
                let next = segments[2]["start"].clone();
                segments[0]["end"] = next.clone();
                segments[1]["start"] = next;
                segments[1]["end"] = end.clone();
                segments[2]["start"] = end;
            }
            "island" => {
                let mut extra = segments.clone();
                for s in &mut extra {
                    for p in ["start", "end"] {
                        s[p]["x"] = json!(24. + (s[p]["x"].as_f64().unwrap() - 24.) * 0.15);
                        s[p]["y"] = json!(24. + (s[p]["y"].as_f64().unwrap() - 24.) * 0.15);
                    }
                }
                segments.extend(extra);
            }
            "curve" => {
                segments[0]["type"] = json!("ARC");
                segments[0]["bulge"] = json!(0.2);
            }
            _ => {}
        }
        let op = &mut v["project"]["operations"][0];
        match change {
            "tabs" => op["kind"]["contour"]["tabs"]["active"] = json!(true),
            "approach" => op["kind"]["contour"]["leads"]["in"] = json!("straight"),
            "collapse" => op["params"]["stock_to_leave_mm"] = json!(30.),
            "tool" => op["params"]["stages"]["finishing"]["cutting"]["tool_id"] = json!(99),
            "small_allowance" => {
                op["params"]["stages"]["finishing"]["lateral_allowance_mm"] = json!(0.049);
            }
            _ => {}
        }
        assert!(run(v).is_err(), "{change} must refuse");
    }
}

#[derive(Clone)]
struct Sink(std::sync::Arc<std::sync::Mutex<Vec<u8>>>);
impl std::io::Write for Sink {
    fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(b);
        Ok(b.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
#[test]
fn streaming_uses_verified_bytes_and_refuses_before_writing() {
    use ivac_core::pipeline::{stream_gcode_to_writer, stream_gcode_with_preview};
    for shape in ["convex", "concave"] {
        let v = request(shape, "cascade", true);
        let expected = run(v.clone()).unwrap();
        for preview in [false, true] {
            let sink = Sink(std::sync::Arc::default());
            let handle = sink.clone();
            let req = serde_json::from_value(v.clone()).unwrap();
            if preview {
                let r = stream_gcode_with_preview(req, Box::new(sink)).unwrap();
                assert_eq!(r.toolpath, expected.toolpath);
            } else {
                stream_gcode_to_writer(req, Box::new(sink)).unwrap();
            }
            assert_eq!(*handle.0.lock().unwrap(), expected.gcode.as_bytes());
        }
    }
    for preview in [false, true] {
        let sink = Sink(std::sync::Arc::default());
        let handle = sink.clone();
        let req = serde_json::from_value(request("convex", "zigzag", true)).unwrap();
        if preview {
            assert!(stream_gcode_with_preview(req, Box::new(sink)).is_err());
        } else {
            assert!(stream_gcode_to_writer(req, Box::new(sink)).is_err());
        }
        assert!(handle.0.lock().unwrap().is_empty());
    }
}

#[cfg(feature = "cps")]
#[test]
fn polygon_shopbot_decoding_and_same_tool_rpm_guard() {
    for shape in ["convex", "concave"] {
        for kind in ["inside", "outside", "cascade"] {
            let mut v = request(shape, kind, true);
            v["post_processor"] = json!("cps");
            v["cps_post"] = json!({"source":{"kind":"bundled","id":"shopbot"}});
            let r = run(v.clone()).unwrap_or_else(|e| panic!("{shape}/{kind}: {e}"));
            oracle(&v, &r);
            assert!(r.gcode.contains("FABSTAGE_1 1 finishing"));
            if kind == "cascade" {
                let mut dual = v.clone();
                let mut tool = dual["project"]["tools"][0].clone();
                tool["id"] = json!(2);
                tool["diameter"] = json!(1.5);
                dual["project"]["tools"].as_array_mut().unwrap().push(tool);
                let op = &mut dual["project"]["operations"][0];
                op["finish_tool_id"] = json!(2);
                op["params"]["stages"]["finishing"]["cutting"]["tool_id"] = json!(2);
                op["params"]["stages"]["finishing"]["cutting"]["rpm"] = json!(19000);
                let outcome = run(dual.clone());
                if shape == "concave" {
                    // This smaller cutter expands the L-shaped clearing paths
                    // beyond the proof ceiling. Never relax it for CPS.
                    assert!(outcome.unwrap_err().contains("proof budget"));
                } else {
                    let output = outcome.unwrap();
                    oracle(&dual, &output);
                    assert_eq!(output.cutting_stages[1].parameters.tool_id, 2);
                    assert_eq!(output.cutting_stages[1].parameters.rpm, Some(19000));
                }
            }
            v["project"]["operations"][0]["params"]["stages"]["finishing"]["cutting"]["rpm"] =
                json!(19000);
            assert!(run(v).unwrap_err().contains("spindleSpeed"));
        }
    }
}

#[test]
fn polygon_work_origin_and_axial_stage_schedules_remain_authoritative() {
    let mut v = request("convex", "cascade", true);
    v["project"]["work_offset"] = json!({"x_mm":3.,"y_mm":2.,"z_mm":0.,"wcs":"G54"});
    let r = run(v.clone()).unwrap();
    // Resolve the independent oracle's source into the same work frame.
    for s in v["project"]["segments"].as_array_mut().unwrap() {
        for p in ["start", "end"] {
            s[p]["x"] = json!(s[p]["x"].as_f64().unwrap() - 3.);
            s[p]["y"] = json!(s[p]["y"].as_f64().unwrap() - 2.);
        }
    }
    oracle(&v, &r);
    for binding in &r.cutting_stages {
        let mut levels = Vec::new();
        for m in &r.toolpath[binding.first_segment..binding.end_segment_exclusive] {
            if m.to.z < 0. && (m.to.x - m.from.x).hypot(m.to.y - m.from.y) > 1e-8 {
                let z = format!("{:.4}", m.to.z);
                if !levels.contains(&z) {
                    levels.push(z);
                }
            }
        }
        assert_eq!(
            levels,
            if binding.stage == "main" {
                vec!["-0.2500", "-0.4000"]
            } else {
                vec!["-0.2000", "-0.4000", "-0.5000"]
            }
        );
    }
}
