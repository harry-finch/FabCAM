//! QUAL-1B: real host atomic, synchronously signalled at named JS boundaries.
//! Instrumentation uses the EXISTING log binding and is unit-test-only. No new
//! host function, feature switch, public script path or production test hook.
use ivac_cps::{ir::Program, library::bundled, run_post_cancellable, PostError};
use serde_json::json;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc, Mutex,
};
use tracing::{subscriber::with_default, Event, Subscriber};

#[derive(Clone)]
struct Observer {
    flag: Arc<AtomicBool>,
    at: &'static str,
    seen: Arc<Mutex<Vec<String>>>,
}
impl Subscriber for Observer {
    fn enabled(&self, _: &tracing::Metadata<'_>) -> bool {
        true
    }
    fn new_span(&self, _: &tracing::span::Attributes<'_>) -> tracing::span::Id {
        tracing::span::Id::from_u64(1)
    }
    fn record(&self, _: &tracing::span::Id, _: &tracing::span::Record<'_>) {}
    fn record_follows_from(&self, _: &tracing::span::Id, _: &tracing::span::Id) {}
    fn event(&self, event: &Event<'_>) {
        struct Message(String);
        impl tracing::field::Visit for Message {
            fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
                if field.name() == "message" {
                    self.0 = format!("{value:?}");
                }
            }
        }
        let mut message = Message(String::new());
        event.record(&mut message);
        if message.0.starts_with("qual1b:") {
            if message.0 == self.at {
                // A separate native thread signals while the JS callback is active.
                std::thread::scope(|scope| {
                    scope
                        .spawn(|| self.flag.store(true, Ordering::Relaxed))
                        .join()
                        .unwrap();
                });
            }
            self.seen.lock().unwrap().push(message.0);
        }
    }
    fn enter(&self, _: &tracing::span::Id) {}
    fn exit(&self, _: &tracing::span::Id) {}
}

pub(crate) fn program(records: usize) -> Program {
    let mut v = super::shopbot_guards::fixture();
    v["sections"][0]["records"] = json!((0..records)
        .map(|i| json!({"kind":"linear","x":i as f64 / 10.,"y":0,"z":-1,"feed":600,"movement":2}))
        .collect::<Vec<_>>());
    serde_json::from_value(v).unwrap()
}
fn instrument(source: &str, record: usize) -> String {
    source
        .replace("function onOpen() {", "function onOpen() { log('qual1b:onOpen');")
        .replace("function preflight() {", "function preflight() { log('qual1b:preflight');")
        .replace("var r = s.records[j];", &format!("if (j == {record}) log('qual1b:record'); var r = s.records[j];"))
        .replace("preflight(); // whole program checked before any output", "preflight(); log('qual1b:after-preflight'); // whole program checked before any output")
        .replace("comment(labSpindle1 ?", "log('qual1b:first-output'); comment(labSpindle1 ?")
        .replace("function onLinear(x,y,z,f) {", "function onLinear(x,y,z,f) { log('qual1b:emission');")
        + "\nfunction onMachine() { log('qual1b:before-onOpen'); }\n"
}
#[test]
fn cancellation_at_each_shopbot_phase_never_returns_partial_success_and_retry_is_exact() {
    let p = program(129);
    let source = bundled("shopbot").unwrap().source;
    let reference = ivac_cps::run::run_bundled("shopbot", &p, &json!({}), None).unwrap();
    for (at, index, output_started) in [
        ("qual1b:before-onOpen", 65, false),
        ("qual1b:preflight", 65, false),
        ("qual1b:record", 65, false),
        ("qual1b:record", 128, false),
        ("qual1b:after-preflight", 65, false),
        ("qual1b:emission", 65, true),
    ] {
        let flag = Arc::new(AtomicBool::new(false));
        let seen = Arc::new(Mutex::new(Vec::new()));
        let observer = Observer {
            flag: flag.clone(),
            at,
            seen: seen.clone(),
        };
        let result = with_default(observer, || {
            run_post_cancellable(
                &instrument(source, index),
                "shopbot.cps",
                &p,
                &json!({}),
                flag.clone(),
            )
        });
        assert!(flag.load(Ordering::Relaxed), "injection not reached: {at}");
        assert!(
            matches!(result, Err(PostError::Cancelled)),
            "{at}: {result:?}"
        );
        let events = seen.lock().unwrap();
        assert_eq!(
            events.iter().any(|s| s == "qual1b:first-output"),
            output_started,
            "{at}: {events:?}"
        );
        if at == "qual1b:record" && index == 65 {
            assert!(!events.iter().any(|s| s == "qual1b:after-preflight"));
        }
        assert_eq!(
            ivac_cps::run::run_bundled("shopbot", &p, &json!({}), None).unwrap(),
            reference
        );
    }
    // A later cancellation cannot retroactively change an already returned value.
    let late = Arc::new(AtomicBool::new(false));
    let completed = ivac_cps::run::run_bundled("shopbot", &p, &json!({}), Some(&late)).unwrap();
    late.store(true, Ordering::Relaxed);
    assert_eq!(completed, reference);
}

#[test]
fn last_invalid_record_is_still_validated_without_cancellation() {
    let mut v = serde_json::to_value(program(129)).unwrap();
    v["sections"][0]["records"][128] = json!({"kind":"passThrough","text":"SO,1,1"});
    let p = serde_json::from_value(v).unwrap();
    let error = ivac_cps::run::run_bundled("shopbot", &p, &json!({}), None).unwrap_err();
    assert!(matches!(error, PostError::PostRuntime { .. }));
    assert!(error.to_string().contains("unsupported record passThrough"));
}

#[test]
fn nested_preflight_loops_observe_real_cancellation() {
    for mode in ["points", "parameters", "helix"] {
        let mut v = serde_json::to_value(program(200)).unwrap();
        let (anchor, marker) = match mode {
            "points" => {
                v["sections"][0]["records"] = json!([{"kind":"cycle","cycleType":"drilling","params":{"feedrate":600,"clearance":5,"retract":2},"points":(0..200).map(|i| json!({"x":i,"y":0,"z":-1})).collect::<Vec<_>>()}]);
                (
                    "point(r.points[i]);",
                    "if (i == 65) log('qual1b:nested'); point(r.points[i]);",
                )
            }
            "parameters" => {
                v["sections"][0]["parameters"] = json!((0..200)
                    .map(|_| json!({"name":"notes","value":"test"}))
                    .collect::<Vec<_>>());
                (
                    "var param = s.parameters[k];",
                    "if (k == 65) log('qual1b:nested'); var param = s.parameters[k];",
                )
            }
            _ => {
                v["sections"][0]["parameters"] =
                    json!([{"name":"ivacam:entry-strategy","value":"helix"}]);
                (
                    "var motion = s.records[h];",
                    "if (h == 65) log('qual1b:nested'); var motion = s.records[h];",
                )
            }
        };
        let flag = Arc::new(AtomicBool::new(false));
        let seen = Arc::new(Mutex::new(Vec::new()));
        let observer = Observer {
            flag: flag.clone(),
            at: "qual1b:nested",
            seen: seen.clone(),
        };
        let source = instrument(bundled("shopbot").unwrap().source, 65).replace(anchor, marker);
        let result = with_default(observer, || {
            run_post_cancellable(
                &source,
                "shopbot.cps",
                &serde_json::from_value(v).unwrap(),
                &json!({}),
                flag.clone(),
            )
        });
        assert!(
            flag.load(Ordering::Relaxed),
            "{mode}: injection not reached"
        );
        assert!(
            matches!(result, Err(PostError::Cancelled)),
            "{mode}: {result:?}"
        );
        assert!(
            !seen
                .lock()
                .unwrap()
                .iter()
                .any(|s| s == "qual1b:after-preflight"),
            "{mode}: traversed to the end instead of aborting"
        );
    }
}
