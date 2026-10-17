//! Boundary characterization only: do not turn measured discontinuities into physical claims.
#[path = "../examples/sensitivity_support/mod.rs"]
mod sensitivity_support;
use ivac_feeds_speeds::*;
use sensitivity_support::*;

#[test]
fn diameter_sweep_is_order_independent_and_preserves_evidence_and_caps() {
    let p = pack();
    let mut reverse = p.clone();
    reverse.records.reverse();
    reverse.dataset.profile_set.profiles.reverse();
    for mode in [
        Aggressiveness::Prudent,
        Aggressiveness::Normal,
        Aggressiveness::Rapid,
    ] {
        for d in diameters() {
            let mut r = request(&p, d);
            r.aggressiveness = mode;
            r.machine.rpm_max = Some(16000);
            r.machine.max_feed_mm_min = Some(700);
            r.machine.max_plunge_mm_min = Some(300);
            r.operation.doc_mm = Some(10.0);
            let a = recommend_packs(std::slice::from_ref(&p), &r).unwrap();
            let b = recommend_packs(std::slice::from_ref(&reverse), &r).unwrap();
            assert_eq!(
                a.recommendation, b.recommendation,
                "diameter {d}, mode {mode:?}"
            );
            assert_eq!(a.explanation, b.explanation);
            if let FeedsSpeedsRecommendation::Available { recommendation: a } = a.recommendation {
                assert_eq!(a.source_evidence, Evidence::ReferenceData);
                assert!((8000..=16000).contains(&a.parameters.rpm));
                assert!(a.parameters.feed_mm_min > 0 && a.parameters.feed_mm_min <= 700);
                assert!(a.parameters.plunge_mm_min > 0 && a.parameters.plunge_mm_min <= 300);
                assert!(a.parameters.doc_mm > 0.0 && a.parameters.doc_mm <= 10.0);
            }
        }
    }
    assert_eq!(p, pack());
}

fn result(d: f64) -> KnowledgeResult {
    let p = pack();
    let mut r = request(&p, d);
    r.operation.doc_mm = Some(10.0);
    recommend_packs(&[p], &r).unwrap()
}

#[test]
fn availability_boundaries_are_explicit_on_both_sides() {
    for (d, below_available) in [(3.175 / 1.5, false), (6.35 / 0.5, true)] {
        for (offset, expected) in [(-0.000001, below_available), (0.000001, !below_available)] {
            let r = result(d + offset);
            assert_eq!(
                matches!(
                    r.recommendation,
                    FeedsSpeedsRecommendation::Available { .. }
                ),
                expected
            );
            if !expected {
                assert!(!r.scope_rejections.is_empty());
            }
        }
    }
}

#[test]
fn ten_percent_small_tool_transition_is_continuous_with_integer_feed_rounding() {
    let boundary = 3.175 / 1.1;
    let low = result(boundary - 0.000001);
    let high = result(boundary + 0.000001);
    assert_eq!(
        low.explanation.as_ref().unwrap().summary.match_quality,
        Some(MatchQuality::Approximate)
    );
    assert_eq!(
        high.explanation.as_ref().unwrap().summary.match_quality,
        Some(MatchQuality::Close)
    );
    let (
        FeedsSpeedsRecommendation::Available {
            recommendation: low,
        },
        FeedsSpeedsRecommendation::Available {
            recommendation: high,
        },
    ) = (low.recommendation, high.recommendation)
    else {
        panic!("expected available boundary");
    };
    // Policy 2 removes the step; integer feed output may still differ by one mm/min.
    assert_eq!(low.parameters.rpm, high.parameters.rpm);
    let ratio = high.parameters.feed_mm_min as f64 / low.parameters.feed_mm_min as f64;
    assert!((ratio - 1.0).abs() < 0.002);
    assert!((high.parameters.doc_mm / low.parameters.doc_mm - 1.0).abs() < 0.00001);
}

#[test]
fn interpolation_entry_exposes_a_larger_selection_jump() {
    let boundary = 6.35 / 1.5;
    let before = result(boundary - 0.000001);
    let after = result(boundary + 0.000001);
    assert_eq!(
        before.explanation.as_ref().unwrap().summary.match_quality,
        Some(MatchQuality::Approximate)
    );
    assert_eq!(
        after.explanation.as_ref().unwrap().summary.match_quality,
        Some(MatchQuality::Interpolated)
    );
    let (
        FeedsSpeedsRecommendation::Available {
            recommendation: before,
        },
        FeedsSpeedsRecommendation::Available {
            recommendation: after,
        },
    ) = (before.recommendation, after.recommendation)
    else {
        panic!("expected available interpolation boundary");
    };
    assert!(after.parameters.feed_mm_min as f64 / before.parameters.feed_mm_min as f64 > 1.5);
    assert_eq!(before.source_evidence, after.source_evidence);
}
