//! Per-pass Z schedule from `start_depth` + step + `finish_step` + `depth_list`. Plus `arc_length` helper used across the cut-emission modules.

#![allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    clippy::many_single_char_names
)]

use crate::geometry::Segment;
use crate::math;

pub(super) fn arc_length(seg: &Segment) -> f64 {
    let chord = seg.start.distance(seg.end);
    if seg.bulge.abs() < 1e-12 || chord < 1e-12 {
        return chord;
    }
    let (_, _, _, radius) = math::bulge_to_arc(seg.start, seg.end, seg.bulge);
    let theta = 4.0 * seg.bulge.atan(); // canonical bulge identity
    radius * theta.abs()
}

/// Build the per-pass Z schedule for `multi_pass`. When `depth_list`
/// is non-empty it wins as an explicit list (clamped to never go above
/// `start_depth` so a stale list doesn't accidentally cut air).
/// Otherwise: descend from `start_depth` by `step` (negative number)
/// per pass until reaching `total_depth`. When `finish_step` is set
/// and smaller in magnitude than `step`, the last pass cuts at
/// `total_depth` from `total_depth - finish_step` instead of one full
/// `step` higher — gives a thin finish pass for cleaner bottom finish.
///
/// NUMSAFE2: at most `max_passes` entries. Past that the loop stops at
/// `max_passes + 1` iterations and returns `Err(passes_seen)` — the same
/// arithmetic as an unbounded run, so the boundary is exact.
pub(super) fn build_z_schedule(
    start_depth: f64,
    total_depth: f64,
    step: f64,
    finish_step: Option<f64>,
    depth_list: &[f64],
    max_passes: usize,
) -> Result<Vec<f64>, usize> {
    if !depth_list.is_empty() {
        // Take the user's list verbatim (clamped above start_depth so
        // we don't accidentally cut air).
        let out: Vec<f64> = depth_list
            .iter()
            .copied()
            .filter(|&z| z <= start_depth + 1e-9)
            .collect();
        return if out.len() > max_passes {
            Err(out.len())
        } else {
            Ok(out)
        };
    }
    let mut out = Vec::new();
    if (step.abs() < 1e-9) || (start_depth - total_depth).abs() < 1e-9 {
        out.push(total_depth);
        return Ok(out);
    }
    // Negative step: start_depth + step < start_depth.
    let mut z = (start_depth + step).max(total_depth);
    let finish_mag = finish_step.map(f64::abs).filter(|f| *f > 1e-9);
    loop {
        // If the next pass would land at total_depth exactly and a
        // finish_step is set, splice it in: emit z one step shallower
        // than total_depth, then a final pass at total_depth.
        if z <= total_depth + 1e-9 {
            // Last pass.
            if let Some(fs) = finish_mag {
                let pre_finish = total_depth + fs;
                // Splice a pre-finish pass whenever the resulting Z
                // sits strictly between total_depth and start_depth,
                // AND it isn't a duplicate of the previous pass.
                // Bug before this: the `!out.is_empty()` guard
                // dropped the pre-finish pass on a single-pass op
                // (step >= total_depth ⇒ loop terminates with
                // `out` still empty), silently losing the finish
                // quality on thin cuts.
                let dup_of_last = out.last().is_some_and(|&l| (l - pre_finish).abs() <= 1e-9);
                if !dup_of_last
                    && pre_finish < start_depth - 1e-9
                    && pre_finish > total_depth + 1e-9
                {
                    out.push(pre_finish);
                }
            }
            out.push(total_depth);
            return if out.len() > max_passes {
                Err(out.len())
            } else {
                Ok(out)
            };
        }
        out.push(z);
        if out.len() > max_passes {
            return Err(out.len());
        }
        z = (z + step).max(total_depth);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// NUMSAFE2: the bound is exact and never changes an in-budget schedule.
    #[test]
    fn bounded_schedule_is_exact_and_identical_below_the_limit() {
        assert_eq!(
            build_z_schedule(0.0, -1000.0, -1.0, None, &[], 1000)
                .unwrap()
                .len(),
            1000
        );
        assert_eq!(
            build_z_schedule(0.0, -1001.0, -1.0, None, &[], 1000),
            Err(1001)
        );
        assert_eq!(
            build_z_schedule(0.0, -1e12, -1.0, None, &[], 1000),
            Err(1001)
        );
        let list: Vec<f64> = (1..=1001).map(|i| -f64::from(i)).collect();
        assert_eq!(
            build_z_schedule(0.0, -1.0, -1.0, None, &list, 1000),
            Err(1001)
        );
        for (start, total, step, finish) in [
            (0.0, -3.0, -0.7, None),
            (0.0, -5.0, -1.0, Some(0.2)),
            (2.0, -18.3, -0.37, Some(0.05)),
            (0.0, -99.9, -0.1, None),
        ] {
            assert_eq!(
                build_z_schedule(start, total, step, finish, &[], 1000).unwrap(),
                build_z_schedule(start, total, step, finish, &[], usize::MAX).unwrap()
            );
        }
    }

    /// Regression: `build_z_schedule` used to drop the
    /// pre-finish pass on single-pass ops because of an `!out.is_empty()`
    /// guard. With step >= depth (one pass at `total_depth`), the user's
    /// `finish_step` was silently lost.
    #[test]
    fn build_z_schedule_inserts_pre_finish_on_single_pass() {
        // Depth = -3, step = -3 (= depth, so one main pass), finish_step = 0.2.
        // Expected: pre-finish at -2.8, then total at -3.
        let s = build_z_schedule(0.0, -3.0, -3.0, Some(0.2), &[], usize::MAX).unwrap();
        assert_eq!(
            s,
            vec![-2.8, -3.0],
            "single-pass with finish_step should splice a pre-finish at depth + finish_step",
        );
    }

    /// Same `finish_step` but multi-pass: the schedule should still
    /// include the pre-finish where it makes sense, AND not duplicate
    /// it when a regular step lands at the same Z.
    #[test]
    fn build_z_schedule_inserts_pre_finish_on_multi_pass() {
        // Depth = -5, step = -1, finish_step = 0.2.
        // Main passes: -1, -2, -3, -4. Final reaches -5 with a
        // pre-finish at -4.8 spliced in, then -5.
        let s = build_z_schedule(0.0, -5.0, -1.0, Some(0.2), &[], usize::MAX).unwrap();
        assert_eq!(s, vec![-1.0, -2.0, -3.0, -4.0, -4.8, -5.0]);
    }

    /// Finish-step of zero behaves like None — no extra pass.
    #[test]
    fn build_z_schedule_finish_step_zero_is_noop() {
        let s = build_z_schedule(0.0, -3.0, -3.0, Some(0.0), &[], usize::MAX).unwrap();
        assert_eq!(s, vec![-3.0]);
    }

    /// Negative `finish_step` normalizes to positive magnitude.
    /// The caller (`multi_pass`) abs()-and-filters before reaching us,
    /// but the schedule builder itself must also be robust against a
    /// negative slipping through — otherwise a stale serialized
    /// project with `finish_step = -0.2` would either produce a
    /// schedule that cuts above `start_depth` or duplicates the final
    /// pass.
    #[test]
    fn build_z_schedule_negative_finish_step_normalized() {
        // Mirror the positive-finish_step single-pass test with the
        // sign flipped — output must be identical.
        let s = build_z_schedule(0.0, -3.0, -3.0, Some(-0.2), &[], usize::MAX).unwrap();
        assert_eq!(
            s,
            vec![-2.8, -3.0],
            "negative finish_step must be treated as its absolute value",
        );
    }
}
