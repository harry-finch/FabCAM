//! Bundled ShopBot output qualification adapter, outside generic CAM.
//!
//! The CPS post resolves/linearizes the generic IR. Decode only that owned,
//! closed command subset so final preview, simulation and timing consume the
//! rounded XYZ segments actually emitted, including ordered rapid axes.
use super::PipelineWarning;
use crate::gcode::cps_recorder::IrPreview;
use crate::gcode::preview::{GcodeIndex, MoveKind, Pose3, ToolpathSegment};

pub(super) fn resolved_preview(text: &str) -> Result<(IrPreview, GcodeIndex), String> {
    let mut preview = IrPreview::default();
    let mut index = GcodeIndex {
        lines_to_segment: vec![u32::MAX; text.lines().count() + 1],
        segments_to_line: Vec::new(),
    };
    // Initial operator position is unknown. First JZ establishes clearance;
    // first J2 establishes XY. Do not invent its unknown starting position.
    let mut position: [Option<f64>; 3] = [None; 3];
    let mut feed = 0.;
    let mut op_id = 0;
    for (i, line) in text.lines().enumerate() {
        let line_no = u32::try_from(i + 1).map_err(|_| "program too long")?;
        if let Some(comment) = line.strip_prefix("' ") {
            if let Some(id) = comment.strip_prefix("OP ").and_then(|s| s.parse().ok()) {
                op_id = id;
            }
            continue;
        }
        if ["VD,,1", "SA", "END", "PAUSE"].contains(&line) {
            continue;
        }
        if let Some(value) = line.strip_prefix("PAUSE ") {
            preview.dwell_s += number(value)?;
            continue;
        }
        let mut fields = line.split(',');
        let command = fields.next().unwrap_or_default();
        let values = fields.map(number).collect::<Result<Vec<_>, _>>()?;
        let from = position;
        let kind = match (command, values.as_slice()) {
            ("MS", [xy, z]) if *xy > 0. && xy == z => {
                feed = xy * 60.;
                continue;
            }
            ("SO", [_, _]) | ("TR", [_]) => continue,
            ("JZ", [z]) => {
                position[2] = Some(*z);
                MoveKind::Rapid
            }
            ("J2", [x, y]) => {
                position[0] = Some(*x);
                position[1] = Some(*y);
                MoveKind::Rapid
            }
            ("M3", [x, y, z]) if feed > 0. => {
                if from.iter().any(Option::is_none) {
                    return Err("cut before established XYZ".into());
                }
                position = [Some(*x), Some(*y), Some(*z)];
                if from[0] == position[0] && from[1] == position[1] {
                    if position[2] < from[2] {
                        MoveKind::Plunge
                    } else if position[2] > from[2] {
                        MoveKind::Retract
                    } else {
                        MoveKind::Cut
                    }
                } else {
                    MoveKind::Cut
                }
            }
            _ => {
                return Err(format!(
                    "unqualified ShopBot preview command at line {line_no}: {command}"
                ))
            }
        };
        if let ([Some(x), Some(y), Some(z)], [Some(tx), Some(ty), Some(tz)]) = (from, position) {
            if from == position {
                continue;
            }
            let segment_no =
                u32::try_from(preview.segments.len()).map_err(|_| "too many segments")?;
            index.lines_to_segment[i + 1] = segment_no;
            index.segments_to_line.push(line_no);
            preview.segments.push(ToolpathSegment {
                from: Pose3 { x, y, z },
                to: Pose3 {
                    x: tx,
                    y: ty,
                    z: tz,
                },
                kind,
                gcode_line: line_no,
                op_id,
                arc: None,
            });
            preview
                .feeds_mm_min
                .push(if kind == MoveKind::Rapid { 0. } else { feed });
        }
    }
    Ok((preview, index))
}
fn number(text: &str) -> Result<f64, String> {
    text.parse::<f64>()
        .ok()
        .filter(|v| v.is_finite())
        .ok_or_else(|| "non-finite/invalid numeric ShopBot output".into())
}

pub(super) fn entry_diagnostics(program: &ivac_cps::ir::Program) -> Vec<PipelineWarning> {
    use ivac_cps::ir::{ParamValue, Record};
    program.sections.iter().filter_map(|section| {
        let requested = section.parameters.iter().find_map(|p| {
            if p.name == "ivacam:entry-strategy" {
                if let ParamValue::Text(value) = &p.value { return Some(value.as_str()); }
            }
            None
        }).unwrap_or("direct");
        if requested != "ramp" { return None; }
        let mut z = section.initial_position.z;
        let curved = section.records.iter().any(|record| {
            let next = match record {
                Record::Rapid { z, .. } | Record::Linear { z, .. } => *z,
                Record::Circular { end, .. } => end.z,
                _ => z,
            };
            let varying = matches!(record, Record::Circular { .. }) && next != z;
            z = next;
            varying
        });
        Some(PipelineWarning::for_op(section.id, "entry_representation", if curved {
            "Requested Ramp; ShopBot uses linearized XYZ Ramp (0.01 mm chord tolerance). Final preview and simulation use emitted SBP movements."
        } else {
            "Requested Ramp; ShopBot uses XYZ Ramp. Final preview and simulation use emitted SBP movements."
        }).with_param("requested", requested)
          .with_param("resolved", if curved { "linearized_ramp" } else { "ramp" })
          .with_param("target", "shopbot")
          .with_param("tolerance_mm", "0.01")
          .with_param("preview_source", "emitted_output"))
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn output_axes_precision_feed_dwell_and_lines_are_authoritative() {
        let text = "VD,,1\nSA\nJZ,5.0000\nJ2,10.1234,20.0000\n' OP 7\nMS,25.0000,25.0000\nM3,11.1234,21.0000,-1.0000\nJZ,5.0000\nJ2,30.0000,40.0000\nPAUSE 2.000\nEND\n";
        let (p, index) = resolved_preview(text).unwrap();
        assert_eq!(p.segments.len(), 3);
        assert_eq!(p.segments[0].from.x, 10.1234);
        assert_eq!(p.segments[0].to.z, -1.);
        assert_eq!(p.segments[1].to.x, 11.1234);
        assert_eq!(p.segments[1].to.z, 5.);
        assert_eq!(p.segments[2].from.z, 5.);
        assert_eq!(p.segments[0].op_id, 7);
        assert_eq!(p.feeds_mm_min, vec![1500., 0., 0.]);
        assert_eq!(p.dwell_s, 2.);
        assert_eq!(index.segments_to_line, vec![7, 8, 9]);
        assert_eq!(index.lines_to_segment[7], 0);
        assert!(p.segments.iter().all(|s| s.arc.is_none()));
    }
    #[test]
    fn unknown_motion_and_unestablished_cut_block_instead_of_showing_ir() {
        for text in ["CG,1,2,3\n", "M3,1,2,3\n", "JZ,NaN\n", "MS,2,3\n"] {
            assert!(resolved_preview(text).is_err());
        }
    }
}
