//! Explicit manual benchmark. Counters compile out of production; no public API.
use crate::pipeline::{clear_pipeline_cache, run_pipeline, PipelineRequest, PipelineResponse};
use std::{cell::Cell, path::PathBuf, time::Instant};
thread_local! {
    static WORK: Cell<Option<[usize; 3]>> = const { Cell::new(None) };
}
pub(super) fn record_work(charged: usize, current: usize) {
    WORK.with(|slot| {
        if let Some([sum, calls, peak]) = slot.get() {
            slot.set(Some([sum + charged, calls + 1, peak.max(current)]));
        }
    });
}
#[test]
#[ignore = "explicit PERF1 benchmark; requires original hash-bound baseline responses"]
fn measured_finishing_proof_baseline_or_candidate() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let out = std::env::var("PERF1_NATIVE_PROOF_OUT").expect("benchmark output path");
    let mut rows = Vec::new();
    let mut fixtures: Vec<_> = std::fs::read_dir(root.join("tests/fixtures/perf1"))
        .unwrap()
        .map(|p| p.unwrap().path())
        .collect();
    fixtures.sort();
    for fixture in fixtures {
        let name = fixture.file_stem().unwrap().to_str().unwrap();
        let request: PipelineRequest =
            serde_json::from_slice(&std::fs::read(&fixture).unwrap()).unwrap();
        let expected: serde_json::Value = serde_json::from_slice(
            &std::fs::read(root.join(format!(
                "dist-test/perf-1/native-baseline/{name}.response.json"
            )))
            .unwrap(),
        )
        .unwrap();
        let mut times = Vec::new();
        let mut work = Vec::new();
        for n in 0..8 {
            clear_pipeline_cache();
            WORK.with(|slot| slot.set(Some([0; 3])));
            let start = Instant::now();
            let result = run_pipeline(request.clone(), |_, _, _| {});
            let elapsed = start.elapsed().as_secs_f64() * 1000.;
            let counters = WORK.with(|slot| slot.take().unwrap());
            assert_eq!(result.is_ok(), expected.get("error").is_none(), "{name}");
            if let Ok(response) = &result {
                assert_eq!(serde_json::to_value(response).unwrap(), expected, "{name}");
            }
            if n > 0 {
                times.push(elapsed);
                work.push(counters);
            }
        }
        let mut decoded = Vec::new();
        if expected.get("error").is_none() {
            let response: PipelineResponse = serde_json::from_value(expected).unwrap();
            let objects = crate::cam::chaining::segments_to_objects(&request.project.segments);
            for _ in 0..7 {
                let start = Instant::now();
                super::pocket::verify_decoded(
                    &request.project,
                    &request.project.operations[0],
                    &objects,
                    &response.toolpath,
                    &response.cutting_stages,
                    None,
                )
                .unwrap();
                decoded.push(start.elapsed().as_secs_f64() * 1000.);
            }
        }
        rows.push(serde_json::json!({"name":name,"pipeline_ms":times,"logical_work_sum_calls_peak":work,"decoded_proof_ms":decoded}));
    }
    std::fs::write(out, serde_json::to_vec_pretty(&rows).unwrap()).unwrap();
}

#[test]
#[ignore = "explicit PERF1 chaining baseline; no production chaining changes"]
fn measured_existing_chaining_scaling() {
    use crate::{
        cam::chaining::segments_to_objects_indexed,
        geometry::{Point2, Segment},
    };
    let out = std::env::var("PERF1_CHAIN_OUT").expect("benchmark output path");
    let mut rows = Vec::new();
    for count in [100, 1000, 5000, 10_000] {
        let mut source = Vec::new();
        for i in 0..count / 4 {
            let x = f64::from(i % 50) * 4.;
            let y = f64::from(i / 50) * 4.;
            let p = [
                Point2::new(x, y),
                Point2::new(x + 1., y),
                Point2::new(x + 1., y + 1.),
                Point2::new(x, y + 1.),
            ];
            for n in 0..4 {
                source.push(Segment::line(p[n], p[(n + 1) % 4], "synthetic", 7));
            }
        }
        for variant in [
            "ordered",
            "random",
            "reversed",
            "micro-gap",
            "near-endpoints",
            "competing",
            "open",
        ] {
            let mut input = source.clone();
            if variant == "random" {
                let mut seed = 11_u64;
                for i in (1..input.len()).rev() {
                    seed = seed.wrapping_mul(6_364_136_223_846_793_005).wrapping_add(1);
                    let j = usize::try_from(seed % u64::try_from(i + 1).unwrap()).unwrap();
                    input.swap(i, j);
                }
            }
            if variant == "reversed" {
                input.reverse();
                for s in &mut input {
                    std::mem::swap(&mut s.start, &mut s.end);
                }
            }
            if variant == "micro-gap" {
                for s in input.iter_mut().skip(3).step_by(4) {
                    s.end.x += 0.00001;
                }
            }
            if variant == "near-endpoints" {
                for s in input.iter_mut().step_by(4) {
                    s.start.y += 0.00001;
                }
            }
            if variant == "open" {
                input = input
                    .into_iter()
                    .enumerate()
                    .filter(|(i, _)| i % 4 != 3)
                    .map(|(_, s)| s)
                    .collect();
            }
            if variant == "competing" {
                input.push(Segment::line(
                    Point2::new(0., 0.),
                    Point2::new(-1., 0.),
                    "synthetic",
                    7,
                ));
            }
            let original = serde_json::to_value(&input).unwrap();
            let expected = segments_to_objects_indexed(&input);
            let mut times = Vec::new();
            for _ in 0..7 {
                let start = Instant::now();
                let actual = segments_to_objects_indexed(&input);
                times.push(start.elapsed().as_secs_f64() * 1000.);
                assert_eq!(
                    serde_json::to_value(&actual).unwrap(),
                    serde_json::to_value(&expected).unwrap()
                );
            }
            assert_eq!(serde_json::to_value(&input).unwrap(), original);
            rows.push(serde_json::json!({"requested_segments":count,"actual_segments":input.len(),"variant":variant,"times_ms":times,"objects":expected.len(),"closed":expected.iter().filter(|(o,_)|o.closed).count(),"ordered_membership":expected.iter().map(|(_,ids)|ids).collect::<Vec<_>>()}));
        }
    }
    std::fs::write(out, serde_json::to_vec_pretty(&rows).unwrap()).unwrap();
}
