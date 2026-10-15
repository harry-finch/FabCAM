use ivac_cps::{ir::Program, run_post, run_post_cancellable, PostError};
use serde_json::json;
use std::sync::{atomic::AtomicBool, Arc};

fn program() -> Program {
    serde_json::from_value(json!({"version":1,"header":{"programName":"cancel","programComment":"","toleranceMm":0.01,"parameters":[]},"sections":[]})).unwrap()
}
#[test]
fn pre_cancelled_returns_no_output_and_never_parses_script() {
    let flag = Arc::new(AtomicBool::new(true));
    assert!(matches!(
        run_post_cancellable(
            "not valid javascript !",
            "bad.cps",
            &program(),
            &json!({}),
            flag
        ),
        Err(PostError::Cancelled)
    ));
}
#[test]
fn uncancelled_output_matches_legacy_and_error_text_cannot_forge_cancel() {
    let flag = Arc::new(AtomicBool::new(false));
    let script =
        "extension='nc'; function onOpen(){writeln('start');} function onClose(){writeln('end');}";
    assert_eq!(
        run_post(script, "test.cps", &program(), &json!({})).unwrap(),
        run_post_cancellable(script, "test.cps", &program(), &json!({}), flag.clone()).unwrap()
    );
    let script = "function onOpen(){ throw new Error('__IVAC_CANCELLED__'); }";
    assert!(matches!(
        run_post_cancellable(script, "test.cps", &program(), &json!({}), flag),
        Err(PostError::PostRuntime { .. })
    ));
}
