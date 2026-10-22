//! Exact-response and invalidation checks with a private cache (no global resets).
use super::*;
use serde_json::{json, Value};

fn fixture() -> Value {
    serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../tests/fixtures/perf-v9/square-outline-cps.json"
    )))
    .unwrap()
}
fn run(value: &Value, cache: Option<&PipelineCache>) -> Result<Value, PipelineError> {
    run_pipeline_impl(
        serde_json::from_value(value.clone()).unwrap(),
        &|_, _, _| {},
        &mut |_| {},
        None,
        cache,
        None,
    )
    .map(|r| serde_json::to_value(r).unwrap())
}
#[cfg(feature = "profiling")]
fn hits() -> usize {
    crate::profiling::take()
        .iter()
        .filter(|r| r.name == "post.cps_cache_hit")
        .map(|r| r.count.unwrap_or(0))
        .sum()
}
#[test]
fn bundled_post_exact_response_invalidation_and_return_to_previous_input() {
    let cache = PipelineCache::new(8);
    let a = fixture();
    let expected = run(&a, None).unwrap();
    #[cfg(feature = "profiling")]
    crate::profiling::reset();
    assert_eq!(run(&a, Some(&cache)).unwrap(), expected);
    #[cfg(feature = "profiling")]
    assert_eq!(hits(), 0);
    assert_eq!(run(&a, Some(&cache)).unwrap(), expected);
    #[cfg(feature = "profiling")]
    assert_eq!(hits(), 1);
    // Each input mutates the actual recorded post program. Validate against
    // an uncached run, not merely a changed-key assertion.
    for (pointer, value) in [
        ("/project/operations/0/params/depth", json!(-0.5)),
        (
            "/project/operations/0/params/cutting/rates/0/feed_mm_min",
            json!(500),
        ),
        ("/project/tools/0/diameter", json!(5)),
        ("/project/operations/0/name", json!("changed operation")),
        ("/project/operations/0/params/fast_move_z", json!(12)),
    ] {
        cache.clear();
        run(&a, Some(&cache)).unwrap();
        let mut b = a.clone();
        *b.pointer_mut(pointer).unwrap() = value;
        let reference = run(&b, None).unwrap();
        #[cfg(feature = "profiling")]
        crate::profiling::reset();
        assert_eq!(run(&b, Some(&cache)).unwrap(), reference);
        #[cfg(feature = "profiling")]
        assert_eq!(hits(), 0);
        assert_eq!(run(&b, Some(&cache)).unwrap(), reference);
        #[cfg(feature = "profiling")]
        assert_eq!(hits(), 1);
        assert_eq!(run(&a, Some(&cache)).unwrap(), expected);
        #[cfg(feature = "profiling")]
        assert_eq!(hits(), 1);
    }
}
#[test]
fn inline_posts_and_property_refusals_cannot_reuse_bundled_result() {
    let cache = PipelineCache::new(8);
    let a = fixture();
    run(&a, Some(&cache)).unwrap();
    let mut b = a.clone();
    b["cps_post"]["properties"] = json!({"labSpindle1Control":true});
    assert!(run(&b, Some(&cache)).is_err());
    assert!(run(&b, Some(&cache)).is_err());
    b = a.clone();
    b["cps_post"]["source"] = json!({"kind":"inline", "filename":"shopbot.cps",
        "script":ivac_cps::library::bundled("shopbot").unwrap().source});
    assert!(run(&b, None)
        .unwrap_err()
        .to_string()
        .contains("External CPS"));
    #[cfg(feature = "profiling")]
    crate::profiling::reset();
    assert!(run(&b, Some(&cache))
        .unwrap_err()
        .to_string()
        .contains("External CPS"));
    assert!(run(&b, Some(&cache))
        .unwrap_err()
        .to_string()
        .contains("External CPS"));
    #[cfg(feature = "profiling")]
    assert_eq!(hits(), 0);
    b["cps_post"]["source"]["script"] = json!("throw new Error('changed script');");
    assert!(run(&b, Some(&cache)).is_err());
}
#[test]
fn cancelled_warm_post_never_returns_valid_output() {
    let cache = PipelineCache::new(8);
    let a = fixture();
    run(&a, Some(&cache)).unwrap();
    let cancel = CancelToken::new();
    let result = run_pipeline_impl(
        serde_json::from_value(a).unwrap(),
        &|phase, _, _| {
            if phase == "post" {
                cancel.cancel();
            }
        },
        &mut |_| {},
        Some(&cancel),
        Some(&cache),
        None,
    );
    assert!(matches!(result, Err(PipelineError::Cancelled)));
}

#[test]
fn remaining_analysis_reuse_preserves_full_results_and_live_stock_guards() {
    let a: Value = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../tests/fixtures/perf-v9/pocket-islands-cps.json"
    )))
    .unwrap();
    let cache = PipelineCache::new(8);
    let expected = run(&a, None).unwrap();
    assert_eq!(expected["remaining_material"][0]["status"], "ready");
    assert_eq!(run(&a, Some(&cache)).unwrap(), expected);
    #[cfg(feature = "profiling")]
    crate::profiling::reset();
    assert_eq!(run(&a, Some(&cache)).unwrap(), expected);
    #[cfg(feature = "profiling")]
    assert!(crate::profiling::take()
        .iter()
        .any(|r| r.name == "preview.remaining_cache_hit"));
    for (pointer, value) in [
        ("/project/stock/width_mm", json!(1)),
        ("/project/stock/thickness_mm", json!(0.2)),
        ("/project/operations/0/params/depth", json!(-0.7)),
        ("/project/tools/0/diameter", json!(4)),
    ] {
        let mut b = a.clone();
        *b.pointer_mut(pointer).unwrap() = value;
        let reference = run(&b, None).unwrap();
        assert_eq!(run(&b, Some(&cache)).unwrap(), reference);
        assert_eq!(run(&b, Some(&cache)).unwrap(), reference);
        assert_eq!(run(&a, Some(&cache)).unwrap(), expected);
    }
    let mut disabled = a.clone();
    disabled["project"]["operations"][0]["enabled"] = json!(false);
    let refusal = run(&disabled, None).unwrap_err().to_string();
    assert_eq!(
        run(&disabled, Some(&cache)).unwrap_err().to_string(),
        refusal
    );
    cache.clear();
    assert_eq!(run(&a, Some(&cache)).unwrap(), expected);
}

#[test]
fn interrupted_cps_execution_has_no_result_or_successful_post_cache_entry() {
    let a: Value = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../tests/fixtures/perf-v9/text-Test-medial-cps.json"
    )))
    .unwrap();
    let cache = PipelineCache::new(8);
    let token = CancelToken::new();
    let worker_token = token.clone();
    let worker = std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(50));
        worker_token.cancel();
    });
    let mut events = Vec::new();
    let result = run_pipeline_impl(
        serde_json::from_value(a.clone()).unwrap(),
        &|_, _, _| {},
        &mut |e| events.push(e),
        Some(&token),
        Some(&cache),
        None,
    );
    worker.join().unwrap();
    assert!(matches!(result, Err(PipelineError::Cancelled)));
    // Retry without cancellation must perform the post and exactly match the
    // uncached reference. A cancelled VM must not have populated the CPS memo.
    #[cfg(feature = "profiling")]
    crate::profiling::reset();
    let retry = run(&a, Some(&cache)).unwrap();
    #[cfg(feature = "profiling")]
    assert_eq!(hits(), 0);
    assert_eq!(retry, run(&a, None).unwrap());
}
