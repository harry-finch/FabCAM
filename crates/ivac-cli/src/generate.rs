//! Buffered v9 generation. JSON is the sole authority for CAM and post settings.
use anyhow::{bail, Context, Result};
use ivac_core::{
    cutting_parameters,
    pipeline::{run_pipeline, PipelineRequest, PostProcessorKind},
    project::{OpKind, OpSource},
    ImportOptions,
};
use std::path::PathBuf;

pub fn run(args: impl Iterator<Item = String>) -> Result<()> {
    let mut request_path = None;
    let mut geometry = None;
    let mut args = args;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--request" => {
                if request_path.is_some() {
                    bail!("Duplicate --request; specify exactly one authoritative v9 request");
                }
                request_path = Some(PathBuf::from(args.next().context("--request needs a JSON path")?));
            }
            flag if flag.starts_with('-') => bail!(
                "Unsupported/conflicting generate flag {flag}; put post, properties, tools and cutting values in --request JSON"
            ),
            _ if geometry.is_none() => geometry = Some(PathBuf::from(arg)),
            _ => bail!("Only one geometry path is allowed with --request"),
        }
    }
    let path = request_path.context(
        "generate requires --request <v9 PipelineRequest.json>; geometry convenience also requires an explicitly reviewed request template",
    )?;
    let contents = std::fs::read_to_string(&path)
        .with_context(|| format!("Cannot read {}", path.display()))?;
    let raw: serde_json::Value =
        serde_json::from_str(&contents).context("Invalid PipelineRequest JSON")?;
    // Avoid accepting misspelled top-level post settings as ignored defaults.
    let object = raw
        .as_object()
        .context("Expected a PipelineRequest object")?;
    for key in object.keys() {
        if !matches!(key.as_str(), "project" | "post_processor" | "cps_post") {
            bail!(
                "Unknown PipelineRequest field {key}; saved ProjectFile envelopes are not requests"
            );
        }
    }
    let mut request: PipelineRequest =
        serde_json::from_str(&contents).context("Invalid v9 request")?;
    cutting_parameters::require_current(&request.project).map_err(anyhow::Error::msg)?;
    if request.cps_post.is_some() && request.post_processor != Some(PostProcessorKind::Cps) {
        bail!("cps_post conflicts with a non-CPS post_processor");
    }
    if let Some(path) = geometry {
        let project = &mut request.project;
        if !project.segments.is_empty()
            || !project.text_layers.is_empty()
            || !project.relief_sources.is_empty()
        {
            bail!("Geometry template must contain no segments, text layers or relief sources");
        }
        if project.operations.len() != 1
            || !project.operations[0].enabled
            || !matches!(project.operations[0].kind, OpKind::Profile { .. })
            || !matches!(project.operations[0].source, OpSource::All)
        {
            bail!("Geometry template requires exactly one enabled Profile with explicit source kind all");
        }
        // These editable fields have serde defaults in the shared model. A convenience
        // template must actually spell them out; never manufacture a machining setup.
        for pointer in [
            "/project/machine",
            "/project/operations/0/tool_id",
            "/project/operations/0/source",
            "/project/operations/0/params/depth",
            "/project/operations/0/params/step",
            "/project/operations/0/params/fast_move_z",
        ] {
            if raw
                .pointer(pointer)
                .map_or(true, serde_json::Value::is_null)
            {
                bail!("Geometry template requires explicit {pointer}");
            }
        }
        cutting_parameters::validate_project(project)?;
        let imported = ivac_core::input::import_path(&path, &ImportOptions::default())
            .with_context(|| format!("Cannot import {}", path.display()))?;
        if imported.segments.is_empty() {
            bail!("Geometry import contains no segments");
        }
        project.segments = imported.segments;
    }
    // The buffered pipeline owns all neutral validation, budgets, simulation and
    // bundled-only CPS restrictions. Nothing is written until it has succeeded.
    let response = run_pipeline(request, |_, _, _| {})?;
    let json = serde_json::to_vec_pretty(&response)?;
    use std::io::Write;
    let mut stdout = std::io::stdout().lock();
    stdout.write_all(&json)?;
    stdout.write_all(b"\n")?;
    Ok(())
}
