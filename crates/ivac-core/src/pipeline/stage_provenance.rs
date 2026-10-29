//! V10 stage ranges in owned, decoded output. Never inferred from tool ID.
use super::provenance::ToolProvenance;
use crate::{
    cutting_stages::CuttingStage,
    gcode::preview::{MoveKind, ToolpathSegment},
    project::Project,
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
pub const MARKER: &str = "FABSTAGE_1";
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct StageBinding {
    pub authority: super::provenance::Authority,
    pub operation_id: u32,
    pub stage: String,
    /// Explicit authored stage, not measured controller state.
    pub parameters: CuttingStage,
    pub output_start_line: u32,
    pub output_end_line: u32,
    pub first_segment: usize,
    pub end_segment_exclusive: usize,
}
pub(super) fn bind(
    project: &Project,
    text: &str,
    path: &[ToolpathSegment],
    tools: &ToolProvenance,
    decoded: bool,
) -> Result<Vec<StageBinding>, String> {
    let expected: Vec<_> = project
        .operations
        .iter()
        .filter(|o| o.enabled)
        .filter_map(|o| o.params.stages.as_ref().map(|s| (o.id, s)))
        .collect();
    if expected.is_empty() {
        return Ok(Vec::new());
    }
    if (!decoded || project.machine.post_profile.is_some())
        && expected.iter().any(|(_, s)| s.finishing().is_some())
    {
        return Err("Explicit stages require an owned decoded output (LinuxCNC, GRBL or bundled ShopBot), without custom post templates for finishing".into());
    }
    if serde_json::to_string(project)
        .map_err(|e| e.to_string())?
        .contains(MARKER)
    {
        return Err("Reserved stage marker in source data".into());
    }
    let mut active: Option<(u32, String, u32)> = None;
    let mut result = Vec::new();
    let mut observed = std::collections::BTreeMap::<u32, Vec<String>>::new();
    for (i, line) in text.lines().enumerate() {
        let line = line.trim();
        let line = if line.starts_with('N') {
            line.split_once(char::is_whitespace)
                .filter(|(number, _)| number[1..].chars().all(|c| c.is_ascii_digit()))
                .map_or(line, |(_, rest)| rest)
        } else {
            line
        };
        let comment = line
            .trim()
            .trim_start_matches([';', '\'', '('])
            .trim()
            .trim_end_matches(')')
            .trim();
        let Some(marker) = comment.strip_prefix("FABSTAGE_1 ") else {
            continue;
        };
        let fields: Vec<_> = marker.split_whitespace().collect();
        if fields.len() != 2 {
            return Err("Malformed stage boundary".into());
        }
        let id = fields[0]
            .parse::<u32>()
            .map_err(|_| "Invalid stage operation")?;
        let line_no = u32::try_from(i + 1).map_err(|_| "Stage output too long")?;
        let stages = expected
            .iter()
            .find(|(op, _)| *op == id)
            .map(|(_, s)| *s)
            .ok_or("Unknown stage operation")?;
        if fields[1] == "end" {
            let (op, name, start) = active.take().ok_or("Stage end without start")?;
            if id != op {
                return Err("Contradictory stage end".into());
            }
            let parameters = match name.as_str() {
                "main" => stages.main(),
                "finishing" => &stages.finishing().ok_or("Unexpected finishing")?.cutting,
                _ => return Err("Unknown stage".into()),
            };
            // Non-resolved CPS exposes recorded IR, with optional/absent line sync.
            // Main-only has one stage per operation, so identity is unambiguous
            // without pretending the IR range is a decoded machine-output range.
            let (first, end) = if decoded {
                (
                    path.partition_point(|s| s.gcode_line <= start),
                    path.partition_point(|s| s.gcode_line < line_no),
                )
            } else {
                let first = path.iter().position(|s| s.op_id == id).unwrap_or(0);
                let end = path
                    .iter()
                    .rposition(|s| s.op_id == id)
                    .map_or(first, |i| i + 1);
                (first, end)
            };
            if first > end {
                return Err("Invalid stage range".into());
            }
            let mut cuts = 0;
            for m in &path[first..end] {
                if m.op_id != id {
                    return Err("Stage/operation movement attribution disagrees".into());
                }
                if matches!(m.kind, MoveKind::Cut | MoveKind::Plunge | MoveKind::Arc) {
                    cuts += 1;
                    let b = tools
                        .bindings
                        .iter()
                        .find(|b| Some(b.reference) == m.tool_ref)
                        .ok_or("Unresolved stage cutter")?;
                    if b.project_tool_id != parameters.tool_id {
                        return Err("Stage cutter differs from actual emitted cutter".into());
                    }
                }
            }
            if cuts == 0 && name == "finishing" {
                return Err("Finishing emitted no cutting movements".into());
            }
            result.push(StageBinding {
                authority: tools.authority.clone(),
                operation_id: id,
                stage: name,
                parameters: parameters.clone(),
                output_start_line: start + 1,
                output_end_line: line_no - 1,
                first_segment: first,
                end_segment_exclusive: end,
            });
        } else {
            if active.is_some() {
                return Err("Overlapping stage boundaries".into());
            }
            observed.entry(id).or_default().push(fields[1].to_owned());
            active = Some((id, fields[1].to_owned(), line_no));
        }
    }
    if active.is_some() {
        return Err("Unclosed stage boundary".into());
    }
    for (id, s) in expected {
        let names = if s.finishing().is_some() {
            vec!["main", "finishing"]
        } else {
            vec!["main"]
        };
        if observed
            .get(&id)
            .map(|v| v.iter().map(String::as_str).collect::<Vec<_>>())
            != Some(names)
        {
            return Err(format!(
                "Operation {id}: emitted stages do not match authored stages"
            ));
        }
        for (i, m) in path.iter().enumerate().filter(|(_, m)| {
            m.op_id == id && matches!(m.kind, MoveKind::Cut | MoveKind::Plunge | MoveKind::Arc)
        }) {
            if !result.iter().any(|b| {
                b.operation_id == id && i >= b.first_segment && i < b.end_segment_exclusive
            }) {
                return Err(format!(
                    "Operation {}: cutting movement outside authored stages",
                    m.op_id
                ));
            }
        }
    }
    Ok(result)
}
