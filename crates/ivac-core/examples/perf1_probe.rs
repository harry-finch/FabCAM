//! PERF1-A: identical request, cold-process first run, then warmup/repeated runs.
//! Build with cps[,profiling]. No optimization or cache-policy changes.
use ivac_core::pipeline::{run_pipeline, PipelineRequest};
use serde_json::json;
use std::{cell::RefCell, fs, time::Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 4 {
        return Err(
            "usage: perf1_probe REQUEST.json RESULT.json REPEATS [sample-loop-seconds]".into(),
        );
    }
    let bytes = fs::read(&args[1])?;
    if args.get(4).is_some_and(|s| s == "import") {
        let repeats: usize = args[3].parse()?;
        let mut runs = Vec::new();
        for index in 0..repeats {
            let start = Instant::now();
            let out = ivac_core::input::import_bytes(&args[1], &bytes, &Default::default())?;
            runs.push(json!({"run":index,"import_ms":start.elapsed().as_secs_f64()*1000.,"segments":out.segments.len(),"objects":out.object_meta.len(),"layers":out.layers.len()}));
        }
        fs::write(
            &args[2],
            serde_json::to_vec_pretty(
                &json!({"path":args[1],"bytes":bytes.len(),"boundary":"in-memory input::import_bytes; excludes disk read","runs":runs}),
            )?,
        )?;
        return Ok(());
    }
    if args.get(4).is_some_and(|s| s == "cancel") {
        let request: PipelineRequest = serde_json::from_slice(&bytes)?;
        let cancel = ivac_core::pipeline::CancelToken::new();
        let clone = cancel.clone();
        let start = Instant::now();
        let delay_ms: u64 = args.get(5).map_or(Ok(1), |s| s.parse())?;
        let worker = std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_millis(delay_ms));
            let requested = start.elapsed().as_secs_f64() * 1000.;
            clone.cancel();
            requested
        });
        let result = ivac_core::pipeline::generate_streaming(request, &cancel, &mut |_| {});
        let returned_ms = start.elapsed().as_secs_f64() * 1000.;
        let requested_ms = worker.join().unwrap();
        fs::write(
            &args[2],
            serde_json::to_vec_pretty(
                &json!({"requested_ms":requested_ms,"returned_ms":returned_ms,"latency_ms":returned_ms-requested_ms,"cancelled":matches!(result,Err(ivac_core::pipeline::PipelineError::Cancelled)),"returned_valid_result":result.is_ok()}),
            )?,
        )?;
        return Ok(());
    }
    let started = Instant::now();
    let request: PipelineRequest = serde_json::from_slice(&bytes)?;
    let parse_ms = started.elapsed().as_secs_f64() * 1000.;
    let repeats: usize = args[3].parse()?;
    let sample_seconds: f64 = args.get(4).map_or(Ok(0.), |s| s.parse())?;
    let loop_start = Instant::now();
    let mut runs = Vec::new();
    let mut first_response: Option<Vec<u8>> = None;
    let mut index = 0;
    loop {
        let clone_start = Instant::now();
        let req = request.clone();
        let clone_ms = clone_start.elapsed().as_secs_f64() * 1000.;
        #[cfg(feature = "profiling")]
        ivac_core::profiling::reset();
        let progress = RefCell::new(Vec::new());
        let start = Instant::now();
        let result = run_pipeline(req, |phase, _, message| {
            progress.borrow_mut().push(json!({"phase":phase,"message":message,"at_ms":start.elapsed().as_secs_f64()*1000.}));
        });
        let pipeline_ms = start.elapsed().as_secs_f64() * 1000.;
        #[cfg(feature = "profiling")]
        let spans = serde_json::to_value(ivac_core::profiling::take())?;
        #[cfg(not(feature = "profiling"))]
        let spans = json!([]);
        let measurement = match result {
            Ok(response) => {
                let serialization = Instant::now();
                let serialized = serde_json::to_vec(&response)?;
                let serialize_ms = serialization.elapsed().as_secs_f64() * 1000.;
                let identical = first_response
                    .as_ref()
                    .map_or(true, |first| first == &serialized);
                let record = json!({
                    "run":index,"cache_state":if index==0 {"process-cold"} else if index==1 {"warmup"} else {"warm"},
                    "success":true,"clone_ms":clone_ms,"pipeline_ms":pipeline_ms,"serialize_ms":serialize_ms,
                    "response_bytes":serialized.len(),"program_bytes":response.gcode.len(),
                    "moves":response.toolpath.len(),"objects":response.stats.object_count,
                    "identical_to_first":identical,"warnings":response.warnings.iter().map(|w| &w.kind).collect::<Vec<_>>(),
                    "progress":progress.into_inner(),"spans":spans,
                });
                if first_response.is_none() {
                    first_response = Some(serialized);
                }
                record
            }
            Err(error) => {
                json!({"run":index,"success":false,"pipeline_ms":pipeline_ms,"error":error.to_string(),"progress":progress.into_inner(),"spans":spans})
            }
        };
        runs.push(measurement);
        index += 1;
        if (sample_seconds == 0. && index >= repeats)
            || (sample_seconds > 0. && loop_start.elapsed().as_secs_f64() >= sample_seconds)
        {
            break;
        }
    }
    if let Some(response) = first_response {
        fs::write(format!("{}.response.json", args[2]), response)?;
    }
    let report = json!({"request":args[1],"profile":if cfg!(debug_assertions){"debug"}else{"release"},"instrumented":cfg!(feature="profiling"),"parse_ms":parse_ms,"request_bytes":bytes.len(),"input_segments":request.project.segments.len(),"text_layers":request.project.text_layers.len(),"post":request.post_processor,"runs":runs});
    fs::write(&args[2], serde_json::to_vec_pretty(&report)?)?;
    Ok(())
}
