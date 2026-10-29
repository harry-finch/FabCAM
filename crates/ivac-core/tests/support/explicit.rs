//! Test/benchmark fixture construction only. Never compiled into a production adapter.
//! Old geometry fixtures remain useful; this authors their tool values into a new v10
//! request before exercising the real pipeline. Already-authored v10/invalid-version
//! requests are untouched, so missing-role and no-fallback regressions stay meaningful.
use ivac_core::{cutting_parameters, pipeline::PipelineRequest, project::MachineMode};
#[allow(dead_code)]
pub fn request(mut req: PipelineRequest) -> PipelineRequest {
    req.project = project(req.project);
    req
}

#[allow(dead_code)]
pub fn project(mut project: ivac_core::project::Project) -> ivac_core::project::Project {
    if project.cutting_version.is_some() {
        return project;
    }
    project.cutting_version = Some(10);
    if project.machine.mode == MachineMode::Mill {
        for op in &mut project.operations {
            if op.is_program_only() {
                continue;
            }
            // Explicit fixture construction: the old geometric pocket cases used 50%.
            // Authored v9 fixtures bypass this helper, including invalid-overlap cases.
            if let Some(pocket) = op.pocket_params_mut() {
                if pocket.xy_overlap == 0.0 {
                    pocket.xy_overlap = 0.5;
                }
            }
            let mut cutting = cutting_parameters::seed(op, &project.tools);
            for rate in &mut cutting.rates {
                if let Some(feed) = op.params.feed_rate_override {
                    rate.feed_mm_min = Some(feed);
                }
                if let Some(plunge) = op.params.plunge_rate_override {
                    rate.plunge_mm_min = Some(plunge);
                }
            }
            op.params.feed_rate_override = None;
            op.params.plunge_rate_override = None;
            if ivac_core::cutting_stages::applies(op) {
                use ivac_core::cutting_stages::{CuttingStage, FinishingStage, MainFinishing};
                let stage = |role| {
                    let r = cutting
                        .rates
                        .iter()
                        .find(|r| r.role == role)
                        .or_else(|| cutting.rates.first())
                        .expect("fixture role");
                    CuttingStage {
                        tool_id: r.tool_id,
                        rpm: r.rpm,
                        feed_mm_min: r.feed_mm_min,
                        plunge_mm_min: r.plunge_mm_min,
                        depth_per_pass_mm: op.params.step.map(|v| -v),
                    }
                };
                let main = stage(cutting_parameters::CuttingRole::Rough);
                let allowance = op
                    .pocket_params()
                    .and_then(|p| p.finish_xy_allowance_mm)
                    .unwrap_or(0.0);
                let finishing = if op.finish_tool_id.is_some() || allowance > 0.0 {
                    Some(FinishingStage {
                        cutting: stage(cutting_parameters::CuttingRole::Finish),
                        lateral_allowance_mm: allowance,
                        floor_allowance_mm: 0.0,
                    })
                } else {
                    None
                };
                op.finish_tool_id = finishing.as_ref().map(|f| f.cutting.tool_id);
                op.params.stages = Some(MainFinishing::V1 { main, finishing });
                op.params.step = None;
                // A legacy final-thin-pass or explicit depth table is not remapped:
                // individual schedule regressions must author their v10 intent.
                if let Some(p) = op.pocket_params_mut() {
                    p.finish_xy_allowance_mm = None;
                }
            } else {
                op.params.cutting = Some(cutting);
            }
        }
    }
    project
}

#[allow(dead_code)]
pub fn drawing(
    mut req: ivac_core::drawing::api::DrawingGenerationRequest,
) -> ivac_core::drawing::api::DrawingGenerationRequest {
    req.machining = request(req.machining);
    req
}

/// Deliberate v10 re-authoring for controlled comparisons, never a project reader.
#[allow(dead_code)]
pub fn reviewed_v10(mut req: PipelineRequest) -> PipelineRequest {
    use ivac_core::cutting_stages::{CuttingStage, FinishingStage, MainFinishing};
    assert_eq!(req.project.cutting_version, Some(9));
    req.project.cutting_version = Some(10);
    for op in &mut req.project.operations {
        if !ivac_core::cutting_stages::applies(op) {
            continue;
        }
        let old = op.params.cutting.take().expect("authored v9 rates");
        let doc = op.params.step.take().map(|v| -v);
        let stage = |role| {
            let r = old
                .rates
                .iter()
                .find(|r| r.role == role)
                .expect("authored role");
            CuttingStage {
                tool_id: r.tool_id,
                rpm: r.rpm,
                feed_mm_min: r.feed_mm_min,
                plunge_mm_min: r.plunge_mm_min,
                depth_per_pass_mm: doc,
            }
        };
        let main = stage(cutting_parameters::CuttingRole::Rough);
        let allowance = op
            .pocket_params()
            .and_then(|p| p.finish_xy_allowance_mm)
            .unwrap_or(0.0);
        let finishing = (op.finish_tool_id.is_some() || allowance > 0.0).then(|| FinishingStage {
            cutting: stage(cutting_parameters::CuttingRole::Finish),
            lateral_allowance_mm: allowance,
            floor_allowance_mm: 0.0,
        });
        op.finish_tool_id = finishing.as_ref().map(|f| f.cutting.tool_id);
        op.params.stages = Some(MainFinishing::V1 { main, finishing });
        if let Some(p) = op.pocket_params_mut() {
            p.finish_xy_allowance_mm = None;
        }
    }
    req
}

/// Reviewed ALPHA-3 difference: stage boundaries and the obsolete geometry
/// `finish` label are comments only. Does not normalize a single machine word.
#[allow(dead_code)]
pub fn neutral_stage_comments(text: &str) -> String {
    text.split_inclusive('\n')
        .filter(|l| {
            !l.trim_start_matches([';', '\'', '('])
                .trim()
                .starts_with("FABSTAGE_1 ")
        })
        .map(|l| {
            if l.trim_start_matches([';', '\'', '('])
                .trim()
                .starts_with("object=")
            {
                l.replace(" finish)", ")").replace(" finish\n", "\n")
            } else {
                l.to_owned()
            }
        })
        .collect()
}
/// Existing Profile-only hashes have the old geometric `finish` comment.
#[allow(dead_code)]
pub fn historical_profile_comments(text: &str) -> String {
    neutral_stage_comments(text)
        .split_inclusive('\n')
        .map(|l| {
            if l.trim_start_matches([';', '('])
                .trim()
                .starts_with("object=")
                && l.contains("pocket=0 ")
            {
                if l.trim_end().ends_with(')') {
                    format!("{} finish)\n", l.trim_end().trim_end_matches(')'))
                } else {
                    format!("{} finish\n", l.trim_end())
                }
            } else {
                l.to_owned()
            }
        })
        .collect()
}
