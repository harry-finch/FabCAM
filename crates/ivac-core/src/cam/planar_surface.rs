//! Bounded planar raster in drawing-frame mm. The first/last scanlines lie
//! on the target edges; along-scan endpoints extend by the cutter radius plus
//! explicit overtravel. Thus every target point, including corners, is at most
//! radius from a scanline when stepover <= diameter. No geometry compensation,
//! feeds, machine syntax or stock inference lives in this module.

use crate::cam::surface_mill::ScanDirection;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
pub struct PlanarSurfaceParams {
    pub origin: [f64; 2],
    pub width_mm: f64,
    pub height_mm: f64,
    /// Expands the target rectangle on all four sides (mm, nonnegative).
    pub margin_mm: f64,
    /// Extra beyond the compulsory radius at each along-scan end (mm).
    pub overtravel_mm: f64,
    pub stepover_mm: f64,
    pub direction: ScanDirection,
}

#[derive(Debug, Clone)]
pub struct PlanarRaster {
    pub lines: Vec<[[f64; 2]; 2]>,
    /// [min X, min Y, max X, max Y] of cutter centers, drawing frame.
    pub center_bounds: [f64; 4],
    pub target_bounds: [f64; 4],
}

/// Allocation limit, independent of physical machining tolerances.
pub const MAX_SURFACE_LINES: usize = 100_000;

// value is range-checked or bounded above; float casts saturate; operand is non-negative by construction (validated, clamped or saturating).
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
pub fn planar_raster(p: &PlanarSurfaceParams, diameter: f64) -> Result<PlanarRaster, String> {
    let values = [
        p.origin[0],
        p.origin[1],
        p.width_mm,
        p.height_mm,
        p.margin_mm,
        p.overtravel_mm,
        p.stepover_mm,
        diameter,
    ];
    if !values.iter().all(|v| v.is_finite()) {
        return Err("Surfacing dimensions must be finite millimetres.".into());
    }
    if p.width_mm <= 0.0
        || p.height_mm <= 0.0
        || diameter <= 0.0
        || p.margin_mm < 0.0
        || p.overtravel_mm < 0.0
        || p.stepover_mm <= 0.0
        || p.stepover_mm > diameter
    {
        return Err("Surfacing needs positive dimensions and a stepover no greater than the flat cutter diameter; margins and overtravel must be nonnegative.".into());
    }
    let [x, y] = p.origin;
    let m = p.margin_mm;
    let target = [x - m, y - m, x + p.width_mm + m, y + p.height_mm + m];
    let along_x = matches!(p.direction, ScanDirection::AlongX);
    let (lo, hi, cross_lo, cross_hi) = if along_x {
        (target[0], target[2], target[1], target[3])
    } else {
        (target[1], target[3], target[0], target[2])
    };
    let end_extension = diameter * 0.5 + p.overtravel_mm;
    let lo = lo - end_extension;
    let hi = hi + end_extension;
    let intervals = ((cross_hi - cross_lo) / p.stepover_mm).ceil().max(1.0);
    if ![lo, hi, cross_lo, cross_hi, intervals]
        .iter()
        .all(|v| v.is_finite())
        || hi <= lo
        || cross_hi <= cross_lo
        || intervals >= MAX_SURFACE_LINES as f64
    {
        return Err(
            "Surfacing rectangle is unrepresentable or exceeds the 100000-scanline limit.".into(),
        );
    }
    let intervals = intervals as usize;
    let mut lines = Vec::with_capacity(intervals + 1);
    for i in 0..=intervals {
        // Redistribute to land exactly on BOTH edges without a short remainder.
        let c = if i == intervals {
            cross_hi
        } else {
            cross_lo + (cross_hi - cross_lo) * (i as f64 / intervals as f64)
        };
        let a = if along_x { [lo, c] } else { [c, lo] };
        let b = if along_x { [hi, c] } else { [c, hi] };
        lines.push(if i % 2 == 0 { [a, b] } else { [b, a] });
    }
    Ok(PlanarRaster {
        lines,
        target_bounds: target,
        center_bounds: if along_x {
            [lo, cross_lo, hi, cross_hi]
        } else {
            [cross_lo, lo, cross_hi, hi]
        },
    })
}

#[cfg(test)]
// Test fixtures assert exact literal values and bounded indices.
#[allow(clippy::float_cmp)]
mod tests {
    use super::*;
    fn params() -> PlanarSurfaceParams {
        PlanarSurfaceParams {
            origin: [13.0, 17.0],
            width_mm: 41.0,
            height_mm: 23.0,
            margin_mm: 2.0,
            overtravel_mm: 1.0,
            stepover_mm: 4.0,
            direction: ScanDirection::AlongX,
        }
    }
    #[test]
    fn complete_target_coverage_including_corners_and_both_directions() {
        for direction in [ScanDirection::AlongX, ScanDirection::AlongY] {
            let p = PlanarSurfaceParams {
                direction,
                ..params()
            };
            let raster = planar_raster(&p, 6.0).unwrap();
            let [xmin, ymin, xmax, ymax] = raster.target_bounds;
            // Independent nearest-segment test on a dense grid AND all edges.
            for ix in 0..=90 {
                for iy in 0..=90 {
                    let q = [
                        xmin + (xmax - xmin) * f64::from(ix) / 90.0,
                        ymin + (ymax - ymin) * f64::from(iy) / 90.0,
                    ];
                    let distance = raster
                        .lines
                        .iter()
                        .map(|[a, b]| {
                            let v = [b[0] - a[0], b[1] - a[1]];
                            let t = (((q[0] - a[0]) * v[0] + (q[1] - a[1]) * v[1])
                                / (v[0] * v[0] + v[1] * v[1]))
                                .clamp(0.0, 1.0);
                            (q[0] - a[0] - t * v[0]).hypot(q[1] - a[1] - t * v[1])
                        })
                        .fold(f64::INFINITY, f64::min);
                    assert!(distance <= 3.0 + 1e-10, "uncovered {q:?}");
                }
            }
        }
    }
    #[test]
    fn radius_margin_extra_and_alternation_are_explicit() {
        let r = planar_raster(&params(), 6.0).unwrap();
        assert_eq!(r.center_bounds, [7.0, 15.0, 60.0, 42.0]);
        assert_eq!(r.lines.first().unwrap(), &[[7.0, 15.0], [60.0, 15.0]]);
        assert!(r.lines[1][0][0] > r.lines[1][1][0]);
        assert_eq!(r.lines.last().unwrap()[0][1], 42.0);
    }
    #[test]
    fn tiny_target_and_diameter_equal_step_cover_edges() {
        let p = PlanarSurfaceParams {
            width_mm: 0.1,
            height_mm: 0.2,
            margin_mm: 0.0,
            overtravel_mm: 0.0,
            stepover_mm: 6.0,
            ..params()
        };
        assert_eq!(planar_raster(&p, 6.0).unwrap().lines.len(), 2);
    }
    #[test]
    fn malformed_and_excessive_parameters_are_refused() {
        for bad in [f64::NAN, f64::INFINITY, 0.0, -1.0, 6.01, 1e-20] {
            let p = PlanarSurfaceParams {
                stepover_mm: bad,
                ..params()
            };
            assert!(planar_raster(&p, 6.0).is_err());
        }
        assert!(planar_raster(&params(), -6.0).is_err());
        assert!(planar_raster(
            &PlanarSurfaceParams {
                margin_mm: -1.0,
                ..params()
            },
            6.0
        )
        .is_err());
        assert!(planar_raster(
            &PlanarSurfaceParams {
                origin: [1e308, 1e308],
                ..params()
            },
            6.0
        )
        .is_err());
    }
}
