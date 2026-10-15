//! Owned ShopBot post must reject records the generic dispatcher can omit.
use ivac_cps::{ir::Program, library::bundled, run_post};
use serde_json::{json, Value};
fn fixture() -> Value {
    json!({"version":1,"header":{"programName":"M2","programComment":"test", "toleranceMm":0.01,
        "parameters":[{"name":"ivacam:machine-config","value":"{\"mode\":\"mill\",\"tool_change\":\"manual_m0_pause\"}"}, {"name":"ivacam:clearance-z","value":5}]},
        "sections":[{"id":1,"strategy":"contour2d","tool":{"number":1,"diameter":6,"toolType":5,"coolant":0},
            "spindleRpm":18000,"spindleClockwise":true,"workOffset":1,"workPlane":[[1,0,0],[0,1,0],[0,0,1]],
            "initialPosition":{"x":0,"y":0,"z":5},"finalPosition":{"x":10,"y":10,"z":5},
            "records":[{"kind":"rapid","x":10,"y":10,"z":5},{"kind":"linear","x":10,"y":10,"z":-1,"feed":600,"movement":2}]}]})
}
fn run(v: Value) -> Result<ivac_cps::PostOutput, ivac_cps::PostError> {
    let p: Program = serde_json::from_value(v).unwrap();
    run_post(
        bundled("shopbot").unwrap().source,
        "shopbot.cps",
        &p,
        &json!({}),
    )
}
#[test]
fn unsupported_records_fail_before_export() {
    let unsupported = vec![
        json!({"kind":"rapidMachine","x":1}),
        json!({"kind":"passThrough","text":"SO,1,1"}),
        json!({"kind":"spindleSpeed","rpm":20000,"clockwise":true}),
        json!({"kind":"command","command":4}),
        json!({"kind":"coolant","mode":1}),
        json!({"kind":"rapid5d","x":1,"y":1,"z":1,"dx":0,"dy":1,"dz":0}),
        json!({"kind":"circular","clockwise":true,"center":{"x":5,"y":10,"z":5},"end":{"x":0,"y":10,"z":4},"normal":[0,0,1],"feed":600}),
        json!({"kind":"cycle","cycleType":"tapping","params":{},"points":[]}),
        json!({"kind":"linear","x":10,"y":10,"z":-1,"feed":0,"movement":2}),
        json!({"kind":"dwell","seconds":-1}),
    ];
    for r in unsupported {
        let mut v = fixture();
        v["sections"][0]["records"][1] = r;
        let err = run(v).unwrap_err();
        assert!(err.to_string().contains("ShopBot MVP"), "{err}");
        assert!(
            matches!(err, ivac_cps::PostError::PostRuntime { .. }),
            "no partial output export"
        );
    }
}
#[test]
fn scaffold_work_plane_and_tools_fail_closed() {
    for i in 0..8 {
        let mut v = fixture();
        match i {
            0 => v["header"]["parameters"] = json!([]),
            1 => v["header"]["parameters"][1]["value"] = json!(0),
            2 => v["sections"][0]["workOffset"] = json!(2),
            3 => v["sections"][0]["workPlane"][0][0] = json!(-1),
            4 => v["sections"][0]["tool"]["toolType"] = json!(32),
            5 => v["sections"][0]["spindleClockwise"] = json!(false),
            6 => v["machine"] = json!({"axes":[]}),
            _ => v["sections"][0]["strategy"] = json!("thread"),
        }
        assert!(run(v).unwrap_err().to_string().contains("ShopBot MVP"));
    }
}
#[test]
fn positive_output_and_comment_injection() {
    let mut v = fixture();
    v["header"]["programComment"] = json!("test\nSO,1,1\rEND");
    v["sections"][0]["records"]
        .as_array_mut()
        .unwrap()
        .push(json!({"kind":"comment","text":"hostile\nM3,999,999,-999"}));
    let a = run(v.clone()).unwrap();
    let b = run(v).unwrap();
    assert_eq!(a.text, b.text);
    assert_eq!(a.extension, "sbp");
    assert!(a.text.contains("MS,10.000000,10.000000"));
    assert!(!a
        .text
        .lines()
        .any(|l| l == "SO,1,1" || l == "M3,999,999,-999"));
}

fn lab_run(v: Value) -> Result<ivac_cps::PostOutput, ivac_cps::PostError> {
    let p: Program = serde_json::from_value(v).unwrap();
    run_post(
        bundled("shopbot").unwrap().source,
        "shopbot.cps",
        &p,
        &json!({"labSpindle1Control": true}),
    )
}

#[test]
fn lab_spindle_sequence_is_ordered_and_has_no_blocking_setup_pause() {
    let result = lab_run(fixture()).unwrap();
    let text = result.text;
    assert!(text.contains("SA\n' Tool 1 installed"));
    assert!(text.contains("SO,1,0\nJZ,5.0000\nTR,18000\nSO,4,1\nSO,1,1\nPAUSE 2.000\n"));
    assert!(text.ends_with("JZ,5.0000\nSO,1,0\nEND\n"));
    assert!(text.find("SO,1,1").unwrap() < text.find("M3,").unwrap());
    assert!(!text.lines().any(|l| l == "PAUSE"
        || l.starts_with("C9")
        || l.starts_with("CN")
        || l.starts_with("SO,2")
        || l.starts_with("VO,")));
    assert_eq!(text, lab_run(fixture()).unwrap().text);
}

#[test]
fn lab_spindle_rejects_unreviewed_tools_rpm_and_operator_stops() {
    for i in 0..5 {
        let mut v = fixture();
        match i {
            0 => v["sections"][0]["tool"]["number"] = json!(2),
            1 => v["sections"][0]["spindleRpm"] = json!(18001),
            2 => v["sections"][0]["spindleRpm"] = json!(17999.5),
            3 => v["sections"][0]["tool"]["toolType"] = json!(11),
            _ => v["sections"][0]["records"]
                .as_array_mut()
                .unwrap()
                .push(json!({"kind":"command","command":0})),
        }
        assert!(lab_run(v).is_err());
    }
}

#[test]
fn planar_surfacing_is_explicit_flat_only_and_does_not_enable_3d_parallel() {
    let mut surface = fixture();
    surface["sections"][0]["strategy"] = json!("planar_surface");
    assert!(run(surface.clone()).is_ok());
    surface["sections"][0]["tool"]["toolType"] = json!(11); // drill
    assert!(run(surface)
        .unwrap_err()
        .to_string()
        .contains("flat end mill"));
    let mut relief = fixture();
    relief["sections"][0]["strategy"] = json!("parallel");
    assert!(run(relief)
        .unwrap_err()
        .to_string()
        .contains("unsupported strategy parallel"));
}

#[test]
fn vcarve_is_a_narrow_strategy_tool_and_linear_motion_pair() {
    let mut v = fixture();
    v["sections"][0]["strategy"] = json!("vcarve");
    v["sections"][0]["tool"]["toolType"] = json!(8);
    assert!(run(v.clone()).is_ok());
    for strategy in ["engrave", "contour2d", "pocket2d", "milling"] {
        let mut bad = v.clone();
        bad["sections"][0]["strategy"] = json!(strategy);
        assert!(
            run(bad).is_err(),
            "V-bit unexpectedly accepted for {strategy}"
        );
    }
    let mut bad = v.clone();
    bad["sections"][0]["tool"]["toolType"] = json!(5);
    assert!(run(bad).is_err());
    for z in [-1., -2.] {
        let mut bad = v.clone();
        bad["sections"][0]["records"].as_array_mut().unwrap().push(json!({"kind":"circular","clockwise":true,"center":{"x":5,"y":10,"z":-1},"end":{"x":0,"y":10,"z":z},"normal":[0,0,1],"feed":600}));
        assert!(run(bad)
            .unwrap_err()
            .to_string()
            .contains("arcs are unsupported"));
    }
}
