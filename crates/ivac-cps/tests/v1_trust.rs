use ivac_cps::{
    inspect_post, inspect_post_with_limits, run_post, run_post_cancellable, run_post_with_limits,
    PostError, RunLimits,
};
use std::sync::{atomic::AtomicBool, Arc};
#[test]
fn every_external_entry_refuses_before_parsing_or_evaluation() {
    let program = serde_json::from_value(serde_json::json!({"version":1,"header":{"programName":"test","programComment":"","toleranceMm":0.01,"parameters":[]},"sections":[]})).unwrap();
    for script in [
        "while(true){}",
        "throw new Error('EXECUTED')",
        "not javascript !",
        ivac_cps::library::bundled("shopbot").unwrap().source,
    ] {
        for name in ["shopbot.cps", "grbl", "renamed.cps"] {
            assert!(matches!(
                inspect_post(script, name),
                Err(PostError::UntrustedPost)
            ));
            assert!(matches!(
                inspect_post_with_limits(script, name, RunLimits::default()),
                Err(PostError::UntrustedPost)
            ));
            assert!(matches!(
                run_post(script, name, &program, &serde_json::json!({})),
                Err(PostError::UntrustedPost)
            ));
            assert!(matches!(
                run_post_with_limits(
                    script,
                    name,
                    &program,
                    &serde_json::json!({}),
                    RunLimits::default()
                ),
                Err(PostError::UntrustedPost)
            ));
            assert!(matches!(
                run_post_cancellable(
                    script,
                    name,
                    &program,
                    &serde_json::json!({}),
                    Arc::new(AtomicBool::new(false))
                ),
                Err(PostError::UntrustedPost)
            ));
        }
    }
}
#[test]
fn registry_is_the_only_authority() {
    for id in ["shopbot", "grbl"] {
        assert!(ivac_cps::run::inspect_bundled(id, RunLimits::default()).is_ok());
    }
    for id in [
        "shopbot.cps",
        "../posts/shopbot",
        "SHOPBOT",
        "while(true){}",
    ] {
        assert!(matches!(
            ivac_cps::run::inspect_bundled(id, RunLimits::default()),
            Err(PostError::UntrustedPost)
        ));
    }
}

#[test]
fn properties_are_data_and_prototype_keys_are_refused() {
    let program = serde_json::from_value(serde_json::json!({"version":1,"header":{"programName":"test","programComment":"","toleranceMm":0.01,"parameters":[]},"sections":[]})).unwrap();
    for properties in [
        serde_json::json!({"__proto__":"code"}),
        serde_json::json!({"constructor":"code"}),
        serde_json::json!({"prototype":true}),
        serde_json::json!({"x":{"code":"evil"}}),
    ] {
        assert!(matches!(
            ivac_cps::run::run_bundled("grbl", &program, &properties, None),
            Err(PostError::Contract(_))
        ));
    }
    let output = ivac_cps::run::run_bundled(
        "grbl",
        &program,
        &serde_json::json!({"unknown":"throw new Error('EXECUTED')"}),
        None,
    )
    .unwrap();
    assert!(!output.text.contains("EXECUTED"));
}
