//! Emit the real structured transport error, not the PERF1 debug display string.
use ivac_core::pipeline::{run_pipeline, PipelineRequest};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1).ok_or("expected request path")?;
    let req: PipelineRequest = serde_json::from_slice(&std::fs::read(path)?)?;
    let project = req.project.clone();
    let error = run_pipeline(req, |_, _, _| {}).expect_err("diagnostic requires a refusal");
    let structured = error.to_structured(Some(&project)).ok_or("cancelled")?;
    println!("{}", serde_json::to_string(&structured)?);
    Ok(())
}
