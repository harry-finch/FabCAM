//! Resolve authored drawing XY to the program work frame, once, before CAM.
//! No target-machine semantics enter the geometry algorithms.
use super::PipelineError;
use crate::project::{MachineMode, OpKind, PatternConfig, Project};

pub(super) fn resolve(project: &mut Project) -> Result<(), PipelineError> {
    let w = &project.work_offset;
    if ![w.x_mm, w.y_mm, w.z_mm].iter().all(|v| v.is_finite()) {
        return Err(PipelineError::WorkOrigin(
            "work origin must be finite".into(),
        ));
    }
    if w.z_mm != 0. {
        return Err(PipelineError::WorkOrigin("numeric Z origin translation is not qualified; use zero work-origin Z and an explicit stock top/depth convention".into()));
    }
    let (x, y) = (w.x_mm, w.y_mm);
    if x == 0. && y == 0. {
        return Ok(());
    }
    if project.machine.mode != MachineMode::Mill
        || project
            .operations
            .iter()
            .any(|op| op.enabled && matches!(op.kind, OpKind::GcodeInclude { .. }))
    {
        return Err(PipelineError::WorkOrigin(
            "XY origin translation requires milling without raw included machine commands".into(),
        ));
    }
    for s in &mut project.segments {
        s.start.x -= x;
        s.start.y -= y;
        s.end.x -= x;
        s.end.y -= y;
        if let Some(center) = &mut s.center {
            center.x -= x;
            center.y -= y;
        }
    }
    if let Some(stock) = &mut project.stock {
        stock.origin[0] -= x;
        stock.origin[1] -= y;
    }
    for f in &mut project.fixtures {
        f.origin.0 -= x;
        f.origin.1 -= y;
    }
    for layer in &mut project.text_layers {
        layer.origin.0 -= x;
        layer.origin.1 -= y;
    }
    for source in &mut project.relief_sources {
        source.origin.x -= x;
        source.origin.y -= y;
    }
    for op in &mut project.operations {
        if let Some(c) = op.contour_params_mut() {
            if let Some(point) = &mut c.approach_point {
                point.0 -= x;
                point.1 -= y;
            }
        }
        if let OpKind::Drill {
            pattern:
                Some(PatternConfig::Polar {
                    center_x, center_y, ..
                }),
            ..
        } = &mut op.kind
        {
            *center_x -= x;
            *center_y -= y;
        }
    }
    let finite = project.segments.iter().all(|s| {
        [s.start.x, s.start.y, s.end.x, s.end.y]
            .iter()
            .all(|v| v.is_finite())
            && s.center
                .map_or(true, |p| p.x.is_finite() && p.y.is_finite())
    }) && project
        .stock
        .as_ref()
        .map_or(true, |s| s.origin.iter().all(|v| v.is_finite()))
        && project
            .fixtures
            .iter()
            .all(|f| f.origin.0.is_finite() && f.origin.1.is_finite());
    if !finite {
        return Err(PipelineError::WorkOrigin(
            "XY origin translation produced non-finite geometry".into(),
        ));
    }
    // All following pipeline geometry is now in WCS. Retain the WCS slot;
    // zero numerical translation to prevent a second application by consumers.
    project.work_offset.x_mm = 0.;
    project.work_offset.y_mm = 0.;
    Ok(())
}

pub(super) fn fixture_warnings(
    project: &Project,
    path: &[crate::gcode::preview::ToolpathSegment],
    warnings: &mut Vec<super::PipelineWarning>,
) {
    use crate::sim::fixture_check::{check_segment_against_fixtures, FixtureCheck};
    use crate::sim::holder::HolderProfile;
    if project.fixtures.is_empty() {
        return;
    }
    let mut seen = std::collections::BTreeSet::new();
    let mut unknown = std::collections::BTreeSet::new();
    // Without a declared body/holder length, do not certify a clamp above
    // the cutting tip as clear. Conservatively test its XY footprint at any
    // height above the tip; this can warn rather than invent tool dimensions.
    let footprints: Vec<_> = project
        .fixtures
        .iter()
        .cloned()
        .map(|mut f| {
            f.z_bottom = f64::MIN;
            f
        })
        .collect();
    for s in path {
        let Some(op) = project.operations.iter().find(|op| op.id == s.op_id) else {
            continue;
        };
        // With dual tools, op_id alone cannot identify the active bit. Check
        // both declared tools conservatively, rather than omit the finish bit.
        for tool in project
            .tools
            .iter()
            .filter(|t| t.id == op.tool_id || Some(t.id) == op.finish_tool_id)
        {
            let holder = HolderProfile::from_tool(tool);
            if holder.is_none() && unknown.insert((s.op_id, tool.id)) {
                warnings.push(super::PipelineWarning::for_op(s.op_id,"fixture_clearance_unknown",
                    format!("operation {}: cutter footprint checked, but complete body/holder clearance is unknown for '{}'",s.op_id,tool.name))
                    .with_param("op_id",s.op_id.to_string()).with_param("tool_name",tool.name.as_str()));
            }
            for hit in check_segment_against_fixtures(
                s,
                tool.diameter * 0.5,
                holder.as_ref(),
                if holder.is_some() {
                    &project.fixtures
                } else {
                    &footprints
                },
            ) {
                if let FixtureCheck::Collision { fixture_id, .. } = hit {
                    if seen.insert((s.op_id, fixture_id)) {
                        let name = project
                            .fixtures
                            .iter()
                            .find(|f| f.id == fixture_id)
                            .map_or("", |f| f.name.as_str());
                        warnings.push(super::PipelineWarning::for_op(s.op_id,"fixture_path_collision",
                            format!("operation {}: tool path may intersect fixture '{name}'; no automatic avoidance",s.op_id))
                            .with_param("op_id",s.op_id.to_string()).with_param("fixture_id",fixture_id.to_string()).with_param("fixture_name",name));
                    }
                }
            }
        }
    }
}

pub(super) fn validate_fixtures(project: &Project) -> Result<(), PipelineError> {
    use crate::project::FixtureKind;
    for f in &project.fixtures {
        let valid = [f.origin.0, f.origin.1, f.z_bottom, f.z_top]
            .iter()
            .all(|v| v.is_finite())
            && f.z_top > f.z_bottom
            && match &f.kind {
                FixtureKind::Box { width, depth } => {
                    width.is_finite() && depth.is_finite() && *width > 0. && *depth > 0.
                }
                FixtureKind::Cylinder { radius } => radius.is_finite() && *radius > 0.,
                FixtureKind::Polygon { vertices } => {
                    vertices.len() >= 3
                        && vertices.iter().all(|p| p.0.is_finite() && p.1.is_finite())
                        && simple_polygon(vertices)
                }
            };
        if !valid {
            return Err(PipelineError::Fixture(format!(
                "obstacle {} has invalid dimensions, height range or polygon; repair or remove it",
                f.id
            )));
        }
    }
    Ok(())
}
fn simple_polygon(vertices: &[(f64, f64)]) -> bool {
    use crate::geometry::Point2;
    let n = vertices.len();
    let points: Vec<_> = vertices.iter().map(|v| Point2 { x: v.0, y: v.1 }).collect();
    let mut area = 0.;
    for i in 0..n {
        let a = points[i];
        let b = points[(i + 1) % n];
        if a.distance(b) <= 1e-9 {
            return false;
        }
        area += a.x * b.y - b.x * a.y;
        for j in (i + 1)..n {
            if j == i + 1 || (i == 0 && j == n - 1) {
                continue;
            }
            if crate::cam::lines_intersect(a, b, points[j], points[(j + 1) % n]).is_some() {
                return false;
            }
        }
    }
    area.abs() > 1e-9
}
