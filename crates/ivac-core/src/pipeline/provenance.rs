//! Generation-local tool definitions and emitted-intent evidence (ADR-0025).
//! No controller-state claim and no project/operation fallback.
use crate::project::ToolEntry;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;

pub const REVISION: u32 = 1;
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Authority {
    #[default]
    Unavailable,
    Planned,
    Resolved,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ToolBinding {
    pub reference: NonZeroU32,
    pub project_tool_id: u32,
    pub program_tool_number: u32,
    pub tool: ToolEntry,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ToolTransition {
    pub output_line: u32,
    pub before_ref: Option<NonZeroU32>,
    pub after_ref: NonZeroU32,
    pub evidence: String,
}
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct ToolProvenance {
    pub revision: u32,
    pub authority: Authority,
    pub bindings: Vec<ToolBinding>,
    pub transitions: Vec<ToolTransition>,
    pub diagnostics: Vec<String>,
}
impl ToolProvenance {
    #[must_use]
    pub fn new() -> Self {
        Self {
            revision: REVISION,
            ..Self::default()
        }
    }
    pub fn bind(&mut self, tools: &[ToolEntry], id: u32) -> Result<NonZeroU32, String> {
        let mut matches = tools.iter().filter(|t| t.id == id);
        let tool = matches
            .next()
            .ok_or_else(|| format!("Missing tool definition {id}"))?;
        if matches.next().is_some() {
            return Err(format!("Duplicate tool identity {id}"));
        }
        if let Some(b) = self.bindings.iter().find(|b| b.project_tool_id == id) {
            return Ok(b.reference);
        }
        let reference = u32::try_from(self.bindings.len() + 1)
            .ok()
            .and_then(NonZeroU32::new)
            .ok_or("Too many tool bindings")?;
        self.bindings.push(ToolBinding {
            reference,
            project_tool_id: id,
            program_tool_number: id,
            tool: tool.clone(),
        });
        Ok(reference)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn legacy_is_unknown_and_zero_reference_is_invalid() {
        let segment = r#"{"from":{"x":0,"y":0,"z":0},"to":{"x":1,"y":0,"z":0},"kind":"cut"}"#;
        let s: crate::gcode::preview::ToolpathSegment = serde_json::from_str(segment).unwrap();
        assert!(s.tool_ref.is_none());
        let mut json = serde_json::to_value(s).unwrap();
        json["tool_ref"] = 0.into();
        assert!(serde_json::from_value::<crate::gcode::preview::ToolpathSegment>(json).is_err());
        assert_eq!(ToolProvenance::default().authority, Authority::Unavailable);
    }
}

/// Only the owned emitter may use this namespace. Included/custom programs are
/// never promoted by recognising their comments.
pub const MARKER: &str = "FABTOOL/1";
fn stamp(segments: &mut [crate::gcode::preview::ToolpathSegment], changes: &[(u32, NonZeroU32)]) {
    for s in segments {
        s.tool_ref = changes
            .iter()
            .rev()
            .find(|(line, _)| *line < s.gcode_line)
            .map(|(_, r)| *r);
    }
}
fn finish(
    mut p: ToolProvenance,
    segments: &[crate::gcode::preview::ToolpathSegment],
    qualified: bool,
) -> ToolProvenance {
    p.authority =
        if qualified && !segments.is_empty() && segments.iter().all(|s| s.tool_ref.is_some()) {
            Authority::Resolved
        } else {
            Authority::Planned
        };
    if p.authority != Authority::Resolved {
        p.diagnostics
            .push("Tool intent or movement coverage is not resolved for this output".into());
    }
    p
}
fn failed(message: String) -> ToolProvenance {
    ToolProvenance {
        revision: REVISION,
        diagnostics: vec![message],
        ..ToolProvenance::default()
    }
}

/// Executable part of one RS274 line: a `(` comment ends at the FIRST `)` (so a
/// nested `(` never re-opens executable text), `;` outside parentheses starts
/// an end-of-line comment, and spaces/tabs are insignificant as in LinuxCNC.
/// Comment text (operator prompts, tool and operation names) never carries
/// machine words.
fn executable_part(line: &str) -> String {
    let mut out = String::new();
    let mut in_comment = false;
    for ch in line.chars() {
        match ch {
            ')' if in_comment => in_comment = false,
            _ if in_comment => {}
            '(' => in_comment = true,
            ';' => break,
            ' ' | '\t' => {}
            _ => out.push(ch.to_ascii_uppercase()),
        }
    }
    out
}

/// Letter/value words of an executable part. Every ASCII letter followed by a
/// number is a word (`AT5` still exposes `T5`); valueless letters are ignored.
fn machine_words(code: &str) -> Vec<(u8, f64)> {
    let b = code.as_bytes();
    let mut words = Vec::new();
    for (i, letter) in b.iter().enumerate() {
        if !letter.is_ascii_alphabetic() {
            continue;
        }
        let mut end = i + 1;
        if matches!(b.get(end), Some(b'+' | b'-')) {
            end += 1;
        }
        let digits = end;
        while b.get(end).is_some_and(|c| c.is_ascii_digit() || *c == b'.') {
            end += 1;
        }
        if let Some(value) = code
            .get(i + 1..end)
            .filter(|_| end > digits)
            .and_then(|v| v.parse::<f64>().ok())
        {
            words.push((*letter, value));
        }
    }
    words
}

/// Words that select or declare a tool: any `T`, `M6`/`M06` (change) and
/// `M61` (set current tool without a change).
fn tool_state_words(words: &[(u8, f64)]) -> bool {
    words
        .iter()
        .any(|&w| w.0 == b'T' || word_is(w, b'M', 6.) || word_is(w, b'M', 61.))
}

fn word_is((letter, value): (u8, f64), expected_letter: u8, expected: f64) -> bool {
    letter == expected_letter && (value - expected).abs() < 1e-9
}

/// Native owned text contract. REQUEST is planned intent; ACTIVE only resolves
/// after the supported output envelope. No inference from op IDs.
///
/// The planned manifest (`expected`) and the markers are recorded by the same
/// `tool_intent` call, so their comparison proves the emitted text, including
/// cache replay, kept the planned boundaries — not that the planner chose the
/// right tool. The executable change envelope between the markers is checked
/// independently against the machine strategy.
pub(super) fn native(
    project: &crate::project::Project,
    post: super::PostProcessorKind,
    text: &str,
    segments: &mut [crate::gcode::preview::ToolpathSegment],
    expected: &[(crate::gcode::ToolIntentPhase, u32)],
) -> ToolProvenance {
    use crate::gcode::ToolIntentPhase;
    use crate::project::{OpKind, ToolChangeStrategy};
    let result = (|| -> Result<ToolProvenance, String> {
        let mut p = ToolProvenance::new();
        let mut active = None;
        let mut request = None;
        let mut changes = Vec::new();
        let mut envelope: Vec<Vec<(u8, f64)>> = Vec::new();
        let mut observed = Vec::new();
        // Each unsupported feature keeps authority at `planned` and says why.
        // None of them is a tool-identity contradiction (those refuse below).
        let machine = &project.machine;
        let mut reasons: Vec<String> = Vec::new();
        if !matches!(
            post,
            super::PostProcessorKind::Linuxcnc | super::PostProcessorKind::Grbl
        ) {
            reasons.push("Only owned LinuxCNC/GRBL output can resolve tool provenance".into());
        }
        if machine.mode != crate::project::MachineMode::Mill {
            reasons.push("Resolved tool provenance is milling-only".into());
        }
        if machine.post_profile.is_some() {
            reasons.push(
                "Custom post profiles (templates, axes, formats) are not certified for resolved tool provenance"
                    .into(),
            );
        }
        if machine.toolchange_xy.is_some()
            || machine.park_xy.is_some()
            || machine.park_at_home
            || !machine.post_change_z.is_none()
        {
            reasons.push(
                "Machine-coordinate change, park and post-change Z moves are not modelled".into(),
            );
        }
        if machine.use_tool_length_offsets {
            reasons.push("Tool-length offsets are not modelled".into());
        }
        // Raw includes/custom macro semantics cannot be certified by markers;
        // the program preview models neither G28 nor a probe trigger position.
        if project.operations.iter().any(|o| {
            o.enabled
                && matches!(
                    o.kind,
                    OpKind::GcodeInclude { .. } | OpKind::Homing { .. } | OpKind::Probe { .. }
                )
        }) {
            reasons.push(
                "Homing (G28), probing (G38.2) and raw G-code includes are not modelled by the program preview"
                    .into(),
            );
        }
        let mut qualified = reasons.is_empty();
        if serde_json::to_string(project)
            .map_err(|e| e.to_string())?
            .contains(MARKER)
        {
            return Err("Reserved tool-provenance marker in source data".into());
        }
        let mut seen_ids = std::collections::HashSet::new();
        if project.tools.iter().any(|t| !seen_ids.insert(t.id)) {
            return Err("Duplicate project tool identities".into());
        }
        for (i, line) in text.lines().enumerate() {
            let n = u32::try_from(i + 1).map_err(|_| "Program too long")?;
            // N-words are controller labels; retain physical output-line indices.
            let line = line.trim();
            let line = line
                .strip_prefix('N')
                .and_then(|s| s.split_once(' '))
                .filter(|(n, _)| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()))
                .map_or(line, |(_, rest)| rest.trim());
            if let Some(m) = line.strip_prefix("; FABTOOL/1 ").or_else(|| {
                line.strip_prefix("(FABTOOL/1 ")
                    .and_then(|v| v.strip_suffix(')'))
            }) {
                let parts: Vec<_> = m.split_whitespace().collect();
                if parts.len() != 2 {
                    return Err("Malformed tool provenance".into());
                }
                let id = parts[1]
                    .parse::<u32>()
                    .map_err(|_| "Invalid tool identity")?;
                let phase = match parts[0] {
                    "PRELOAD" => ToolIntentPhase::Preload,
                    "REQUEST" => ToolIntentPhase::Request,
                    "ACTIVE" => ToolIntentPhase::Active,
                    _ => return Err("Unknown tool boundary".into()),
                };
                observed.push((phase, id));
                let reference = p.bind(&project.tools, id)?;
                match parts[0] {
                    "PRELOAD" if active.is_none() && request.is_none() && changes.is_empty() => {
                        active = Some(reference);
                        changes.push((n, reference));
                        p.transitions.push(ToolTransition {
                            output_line: n,
                            before_ref: None,
                            after_ref: reference,
                            evidence: "declared_preload_not_physical_state".into(),
                        });
                    }
                    "REQUEST" if request.is_none() => {
                        request = Some(reference);
                        envelope.clear();
                    }
                    "ACTIVE" if request == Some(reference) => {
                        let same = active == Some(reference);
                        if matches!(
                            project.machine.tool_change,
                            ToolChangeStrategy::ManualM0Pause | ToolChangeStrategy::Ignore
                        ) && envelope.iter().any(|words| tool_state_words(words))
                        {
                            return Err(
                                "Unexpected tool selection in manual/ignored envelope".into()
                            );
                        }
                        let evidence = match project.machine.tool_change {
                            ToolChangeStrategy::Atc | ToolChangeStrategy::ManualM6Prompt => {
                                // Exactly one owned `T<id> M6` line, nothing else
                                // selecting or declaring a tool.
                                let selections: Vec<&[(u8, f64)]> = envelope
                                    .iter()
                                    .map(Vec::as_slice)
                                    .filter(|words| tool_state_words(words))
                                    .collect();
                                if !matches!(
                                    selections.as_slice(),
                                    [[t, m]] if word_is(*t, b'T', f64::from(id))
                                        && word_is(*m, b'M', 6.)
                                ) {
                                    qualified = false;
                                    reasons.push(format!(
                                        "Change to tool {id} lacks exactly one owned `T{id} M6` request"
                                    ));
                                }
                                "emitted_m6_request"
                            }
                            ToolChangeStrategy::ManualM0Pause => {
                                // A standalone M0 only: an optional stop (M1) may
                                // be skipped by the controller.
                                if !same
                                    && !envelope.iter().any(|words| {
                                        matches!(words.as_slice(), [m] if word_is(*m, b'M', 0.))
                                    })
                                {
                                    qualified = false;
                                    reasons.push(format!(
                                        "Manual change to tool {id} lacks a standalone M0 stop (an optional stop may be skipped)"
                                    ));
                                }
                                "manual_load_assumption"
                            }
                            ToolChangeStrategy::Ignore => {
                                if !same {
                                    qualified = false;
                                    reasons.push(format!(
                                        "Ignored change to tool {id} cannot establish the mounted tool"
                                    ));
                                }
                                "ignored_change"
                            }
                        };
                        p.transitions.push(ToolTransition {
                            output_line: n,
                            before_ref: active,
                            after_ref: reference,
                            evidence: evidence.into(),
                        });
                        active = Some(reference);
                        changes.push((n, reference));
                        request = None;
                    }
                    _ => return Err("Missing, duplicate or contradictory tool boundary".into()),
                }
            } else if request.is_some() {
                // Only the owned template-free envelope may select/activate a tool.
                envelope.push(machine_words(&executable_part(line)));
            } else if tool_state_words(&machine_words(&executable_part(line))) {
                return Err("Tool command outside planned transition".into());
            }
        }
        if request.is_some() || observed != expected {
            return Err(
                "Emitted tool boundaries disagree with the independent planned intents".into(),
            );
        }
        stamp(segments, &changes);
        p.diagnostics.extend(reasons);
        Ok(finish(p, segments, qualified))
    })();
    result.unwrap_or_else(failed)
}

#[cfg(feature = "cps")]
pub(super) fn cps(
    project: &crate::project::Project,
    program: &ivac_cps::ir::Program,
    text: &str,
    segments: &mut [crate::gcode::preview::ToolpathSegment],
    shopbot: bool,
    lab: bool,
) -> ToolProvenance {
    let result = (|| -> Result<ToolProvenance, String> {
        let mut p = ToolProvenance::new();
        let mut seen = std::collections::HashSet::new();
        if project.tools.iter().any(|t| !seen.insert(t.id)) {
            return Err("Duplicate project tool identities".into());
        }
        if serde_json::to_string(project)
            .map_err(|e| e.to_string())?
            .contains(MARKER)
        {
            return Err("Reserved tool-provenance marker in source data".into());
        }
        if !shopbot {
            // IR's section ranges, not output line-count matching, own planned
            // identity. Repeat extraction per section includes cycle expansion.
            let mut offset = 0;
            for section in &program.sections {
                let r = p.bind(
                    &project.tools,
                    section
                        .tool
                        .project_tool_id
                        .ok_or("Missing planned project tool identity")?,
                )?;
                if p.bindings[r.get() as usize - 1].program_tool_number != section.tool.number {
                    return Err("Unqualified program tool number mapping".into());
                }
                let count =
                    crate::gcode::cps_recorder::ir_preview_sections(std::slice::from_ref(section))
                        .segments
                        .len();
                let range = segments
                    .get_mut(offset..offset + count)
                    .ok_or("IR preview mismatch")?;
                for s in range {
                    s.tool_ref = Some(r);
                }
                offset += count;
            }
            if offset != segments.len() {
                return Err("IR preview coverage mismatch".into());
            }
            return Ok(finish(p, segments, false));
        }
        let mut active = None;
        let mut pending = None;
        let mut next = 0usize;
        let mut changes = Vec::new();
        let mut load = false;
        let mut stopped = false;
        for (i, line) in text.lines().enumerate() {
            let n = u32::try_from(i + 1).map_err(|_| "Program too long")?;
            if let Some(m) = line.strip_prefix("' FABTOOL/1 ") {
                let fields: Vec<_> = m.split_whitespace().collect();
                if fields.len() != 3 {
                    return Err("Malformed ShopBot tool marker".into());
                }
                let ordinal = fields[1]
                    .parse::<usize>()
                    .map_err(|_| "Invalid section ordinal")?;
                let id = fields[2]
                    .parse::<u32>()
                    .map_err(|_| "Invalid tool identity")?;
                let section = program
                    .sections
                    .get(ordinal)
                    .ok_or("Unknown emitted section")?;
                if section.tool.number != id {
                    return Err("Planned/emitted tool disagreement".into());
                }
                let r = p.bind(
                    &project.tools,
                    section
                        .tool
                        .project_tool_id
                        .ok_or("Missing planned project tool identity")?,
                )?;
                if p.bindings[r.get() as usize - 1].program_tool_number != id {
                    return Err("Unqualified program tool number mapping".into());
                }
                let tool = &p.bindings[r.get() as usize - 1].tool;
                if (tool.effective_diameter() - section.tool.diameter).abs() > 1e-9 {
                    return Err("Planned tool geometry disagreement".into());
                }
                match fields[0] {
                    "REQUEST" if ordinal == next && pending.is_none() => {
                        pending = Some((ordinal, r));
                        load = false;
                        stopped = false;
                    }
                    "ACTIVE" if pending == Some((ordinal, r)) => {
                        if lab {
                            if id != 1 || active.is_some_and(|old| old != r) {
                                return Err("Unqualified lab tool change".into());
                            }
                        } else if !load || !stopped {
                            return Err("Missing manual tool load/PAUSE envelope".into());
                        }
                        p.transitions.push(ToolTransition {
                            output_line: n,
                            before_ref: active,
                            after_ref: r,
                            evidence: if lab {
                                "declared_preloaded_lab_tool"
                            } else {
                                "manual_load_and_pause"
                            }
                            .into(),
                        });
                        active = Some(r);
                        changes.push((n, r));
                        pending = None;
                        next += 1;
                    }
                    _ => {
                        return Err("Missing, repeated or reordered ShopBot section boundary".into())
                    }
                }
            } else if let Some((ordinal, _)) = pending {
                use super::shopbot_resolution::{load_instruction, xyz_renders};
                let expected = &program.sections[ordinal].tool;
                if let Some(instruction) = load_instruction(line) {
                    // Identity comes from the section ordinal/number binding above;
                    // the diameter only has to be the post's rounding of the plan.
                    let (number, diameter) = instruction?;
                    if number != expected.number || !xyz_renders(diameter, expected.diameter) {
                        return Err("Manual load instruction disagrees with planned tool".into());
                    }
                    load = true;
                } else if line == "PAUSE" && load {
                    stopped = true;
                } else if line.starts_with("M3,") || line.starts_with("J2,") {
                    return Err("Movement before completed tool change".into());
                }
            }
        }
        if pending.is_some() || next != program.sections.len() {
            return Err("Incomplete ShopBot provenance".into());
        }
        stamp(segments, &changes);
        Ok(finish(p, segments, true))
    })();
    result.unwrap_or_else(failed)
}

#[cfg(test)]
mod output_contract_tests {
    use super::*;
    use crate::gcode::preview::interpret_with_index;
    use crate::pipeline::test_helpers::{endmill, project_with};
    #[test]
    fn native_contradictions_and_deleted_changes_cannot_resolve() {
        let mut project = project_with(vec![], vec![endmill(1, 6.), endmill(2, 2.)]);
        project.machine.tool_change = crate::project::ToolChangeStrategy::Atc;
        use crate::gcode::ToolIntentPhase::{Active, Preload, Request};
        let expected = [(Preload, 1), (Request, 2), (Active, 2)];
        let good="; FABTOOL/1 PRELOAD 1\nG0 Z5\n; FABTOOL/1 REQUEST 2\nT2 M6\n; FABTOOL/1 ACTIVE 2\nG1 X10 Z-1 F600\n";
        for text in [
            good.replace("T2 M6", "T99 M6\nT2 M6"),
            good.replace("T2 M6", ""),
            good.replace("; FABTOOL/1 REQUEST 2\nT2 M6\n; FABTOOL/1 ACTIVE 2\n", ""),
            good.replace("ACTIVE 2", "ACTIVE 1"),
            good.replace("REQUEST 2", "REQUEST 99"),
        ] {
            let (mut segments, _) = interpret_with_index(&text);
            assert_ne!(
                native(
                    &project,
                    super::super::PostProcessorKind::Linuxcnc,
                    &text,
                    &mut segments,
                    &expected
                )
                .authority,
                Authority::Resolved
            );
        }
        let (mut segments, _) = interpret_with_index(good);
        assert_eq!(
            native(
                &project,
                super::super::PostProcessorKind::Linuxcnc,
                good,
                &mut segments,
                &expected
            )
            .authority,
            Authority::Resolved
        );
        project.machine.tool_change = crate::project::ToolChangeStrategy::ManualM0Pause;
        let contradictory = good.replace("T2 M6", "T99\nM0");
        let (mut path, _) = interpret_with_index(&contradictory);
        assert_eq!(
            native(
                &project,
                super::super::PostProcessorKind::Linuxcnc,
                &contradictory,
                &mut path,
                &expected
            )
            .authority,
            Authority::Unavailable
        );
    }
    #[cfg(feature = "cps")]
    #[test]
    fn shopbot_marker_is_not_sufficient_without_matching_pause_and_expected_section() {
        use serde_json::json;
        let project = project_with(vec![], vec![endmill(1, 6.)]);
        let program:ivac_cps::ir::Program=serde_json::from_value(json!({"version":1,"header":{"programName":"test","toleranceMm":0.01},"sections":[{
            "id":1,"strategy":"pocket2d","tool":{"projectToolId":1,"number":1,"diameter":6,"toolType":5},
            "spindleRpm":18000,"spindleClockwise":true,"workOffset":1,"workPlane":[[1,0,0],[0,1,0],[0,0,1]],
            "initialPosition":{"x":0,"y":0,"z":5},"finalPosition":{"x":1,"y":0,"z":-1},"records":[]
        }]})).unwrap();
        let good="' FABTOOL/1 REQUEST 0 1\nJZ,5\n' Spindle OFF and stationary. Load tool 1 D=6.0000; set work Z zero for this tool. Return to clearance before OK.\nPAUSE\n' FABTOOL/1 ACTIVE 0 1\nJ2,0,0\nMS,10,10\nM3,1,0,-1\n";
        for text in [
            good.replace("PAUSE\n", ""),
            good.replace("Load tool 1", "Load tool 2"),
            good.replace("ACTIVE 0 1", "ACTIVE 0 2"),
            good.replace("REQUEST 0 1", "REQUEST 1 1"),
            good.replace("' FABTOOL/1 ACTIVE 0 1\n", ""),
        ] {
            let (mut path, _) = super::super::shopbot_resolution::resolved_preview(&text).unwrap();
            assert_ne!(
                cps(&project, &program, &text, &mut path.segments, true, false).authority,
                Authority::Resolved
            );
        }
        let (mut path, _) = super::super::shopbot_resolution::resolved_preview(good).unwrap();
        assert_eq!(
            cps(&project, &program, good, &mut path.segments, true, false).authority,
            Authority::Resolved
        );
    }

    fn native_authority(
        project: &crate::project::Project,
        post: super::super::PostProcessorKind,
        text: &str,
        expected: &[(crate::gcode::ToolIntentPhase, u32)],
    ) -> Authority {
        let (mut segments, _) = interpret_with_index(text);
        native(project, post, text, &mut segments, expected).authority
    }

    #[test]
    fn comments_never_carry_machine_words_but_executable_text_does() {
        let words = |line: &str| machine_words(&executable_part(line));
        for comment in [
            "(pause: swap to tool 2 (Fraise filetage M6))",
            "(pause: swap to tool 2 (Taraud M06))",
            "(pause: swap to tool 2 (T2 special))",
            "(pause: swap to tool 2 (Fraise ø3 ünïcødé M6))",
            "; pause: swap to tool 2 (x) T5 (y)",
            "(a;b M6)",
            "#<_ivac_tlref> = #5063",
        ] {
            assert!(!tool_state_words(&words(comment)), "{comment}");
        }
        // First `)` closes a comment: text after it is executable again.
        assert!(tool_state_words(&words(
            "(pause: swap to tool 2 (x) T5 (y))"
        )));
        assert!(tool_state_words(&words("(a;b) T2")));
        assert!(tool_state_words(&words("AT5")));
        assert_eq!(words("t 2 m 06"), vec![(b'T', 2.), (b'M', 6.)]);
        assert_eq!(
            words("N30 T2 M6 (load)"),
            vec![(b'N', 30.), (b'T', 2.), (b'M', 6.)]
        );
        assert!(tool_state_words(&words("M61 Q2")));
        assert!(!tool_state_words(&words("M60")));
    }

    #[test]
    fn native_tool_names_are_comments_and_real_tool_words_still_refuse() {
        use crate::gcode::ToolIntentPhase::{Active, Preload, Request};
        use crate::project::ToolChangeStrategy::{Atc, ManualM0Pause};
        let linuxcnc = super::super::PostProcessorKind::Linuxcnc;
        let mut project = project_with(vec![], vec![endmill(1, 6.), endmill(2, 2.)]);
        let expected = [(Preload, 1), (Request, 2), (Active, 2)];
        let manual = |pause: &str| {
            format!(
                "(FABTOOL/1 PRELOAD 1)\nG0 Z5\n(FABTOOL/1 REQUEST 2)\nM5\n{pause}\nM0\nM3 S10000\n(FABTOOL/1 ACTIVE 2)\nG1 X10 Z-1 F600\n"
            )
        };
        project.machine.tool_change = ManualM0Pause;
        for name in [
            "Fraise filetage M6",
            "Taraud M06",
            "T2 special",
            "Fraise (6mm)",
            "ø3 ünïcødé",
            "a;b M6",
            "M6",
        ] {
            let text = manual(&format!("(pause: swap to tool 2 ({name}))"));
            assert_eq!(
                native_authority(&project, linuxcnc, &text, &expected),
                Authority::Resolved,
                "{name}"
            );
            let text = manual(&format!("; pause: swap to tool 2 ({name})"));
            assert_eq!(
                native_authority(&project, linuxcnc, &text, &expected),
                Authority::Resolved,
                "{name}"
            );
        }
        // Executable selections/changes in a manual envelope still refuse.
        for executable in ["T99", "M06", "M6", "M61 Q2", "(x) T5 (y)", "G0 X1 T2"] {
            let text = manual(&format!("(pause)\n{executable}"));
            assert_eq!(
                native_authority(&project, linuxcnc, &text, &expected),
                Authority::Unavailable,
                "{executable}"
            );
        }
        // An optional stop is not a manual change; neither is a missing stop.
        let m1 = manual("(pause)").replace("\nM0\n", "\nM1\n");
        assert_eq!(
            native_authority(&project, linuxcnc, &m1, &expected),
            Authority::Planned
        );
        let none = manual("(pause)").replace("\nM0\n", "\n");
        assert_eq!(
            native_authority(&project, linuxcnc, &none, &expected),
            Authority::Planned
        );
        // Tool words outside a planned transition refuse.
        let stray = manual("(pause)").replace("G1 X10", "T3\nG1 X10");
        assert_eq!(
            native_authority(&project, linuxcnc, &stray, &expected),
            Authority::Unavailable
        );

        project.machine.tool_change = Atc;
        let atc = |change: &str| {
            format!(
                "(FABTOOL/1 PRELOAD 1)\nG0 Z5\n(FABTOOL/1 REQUEST 2)\n{change}\n(FABTOOL/1 ACTIVE 2)\nG1 X10 Z-1 F600\n"
            )
        };
        for good in [
            "T2 M6",
            "T2 M6 (Fraise filetage M6)",
            "(toolchange: T2 (M6 tap))\nT2 M6",
            "N40 T2 M6",
        ] {
            assert_eq!(
                native_authority(&project, linuxcnc, &atc(good), &expected),
                Authority::Resolved,
                "{good}"
            );
        }
        for bad in [
            "M6 T2",
            "T2 M6\nM6",
            "T3 M6",
            "T2",
            "M6",
            "T2 M6\nM61 Q2",
            "T2 M6 T2",
        ] {
            assert_ne!(
                native_authority(&project, linuxcnc, &atc(bad), &expected),
                Authority::Resolved,
                "{bad}"
            );
        }
    }

    #[cfg(feature = "cps")]
    #[test]
    fn shopbot_load_diameter_follows_emitted_rounding_but_never_proves_identity() {
        use super::super::shopbot_resolution::{
            load_instruction, xyz_renders, XYZ_HALF_QUANTUM_MM,
        };
        use serde_json::json;
        // Emitted 4-decimal renderings of imperial fractions (half away from zero).
        for (planned, emitted) in [
            (25.4 / 32., 0.7938),
            (3. * 25.4 / 32., 2.3813),
            (7. * 25.4 / 32., 5.5563),
            (9. * 25.4 / 32., 7.1438),
            (25.4 / 8., 3.175),
            (6., 6.),
        ] {
            assert!(xyz_renders(emitted, planned), "{planned} -> {emitted}");
            // One output unit away is the other legal rounding of a tie; 1.5 is not.
            assert!(!xyz_renders(emitted + 3. * XYZ_HALF_QUANTUM_MM, planned));
            assert!(!xyz_renders(emitted - 3. * XYZ_HALF_QUANTUM_MM, planned));
        }
        assert!(!xyz_renders(f64::NAN, 1.) && !xyz_renders(1., f64::INFINITY));
        let prefix = "' Spindle OFF and stationary. Load tool ";
        assert_eq!(
            load_instruction(&format!("{prefix}12 D=2.3813; set work Z zero.")),
            Some(Ok((12, 2.3813)))
        );
        assert_eq!(load_instruction("' Other comment"), None);
        for malformed in [
            "1 D=2.381;",
            "1 D=2.38130;",
            "1 D=-2.3813;",
            "1 D=NaN;",
            "1 D=inf;",
            "1 D=2.3813",
            "x D=2.3813;",
            "1 D=0.0000;",
            "1 D=1e1.0000;",
            "1D=2.3813;",
        ] {
            assert!(
                matches!(
                    load_instruction(&format!("{prefix}{malformed}")),
                    Some(Err(_))
                ),
                "{malformed}"
            );
        }

        let mut project = project_with(
            vec![],
            vec![endmill(1, 3. * 25.4 / 32.), endmill(2, 3. * 25.4 / 32.)],
        );
        project.tools[1].id = 2;
        let section = |number: u32, project_tool_id: u32| {
            json!({
                "id":1,"strategy":"pocket2d","tool":{"projectToolId":project_tool_id,"number":number,"diameter":3. * 25.4 / 32.,"toolType":5},
                "spindleRpm":18000,"spindleClockwise":true,"workOffset":1,"workPlane":[[1,0,0],[0,1,0],[0,0,1]],
                "initialPosition":{"x":0,"y":0,"z":5},"finalPosition":{"x":1,"y":0,"z":-1},"records":[]
            })
        };
        let program = |number: u32, project_tool_id: u32| -> ivac_cps::ir::Program {
            serde_json::from_value(
                json!({"version":1,"header":{"programName":"test","toleranceMm":0.01},
                "sections":[section(number, project_tool_id)]}),
            )
            .unwrap()
        };
        let text = |number: u32, d: &str| {
            format!(
            "' FABTOOL/1 REQUEST 0 {number}\nJZ,5\n{prefix}{number} D={d}; set work Z zero for this tool. Return to clearance before OK.\nPAUSE\n' FABTOOL/1 ACTIVE 0 {number}\nJ2,0,0\nMS,10,10\nM3,1,0,-1\n"
        )
        };
        let authority = |program: &ivac_cps::ir::Program, text: &str| {
            let (mut path, _) = super::super::shopbot_resolution::resolved_preview(text).unwrap();
            cps(&project, program, text, &mut path.segments, true, false).authority
        };
        assert_eq!(
            authority(&program(1, 1), &text(1, "2.3813")),
            Authority::Resolved
        );
        assert_eq!(
            authority(&program(1, 1), &text(1, "2.3812")),
            Authority::Resolved
        );
        for wrong in ["2.3815", "2.3800", "2.381", "6.0000"] {
            assert_eq!(
                authority(&program(1, 1), &text(1, wrong)),
                Authority::Unavailable,
                "{wrong}"
            );
        }
        // Same diameter, different tool number: identity mismatch still refuses.
        let swapped = text(1, "2.3813").replace("Load tool 1 ", "Load tool 2 ");
        assert_eq!(authority(&program(1, 1), &swapped), Authority::Unavailable);
        // Same diameter on another project tool cannot substitute the planned binding.
        assert_eq!(
            authority(&program(2, 2), &text(1, "2.3813")),
            Authority::Unavailable
        );
    }
}
