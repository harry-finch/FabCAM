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
