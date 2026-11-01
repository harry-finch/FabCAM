use ivac_core::pipeline::*;
use std::sync::{Arc, Mutex};
#[derive(Clone)]
struct Capture(Arc<Mutex<Vec<u8>>>);
impl std::io::Write for Capture {
    fn write(&mut self, b: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(b);
        Ok(b.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
fn request() -> PipelineRequest {
    let mut r: PipelineRequest = serde_json::from_str(include_str!(
        "../../../tests/fixtures/alpha3-v10/profile.request.json"
    ))
    .unwrap();
    r.post_processor = Some(PostProcessorKind::Linuxcnc);
    r.cps_post = None;
    r
}
#[test]
fn cancelled_adapters_refuse_before_output_and_live_adapters_preserve_bytes() {
    let cancel = CancelToken::new();
    cancel.cancel();
    assert!(matches!(
        run_pipeline_cancellable(request(), |_, _, _| {}, &cancel),
        Err(PipelineError::Cancelled)
    ));
    assert!(matches!(
        run_pipeline_two_sided_cancellable(request(), |_, _, _| {}, &cancel),
        Err(PipelineError::Cancelled)
    ));
    let cap = Capture(Arc::default());
    let bytes = cap.0.clone();
    assert!(stream_gcode_to_writer_cancellable(request(), Box::new(cap), &cancel).is_err());
    assert!(bytes.lock().unwrap().is_empty());
    let live = CancelToken::new();
    let baseline = run_pipeline(request(), |_, _, _| {}).unwrap();
    let actual = run_pipeline_cancellable(request(), |_, _, _| {}, &live).unwrap();
    assert_eq!(
        serde_json::to_value(&baseline).unwrap(),
        serde_json::to_value(actual).unwrap()
    );
    let a = Capture(Arc::default());
    let b = Capture(Arc::default());
    let aa = a.0.clone();
    let bb = b.0.clone();
    stream_gcode_to_writer(request(), Box::new(a)).unwrap();
    stream_gcode_to_writer_cancellable(request(), Box::new(b), &live).unwrap();
    assert_eq!(*aa.lock().unwrap(), *bb.lock().unwrap());
}
