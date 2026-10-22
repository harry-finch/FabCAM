//! Real CLI process checks; temporary software-only outputs, no machine activity.
use ivac_core::{
    cutting_parameters,
    pipeline::{run_pipeline, PipelineRequest, PostProcessorKind},
};
use std::{fs, path::PathBuf, process::Command};
struct Scratch(PathBuf);
impl Scratch {
    fn new() -> Self {
        let name = format!(
            "fabcam-fs-reset-{}-{}",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );
        let path = std::env::temp_dir().join(name);
        fs::create_dir(&path).unwrap();
        Self(path)
    }
}
impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
#[test]
fn serialized_v9_cli_preserves_output_and_rejects_before_publication() {
    let dir = Scratch::new();
    let input = dir.0.join("request.json");
    let out = dir.0.join("program.ngc");
    let mut req: PipelineRequest = serde_json::from_str(include_str!(
        "../../../tests/fixtures/test1/profile.request.json"
    ))
    .unwrap();
    req.post_processor = Some(PostProcessorKind::Linuxcnc);
    req.cps_post = None;
    let invoke = || {
        Command::new(env!("CARGO_BIN_EXE_ivac"))
            .args(["stream-gcode"])
            .arg(&input)
            .arg("--output")
            .arg(&out)
            .output()
            .unwrap()
    };
    fs::write(&out, "original user output").unwrap();
    fs::write(&input, serde_json::to_vec(&req).unwrap()).unwrap();
    let r = invoke();
    assert!(!r.status.success());
    assert!(String::from_utf8_lossy(&r.stderr).contains("v9"));
    assert_eq!(fs::read_to_string(&out).unwrap(), "original user output");
    req.project.cutting_version = Some(9);
    for op in &mut req.project.operations {
        op.params.cutting = Some(cutting_parameters::seed(op, &req.project.tools));
    }
    let mut missing = req.clone();
    missing.project.operations[0]
        .params
        .cutting
        .as_mut()
        .unwrap()
        .rates[1]
        .rpm = None;
    fs::write(&input, serde_json::to_vec(&missing).unwrap()).unwrap();
    assert!(!invoke().status.success());
    assert_eq!(fs::read_to_string(&out).unwrap(), "original user output");
    fs::write(&input, serde_json::to_vec(&req).unwrap()).unwrap();
    let r = invoke();
    assert!(r.status.success(), "{}", String::from_utf8_lossy(&r.stderr));
    assert_eq!(
        fs::read_to_string(&out).unwrap(),
        run_pipeline(req, |_, _, _| {}).unwrap().gcode
    );
    assert_eq!(
        fs::read_dir(&dir.0).unwrap().count(),
        2,
        "no staging artifacts"
    );
    // Successful streaming checks output bytes, not full-program simulation qualification.
}
