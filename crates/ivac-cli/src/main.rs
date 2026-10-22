//! `ivac` — headless converter and JSON-API surface for the Rust core.
//!
//! Subcommands:
//!   * `ivac import <file>`            — emit /import-shaped JSON to stdout
//!   * `ivac generate [geometry] --request <request.json>` — emit /generate-shaped JSON
//!     (gcode + 3D preview toolpath) to stdout
//!   * `ivac stream-gcode <project.json> [--output FILE]` — stream gcode
//!     from a full project (a /generate request body) straight to a file or
//!     stdout, never holding the whole program in memory (ivac-3j1p).
//!
//! Mirrors the JSON contract in `schema/openapi.yaml`.

mod generate;
mod i18n;

use std::path::PathBuf;

use anyhow::{bail, Context, Result};
use ivac_core::ImportOptions;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .try_init()
        .ok();
    // Resolve the UI language once, before anything user-facing is printed:
    // a `--lang` override (pulled out of argv here) beats the LANG/LC_ALL
    // environment. Everything below goes through `i18n::t`/`tp`.
    let (lang_override, rest) = i18n::extract_lang(std::env::args().skip(1).collect());
    i18n::set_locale(i18n::detect_locale(lang_override.as_deref(), |k| {
        std::env::var(k).ok()
    }));

    let mut args = rest.into_iter();
    let cmd = args.next().unwrap_or_default();
    match cmd.as_str() {
        "import" => cmd_import(args),
        "generate" => generate::run(args),
        "stream-gcode" => cmd_stream(args),
        "posts" => cmd_posts(args),
        "" | "-h" | "--help" => {
            print_help();
            Ok(())
        }
        other => {
            print_help();
            bail!(
                "{}",
                i18n::tp("cli.err.unknown_subcommand", &[("name", other)])
            );
        }
    }
}

fn print_help() {
    // The command skeletons (`ivac import <path>`, flag names) are literal and
    // language-agnostic; only the descriptions and labels go through the
    // catalog. Help prints to stderr so stdout stays clean JSON for pipes.
    eprintln!("{}\n", i18n::t("cli.help.tagline"));
    eprintln!("{}", i18n::t("cli.help.usage"));
    eprintln!("  ivac import <path>");
    eprintln!("      {}", i18n::t("cli.help.import"));
    eprintln!("  ivac generate --request <request.json>");
    eprintln!("  ivac generate <geometry> --request <template.json>");
    eprintln!("      {}", i18n::t("cli.help.generate"));
    eprintln!("  ivac stream-gcode <project.json> [--output FILE] [--two-sided]");
    eprintln!("      {}", i18n::t("cli.help.stream"));
    eprintln!("  ivac posts [inspect <file.cps>]");
    eprintln!("      {}", i18n::t("cli.help.posts"));
    eprintln!("  ivac --help");
    eprintln!("      {}", i18n::t("cli.help.help"));
    eprintln!("\n  --lang <en|de>   {}", i18n::t("cli.help.lang"));
}

fn cmd_import(mut args: impl Iterator<Item = String>) -> Result<()> {
    let path = args.next().context(i18n::t("cli.err.missing_path"))?;
    let path = PathBuf::from(path);
    let opts = ImportOptions::default();
    let out = ivac_core::input::import_path(&path, &opts).with_context(|| {
        i18n::tp(
            "cli.err.importing",
            &[("path", &path.display().to_string())],
        )
    })?;
    // Serialize the full ImportOutput — it already derives the /import
    // contract (snake_case field names matching the TS ImportResponse and
    // the wasm/server output), so emitting it directly keeps `objects`,
    // `object_meta`, and `text_entities`. The previous hand-rolled subset
    // dropped those, producing "degraded" sample fixtures the 2D canvas
    // couldn't select features from.
    serde_json::to_writer_pretty(std::io::stdout(), &out)?;
    println!();
    Ok(())
}

/// `ivac posts` — table of the bundled `.cps` posts;
/// `ivac posts inspect <file.cps>` — `PostMeta` JSON for a script.
#[cfg(feature = "cps")]
fn cmd_posts(mut args: impl Iterator<Item = String>) -> Result<()> {
    match args.next().as_deref() {
        None => {
            println!("{:<12} {:<28} {:<8}", "ID", "DESCRIPTION", "EXT");
            for post in ivac_cps::library::BUNDLED {
                let meta = ivac_cps::run::inspect_bundled(post.id, ivac_cps::RunLimits::default())
                    .map_err(|e| {
                        anyhow::anyhow!("bundled post {} failed inspection: {e}", post.id)
                    })?;
                println!(
                    "{:<12} {:<28} {:<8}",
                    post.id, meta.description, meta.extension
                );
            }
            Ok(())
        }
        Some("inspect") => {
            let path = args
                .next()
                .context("posts inspect needs a .cps file path")?;
            let script =
                std::fs::read_to_string(&path).with_context(|| format!("cannot read {path}"))?;
            let name = std::path::Path::new(&path)
                .file_name()
                .map_or_else(|| path.clone(), |n| n.to_string_lossy().into_owned());
            let meta =
                ivac_cps::inspect_post(&script, &name).map_err(|e| anyhow::anyhow!("{e}"))?;
            serde_json::to_writer_pretty(std::io::stdout(), &meta)?;
            println!();
            Ok(())
        }
        Some(other) => bail!("unknown posts subcommand: {other}"),
    }
}

#[cfg(not(feature = "cps"))]
fn cmd_posts(_args: impl Iterator<Item = String>) -> Result<()> {
    bail!("this build lacks CPS support")
}

/// Stream g-code from a full project JSON (a serialized `PipelineRequest` —
/// the same `/generate` request body the server + tauri accept) straight to
/// `--output FILE`, or to stdout when omitted. Runs the real CAM pipeline
/// through the write-through post, so peak memory is O(largest single op)
/// rather than O(whole program) (ivac-3j1p). The preview toolpath + time
/// estimate are skipped (they'd need a second pass over the program); a
/// short stats + warnings summary prints to stderr so stdout stays pure
/// g-code.
fn cmd_stream(args: impl Iterator<Item = String>) -> Result<()> {
    let mut path: Option<PathBuf> = None;
    let mut output: Option<PathBuf> = None;
    let mut two_sided = false;
    let mut iter = args.peekable();
    while let Some(arg) = iter.next() {
        let needs_value =
            |opt: &'static str| move || i18n::tp("cli.err.opt_needs_value", &[("opt", opt)]);
        match arg.as_str() {
            "--output" | "-o" => {
                output = Some(PathBuf::from(
                    iter.next().with_context(needs_value("--output"))?,
                ));
            }
            "--two-sided" => two_sided = true,
            other if path.is_none() => path = Some(PathBuf::from(other)),
            other => bail!("{}", i18n::tp("cli.err.unexpected_arg", &[("arg", other)])),
        }
    }
    let path = path.context(i18n::t("cli.err.missing_input_path"))?;

    // The input is a serialized PipelineRequest (project + optional post).
    let file = std::fs::File::open(&path)
        .with_context(|| i18n::tp("cli.err.reading", &[("path", &path.display().to_string())]))?;
    let request: ivac_core::pipeline::PipelineRequest =
        serde_json::from_reader(std::io::BufReader::new(file)).with_context(|| {
            i18n::tp(
                "cli.err.parse_project",
                &[("path", &path.display().to_string())],
            )
        })?;

    ivac_core::cutting_parameters::require_current(&request.project).map_err(anyhow::Error::msg)?;

    // Two-sided (flip-stock) jobs emit two programs, so they can't stream to a
    // single sink — they write `<base>.front.<ext>` / `<base>.back.<ext>` and
    // require an explicit --output base.
    if two_sided {
        return cmd_stream_two_sided(request, output);
    }

    // Destination: a file if --output was given, else stdout. Both buffered —
    // the streaming sink issues one write per emitted line.
    let staged = output
        .as_ref()
        .map(|out| ivac_file_export::StagedFile::new(out))
        .transpose()?;
    let (writer, dest): (Box<dyn std::io::Write + Send>, String) = match &output {
        Some(out) => {
            let f = staged.as_ref().context("missing staged output")?.writer()?;
            (
                Box::new(std::io::BufWriter::new(f)),
                out.display().to_string(),
            )
        }
        None => (
            Box::new(std::io::BufWriter::new(std::io::stdout())),
            "stdout".to_string(),
        ),
    };

    let outcome = ivac_core::pipeline::stream_gcode_to_writer(request, writer).map_err(|e| {
        anyhow::anyhow!(i18n::tp("cli.err.streaming", &[("detail", &e.to_string())]))
    })?;

    if let Some(staged) = staged {
        report_publication(staged.publish(false)?);
    }

    // Summary + warnings to stderr — stdout stays pure g-code.
    eprintln!(
        "{}",
        i18n::tp(
            "cli.stream.done",
            &[
                ("dest", &dest),
                ("objects", &outcome.stats.object_count.to_string()),
                ("offsets", &outcome.stats.offset_count.to_string()),
            ],
        )
    );
    for w in &outcome.warnings {
        eprintln!("  ! {}", w.message);
    }
    Ok(())
}

/// Two-sided (flip-stock) emission: write `<base>.front.<ext>` and, for a
/// genuine two-sided job, `<base>.back.<ext>`. Buffered (not streamed) because
/// two programs can't share one sink; two-sided jobs are small relative to the
/// unbounded-raster case the streaming path targets.
fn cmd_stream_two_sided(
    request: ivac_core::pipeline::PipelineRequest,
    output: Option<PathBuf>,
) -> Result<()> {
    let base = output.context(i18n::t("cli.err.two_sided_needs_output"))?;
    let front_path = side_path(&base, "front");
    let back_path = side_path(&base, "back");
    ivac_file_export::require_new(&[&front_path, &back_path])?;
    let two =
        ivac_core::pipeline::run_pipeline_two_sided(request, |_p, _f, _m| {}).map_err(|e| {
            anyhow::anyhow!(i18n::tp("cli.err.streaming", &[("detail", &e.to_string())]))
        })?;

    let mut front = ivac_file_export::StagedFile::new(&front_path)?;
    front.write_all(two.front.gcode.as_bytes())?;
    front.finish()?;
    let back = two
        .back
        .as_ref()
        .map(|program| -> std::io::Result<_> {
            let mut staged = ivac_file_export::StagedFile::new(&back_path)?;
            staged.write_all(program.gcode.as_bytes())?;
            staged.finish()?;
            Ok(staged)
        })
        .transpose()?;
    // Both complete programs are staged before either final name is created.
    report_publication(front.publish(true)?);
    if let Some(back) = back {
        report_publication(back.publish(true).with_context(|| format!(
            "Partial publication: {} was created, but {} was not published. No existing destination was overwritten; choose a fresh output base.",
            front_path.display(), back_path.display()))?);
    }
    report_program(&front_path, &two.front);
    if let Some(back) = &two.back {
        report_program(&back_path, back);
    }
    Ok(())
}

/// Insert `.<side>` before the extension of `base` (`out.gcode` + `front` →
/// `out.front.gcode`; an extensionless `out` → `out.front`).
fn side_path(base: &std::path::Path, side: &str) -> PathBuf {
    let stem = base
        .file_stem()
        .map(|s| s.to_string_lossy().into_owned())
        .unwrap_or_default();
    let name = match base.extension() {
        Some(ext) => format!("{stem}.{side}.{}", ext.to_string_lossy()),
        None => format!("{stem}.{side}"),
    };
    base.with_file_name(name)
}

/// Write one program's gcode to `path` and print the standard done summary.
fn report_program(path: &std::path::Path, resp: &ivac_core::pipeline::PipelineResponse) {
    eprintln!(
        "{}",
        i18n::tp(
            "cli.stream.done",
            &[
                ("dest", &path.display().to_string()),
                ("objects", &resp.stats.object_count.to_string()),
                ("offsets", &resp.stats.offset_count.to_string()),
            ],
        )
    );
    for w in &resp.warnings {
        eprintln!("  ! {}", w.message);
    }
}

fn report_publication(published: ivac_file_export::PublishedFile) {
    if let Some(warning) = published.cleanup_warning {
        eprintln!(
            "Published {} successfully; temporary-file cleanup warning: {warning}",
            published.path.display()
        );
    }
}
