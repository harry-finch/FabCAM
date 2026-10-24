//! Deterministic V-Carve arithmetic, qualified in QUAL-1A.4.
//!
//! Pinned software routines avoid platform libm selection at V-Carve's tangent
//! and planner-length boundaries. No rounding, predicate or geometry budget change.
#[inline]
#[must_use]
pub fn tan(radians: f64) -> f64 {
    libm::tan(radians)
}

#[inline]
#[must_use]
pub fn hypot(x: f64, y: f64) -> f64 {
    libm::hypot(x, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn length_extremes_do_not_overflow_or_underflow_prematurely() {
        for magnitude in [1e-300, 1e-100, 1., 1e100, 1e300] {
            let value = hypot(3. * magnitude, 4. * magnitude);
            assert!((value / magnitude - 5.).abs() < 2e-15);
        }
        assert!(hypot(f64::INFINITY, f64::NAN).is_infinite());
        assert!(hypot(f64::NAN, 1.).is_nan());
        assert_eq!(hypot(-0., 0.).to_bits(), 0);
    }

    #[test]
    fn tangent_domain_extremes_and_nonfinite_behavior() {
        for angle in [0.5_f64, 15., 30., 45., 60., 89.5] {
            let value = tan(angle.to_radians());
            assert!(value.is_finite() && value > 0.);
        }
        assert_eq!(tan(-0.).to_bits(), (-0.0_f64).to_bits());
        assert!(tan(f64::INFINITY).is_nan());
        assert!(tan(f64::NAN).is_nan());
    }
}
