//! Tab attachment — projecting imported-segment-keyed tab points onto the
//! generated offset polylines. Split out of `offsets.rs`. The tab
//! [`TabPoint`] type lives here and is re-exported by the parent so
//! `PolylineOffset.tabs` and external `cam::offsets::TabPoint` users resolve
//! unchanged.

use super::PolylineOffset;
use crate::geometry::{Segment, SegmentKind};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, JsonSchema)]
pub struct TabPoint {
    pub x: f64,
    pub y: f64,
    /// Per-tab width override (mm). When `Some`, this tab uses the
    /// override; when `None`, falls back to the op-level setup width
    /// (`setup.tabs.width`). Audit finding: was hashed into the cache key
    /// but never consumed by `emit_path_with_tabs` — toggling overrides
    /// produced identical output. Now drives the per-tab crossing
    /// radius + zone half-width.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub width_override_mm: Option<f64>,
    /// Per-tab height override (mm). When `Some`, the cutter lifts to
    /// `cut_z + override` over this tab instead of the op-level
    /// `setup.tabs.height`. None ⇒ inherit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub height_override_mm: Option<f64>,
}

impl TabPoint {
    /// Effective tab-zone half-width (mm) — per-tab override wins
    /// over the op-level fallback.
    #[must_use]
    pub fn radius(&self, fallback_full_width_mm: f64) -> f64 {
        match self.width_override_mm {
            Some(w) if w > 0.0 => w * 0.5,
            _ => fallback_full_width_mm * 0.5,
        }
    }

    /// Effective lift over this tab (mm). Per-tab override wins.
    #[must_use]
    pub fn lift(&self, fallback_mm: f64) -> f64 {
        self.height_override_mm
            .filter(|v| *v > 0.0)
            .unwrap_or(fallback_mm)
    }
}

/// Project a list of imported-segment-keyed tab points onto a generated
/// offset's tab list. We snap each tab to the closest point on the
/// offset's polyline; tabs that land further than `max_distance` from the
/// nearest segment are dropped (they belong to a different object).
pub fn attach_tabs_to_offsets(
    offsets: &mut [PolylineOffset],
    tabs_by_object: &HashMap<usize, Vec<TabPoint>>,
    max_distance: f64,
) {
    for offset in offsets.iter_mut() {
        let Some(tabs) = tabs_by_object.get(&offset.source_object_idx) else {
            continue;
        };
        for tab in tabs {
            // Snap to closest point on any segment of this offset.
            if let Some(snap) = snap_to_offset(offset, *tab, max_distance) {
                offset.tabs.push(snap);
            }
        }
    }
}

fn snap_to_offset(offset: &PolylineOffset, tab: TabPoint, max_distance: f64) -> Option<TabPoint> {
    let mut best: Option<(TabPoint, f64)> = None;
    for seg in &offset.segments {
        let p = closest_point_on_segment(seg, tab);
        let d = (p.x - tab.x).hypot(p.y - tab.y);
        if d > max_distance {
            continue;
        }
        if best.map_or(true, |(_, bd)| d < bd) {
            best = Some((p, d));
        }
    }
    best.map(|(p, _)| p)
}

fn closest_point_on_segment(seg: &Segment, tab: TabPoint) -> TabPoint {
    // Curved offset tabs must attach to the arc, not its chord. Chord
    // projection can discard a valid placement or move it off the cutter path.
    if matches!(seg.kind, SegmentKind::Arc | SegmentKind::Circle) {
        let center = seg
            .center
            .unwrap_or_else(|| crate::math::bulge_to_arc(seg.start, seg.end, seg.bulge).0);
        let radius = seg.start.distance(center);
        let angle = (tab.y - center.y).atan2(tab.x - center.x);
        let start_angle = (seg.start.y - center.y).atan2(seg.start.x - center.x);
        let point = if crate::math::arc_contains_angle(start_angle, 4.0 * seg.bulge.atan(), angle) {
            crate::geometry::Point2::new(
                center.x + radius * angle.cos(),
                center.y + radius * angle.sin(),
            )
        } else if (tab.x - seg.start.x).hypot(tab.y - seg.start.y)
            <= (tab.x - seg.end.x).hypot(tab.y - seg.end.y)
        {
            seg.start
        } else {
            seg.end
        };
        return TabPoint {
            x: point.x,
            y: point.y,
            ..tab
        };
    }
    let dx = seg.end.x - seg.start.x;
    let dy = seg.end.y - seg.start.y;
    let len_sq = dx * dx + dy * dy;
    if len_sq < 1e-12 {
        return TabPoint {
            x: seg.start.x,
            y: seg.start.y,
            width_override_mm: tab.width_override_mm,
            height_override_mm: tab.height_override_mm,
        };
    }
    let t = (((tab.x - seg.start.x) * dx + (tab.y - seg.start.y) * dy) / len_sq).clamp(0.0, 1.0);
    TabPoint {
        x: seg.start.x + t * dx,
        y: seg.start.y + t * dy,
        width_override_mm: tab.width_override_mm,
        height_override_mm: tab.height_override_mm,
    }
}
