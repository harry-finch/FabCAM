#!/usr/bin/env python3
"""Validate documentation examples with the real local Rust loader/resolver.

Writes build/harness artifacts only to temporary directories, never the source tree.
No network, source registration, application storage or machining is involved.
"""
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile

RUST = r'''
use ivac_feeds_speeds::*;
use std::{env, fs};
fn main() {
    let mut packs = Vec::new();
    for path in env::args().skip(1) {
        let raw = fs::read_to_string(&path).expect("read example");
        let pack = load_source_pack_json_checked(&raw).expect("real loader accepts example");
        assert!(!public_bundle_eligible(&pack));
        assert_eq!(pack.dataset.profile_set.profiles.len(), 3);
        for p in &pack.dataset.profile_set.profiles {
            assert!(matches!(p.evidence, Evidence::ReferenceData | Evidence::Theoretical));
            assert!(p.source.evidence_record.is_none());
            let engagement = match p.scope.engagement {
                EngagementScope::FullWidth => Engagement::KnownFullWidth,
                EngagementScope::Nominal { ratio } => Engagement::Nominal { ratio: ratio.max },
                EngagementScope::UnknownSafe => Engagement::Unknown,
            };
            let exact = |s: &IdentitySelector| match s {
                IdentitySelector::Exact { identity } => Some(identity.clone()),
                _ => None,
            };
            let request = FeedsSpeedsRequest {
                schema_version: 1, dataset: pack.dataset.profile_set.identity.clone(),
                machine: MachineSnapshot { identity: exact(&p.scope.machine), rpm_min: None,
                    rpm_max: None, max_feed_mm_min: None, max_plunge_mm_min: None },
                material: MaterialSnapshot { identity: exact(&p.scope.material), thickness_mm: None },
                tool: ToolSnapshot { identity: p.scope.tool.clone(), cutting_family: p.scope.cutting_family,
                    diameter_mm: 6.0, flute_count: 2, max_rpm: None },
                operation: OperationContext { kind: p.scope.operation,
                    doc_mm: p.scope.doc_mm.map(|r| r.max), engagement,
                    entry: p.scope.entry.unwrap_or(EntryStrategy::Unknown) },
                aggressiveness: p.aggressiveness,
            };
            let result = recommend_packs_checked(std::slice::from_ref(&pack), &request)
                .expect("resolver accepts matching context");
            assert!(matches!(result.recommendation, FeedsSpeedsRecommendation::Available { .. }),
                "matching demo record unavailable: {:?}", result.recommendation);
        }
        let mut unknown: serde_json::Value = serde_json::from_str(&raw).unwrap();
        unknown["manifest"]["unsupported_demo_field"] = true.into();
        assert!(load_source_pack_json_checked(&unknown.to_string()).is_err());
        let mut false_claim: serde_json::Value = serde_json::from_str(&raw).unwrap();
        false_claim["dataset"]["profile_set"]["profiles"][0]["evidence"] = "tested".into();
        assert!(load_source_pack_json_checked(&false_claim.to_string()).is_err());
        let mut missing_origin = pack.clone(); missing_origin.records.pop();
        assert!(validate_source_pack_checked(&missing_origin).is_err());
        println!("PASS {}: loader, 3 matching contexts, unknown-field/evidence/origin refusals", path);
        packs.push(pack);
    }
    assert_eq!(packs.len(), 2);
    assert_eq!(packs[0].manifest.identity.id, packs[1].manifest.identity.id);
    assert_ne!(packs[0].manifest.identity.revision, packs[1].manifest.identity.revision);
    assert_eq!(packs[0].dataset.machines, packs[1].dataset.machines);
    assert_eq!(packs[0].dataset.materials, packs[1].dataset.materials);
    assert_eq!(packs[0].dataset.tools, packs[1].dataset.tools);
    assert_eq!(packs[0].dataset.profile_set.profiles[0].identity.id,
               packs[1].dataset.profile_set.profiles[0].identity.id);
    assert_ne!(packs[0].dataset.profile_set.profiles[0].identity.revision,
               packs[1].dataset.profile_set.profiles[0].identity.revision);
    assert_eq!(packs[0].dataset.profile_set.profiles[1..], packs[1].dataset.profile_set.profiles[1..]);
    println!("PASS revision pair: stable pack/catalog/record IDs, new immutable version, unchanged other records");
}
'''

def main():
    here = Path(__file__).resolve().parent
    root = here.parents[2]
    files = [here / "workshop-demo-v1.json", here / "workshop-demo-v2.json"]
    cargo = shutil.which("cargo")
    if not cargo:
        raise SystemExit("cargo is required; use the repository's documented Rust setup")
    for file in files:
        json.loads(file.read_text())
        print(f"SHA256 {hashlib.sha256(file.read_bytes()).hexdigest()}  {file.name}", flush=True)
    with tempfile.TemporaryDirectory(prefix="fabcam-fs-guide-") as temporary:
        tmp = Path(temporary)
        dependency = json.dumps(str(root / "crates/ivac-feeds-speeds"))
        (tmp / "Cargo.toml").write_text(
            '[package]\nname = "fabcam-fs-guide-check"\nversion = "0.0.0"\nedition = "2021"\n'
            '[workspace]\n[dependencies]\nivac-feeds-speeds = { path = ' + dependency + ' }\n'
            'serde_json = "1"\n'
        )
        (tmp / "src").mkdir()
        (tmp / "src/main.rs").write_text(RUST)
        # Seed dependency versions from this checkout, without modifying its lockfile.
        shutil.copyfile(root / "Cargo.lock", tmp / "Cargo.lock")
        environment = dict(os.environ)
        environment["RUSTC_WRAPPER"] = ""
        environment["CARGO_TARGET_DIR"] = str(Path(tempfile.gettempdir()) / "fabcam-fs-guide-target")
        subprocess.run([cargo, "run", "--offline", "--quiet", "--manifest-path",
                        str(tmp / "Cargo.toml"), "--", *map(str, files)],
                       cwd=root, env=environment, check=True)

if __name__ == "__main__":
    main()
