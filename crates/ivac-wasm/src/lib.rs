//! ivaCAM WASM bindings — exposes the same JSON contract the HTTP
//! and Tauri transports speak so the frontend can run the entire CAM
//! pipeline in-browser without any server.
//!
//! Built with `wasm-pack build crates/ivac-wasm --target web --release`.
//! The resulting `pkg/` ships JS glue + a `ivac_wasm_bg.wasm` blob that the
//! Vite frontend can import directly.
//!
//! ## Streaming + cancellation
//!
//! WASM threading (web workers + COOP/COEP) is out of scope for v1.
//! `generate_streaming` runs synchronously on the JS event loop and
//! invokes the supplied callback once per [`ivac_core::pipeline::PipelineEvent`].
//! The callback may flip a shared cancel flag (returned at start of
//! the call) to bail out of long inner loops without blocking the
//! JS task queue.

#![forbid(unsafe_code)]
// Every #[wasm_bindgen] entry returns `JsValue` errors that surface
// to JS as a thrown Error with the message attached. The Rust-side
// `# Errors` doc would be redundant — the JS caller already sees the
// message via the standard try/catch. Whole crate gets the allow.
#![allow(clippy::missing_errors_doc)]

use serde::Serialize;
use wasm_bindgen::prelude::*;

use ivac_core::input::text::{render_text_api, render_text_layer_api, RenderTextRequest};
use ivac_core::pipeline::{
    generate_streaming, run_pipeline, run_pipeline_two_sided, CancelToken, PipelineRequest,
};
use ivac_core::project::TextLayer;
use ivac_core::{
    compute_helix_radius as core_compute_helix_radius, HelixRadiusRequest, ImportOptions,
};

pub mod sim;

#[wasm_bindgen(start)]
pub fn start() {
    // Surface Rust panics in the JS console with stack traces. Silently
    // becomes a no-op if the hook is already installed.
    console_error_panic_hook::set_once();
}

#[derive(Serialize)]
struct HealthResponse {
    ok: bool,
}

#[derive(Serialize)]
struct VersionResponse<'a> {
    version: &'a str,
    transport: &'a str,
    git_sha: Option<&'a str>,
    /// Feature probe, same vocabulary as the server's `/version`
    /// (`post-<dialect>`, …) so the frontend has ONE uniform check.
    capabilities: Vec<&'a str>,
}

#[wasm_bindgen]
pub fn healthz() -> Result<JsValue, JsValue> {
    serde_wasm_bindgen::to_value(&HealthResponse { ok: true }).map_err(into_js_error)
}

#[wasm_bindgen]
pub fn version() -> Result<JsValue, JsValue> {
    #[allow(unused_mut)]
    let mut capabilities = vec![
        "movement-tool-provenance-1",
        "import-dxf",
        "generate-gcode",
        "feeds-speeds",
        "post-linuxcnc",
        "post-grbl",
        "post-hpgl",
    ];
    #[cfg(feature = "cps")]
    capabilities.push("post-cps");
    let v = VersionResponse {
        version: env!("CARGO_PKG_VERSION"),
        transport: "wasm",
        git_sha: option_env!("GIT_SHA"),
        capabilities,
    };
    serde_wasm_bindgen::to_value(&v).map_err(into_js_error)
}

/// Bundled `.cps` posts with their inspected metadata.
#[cfg(feature = "cps")]
#[wasm_bindgen(js_name = listPosts)]
pub fn list_posts() -> Result<JsValue, JsValue> {
    guard(|| {
        let entries = ivac_cps::library::BUNDLED
            .iter()
            .map(|post| {
                ivac_cps::run::inspect_bundled(post.id, ivac_cps::RunLimits::default()).map(
                    |meta| ivac_cps::meta::PostListEntry {
                        id: post.id.to_string(),
                        meta,
                    },
                )
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| into_js_error(format!("bundled post failed inspection: {e}")))?;
        serde_wasm_bindgen::to_value(&entries).map_err(into_js_error)
    })
}

/// Inspect a user-supplied `.cps` script → `PostMeta` for the
/// properties form.
#[cfg(feature = "cps")]
#[wasm_bindgen(js_name = inspectPost)]
pub fn inspect_post(script: &str, filename: Option<String>) -> Result<JsValue, JsValue> {
    guard(|| {
        let name = filename.unwrap_or_else(|| "inline.cps".to_string());
        let meta =
            ivac_cps::inspect_post(script, &name).map_err(|e| into_js_error(e.to_string()))?;
        serde_wasm_bindgen::to_value(&meta).map_err(into_js_error)
    })
}

/// Import a DXF/SVG/HPGL byte buffer. The web client sends `File`
/// contents as a Uint8Array and provides the filename so the Rust core
/// can match the format detector.
#[wasm_bindgen(js_name = importBytes)]
pub fn import_bytes(filename: &str, bytes: &[u8], scale: Option<f64>) -> Result<JsValue, JsValue> {
    guard(|| {
        // Explicit source-unit choice (ADR-0026), mm per source unit.
        let scale = match scale {
            Some(v) => ivac_core::input::checked_import_scale(v).map_err(structured_error_to_js)?,
            None => 0.0,
        };
        let opts = ImportOptions {
            scale,
            ..ImportOptions::default()
        };
        let out = ivac_core::input::import_bytes(filename, bytes, &opts)
            .map_err(structured_error_to_js)?;
        serde_wasm_bindgen::to_value(&out).map_err(into_js_error)
    })
}

/// Rasterize an STL byte stream (binary or ASCII) into a relief height
/// grid. Returns the serialized [`ivac_core::cam::surface::SurfaceField`]
/// (`{origin, cell, cols, rows, z}` — real target Z per cell, the model top
/// shifted to the stock top 0), or JS `null` when the mesh has no XY
/// footprint to sample. `maxDim` bounds the grid so the longer XY side spans
/// at most that many cells. The frontend stores the result as a
/// `heightgrid` [`ivac_core::project::ReliefSource`] the `relief_mill` op
/// then surfaces. Called at load-time, mirroring the client-side image
/// decode — the STL bytes never enter the project JSON.
#[wasm_bindgen(js_name = fromStl)]
pub fn from_stl(bytes: &[u8], max_dim: u32) -> Result<JsValue, JsValue> {
    guard(|| {
        match ivac_core::cam::surface::SurfaceField::from_stl_capped(bytes, max_dim)
            .map_err(|e| structured_error_to_js(ivac_core::Error::bad_input(e.to_string())))?
        {
            Some(field) => serde_wasm_bindgen::to_value(&field).map_err(into_js_error),
            None => Ok(JsValue::NULL),
        }
    })
}

#[wasm_bindgen]
pub fn generate(request: JsValue) -> Result<JsValue, JsValue> {
    let req: PipelineRequest = serde_wasm_bindgen::from_value(request).map_err(into_js_error)?;
    let project = req.project.clone();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| generate_request(req)));
    match result {
        Ok(Ok(resp)) => serde_wasm_bindgen::to_value(&resp).map_err(into_js_error),
        Ok(Err(e)) => match e.to_structured(Some(&project)) {
            Some(structured) => Err(structured_error_to_js(structured)),
            None => Err(JsValue::from_str("cancelled")),
        },
        Err(panic) => Err(structured_error_to_js(
            ivac_core::Error::internal(format!("panic: {}", panic_message(&panic)))
                .with_hint("Please report this bug — see the toast for details."),
        )),
    }
}

/// Two-sided (flip-stock) generate: returns `{ front, back }` — the back
/// program present only for a two-sided job (mirrored geometry + flip/re-zero
/// header). A single-sided project comes back with `back: null` and a front
/// identical to [`generate`].
#[wasm_bindgen(js_name = generateTwoSided)]
pub fn generate_two_sided(request: JsValue) -> Result<JsValue, JsValue> {
    let req: PipelineRequest = serde_wasm_bindgen::from_value(request).map_err(into_js_error)?;
    let project = req.project.clone();
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        run_pipeline_two_sided(req, |_phase, _fraction, _msg| {})
    }));
    match result {
        Ok(Ok(resp)) => serde_wasm_bindgen::to_value(&resp).map_err(into_js_error),
        Ok(Err(e)) => match e.to_structured(Some(&project)) {
            Some(structured) => Err(structured_error_to_js(structured)),
            None => Err(JsValue::from_str("cancelled")),
        },
        Err(panic) => Err(structured_error_to_js(
            ivac_core::Error::internal(format!("panic: {}", panic_message(&panic)))
                .with_hint("Please report this bug — see the toast for details."),
        )),
    }
}

/// Streaming variant: invokes `on_event` once per
/// [`ivac_core::pipeline::PipelineEvent`] as the pipeline runs. WASM v1
/// is single-threaded — the JS callback cannot actually flip the cancel
/// flag mid-run because the Rust call holds the event loop. The
/// streaming shape exists so the frontend's progress UI updates per
/// op without an extra synthetic-events shim. Cancel support arrives
/// with web-worker threading in v2.
// `js_sys::Function` is exchange-by-value at the wasm-bindgen ABI; taking
// `&Function` would force callers (and wasm-bindgen's generated JS glue)
// to keep a stable reference, which the SSE-callback pattern doesn't.
#[allow(clippy::needless_pass_by_value)]
#[wasm_bindgen(js_name = generateStreaming)]
pub fn generate_streaming_wasm(
    request: JsValue,
    on_event: js_sys::Function,
) -> Result<JsValue, JsValue> {
    let req: PipelineRequest = serde_wasm_bindgen::from_value(request).map_err(into_js_error)?;
    let cancel = CancelToken::new();
    let project = req.project.clone();
    let mut sink = |ev| {
        if let Ok(js) = serde_wasm_bindgen::to_value(&ev) {
            let _ = on_event.call1(&JsValue::NULL, &js);
        }
    };
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        generate_streaming(req, &cancel, &mut sink)
    }));
    match result {
        Ok(Ok(resp)) => serde_wasm_bindgen::to_value(&resp).map_err(into_js_error),
        Ok(Err(ivac_core::pipeline::PipelineError::Cancelled)) => Ok(JsValue::NULL),
        Ok(Err(e)) => match e.to_structured(Some(&project)) {
            Some(structured) => Err(structured_error_to_js(structured)),
            None => Ok(JsValue::NULL),
        },
        Err(panic) => Err(structured_error_to_js(
            ivac_core::Error::internal(format!("panic: {}", panic_message(&panic)))
                .with_hint("Please report this bug — see the toast for details."),
        )),
    }
}

#[wasm_bindgen(js_name = renderText)]
pub fn render_text(request: JsValue) -> Result<JsValue, JsValue> {
    let req: RenderTextRequest = serde_wasm_bindgen::from_value(request).map_err(into_js_error)?;
    // Guarded: parses untrusted font bytes (ttf-parser / svg_font), which
    // can panic on malformed input.
    guard(|| {
        let resp = render_text_api(&req).map_err(structured_error_to_js)?;
        serde_wasm_bindgen::to_value(&resp).map_err(into_js_error)
    })
}

#[wasm_bindgen(js_name = renderTextLayer)]
pub fn render_text_layer(layer: JsValue) -> Result<JsValue, JsValue> {
    let layer: TextLayer = serde_wasm_bindgen::from_value(layer).map_err(into_js_error)?;
    // Guarded: same untrusted-font-bytes parsing as `render_text`.
    guard(|| {
        let resp = render_text_layer_api(&layer).map_err(structured_error_to_js)?;
        serde_wasm_bindgen::to_value(&resp).map_err(into_js_error)
    })
}

#[wasm_bindgen(js_name = computeHelixRadius)]
pub fn compute_helix_radius(request: JsValue) -> Result<JsValue, JsValue> {
    let req: HelixRadiusRequest = serde_wasm_bindgen::from_value(request).map_err(into_js_error)?;
    let resp = core_compute_helix_radius(req);
    serde_wasm_bindgen::to_value(&resp).map_err(into_js_error)
}

pub(crate) fn into_js_error<E: std::fmt::Display>(err: E) -> JsValue {
    JsValue::from_str(&err.to_string())
}

/// Run `f` under a panic guard so a panic in a transitive parser
/// (dxf-rs / usvg / ttf font shaping) on malformed user input surfaces as a
/// structured JS error instead of aborting the whole wasm instance — the
/// same protection `generate` / `generate_streaming` already have. Wraps
/// the entry points that take untrusted file / font bytes. `AssertUnwindSafe`
/// is sound here: the wasm module is single-threaded and the closure owns
/// no state we read again after a panic.
fn guard<T>(f: impl FnOnce() -> Result<T, JsValue>) -> Result<T, JsValue> {
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(f)) {
        Ok(r) => r,
        Err(panic) => Err(structured_error_to_js(
            ivac_core::Error::internal(format!("panic: {}", panic_message(&panic)))
                .with_hint("Please report this bug — see the toast for details."),
        )),
    }
}

/// Serialize a structured `ivac_core::Error` to a JS value the frontend
/// can detect (object with a `kind` field) and render through
/// `ErrorToast.svelte`. Falls back to the message string if serialization
/// somehow fails — the frontend's plain-string path catches that.
// Takes `err` by value because every caller produces it from a fallible
// step (`map_err(structured_error_to_js)`) and drops it after the JsValue
// is built — borrowing would force the call sites to keep the Error
// alive past the conversion.
#[allow(clippy::needless_pass_by_value)]
pub(crate) fn structured_error_to_js(err: ivac_core::Error) -> JsValue {
    serde_wasm_bindgen::to_value(&err).unwrap_or_else(|_| JsValue::from_str(&err.to_string()))
}

pub(crate) fn panic_message(p: &Box<dyn std::any::Any + Send>) -> String {
    if let Some(s) = p.downcast_ref::<&str>() {
        (*s).to_string()
    } else if let Some(s) = p.downcast_ref::<String>() {
        s.clone()
    } else {
        "unknown panic payload".to_string()
    }
}

#[wasm_bindgen(js_name = feedsSpeeds)]
pub fn feeds_speeds(request: JsValue) -> Result<JsValue, JsValue> {
    guard(|| {
        let request = serde_wasm_bindgen::from_value(request).map_err(into_js_error)?;
        let response = ivac_core::feeds_speeds::feeds_speeds(request)
            .map_err(|e| structured_error_to_js(ivac_core::Error::bad_input(e)))?;
        serde_wasm_bindgen::to_value(&response).map_err(into_js_error)
    })
}

#[wasm_bindgen(js_name = validateSourcePack)]
pub fn validate_source_pack(json: &str) -> Result<JsValue, JsValue> {
    guard(|| {
        let pack =
            ivac_core::feeds_speeds::validate_source_pack_json(json).map_err(into_js_error)?;
        serde_wasm_bindgen::to_value(&pack).map_err(into_js_error)
    })
}

#[wasm_bindgen(js_name = captureLabPreset)]
pub fn capture_lab_preset(project: JsValue, op_id: u32) -> Result<JsValue, JsValue> {
    guard(|| {
        let project = serde_wasm_bindgen::from_value(project).map_err(into_js_error)?;
        let capture =
            ivac_core::feeds_speeds::capture_lab_preset(&project, op_id).map_err(into_js_error)?;
        serde_wasm_bindgen::to_value(&capture).map_err(into_js_error)
    })
}

#[wasm_bindgen(js_name = cornerRelief)]
pub fn corner_relief(request: JsValue) -> Result<JsValue, JsValue> {
    let req: ivac_core::corner_relief::CornerReliefRequest =
        serde_wasm_bindgen::from_value(request).map_err(into_js_error)?;
    guard(|| {
        serde_wasm_bindgen::to_value(&ivac_core::corner_relief::preview(&req))
            .map_err(into_js_error)
    })
}

#[wasm_bindgen(js_name = evaluateDrawing)]
pub fn evaluate_drawing(request: JsValue) -> Result<JsValue, JsValue> {
    guard(|| {
        let req = serde_wasm_bindgen::from_value(request).map_err(into_js_error)?;
        let result = ivac_core::drawing::api::evaluate(req).map_err(into_js_error)?;
        serde_wasm_bindgen::to_value(&result).map_err(into_js_error)
    })
}
#[wasm_bindgen(js_name = generateDrawing)]
pub fn generate_drawing(request: JsValue) -> Result<JsValue, JsValue> {
    guard(|| {
        let req = serde_wasm_bindgen::from_value(request).map_err(into_js_error)?;
        let result = ivac_core::pipeline::generate_drawing(req).map_err(|e| {
            e.to_structured(None)
                .map_or_else(|| into_js_error(e), structured_error_to_js)
        })?;
        serde_wasm_bindgen::to_value(&result).map_err(into_js_error)
    })
}

// Pure request boundary shared by the JS entry point and native bridge replay.
// This does not qualify JS serialization or a browser runtime.
fn generate_request(
    req: PipelineRequest,
) -> Result<ivac_core::pipeline::PipelineResponse, ivac_core::pipeline::PipelineError> {
    run_pipeline(req, |_phase, _fraction, _msg| {})
}
#[cfg(all(test, feature = "cps"))]
mod test1_bridge {
    use super::*;
    #[test]
    fn native_bridge_preserves_non_executable_collision_diagnostic() {
        let mut req: PipelineRequest = serde_json::from_str(include_str!(
            "../../../tests/fixtures/test1/profile.request.json"
        ))
        .unwrap();
        req.project.fixtures.push(ivac_core::project::Fixture {
            id: 1,
            name: "clamp".into(),
            origin: (0., 0.),
            z_bottom: -100.,
            z_top: 100.,
            color: 0,
            kind: ivac_core::project::FixtureKind::Box {
                width: 1000.,
                depth: 1000.,
            },
        });
        let error = generate_request(req)
            .unwrap_err()
            .to_structured(None)
            .unwrap();
        assert!(!error
            .diagnostic_preview
            .as_ref()
            .unwrap()
            .toolpath
            .is_empty());
        assert!(serde_json::to_value(error).unwrap()["diagnostic_preview"]
            .get("gcode")
            .is_none());
    }
    #[test]
    fn shared_profile_and_vcarve_requests_replay_through_native_bridge() {
        for (request, expected) in [
            (
                include_str!("../../../tests/fixtures/test1/profile.request.json"),
                include_str!("../../../tests/fixtures/test1/profile.sbp"),
            ),
            (
                include_str!("../../../tests/fixtures/test1/vcarve.request.json"),
                include_str!("../../../tests/fixtures/test1/vcarve.sbp"),
            ),
        ] {
            let req: PipelineRequest = serde_json::from_str(request).unwrap();
            let result = generate_request(req.clone()).unwrap();
            // ADR-0025 adds only `' FABTOOL/1` provenance comments; every other
            // byte still equals the saved TEST1 reference.
            let (markers, rest): (Vec<&str>, Vec<&str>) = result
                .gcode
                .split_inclusive('\n')
                .partition(|l| l.starts_with("' FABTOOL/1 "));
            assert_eq!(
                markers,
                ["' FABTOOL/1 REQUEST 0 1\n", "' FABTOOL/1 ACTIVE 0 1\n"]
            );
            assert_eq!(rest.concat(), expected);
            let mut invalid = req;
            invalid.project.stock.as_mut().unwrap().top_z_mm = 18.;
            assert!(matches!(
                generate_request(invalid),
                Err(ivac_core::pipeline::PipelineError::UnsafeClearance(..))
            ));
        }
    }
}
