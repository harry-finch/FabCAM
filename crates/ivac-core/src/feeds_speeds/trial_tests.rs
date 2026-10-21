fn trial_fixture() -> FsCamRequest {
    let mut project = fixture();
    project.machine.spindle_rpm_min = Some(5000);
    project.machine.spindle_rpm_max = Some(24000);
    project.machine.max_feed_mm_min = Some(6000);
    project.fs_context.as_mut().unwrap().tools[0].max_rpm = Some(22000);
    project.tools[0].flute_length_mm = Some(20.0);
    let mut pack = fs::load_source_pack_json(include_str!(
        "../../../../data/feeds-speeds/packs/sienci-longmill.v1.json"
    ))
    .unwrap();
    pack.manifest.identity = fs::Identity {
        id: fs::LAB_PACK_ID.into(),
        revision: "0001791792000000".into(),
    };
    pack.manifest.schema_version = 2;
    pack.manifest.visibility = fs::Visibility::Private;
    pack.manifest.distribution = fs::Distribution::LocalOnly;
    pack.records.truncate(1);
    pack.dataset.profile_set.profiles.truncate(1);
    let p = &mut pack.dataset.profile_set.profiles[0];
    p.category = fs::ProfileCategory::Theoretical;
    p.evidence = fs::Evidence::Theoretical;
    p.source.notes = "PRIVATE-NOTES-SENTINEL /Users/private/trial".into();
    p.data = fs::CuttingData::ExactValues {
        rpm: 18000.0,
        feed_mm_min: 1200.0,
        plunge_mm_min: 300.0,
        doc_mm: 2.0,
        stepover_ratio: None,
    };
    p.scope.doc_mm = None;
    p.scope.operation = fs::OperationKind::FullSlot;
    pack.records[0].lab = Some(fs::LabPreset {
        name: "Synthetic Draft".into(),
        state: fs::LabState::Draft,
        created_at: "2026-10-21".into(),
        updated_at: "2026-10-21".into(),
        tested_at: None,
        physically_tested: false,
        no_critical_failure: false,
        operator: "PRIVATE-NOTES-SENTINEL".into(),
        job_reference: String::new(),
        observations: String::new(),
        detailed_notes: "PRIVATE-NOTES-SENTINEL".into(),
        finishing_context: String::new(),
        enabled: false,
        deleted: false,
        review: None,
    });
    let selected = fs::SelectedRecord {
        pack: pack.manifest.identity.clone(),
        record_id: pack.records[0].source_record_id.clone(),
        profile: p.identity.clone(),
    };
    fs::validate_source_pack(&pack).unwrap();
    FsCamRequest {
        project,
        op_id: 1,
        dataset_json: serde_json::to_string(&fs::PackCollection {
            source_packs: vec![pack],
        })
        .unwrap(),
        selected_record: Some(selected),
        aggressiveness: fs::Aggressiveness::Prudent,
        apply: true,
        experimental_trial: true,
        trial_acknowledged: true,
    }
}
fn change_trial_pack(r: &mut FsCamRequest, f: impl FnOnce(&mut fs::SourcePack)) {
    let mut collection: fs::PackCollection = serde_json::from_str(&r.dataset_json).unwrap();
    f(&mut collection.source_packs[0]);
    r.dataset_json = serde_json::to_string(&collection).unwrap();
}
#[test]
fn trial_exact_private_portable_and_no_source_dependency() {
    let r = trial_fixture();
    let mut project = r.project.clone();
    let result = feeds_speeds(r).unwrap();
    project.operations[0] = result.applied_op.unwrap();
    let wire = serde_json::to_string(&project).unwrap();
    assert!(!wire.contains("PRIVATE-NOTES-SENTINEL"));
    assert!(!wire.contains("/Users/private"));
    let restored: Project = serde_json::from_str(&wire).unwrap();
    validate_applied_project(&restored).unwrap();
    assert_eq!(
        saved_rates(
            &restored.operations[0],
            &restored.tools[0],
            PassKind::Finish
        )
        .unwrap(),
        (18000, 300, 1200)
    );
    assert_eq!(restored.operations[0].params.step, Some(-2.0));
    assert_eq!(
        serde_json::to_value(restored.machine).unwrap(),
        serde_json::to_value(project.machine).unwrap()
    );
}
#[test]
fn trial_does_not_enable_suggestion_and_requires_acknowledgment() {
    let mut r = trial_fixture();
    r.experimental_trial = false;
    assert!(feeds_speeds(r).is_err());
    let mut r = trial_fixture();
    r.trial_acknowledged = false;
    assert!(feeds_speeds(r.clone()).unwrap_err().contains("Acknowledge"));
    r.apply = false;
    let preview = feeds_speeds(r).unwrap();
    assert!(preview.applied_op.is_none());
    assert!(preview.trial.is_some());
}
#[test]
fn trial_refuses_fractional_rates_and_scaling() {
    for rate in [0.0, -1.0, 1200.5, f64::from(u32::MAX) + 1.0, 7000.0] {
        let mut r = trial_fixture();
        change_trial_pack(&mut r, |p| {
            if let fs::CuttingData::ExactValues { feed_mm_min, .. } =
                &mut p.dataset.profile_set.profiles[0].data
            {
                *feed_mm_min = rate;
            }
        });
        assert!(feeds_speeds(r).is_err(), "rate {rate}");
    }
    let mut r = trial_fixture();
    change_trial_pack(&mut r, |p| {
        p.dataset.profile_set.profiles[0].data = fs::CuttingData::ChiploadTarget {
            rpm: 18000.0,
            chipload_mm_tooth: 0.1,
            plunge_mm_min: 300.0,
            doc_mm: 2.0,
            stepover_ratio: None,
        }
    });
    assert!(feeds_speeds(r).unwrap_err().contains("explicit feed"));
}
#[test]
fn trial_refuses_missing_and_exceeded_limits() {
    for case in 0..8 {
        let mut r = trial_fixture();
        match case {
            0 => r.project.machine.spindle_rpm_min = None,
            1 => r.project.machine.spindle_rpm_max = None,
            2 => r.project.machine.max_feed_mm_min = None,
            3 => r.project.fs_context.as_mut().unwrap().tools[0].max_rpm = None,
            4 => r.project.tools[0].flute_length_mm = None,
            5 => r.project.machine.spindle_rpm_min = Some(19000),
            6 => r.project.fs_context.as_mut().unwrap().tools[0].max_rpm = Some(17000),
            _ => r.project.tools[0].flute_length_mm = Some(1.0),
        }
        assert!(feeds_speeds(r).is_err(), "case {case}");
    }
}
#[test]
fn trial_refuses_deleted_stale_and_missing_source() {
    for case in 0..4 {
        let mut r = trial_fixture();
        match case {
            0 => change_trial_pack(&mut r, |p| {
                p.records[0].lab.as_mut().unwrap().deleted = true
            }),
            1 => r.selected_record.as_mut().unwrap().pack.revision.push('1'),
            2 => r
                .selected_record
                .as_mut()
                .unwrap()
                .profile
                .revision
                .push('1'),
            _ => r.dataset_json = "{\"source_packs\":[]}".into(),
        }
        assert!(feeds_speeds(r).is_err());
    }
}
#[test]
fn trial_generation_revalidates_manual_edits_context_and_metadata() {
    let r = trial_fixture();
    let mut p = r.project.clone();
    p.operations[0] = feeds_speeds(r).unwrap().applied_op.unwrap();
    let baseline = p.operations[0]
        .params
        .applied_cutting
        .clone()
        .unwrap()
        .experimental_recipe
        .unwrap();
    p.operations[0]
        .params
        .applied_cutting
        .as_mut()
        .unwrap()
        .slots[0]
        .feed_mm_min = 1000;
    validate_applied_project(&p).unwrap();
    assert_eq!(
        p.operations[0]
            .params
            .applied_cutting
            .as_ref()
            .unwrap()
            .experimental_recipe
            .as_ref()
            .unwrap(),
        &baseline
    );
    for case in 0..5 {
        let mut bad = p.clone();
        match case {
            0 => {
                bad.operations[0]
                    .params
                    .applied_cutting
                    .as_mut()
                    .unwrap()
                    .slots[0]
                    .rpm = 25000
            }
            1 => bad.material.as_mut().unwrap().revision.push('1'),
            2 => bad.operations[0].finish_tool_id = Some(99),
            3 => bad.operations[0].params.step = Some(-21.0),
            _ => {
                bad.operations[0]
                    .params
                    .applied_cutting
                    .as_mut()
                    .unwrap()
                    .experimental_recipe
                    .as_mut()
                    .unwrap()
                    .source
                    .record_id = "/private/source".into()
            }
        }
        assert!(validate_applied_project(&bad).is_err());
        assert!(run_pipeline(
            PipelineRequest {
                project: bad,
                post_processor: None,
                cps_post: None
            },
            |_, _, _| {}
        )
        .is_err());
    }
}
#[test]
fn trial_direct_json_refuses_private_extensions_and_invalid_discriminator() {
    let r = trial_fixture();
    let op = feeds_speeds(r).unwrap().applied_op.unwrap();
    let mut json = serde_json::to_value(&op).unwrap();
    json["params"]["applied_cutting"]["experimental_recipe"]["notes"] = "private".into();
    assert!(serde_json::from_value::<Op>(json).is_err());
}
#[test]
fn trial_pocket_snapshots_both_roles_and_refuses_unknown_stepover() {
    let mut r = trial_fixture();
    r.project.operations[0].kind = OpKind::Pocket {
        strategy: crate::project::PocketStrategy::Spiral,
        contour: crate::project::ContourParams::default(),
        pocket: crate::project::PocketParams::default(),
    };
    change_trial_pack(&mut r, |p| {
        let profile = &mut p.dataset.profile_set.profiles[0];
        profile.scope.operation = fs::OperationKind::Pocket;
        profile.scope.engagement = fs::EngagementScope::Nominal {
            ratio: fs::Range { min: 0.4, max: 0.4 },
        };
        if let fs::CuttingData::ExactValues { stepover_ratio, .. } = &mut profile.data {
            *stepover_ratio = Some(0.4);
        }
    });
    let next = feeds_speeds(r.clone()).unwrap().applied_op.unwrap();
    assert_eq!(next.pocket_params().unwrap().xy_overlap, 0.6);
    assert!(next
        .params
        .applied_cutting
        .unwrap()
        .slots
        .iter()
        .all(|s| s.rpm == 18000 && s.feed_mm_min == 1200 && s.plunge_mm_min == 300));
    change_trial_pack(&mut r, |p| {
        if let fs::CuttingData::ExactValues { stepover_ratio, .. } =
            &mut p.dataset.profile_set.profiles[0].data
        {
            *stepover_ratio = None;
        }
    });
    assert!(feeds_speeds(r).is_err());
}
#[test]
fn trial_refuses_unsupported_tool_or_process_without_relaxing_shopbot() {
    for case in 0..3 {
        let mut r = trial_fixture();
        match case {
            0 => r.project.tools[0].kind = ToolKind::VBit,
            1 => r.project.machine.mode = MachineMode::Laser,
            _ => r.project.operations[0].finish_tool_id = Some(2),
        }
        assert!(feeds_speeds(r).is_err());
    }
}

#[test]
fn trial_refuses_duplicate_binding_before_application() {
    let mut request = trial_fixture();
    let c = request.project.fs_context.as_mut().unwrap();
    c.tools.push(c.tools[0].clone());
    assert!(feeds_speeds(request).unwrap_err().contains("Ambiguous"));
}
