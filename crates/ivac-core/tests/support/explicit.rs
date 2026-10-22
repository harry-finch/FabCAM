//! Test/benchmark fixture construction only. Never compiled into a production adapter.
//! Old geometry fixtures remain useful; this authors their tool values into a new v9
//! request before exercising the real pipeline. Already-authored v9/invalid-version
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
    project.cutting_version = Some(9);
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
            op.params.cutting = Some(cutting);
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
