//! Operation-owned cycle/engagement fields copied by a reviewed preset load.
use crate::project::{DrillCycle, Op, OpKind};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum SpecializedParameters {
    Drill {
        cycle: DrillCycle,
        spot_depth_mm: Option<f64>,
        chamfer_width_mm: Option<f64>,
    },
    Relief {
        scallop_height_mm: f64,
        stepover_mm: Option<f64>,
        along_step_mm: f64,
    },
    Waterline {
        z_step_mm: f64,
        stepover_mm: f64,
    },
}
impl SpecializedParameters {
    #[must_use]
    pub fn capture(op: &Op) -> Option<Self> {
        match op.kind {
            OpKind::Drill {
                cycle,
                spot_first,
                chamfer_after_width_mm,
                ..
            } => Some(Self::Drill {
                cycle,
                spot_depth_mm: spot_first.map(|s| s.spot_depth_mm),
                chamfer_width_mm: chamfer_after_width_mm,
            }),
            OpKind::ReliefMill {
                scallop_height_mm,
                stepover_mm,
                along_step_mm,
                ..
            } => Some(Self::Relief {
                scallop_height_mm,
                stepover_mm,
                along_step_mm,
            }),
            OpKind::WaterlineRough {
                z_step_mm,
                stepover_mm,
                ..
            } => Some(Self::Waterline {
                z_step_mm,
                stepover_mm,
            }),
            _ => None,
        }
    }
    pub fn validate(&self) -> Result<(), String> {
        let pos = |v: f64| v.is_finite() && v > 0.;
        let valid = match *self {
            Self::Drill {
                cycle,
                spot_depth_mm,
                chamfer_width_mm,
            } => {
                let (step, dwell) = match cycle {
                    DrillCycle::Simple { dwell_sec } => (None, dwell_sec),
                    DrillCycle::Peck {
                        peck_step_mm,
                        dwell_sec,
                    }
                    | DrillCycle::ChipBreak {
                        peck_step_mm,
                        dwell_sec,
                    } => (Some(peck_step_mm), dwell_sec),
                };
                step.map_or(true, pos)
                    && dwell.is_finite()
                    && dwell >= 0.
                    && spot_depth_mm.map_or(true, |v| v.is_finite() && v < 0.)
                    && chamfer_width_mm.map_or(true, pos)
            }
            Self::Relief {
                scallop_height_mm,
                stepover_mm,
                along_step_mm,
            } => {
                scallop_height_mm.is_finite()
                    && scallop_height_mm >= 0.
                    && (stepover_mm.is_some() || pos(scallop_height_mm))
                    && stepover_mm.map_or(true, pos)
                    && pos(along_step_mm)
            }
            Self::Waterline {
                z_step_mm,
                stepover_mm,
            } => pos(z_step_mm) && pos(stepover_mm),
        };
        if valid {
            Ok(())
        } else {
            Err("Invalid specialized preset cycle or engagement".into())
        }
    }
    pub fn apply(&self, op: &mut Op) -> Result<(), String> {
        self.validate()?;
        match (self, &mut op.kind) {
            (
                Self::Drill {
                    cycle,
                    spot_depth_mm,
                    chamfer_width_mm,
                },
                OpKind::Drill {
                    cycle: target,
                    spot_first,
                    chamfer_after_width_mm,
                    ..
                },
            ) => {
                if spot_depth_mm.is_some() != spot_first.is_some()
                    || chamfer_width_mm.is_some() != chamfer_after_width_mm.is_some()
                {
                    return Err("Preset cannot add or remove drilling roles".into());
                }
                *target = *cycle;
                if let (Some(v), Some(spot)) = (spot_depth_mm, spot_first) {
                    spot.spot_depth_mm = *v;
                }
                *chamfer_after_width_mm = *chamfer_width_mm;
            }
            (
                Self::Relief {
                    scallop_height_mm,
                    stepover_mm,
                    along_step_mm,
                },
                OpKind::ReliefMill {
                    scallop_height_mm: a,
                    stepover_mm: b,
                    along_step_mm: c,
                    ..
                },
            ) => {
                *a = *scallop_height_mm;
                *b = *stepover_mm;
                *c = *along_step_mm;
            }
            (
                Self::Waterline {
                    z_step_mm,
                    stepover_mm,
                },
                OpKind::WaterlineRough {
                    z_step_mm: a,
                    stepover_mm: b,
                    ..
                },
            ) => {
                *a = *z_step_mm;
                *b = *stepover_mm;
            }
            _ => return Err("Preset specialized values do not match this operation".into()),
        }
        Ok(())
    }
}
/// Geometry, strategy and role presence stay in compatibility; copied numbers do not.
pub fn remove_copied_values(kind: &mut serde_json::Value) {
    let Some(map) = kind.as_object_mut() else {
        return;
    };
    match map
        .get("type")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("")
    {
        "drill" => {
            map.remove("cycle");
            if let Some(spot) = map
                .get_mut("spot_first")
                .and_then(serde_json::Value::as_object_mut)
            {
                spot.remove("spot_depth_mm");
            }
            if map
                .get("chamfer_after_width_mm")
                .is_some_and(|v| !v.is_null())
            {
                map.insert("chamfer_after_width_mm".into(), serde_json::json!(true));
            }
        }
        "relief_mill" => {
            for key in ["scallop_height_mm", "stepover_mm", "along_step_mm"] {
                map.remove(key);
            }
        }
        "waterline_rough" => {
            map.remove("z_step_mm");
            map.remove("stepover_mm");
        }
        _ => {}
    }
}
