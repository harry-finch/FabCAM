//! Reproducible coverage audit of the reviewed bundled pack; no machining validation.
use ivac_feeds_speeds::{
    load_source_pack_json, material_taxonomy, recommend_packs, taxon_identity, Aggressiveness,
    CuttingFamily, Engagement, EntryStrategy, FeedsSpeedsRequest, Identity, MachineSnapshot,
    MatchQuality, MaterialSnapshot, OperationContext, OperationKind, ToolSnapshot,
    KNOWLEDGE_POLICY,
};
fn main() {
    let pack = load_source_pack_json(include_str!(
        "../../../data/feeds-speeds/packs/sienci-longmill.v3.json"
    ))
    .unwrap();
    let diameters = [
        1.0, 2.0, 3.0, 3.175, 4.0, 5.0, 6.0, 6.35, 8.0, 10.0, 12.0, 12.7,
    ];
    println!("# FS3.9 — Public coverage audit\n\nGenerated with `cargo run -p ivac-feeds-speeds --example coverage`.\nPack: `{}` @ `{}`; policy: `{}`. Six original records, no new cutting rows.\n",pack.manifest.identity.id,pack.manifest.identity.revision,KNOWLEDGE_POLICY);
    println!("## Controlled audit context\n\nExplicit fablab-router class, confirmed flat upcut 1F source series, Prudent, nominal Pocket 50%, DOC 1 mm, direct entry (source entry unknown), no invented hard limits. This is derived coverage, not physical validation. Quality labels describe diameter adaptation; material/machine/full-slot transfers remain separately disclosed.\n\nC = close normalized derivation; A = approximate; I = interpolation; E = conservative extrapolation; — = missing.\n");
    print!("| Material / diameter mm |");
    for d in diameters {
        print!(" {d} |");
    }
    println!();
    print!("| --- |");
    for _ in diameters {
        print!(" --- |");
    }
    println!();
    for taxon in material_taxonomy().into_iter().filter(|t| {
        t.parent.is_some() && !matches!(t.id.as_str(), "plastic.pmma" | "metal.aluminium")
    }) {
        print!("| {} |", taxon.label);
        for d in diameters {
            let r = FeedsSpeedsRequest {
                schema_version: 1,
                dataset: pack.dataset.profile_set.identity.clone(),
                machine: MachineSnapshot {
                    identity: Some(Identity {
                        id: "machine/fablab-router".into(),
                        revision: "1".into(),
                    }),
                    rpm_min: None,
                    rpm_max: None,
                    max_feed_mm_min: None,
                    max_plunge_mm_min: None,
                },
                material: MaterialSnapshot {
                    identity: Some(taxon_identity(&taxon.id)),
                    thickness_mm: Some(12.0),
                },
                tool: ToolSnapshot {
                    identity: pack.dataset.profile_set.profiles[0].scope.tool.clone(),
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
            };
            let result = recommend_packs(&[pack.clone()], &r).unwrap();
            let label = result
                .explanation
                .map_or("—", |e| match e.summary.match_quality {
                    Some(MatchQuality::Close) => "C",
                    Some(MatchQuality::Approximate) => "A",
                    Some(MatchQuality::Interpolated) => "I",
                    Some(MatchQuality::Extrapolated) => "E",
                    _ => "—",
                });
            print!(" {label} |");
        }
        println!();
    }
    println!("\nGeneric aluminium (including 6061/7075) has no data. Plywood, birch and poplar are not equated to the source's **soft plywood** category. Cast/extruded PMMA, POM, HDPE, polycarbonate and PVC have no data. No generic wood, plastic, metal or theoretical fallback records are bundled.\n\n## Exact source coverage\n\n| Material scope | Tool | Diameter | Operation | Machine | Modes |\n| --- | --- | --- | --- | --- | --- |\n| Source softwood / soft plywood / MDF category | Flat upcut 1F | 3.175, 6.35 | Full slot | LongMill reference identity | Prudent / Normal / Rapid |\n\nThese six exact combinations retain original RPM/feed/plunge/DOC values. Exact diameter alone does not make a transferred Pocket/ShopBot/material context an exact reference.\n\n## Operation and physical-family coverage\n\n| Context | Coverage |\n| --- | --- |\n| Profile with explicit full-width confirmation | Full-slot basis, bounded diameter adaptation |\n| Profile without known engagement | Missing |\n| Pocket with nominal engagement | Explicit full-slot-derived policy, no chip-thinning uplift |\n| Full slot | Supported within material/tool/machine policy |\n| Finishing | Missing; no silent slot-to-finish transfer |\n| Surfacing | Unsupported engine operation; missing dedicated data |\n| Flat upcut 2F | Missing for every audited diameter |\n| Compression, downcut, ball, V-bit | Missing / incompatible |\n| Unknown machine or physical tool traits | Refused; explicit applicability selection required |\n| Industrial router | No automatic fablab-class transfer |\n\n## Priorities for later work\n\nP0: real ShopBot softwood/MDF Pocket Ø6 and Ø3, distinguish 1F and 2F, DOC/WOC/entry and tool identity. P1: plywood/birch/poplar and common plastics with approved rights. P2: small-tool Ø1/Ø2, aluminium and dedicated surfacing cutters. Physical testing remains FS4 **DEFERRED, NONBLOCKING**. Public growth: first review additional rows from the already licensed pinned Sienci resource; other vendors require data-specific rights review. No proprietary tables imported.\n");
}
