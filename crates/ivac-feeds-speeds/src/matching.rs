use crate::*;

fn members<'a>(id: &str, families: &'a [FamilyMembership]) -> &'a [Identity] {
    families
        .iter()
        .find(|f| f.id == id)
        .map_or(&[], |f| f.members.as_slice())
}

fn identity_matches(
    s: &IdentitySelector,
    value: Option<&Identity>,
    families: &[FamilyMembership],
) -> bool {
    match s {
        IdentitySelector::Any => true,
        IdentitySelector::Exact { identity } => value == Some(identity),
        IdentitySelector::Family { id } => value.is_some_and(|v| members(id, families).contains(v)),
    }
}

fn range_matches(range: Option<Range>, value: Option<f64>) -> bool {
    range.map_or(true, |range| {
        value.is_some_and(|v| v >= range.min && v <= range.max)
    })
}

pub(crate) fn mismatches(
    p: &Profile,
    r: &FeedsSpeedsRequest,
    set: &ProfileSet,
) -> Vec<ScopeMismatch> {
    let s = &p.scope;
    let mut result = Vec::new();
    macro_rules! check {
        ($field:literal, $condition:expr, $actual:expr, $required:expr) => {
            if !$condition {
                result.push(ScopeMismatch {
                    field: $field.into(),
                    actual: serde_json::to_string(&$actual).expect("serializable scope value"),
                    required: serde_json::to_string(&$required).expect("serializable scope value"),
                });
            }
        };
    }
    check!(
        "mode",
        p.aggressiveness == r.aggressiveness,
        r.aggressiveness,
        p.aggressiveness
    );
    check!(
        "machine",
        identity_matches(
            &s.machine,
            r.machine.identity.as_ref(),
            &set.machine_families
        ),
        r.machine.identity,
        s.machine
    );
    check!(
        "material",
        identity_matches(
            &s.material,
            r.material.identity.as_ref(),
            &set.material_families
        ),
        r.material.identity,
        s.material
    );
    check!(
        "tool",
        s.tool
            .as_ref()
            .map_or(true, |id| r.tool.identity.as_ref() == Some(id)),
        r.tool.identity,
        s.tool
    );
    check!(
        "cutting_family",
        s.cutting_family == r.tool.cutting_family,
        r.tool.cutting_family,
        s.cutting_family
    );
    check!(
        "diameter_mm",
        range_matches(Some(s.diameter_mm), Some(r.tool.diameter_mm)),
        r.tool.diameter_mm,
        s.diameter_mm
    );
    check!(
        "flute_count",
        s.flute_count.map_or(true, |n| n == r.tool.flute_count),
        r.tool.flute_count,
        s.flute_count
    );
    check!(
        "operation",
        s.operation == r.operation.kind,
        r.operation.kind,
        s.operation
    );
    check!(
        "doc_mm",
        range_matches(s.doc_mm, r.operation.doc_mm),
        r.operation.doc_mm,
        s.doc_mm
    );
    check!(
        "thickness_mm",
        range_matches(s.thickness_mm, r.material.thickness_mm),
        r.material.thickness_mm,
        s.thickness_mm
    );
    check!(
        "entry",
        s.entry.map_or(true, |e| e == r.operation.entry),
        r.operation.entry,
        s.entry
    );
    let engagement_matches = match (s.engagement, r.operation.engagement) {
        (EngagementScope::FullWidth, Engagement::KnownFullWidth)
        | (EngagementScope::UnknownSafe, Engagement::Unknown) => true,
        (EngagementScope::Nominal { ratio: range }, Engagement::Nominal { ratio }) => {
            range_matches(Some(range), Some(ratio))
        }
        _ => false,
    };
    check!(
        "engagement",
        engagement_matches,
        r.operation.engagement,
        s.engagement
    );
    result
}

pub(crate) fn matches(p: &Profile, r: &FeedsSpeedsRequest, set: &ProfileSet) -> bool {
    mismatches(p, r, set).is_empty()
}

pub(crate) fn tier(p: &Profile) -> u8 {
    match (p.category, p.evidence) {
        (ProfileCategory::ExactPreset, Evidence::Validated) => 0,
        (ProfileCategory::ExactPreset, _) => 1,
        (ProfileCategory::MachineFamily, _) => 2,
        (ProfileCategory::Reference, _) => 3,
        (ProfileCategory::Theoretical, _) => 4,
    }
}

fn selector_subset(
    a: &IdentitySelector,
    b: &IdentitySelector,
    families: &[FamilyMembership],
) -> bool {
    match (a, b) {
        (_, IdentitySelector::Any) => true,
        (IdentitySelector::Any, _) => false,
        (IdentitySelector::Exact { identity: a }, IdentitySelector::Exact { identity: b }) => {
            a == b
        }
        (IdentitySelector::Exact { identity }, IdentitySelector::Family { id }) => {
            members(id, families).contains(identity)
        }
        (IdentitySelector::Family { id }, IdentitySelector::Exact { identity }) => {
            members(id, families) == [identity.clone()]
        }
        (IdentitySelector::Family { id: a }, IdentitySelector::Family { id: b }) => {
            members(a, families)
                .iter()
                .all(|v| members(b, families).contains(v))
        }
    }
}

fn optional_subset<T: PartialEq>(a: Option<&T>, b: Option<&T>) -> bool {
    b.is_none() || a == b
}

fn range_subset(a: Option<Range>, b: Option<Range>) -> bool {
    match (a, b) {
        (_, None) => true,
        (Some(a), Some(b)) => a.min >= b.min && a.max <= b.max,
        _ => false,
    }
}

fn scope_subset(a: &ProfileScope, b: &ProfileScope, set: &ProfileSet) -> bool {
    selector_subset(&a.machine, &b.machine, &set.machine_families)
        && selector_subset(&a.material, &b.material, &set.material_families)
        && optional_subset(a.tool.as_ref(), b.tool.as_ref())
        && a.cutting_family == b.cutting_family
        && range_subset(Some(a.diameter_mm), Some(b.diameter_mm))
        && optional_subset(a.flute_count.as_ref(), b.flute_count.as_ref())
        && a.operation == b.operation
        && range_subset(a.doc_mm, b.doc_mm)
        && range_subset(a.thickness_mm, b.thickness_mm)
        && optional_subset(a.entry.as_ref(), b.entry.as_ref())
        && match (a.engagement, b.engagement) {
            (EngagementScope::Nominal { ratio: a }, EngagementScope::Nominal { ratio: b }) => {
                range_subset(Some(a), Some(b))
            }
            (a, b) => a == b,
        }
}

pub(crate) fn strictly_narrower(a: &Profile, b: &Profile, set: &ProfileSet) -> bool {
    scope_subset(&a.scope, &b.scope, set) && !scope_subset(&b.scope, &a.scope, set)
}
