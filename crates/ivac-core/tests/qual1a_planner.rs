//! Isolated QUAL-1A adversarial planner regressions; unchanged accepted budgets.
use ivac_core::cam::vcarve_plan::{plan, DEPTH_EPS};

#[test]
fn malformed_and_budget_inputs_refuse() {
    for axis in [
        vec![(0., 0., 0.)],
        vec![(0., 0., -1.), (0., 0., -1.)],
        vec![(0., 0., -1.), (f64::NAN, 0., -1.)],
        vec![(0., 0., -1.), (1., 0., f64::INFINITY)],
        vec![(0., 0., -1.); 20_001],
    ] {
        assert!(plan(&axis, 0.5, 10.).is_err());
    }
    for step in [0., -0.1, f64::NAN, f64::INFINITY] {
        assert!(plan(&[(0., 0., -1.), (1., 0., -1.)], step, 10.).is_err());
    }
    for angle in [0., 90., f64::NAN, f64::INFINITY] {
        assert!(plan(&[(0., 0., -1.), (1., 0., -1.)], 0.5, angle).is_err());
    }
}

#[test]
fn affine_and_one_ulp_kinks_preserve_depth_and_completion() {
    let next = f64::from_bits(1_f64.to_bits() + 1);
    for middle in [1., next] {
        for angle in [1., 10., 45., 89.] {
            let axis = [(0., 0., -1.), (1., 0., -middle), (2., 0., -1.)];
            let a = plan(&axis, 0.25, angle).unwrap();
            let b = plan(&axis, 0.25, angle).unwrap();
            assert_eq!(a, b);
            assert!(a.iter().all(|p| p.2 >= -middle - DEPTH_EPS && p.2 <= 0.));
            for x in [0.0_f64, 2.] {
                assert!(a
                    .iter()
                    .any(|p| p.0.to_bits() == x.to_bits() && (p.2 + 1.).abs() <= DEPTH_EPS));
            }
        }
    }
}
