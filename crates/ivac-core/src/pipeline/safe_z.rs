//! Known-stock clearance preflight, in the resolved program frame (mm).
//! Not a stock-removal, fixture or machine-kinematics safety certificate.
use super::PipelineError;
use crate::project::{MachineMode, OpKind, Project, UnitSystem};

pub(super) fn validate(project: &Project) -> Result<(), PipelineError> {
    if project.machine.mode != MachineMode::Mill {
        return Ok(());
    }
    let Some(stock) = &project.stock else {
        // No stock model: preserve the existing operator/setup contract.
        return Ok(());
    };
    // One standard four-decimal output quantum plus arithmetic uncertainty.
    // This is a serialization guard, NOT a recommended machining clearance.
    let output_guard = match project.machine.unit {
        UnitSystem::Inch => {
            let decimals = project
                .machine
                .post_profile
                .as_ref()
                .and_then(|p| p.decimal_places_inch)
                .unwrap_or(4);
            25.4 * 10_f64.powi(-i32::from(decimals))
        }
        UnitSystem::Mm => 0.0001,
    } + 1e-8;
    let header = super::setup_resolver::header_setup_for(project);
    let clearances = project
        .operations
        .iter()
        .filter(|op| {
            op.enabled
                && (!op.is_program_only()
                    || matches!(
                        op.kind,
                        OpKind::Homing {
                            retract_to_safe_z: true
                        }
                    ))
        })
        .map(|op| (op.id, op.params.fast_move_z))
        .chain(std::iter::once((0, header.mill.fast_move_z)));
    for (op_id, clearance) in clearances {
        // Identity Z custom formats may have fewer decimals than the standard
        // post. Check their actual rounded height too. Renamed/scaled/disabled
        // axes do not establish a standard work-Z contract here.
        let rounded = project
            .machine
            .post_profile
            .as_ref()
            .and_then(|p| p.axes.as_ref())
            .filter(|axes| axes.z.enabled && axes.z.name == "Z" && axes.z.scale == 1.)
            .and_then(|axes| {
                let unit_mm = if project.machine.unit == UnitSystem::Inch {
                    25.4
                } else {
                    1.
                };
                axes.z
                    .render(clearance / unit_mm)
                    .and_then(|word| word.strip_prefix('Z')?.parse::<f64>().ok())
                    .map(|value| value * unit_mm)
            });
        if !stock.top_z_mm.is_finite()
            || !clearance.is_finite()
            || clearance - stock.top_z_mm <= output_guard
            || rounded.is_some_and(|z| !z.is_finite() || z - stock.top_z_mm <= 1e-8)
        {
            return Err(PipelineError::UnsafeClearance(op_id, format!(
                "clearance Z {clearance} mm must be above stock top Z {} mm in the same work frame, by more than {output_guard} mm output uncertainty. Set a suitable clearance; values were not changed.",
                stock.top_z_mm
            )));
        }
    }
    Ok(())
}
