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
        if let OpKind::PlanarSurface { surface } = &mut op.kind {
            surface.origin[0] -= x;
            surface.origin[1] -= y;
        }
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
    fixture_warnings_cancellable(project, path, warnings, None);
}

pub(super) fn fixture_warnings_cancellable(
    project: &Project,
    path: &[crate::gcode::preview::ToolpathSegment],
    warnings: &mut Vec<super::PipelineWarning>,
    cancel: Option<&super::CancelToken>,
) {
    use crate::sim::fixture_check::{check_segment_against_fixtures, FixtureCheck};
    use crate::sim::holder::HolderProfile;
    if project.fixtures.is_empty() {
        return;
    }
    let uses_tool = |o: &crate::project::Op, id: u32| {
        id == o.tool_id
            || Some(id) == o.finish_tool_id
            || matches!(&o.kind, OpKind::Drill { spot_first: Some(spot), .. } if spot.spot_tool_id == id)
    };
    let mut seen = std::collections::BTreeSet::new();
    let mut unknown = std::collections::BTreeSet::new();
    for s in path {
        if super::cancelled(cancel) {
            return;
        }
        let op = project
            .operations
            .iter()
            .find(|op| s.op_id != 0 && op.id == s.op_id);
        // With dual tools, op_id alone cannot identify the active bit. Check
        // both declared tools conservatively, rather than omit the finish bit.
        for tool in project.tools.iter().filter(|t| {
            op.map_or_else(
                // Program/header/footer movements lack reliable active-tool
                // provenance. Cover every enabled operation's possible tool,
                // including dual-tool finish, rather than guess list order.
                || {
                    project
                        .operations
                        .iter()
                        .any(|o| o.enabled && uses_tool(o, t.id))
                },
                |o| uses_tool(o, t.id),
            )
        }) {
            let (holder, complete) = HolderProfile::declared_fixture_envelope(tool);
            if !complete && unknown.insert((s.op_id, tool.id)) {
                warnings.push(super::PipelineWarning::for_op(s.op_id,"fixture_clearance_unknown",
                    format!("operation {}: cutter footprint checked, but complete body/holder clearance is unknown for '{}'",s.op_id,tool.name))
                    .with_param("op_id",s.op_id.to_string()).with_param("tool_name",tool.name.as_str()));
            }
            for hit in check_segment_against_fixtures(
                s,
                tool.diameter.max(tool.effective_diameter()) * 0.5,
                holder.as_ref(),
                &project.fixtures,
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

#[cfg(test)]
mod lot7_tests {
    use super::*;
    use crate::gcode::preview::{MoveKind, Pose3, ToolpathSegment};
    use crate::pipeline::test_helpers::{endmill, profile_op, project_with};
    use crate::project::{Fixture, FixtureKind, ToolOffset};
    #[test]
    fn lot7_unowned_rapids_check_nominal_and_larger_effective_envelopes() {
        let op = profile_op(1, 1, ToolOffset::Outside);
        let mut project = project_with(vec![op], vec![endmill(1, 2.)]);
        project.fixtures.push(Fixture {
            id: 7,
            name: "clamp".into(),
            kind: FixtureKind::Box {
                width: 0.2,
                depth: 0.2,
            },
            origin: (0., 0.),
            z_bottom: 0.,
            z_top: 20.,
            color: 0,
        });
        let mut segment = ToolpathSegment {
            tool_ref: None,
            from: Pose3 {
                x: -5.,
                y: 1.05,
                z: 5.,
            },
            to: Pose3 {
                x: 5.,
                y: 1.05,
                z: 5.,
            },
            kind: MoveKind::Rapid,
            gcode_line: 1,
            op_id: 0,
            arc: None,
        };
        for wear in [1., -1.] {
            project.tools[0].wear_offset_mm = wear;
            if wear < 0. {
                segment.from.y = 1.4;
                segment.to.y = 1.4;
            }
            let mut warnings = Vec::new();
            fixture_warnings(&project, &[segment.clone()], &mut warnings);
            assert!(warnings
                .iter()
                .any(|w| w.kind == "fixture_path_collision" && w.op_id == Some(0)));
        }
        // Zero is also a program-motion sentinel: even if an authored op has
        // ID zero, it must not hide another possible tool on a header/footer.
        project.operations[0].id = 0;
        project.tools[0].wear_offset_mm = 0.;
        project
            .operations
            .push(profile_op(2, 2, ToolOffset::Outside));
        project.tools.push(endmill(2, 4.));
        segment.from.y = 1.8;
        segment.to.y = 1.8;
        let mut warnings = Vec::new();
        fixture_warnings(&project, &[segment.clone()], &mut warnings);
        assert!(warnings.iter().any(|w| w.kind == "fixture_path_collision"));
        segment.from.y = 50.;
        segment.to.y = 50.;
        let mut warnings = Vec::new();
        fixture_warnings(&project, &[segment], &mut warnings);
        assert!(!warnings.iter().any(|w| w.kind == "fixture_path_collision"));
    }
}
