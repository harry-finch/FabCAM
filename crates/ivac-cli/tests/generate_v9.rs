//! Public-process tests: explicit reviewed values, no physical machine qualification.
use ivac_core::pipeline::{run_pipeline, PipelineRequest};
use serde_json::{json, Value};
use std::{
    fs,
    path::PathBuf,
    process::{Command, Output},
};
static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
struct Scratch(PathBuf);
impl Scratch {
    fn new() -> Self {
        let path = std::env::temp_dir().join(format!(
            "ivac-generate-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        Self(path)
    }
    fn run(&self, request: &Value, args: &[&str]) -> Output {
        let path = self.0.join("request.json");
        fs::write(&path, serde_json::to_vec(request).unwrap()).unwrap();
        Command::new(env!("CARGO_BIN_EXE_ivac"))
            .args(["generate", "--request"])
            .arg(path)
            .args(args)
            .output()
            .unwrap()
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn request() -> Value {
    serde_json::from_str(include_str!(
        "../../../tests/fixtures/cli-v9/profile.request.json"
    ))
    .unwrap()
}
fn success(out: &Output) -> Value {
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    serde_json::from_slice(&out.stdout).expect("stdout must contain only JSON")
}
fn refused(out: &Output) {
    assert!(!out.status.success());
    assert!(out.stdout.is_empty(), "failure emitted stdout");
}
#[test]
fn canonical_posts_match_authoritative_pipeline_and_shopbot_golden() {
    let dir = Scratch::new();
    for post in ["linuxcnc", "grbl", "hpgl", "cps"] {
        if post == "cps" && !cfg!(feature = "cps") {
            continue;
        }
        let mut req = request();
        req["post_processor"] = json!(post);
        if post != "cps" {
            req.as_object_mut().unwrap().remove("cps_post");
        }
        let expected = run_pipeline(
            serde_json::from_value::<PipelineRequest>(req.clone()).unwrap(),
            |_, _, _| {},
        )
        .unwrap();
        let actual = success(&dir.run(&req, &[]));
        assert_eq!(actual["gcode"], expected.gcode);
        assert_eq!(
            actual["tool_provenance"],
            serde_json::to_value(expected.tool_provenance).unwrap()
        );
        assert!(actual.get("stats").is_some() && actual.get("toolpath").is_some());
        if post == "cps" {
            // The historical fixture predates exactly these two provenance comments.
            let gcode = actual["gcode"].as_str().unwrap();
            for marker in ["' FABTOOL/1 REQUEST 0 1\n", "' FABTOOL/1 ACTIVE 0 1\n"] {
                assert_eq!(gcode.matches(marker).count(), 1);
            }
            assert_eq!(
                gcode
                    .replace("' FABTOOL/1 REQUEST 0 1\n", "")
                    .replace("' FABTOOL/1 ACTIVE 0 1\n", ""),
                include_str!("../../../tests/fixtures/test1/profile.sbp")
            );
        }
    }
}
#[test]
fn effective_finish_values_are_not_library_or_rough_defaults() {
    let dir = Scratch::new();
    let mut req = request();
    req["post_processor"] = json!("linuxcnc");
    req.as_object_mut().unwrap().remove("cps_post");
    req["project"]["operations"][0]["params"]["cutting"]["rates"][1] =
        json!({"role":"finish","tool_id":1,"rpm":13000,"feed_mm_min":410,"plunge_mm_min":110});
    let actual = success(&dir.run(&req, &[]));
    let g = actual["gcode"].as_str().unwrap();
    assert!(
        g.contains("S13000") && g.contains("F410") && g.contains("F110"),
        "{g}"
    );
    assert!(!g.contains("S18000") && !g.contains("F1200"));
    assert!(
        !actual["warnings"].as_array().unwrap().is_empty(),
        "unknown limits disclosed"
    );
}
#[test]
fn geometry_template_matches_canonical_import_and_rejects_ambiguous_selection() {
    let dir = Scratch::new();
    let mut req = request();
    req["project"]["segments"] = json!([]);
    req["post_processor"] = json!("linuxcnc");
    req.as_object_mut().unwrap().remove("cps_post");
    let geometry = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../tests/fixtures/shopbot-m1/circle_d50.dxf");
    let path = geometry.to_str().unwrap();
    let actual = success(&dir.run(&req, &[path]));
    let imported =
        ivac_core::input::import_path(&geometry, &ivac_core::input::ImportOptions::default())
            .unwrap();
    let mut canonical = req.clone();
    canonical["project"]["segments"] = serde_json::to_value(imported.segments).unwrap();
    assert_eq!(actual, success(&dir.run(&canonical, &[])));
    refused(&dir.run(&canonical, &[path]));
    for pointer in [
        "/project/machine",
        "/project/operations/0/source",
        "/project/operations/0/tool_id",
        "/project/operations/0/params/depth",
        "/project/operations/0/params/step",
        "/project/operations/0/params/fast_move_z",
    ] {
        let mut bad = req.clone();
        *bad.pointer_mut(pointer).unwrap() = Value::Null;
        refused(&dir.run(&bad, &[path]));
    }
    for source in [
        json!({"kind":"objects","ids":[1]}),
        json!({"kind":"layers","layers":["0"]}),
    ] {
        let mut bad = req.clone();
        bad["project"]["operations"][0]["source"] = source;
        refused(&dir.run(&bad, &[path]));
    }
    let mut bad = req.clone();
    bad["project"]["operations"][0]["enabled"] = json!(false);
    refused(&dir.run(&bad, &[path]));
    let mut bad = req.clone();
    bad["project"]["operations"]
        .as_array_mut()
        .unwrap()
        .push(req["project"]["operations"][0].clone());
    refused(&dir.run(&bad, &[path]));
}
#[test]
fn invalid_rates_roles_limits_and_conflicting_inputs_emit_nothing() {
    let dir = Scratch::new();
    let base = request();
    for value in [
        Value::Null,
        json!(0),
        json!(-1),
        json!(1.5),
        json!(4_294_967_296_u64),
        json!("NaN"),
    ] {
        for field in ["rpm", "feed_mm_min", "plunge_mm_min"] {
            let mut bad = base.clone();
            bad["project"]["operations"][0]["params"]["cutting"]["rates"][1][field] = value.clone();
            refused(&dir.run(&bad, &[]));
        }
    }
    for (pointer, value) in [
        ("/project/cutting_version", json!(8)),
        ("/project/cutting_version", Value::Null),
        ("/project/operations/0/params/cutting", Value::Null),
        ("/project/operations/0/params/step", Value::Null),
        ("/project/tools/0/diameter", json!(0)),
        (
            "/project/operations/0/params/cutting/rates/1/tool_id",
            json!(999),
        ),
        (
            "/project/operations/0/params/cutting/rates/1/role",
            json!("rough"),
        ),
    ] {
        let mut bad = base.clone();
        *bad.pointer_mut(pointer).unwrap() = value;
        refused(&dir.run(&bad, &[]));
    }
    for (object, key, value) in [
        ("machine", "spindle_rpm_max", json!(100)),
        ("machine", "spindle_rpm_min", json!(20000)),
        ("machine", "max_feed_mm_min", json!(100)),
        ("tool", "max_rpm", json!(100)),
        ("tool", "flute_length_mm", json!(0.1)),
    ] {
        let mut bad = base.clone();
        if object == "machine" {
            bad["project"]["machine"][key] = value;
        } else {
            bad["project"]["tools"][0][key] = value;
        }
        refused(&dir.run(&bad, &[]));
    }
    for args in [
        vec!["--post", "grbl"],
        vec!["--depth", "-2"],
        vec!["--request", "other.json"],
        vec!["--post-prop", "x=1"],
        vec!["--outside"],
        vec!["--typo"],
    ] {
        refused(&dir.run(&base, &args));
    }
    let mut bad = base.clone();
    bad["post_processor"] = json!("grbl");
    refused(&dir.run(&bad, &[]));
    let mut bad = base.clone();
    bad["post_procesor"] = json!("grbl");
    refused(&dir.run(&bad, &[]));
    refused(
        &Command::new(env!("CARGO_BIN_EXE_ivac"))
            .args(["generate", "part.dxf"])
            .output()
            .unwrap(),
    );
    let malformed = dir.0.join("bad.json");
    fs::write(&malformed, b"{bad").unwrap();
    refused(
        &Command::new(env!("CARGO_BIN_EXE_ivac"))
            .arg("generate")
            .arg("--request")
            .arg(malformed)
            .output()
            .unwrap(),
    );
}
#[cfg(feature = "cps")]
#[test]
fn external_cps_reaches_security_guard_and_never_executes() {
    let dir = Scratch::new();
    let path = dir.0.join("shopbot.cps");
    for script in [
        "while(true){}",
        "throw new Error('EXECUTED')",
        include_str!("../../ivac-cps/posts/shopbot.cps"),
    ] {
        fs::write(&path, script).unwrap();
        let mut req = request();
        req["cps_post"]["source"] = json!({"kind":"path","path":path});
        let out = dir.run(&req, &[]);
        assert!(
            String::from_utf8_lossy(&out.stderr).contains("External CPS"),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        refused(&out);
    }
}

#[test]
fn duplicate_json_post_authorities_are_rejected() {
    let dir = Scratch::new();
    let path = dir.0.join("duplicate.json");
    let req = serde_json::to_string(&request()).unwrap();
    fs::write(
        &path,
        format!("{{\"post_processor\":\"linuxcnc\",{}", &req[1..]),
    )
    .unwrap();
    refused(
        &Command::new(env!("CARGO_BIN_EXE_ivac"))
            .args(["generate", "--request"])
            .arg(path)
            .output()
            .unwrap(),
    );
}
