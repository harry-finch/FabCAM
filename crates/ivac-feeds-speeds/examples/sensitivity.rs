//! CSV audit: cargo run -p ivac-feeds-speeds --example sensitivity > sensitivity.csv
mod sensitivity_support;
use ivac_feeds_speeds::*;
use sensitivity_support::*;
fn main() {
    let p = pack();
    println!("scenario,mode,diameter_mm,quality,record,source_diameters,rpm,feed_mm_min,plunge_mm_min,doc_mm,transition");
    for scenario in ["doc1", "doc10", "limited"] {
        for mode in [
            Aggressiveness::Prudent,
            Aggressiveness::Normal,
            Aggressiveness::Rapid,
        ] {
            let mut previous = String::new();
            for d in diameters() {
                let mut r = request(&p, d);
                r.aggressiveness = mode;
                r.operation.doc_mm = Some(if scenario == "doc1" { 1.0 } else { 10.0 });
                // Two unconstrained scenarios isolate policy; third uses synthetic hard caps.
                r.machine.rpm_min = None;
                r.machine.rpm_max = None;
                r.machine.max_feed_mm_min = None;
                r.machine.max_plunge_mm_min = None;
                if scenario == "limited" {
                    r.machine.rpm_max = Some(16000);
                    r.machine.max_feed_mm_min = Some(700);
                    r.machine.max_plunge_mm_min = Some(300);
                }
                let result = recommend_packs(std::slice::from_ref(&p), &r).unwrap();
                let (key, values) = match result.recommendation {
                    FeedsSpeedsRecommendation::Available { recommendation: a } => {
                        let e = result.explanation.unwrap();
                        let record = e
                            .records
                            .iter()
                            .find(|v| {
                                v.record_id == e.summary.selected_record
                                    && v.pack == e.summary.selected_pack
                            })
                            .unwrap();
                        let ds = record
                            .derivation
                            .as_ref()
                            .map(|v| {
                                v.source_diameters_mm
                                    .iter()
                                    .map(ToString::to_string)
                                    .collect::<Vec<_>>()
                                    .join("+")
                            })
                            .unwrap_or_default();
                        let key = format!(
                            "{:?},{},{}",
                            e.summary.match_quality.unwrap(),
                            e.summary.selected_record,
                            ds
                        );
                        let v = a.parameters;
                        (
                            key,
                            format!(
                                "{},{},{},{}",
                                v.rpm, v.feed_mm_min, v.plunge_mm_min, v.doc_mm
                            ),
                        )
                    }
                    _ => ("Unavailable,none,none".into(), ",,,".into()),
                };
                let transition = key != previous;
                println!("{scenario},{mode:?},{d:.9},{key},{values},{transition}");
                previous = key;
            }
        }
    }
}
