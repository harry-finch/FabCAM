//! Software audit fixtures; machine limits are synthetic, not hardware specifications.
use ivac_feeds_speeds::{
    load_source_pack_json, taxon_identity, Aggressiveness, CuttingFamily, Engagement,
    EntryStrategy, FeedsSpeedsRequest, Identity, MachineSnapshot, MaterialSnapshot,
    OperationContext, OperationKind, SourcePack, ToolSnapshot,
};
pub fn pack() -> SourcePack {
    load_source_pack_json(include_str!(
        "../../../../data/feeds-speeds/packs/sienci-longmill.v3.json"
    ))
    .unwrap()
}
pub fn request(p: &SourcePack, d: f64) -> FeedsSpeedsRequest {
    let s = &p.dataset.profile_set.profiles[0];
    FeedsSpeedsRequest {
        schema_version: 1,
        dataset: p.dataset.profile_set.identity.clone(),
        machine: MachineSnapshot {
            identity: Some(Identity {
                id: "machine/fablab-router".into(),
                revision: "1".into(),
            }),
            rpm_min: Some(8000),
            rpm_max: Some(24000),
            max_feed_mm_min: Some(6000),
            max_plunge_mm_min: Some(2000),
        },
        material: MaterialSnapshot {
            identity: Some(taxon_identity("wood.softwood")),
            thickness_mm: Some(12.0),
        },
        tool: ToolSnapshot {
            identity: s.scope.tool.clone(),
            cutting_family: CuttingFamily::FlatEndmill,
            diameter_mm: d,
            flute_count: 1,
            max_rpm: None,
        },
        operation: OperationContext {
            kind: OperationKind::Pocket,
            doc_mm: Some(1.0),
            engagement: Engagement::Nominal { ratio: 0.5 },
            entry: EntryStrategy::Direct,
        },
        aggressiveness: Aggressiveness::Prudent,
    }
}

pub fn diameters() -> Vec<f64> {
    let mut ds: Vec<f64> = (100..=1400).map(|i| f64::from(i) / 100.0).collect();
    for source in [3.175, 6.35] {
        for delta in [0.0, 0.1, 0.25, 0.5] {
            for boundary in [source / (1.0 + delta), source / (1.0 - delta)] {
                for epsilon in [-0.000_001, 0.0, 0.000_001] {
                    ds.push(boundary + epsilon);
                }
            }
        }
    }
    ds.sort_by(f64::total_cmp);
    ds.dedup();
    ds
}
