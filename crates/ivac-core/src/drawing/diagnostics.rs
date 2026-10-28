//! V2 bounded diagnostics. Geometry IDs are evidence, never guessed labels.
use super::{Drawing, Identity};
use crate::{cancel::CancelToken, geometry::SegmentKind};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TopologyIssue {
    pub code: String,
    pub message: String,
    pub contours: Vec<Identity>,
    pub edges: Vec<Identity>,
}
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct TopologyReport {
    pub complete: bool,
    pub issues: Vec<TopologyIssue>,
}
impl TopologyReport {
    fn incomplete(&mut self, message: &str) {
        self.complete = false;
        self.issues.push(TopologyIssue {
            code: "incomplete".into(),
            message: message.into(),
            contours: vec![],
            edges: vec![],
        });
    }
}
#[must_use]
pub fn analyze(d: &Drawing, cancel: &CancelToken) -> TopologyReport {
    let mut report = TopologyReport {
        complete: true,
        issues: vec![],
    };
    if cancel.is_cancelled() {
        report.incomplete("Analysis cancelled");
        return report;
    }
    let count = d
        .contours
        .iter()
        .try_fold(0usize, |n, c| n.checked_add(c.edges.len()));
    if count.map_or(true, |n| n > 2000) {
        report.incomplete("Topology analysis budget exceeded (2000 segments)");
        return report;
    }
    let e = match d.evaluate() {
        Ok(e) => e,
        Err(err) => {
            report.incomplete(&err);
            return report;
        }
    };
    let edges: Vec<_> = d
        .contours
        .iter()
        .flat_map(|c| c.edges.iter().map(|edge| (c.id, edge.id)))
        .collect();
    for (i, s) in e.segments.iter().enumerate() {
        if let Err(message) = super::topology::validate_segment(s) {
            report.issues.push(TopologyIssue {
                code: "unsupported_primitive".into(),
                message,
                contours: vec![edges[i].0],
                edges: vec![edges[i].1],
            });
            report.complete = false;
        }
        if report.issues.len() >= 128 {
            report.incomplete("Diagnostic budget exceeded (128 issues)");
            return report;
        }
    }
    // Invalid primitives cannot participate in a trustworthy pair proof.
    if !report.complete {
        return report;
    }
    let bounds: Vec<_> = e
        .segments
        .iter()
        .map(|s| crate::geometry::BBox::from_segments(std::slice::from_ref(s)))
        .collect();
    for i in 0..e.segments.len() {
        for j in i + 1..e.segments.len() {
            if cancel.is_cancelled() {
                report.incomplete("Analysis cancelled");
                return report;
            }
            let a = bounds[i];
            let b = bounds[j];
            if a.max_x < b.min_x || b.max_x < a.min_x || a.max_y < b.min_y || b.max_y < a.min_y {
                continue;
            }
            let a = &e.segments[i];
            let b = &e.segments[j];
            if a.kind == SegmentKind::Point || b.kind == SegmentKind::Point {
                continue;
            }
            let same = edges[i].0 == edges[j].0;
            let adjacent = same
                && (j == i + 1
                    || ((i == 0 || edges[i - 1].0 != edges[i].0)
                        && edges.get(j + 1).map_or(true, |x| x.0 != edges[i].0)));
            if let Err(message) = super::topology::validate_pair(a, b, adjacent) {
                // Curved pair failures may include numerical uncertainty; these
                // are localized, but not admitted to scoped-repair proofs.
                let code = if a.kind == SegmentKind::Line && b.kind == SegmentKind::Line {
                    "line_intersection"
                } else {
                    "curve_interaction"
                };
                if code == "curve_interaction" {
                    report.complete = false;
                }
                report.issues.push(TopologyIssue {
                    code: code.into(),
                    message,
                    contours: if same {
                        vec![edges[i].0]
                    } else {
                        vec![edges[i].0, edges[j].0]
                    },
                    edges: vec![edges[i].1, edges[j].1],
                });
                if report.issues.len() >= 128 {
                    report.incomplete("Diagnostic budget exceeded (128 issues)");
                    return report;
                }
            }
        }
    }
    report
}
