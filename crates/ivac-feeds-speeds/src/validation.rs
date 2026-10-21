use std::collections::BTreeSet;

use crate::{
    CuttingData, CuttingFamily, Engagement, EngagementScope, EntryStrategy, Evidence,
    FamilyMembership, FeedsSpeedsRequest, Identity, IdentitySelector, OperationKind,
    ProfileCategory, ProfileSet, Range, ValidationIssue, DOMAIN_SCHEMA_VERSION,
};

#[derive(Default)]
struct Checks(Vec<ValidationIssue>);

impl Checks {
    fn require(&mut self, valid: bool, field: &str, message: &str) {
        if !valid {
            self.0.push(ValidationIssue {
                field: field.into(),
                message: message.into(),
            });
        }
    }

    fn positive(&mut self, value: f64, field: &str) {
        self.require(
            value.is_finite() && value > 0.0,
            field,
            "must be finite and positive",
        );
    }

    fn identity(&mut self, value: &Identity, field: &str) {
        self.require(
            !value.id.trim().is_empty() && !value.revision.trim().is_empty(),
            field,
            "identity and revision must be nonempty",
        );
    }

    fn range(&mut self, value: Range, field: &str, ratio: bool) {
        self.positive(value.min, field);
        self.positive(value.max, field);
        self.require(value.min <= value.max, field, "range must be ordered");
        if ratio {
            self.require(value.max <= 1.0, field, "ratio must not exceed one");
        }
    }

    fn finish(mut self) -> Vec<ValidationIssue> {
        self.0
            .sort_by(|a, b| (&a.field, &a.message).cmp(&(&b.field, &b.message)));
        self.0.dedup();
        self.0
    }
}

pub(crate) fn request_issues(r: &FeedsSpeedsRequest) -> Vec<ValidationIssue> {
    let mut c = Checks::default();
    c.require(
        r.schema_version == DOMAIN_SCHEMA_VERSION,
        "schema_version",
        "unsupported domain version",
    );
    c.identity(&r.dataset, "dataset");
    for (field, identity) in [
        ("machine.identity", &r.machine.identity),
        ("machine.local_identity", &r.machine.local_identity),
        ("material.identity", &r.material.identity),
        ("tool.identity", &r.tool.identity),
        ("tool.local_identity", &r.tool.local_identity),
    ] {
        if let Some(identity) = identity {
            c.identity(identity, field);
        }
    }
    for (field, bound) in [
        ("machine.rpm_min", r.machine.rpm_min),
        ("machine.rpm_max", r.machine.rpm_max),
        ("machine.max_feed_mm_min", r.machine.max_feed_mm_min),
        ("machine.max_plunge_mm_min", r.machine.max_plunge_mm_min),
        ("tool.max_rpm", r.tool.max_rpm),
    ] {
        if let Some(bound) = bound {
            c.require(bound > 0, field, "limit must be positive or unknown");
        }
    }
    let upper = match (r.machine.rpm_max, r.tool.max_rpm) {
        (Some(a), Some(b)) => Some(a.min(b)),
        (a, b) => a.or(b),
    };
    if let (Some(min), Some(max)) = (r.machine.rpm_min, upper) {
        c.require(
            min <= max,
            "machine.rpm_min",
            "machine/tool RPM limits do not intersect",
        );
    }
    c.positive(r.tool.diameter_mm, "tool.diameter_mm");
    c.require(
        r.tool.flute_count > 0,
        "tool.flute_count",
        "flute count must be positive",
    );
    if let Some(doc) = r.operation.doc_mm {
        c.positive(doc, "operation.doc_mm");
    }
    if let Some(thickness) = r.material.thickness_mm {
        c.positive(thickness, "material.thickness_mm");
    }
    if let Engagement::Nominal { ratio } = r.operation.engagement {
        c.positive(ratio, "operation.engagement.ratio");
        c.require(
            ratio <= 1.0,
            "operation.engagement.ratio",
            "ratio must not exceed one",
        );
    }
    if r.operation.kind == OperationKind::FullSlot {
        c.require(
            r.operation.engagement == Engagement::KnownFullWidth,
            "operation.engagement",
            "full-slot request requires known full-width engagement",
        );
    }
    c.finish()
}

fn families(c: &mut Checks, families: &[FamilyMembership], field: &str) {
    let mut ids = BTreeSet::new();
    for family in families {
        c.require(
            !family.id.trim().is_empty(),
            field,
            "family ID must be nonempty",
        );
        c.require(ids.insert(&family.id), field, "duplicate family ID");
        c.require(
            !family.members.is_empty(),
            field,
            "family must have explicit members",
        );
        let mut members = BTreeSet::new();
        for member in &family.members {
            c.identity(member, field);
            c.require(members.insert(member), field, "duplicate family member");
        }
    }
}

fn selector(c: &mut Checks, s: &IdentitySelector, families: &[FamilyMembership], field: &str) {
    match s {
        IdentitySelector::Any => {}
        IdentitySelector::Exact { identity } => c.identity(identity, field),
        IdentitySelector::Family { id } => c.require(
            families.iter().any(|f| &f.id == id),
            field,
            "unresolved family reference",
        ),
    }
}

pub(crate) fn set_issues(set: &ProfileSet) -> Vec<ValidationIssue> {
    let mut c = Checks::default();
    c.require(
        set.schema_version == DOMAIN_SCHEMA_VERSION,
        "schema_version",
        "unsupported domain version",
    );
    c.identity(&set.identity, "identity");
    families(&mut c, &set.machine_families, "machine_families");
    families(&mut c, &set.material_families, "material_families");
    let mut ids = BTreeSet::new();
    for p in &set.profiles {
        let field = format!("profiles[{}@{}]", p.identity.id, p.identity.revision);
        c.identity(&p.identity, &field);
        // Two active revisions of the same profile are ambiguous authoring.
        c.require(ids.insert(&p.identity.id), &field, "duplicate profile ID");
        let s = &p.scope;
        selector(&mut c, &s.machine, &set.machine_families, &field);
        selector(&mut c, &s.material, &set.material_families, &field);
        c.require(
            !matches!(s.material, IdentitySelector::Any),
            &field,
            "material scope is required",
        );
        if let Some(tool) = &s.tool {
            c.identity(tool, &field);
        }
        c.require(
            s.cutting_family == CuttingFamily::FlatEndmill,
            &field,
            "unsupported cutting family in FS1",
        );
        c.require(
            matches!(
                s.operation,
                OperationKind::Profile
                    | OperationKind::Pocket
                    | OperationKind::FullSlot
                    | OperationKind::Finish
            ),
            &field,
            "unsupported operation in FS1",
        );
        c.range(s.diameter_mm, &field, false);
        if let Some(flutes) = s.flute_count {
            c.require(flutes > 0, &field, "flute count must be positive");
        }
        if let Some(doc) = s.doc_mm {
            c.range(doc, &field, false);
        }
        if let Some(thickness) = s.thickness_mm {
            c.range(thickness, &field, false);
        }
        if let EngagementScope::Nominal { ratio } = s.engagement {
            c.range(ratio, &field, true);
        }
        if s.operation == OperationKind::FullSlot {
            c.require(
                s.engagement == EngagementScope::FullWidth,
                &field,
                "slot profile must cover full width",
            );
        }
        for text in [
            &p.source.reference,
            &p.source.author,
            &p.source.date,
            &p.source.permission_or_license,
        ] {
            c.require(
                !text.trim().is_empty(),
                &field,
                "source, author, date and permission are required",
            );
        }
        match p.category {
            ProfileCategory::ExactPreset => {
                c.require(
                    matches!(p.evidence, Evidence::Tested | Evidence::Validated),
                    &field,
                    "exact preset requires tested/validated evidence",
                );
                c.require(
                    matches!(s.machine, IdentitySelector::Exact { .. })
                        && matches!(s.material, IdentitySelector::Exact { .. })
                        && s.tool.is_some()
                        && s.flute_count.is_some()
                        && s.doc_mm.is_some()
                        && s.entry.is_some_and(|e| e != EntryStrategy::Unknown)
                        && s.engagement != EngagementScope::UnknownSafe,
                    &field,
                    "exact preset requires complete machine/material/tool/pass/entry/engagement scope",
                );
                c.require(
                    matches!(p.data, CuttingData::ExactValues { .. }) && p.reductions.is_empty(),
                    &field,
                    "exact presets require complete values without reductions",
                );
                let record = p.source.evidence_record.as_ref();
                c.require(
                    record.is_some_and(|r| !r.trial_reference.trim().is_empty()),
                    &field,
                    "trial reference required",
                );
                if p.evidence == Evidence::Validated {
                    c.require(
                        record
                            .and_then(|r| r.validation_criteria_reference.as_ref())
                            .is_some_and(|s| !s.trim().is_empty()),
                        &field,
                        "validated preset requires criteria reference",
                    );
                }
            }
            ProfileCategory::MachineFamily => {
                c.require(
                    !matches!(s.machine, IdentitySelector::Any)
                        && p.evidence == Evidence::ReferenceData,
                    &field,
                    "machine profile needs machine scope and reference evidence",
                );
            }
            ProfileCategory::Reference => c.require(
                p.evidence == Evidence::ReferenceData,
                &field,
                "reference profile needs reference evidence",
            ),
            ProfileCategory::Theoretical => c.require(
                p.evidence == Evidence::Theoretical,
                &field,
                "theoretical profile needs theoretical evidence",
            ),
        }
        let (rpm, rate, plunge, doc, stepover) = match p.data {
            CuttingData::ChiploadTarget {
                rpm,
                chipload_mm_tooth,
                plunge_mm_min,
                doc_mm,
                stepover_ratio,
            } => (
                rpm,
                chipload_mm_tooth,
                plunge_mm_min,
                doc_mm,
                stepover_ratio,
            ),
            CuttingData::ExactValues {
                rpm,
                feed_mm_min,
                plunge_mm_min,
                doc_mm,
                stepover_ratio,
            } => (rpm, feed_mm_min, plunge_mm_min, doc_mm, stepover_ratio),
        };
        for value in [rpm, rate, plunge, doc] {
            c.positive(value, &field);
        }
        if let Some(range) = s.doc_mm {
            c.require(
                doc >= range.min && doc <= range.max,
                &field,
                "payload DOC outside declared scope",
            );
        }
        if let Some(ratio) = stepover {
            c.positive(ratio, &field);
            c.require(ratio <= 1.0, &field, "stepover ratio must not exceed one");
            if let EngagementScope::Nominal { ratio: range } = s.engagement {
                c.require(
                    ratio >= range.min && ratio <= range.max,
                    &field,
                    "payload stepover outside declared scope",
                );
            }
        }
        if s.operation == OperationKind::Pocket {
            c.require(
                stepover.is_some(),
                &field,
                "pocket requires supplied stepover",
            );
        }
        if matches!(
            s.operation,
            OperationKind::Profile | OperationKind::FullSlot
        ) {
            c.require(
                stepover.is_none(),
                &field,
                "profile/slot does not recommend a lateral cascade step",
            );
        }
        let mut rule_ids = BTreeSet::new();
        for rule in &p.reductions {
            c.identity(&rule.identity, &field);
            c.require(
                rule_ids.insert(&rule.identity.id),
                &field,
                "duplicate reduction ID",
            );
            c.require(
                matches!(p.data, CuttingData::ChiploadTarget { .. }),
                &field,
                "reductions require chipload-target data",
            );
            c.require(
                !rule.source_reference.trim().is_empty(),
                &field,
                "reduction source required",
            );
            for factor in [rule.chipload_factor, rule.plunge_factor, rule.doc_factor] {
                c.positive(factor, &field);
                c.require(
                    factor <= 1.0,
                    &field,
                    "FS1 reductions cannot expand a target",
                );
            }
        }
    }
    c.finish()
}
