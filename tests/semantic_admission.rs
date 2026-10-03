use codegate::{semantic::validate_snapshot, semantic_model::*, semantic_wire, source_identity};
use serde_json::json;

fn fixture() -> FactSnapshot {
    let mut snapshot=semantic_wire::decode_snapshot(&serde_json::to_vec(&json!({
        "format":"codegate.semantic-facts/1","mode":"SourceOnly",
        "source":{"id":"","selection":{"include_paths":[],"exclude_paths":[],"include_tests":true,"include_generated":true,"languages":[]},
        "configuration":{"id":"","modules":[],"go_build_tags":[],"rust_features":[],"rust_default_features":true,"rust_cfg":[],"build_profiles":[],"configuration_files":[]},
        "files":[{"path":"src/main.rs","content_sha256":"a".repeat(64),"byte_length":5,"language":"Rust","classification":"Production","origin":"Untracked","lines":[{"number":0,"start_byte":0,"end_byte":5,"classification":"Code"}]}],"manifests":[]},
        "tools":[],"coverage":[],"units":[],"declarations":[],"occurrences":[],"dependencies":[],"references":[],"calls":[],"implementations":[],"decisions":[],"structure":[],"effects":[],"framework":[]
    })).unwrap()).unwrap();
    refresh(&mut snapshot);
    let evidence = Evidence {
        id: FactId("e-unit".into()),
        snapshot_id: snapshot.source.id.clone(),
        configuration_id: snapshot.source.configuration.id.clone(),
        producer: "fixture".into(),
        producer_version: "1".into(),
        location: None,
        resolution: Resolution::Resolved,
        basis: ResolutionBasis::SourceStructure,
        gaps: vec![],
    };
    snapshot.units.push(Unit {
        id: UnitId("u".into()),
        name: "u".into(),
        language: Language::Rust,
        paths: vec!["src/main.rs".into()],
        evidence: evidence.clone(),
    });
    let mut declaration_evidence = evidence;
    declaration_evidence.id = FactId("e-declaration".into());
    declaration_evidence.location = Some(range());
    snapshot.declarations.push(Declaration {
        id: DeclarationId("d".into()),
        unit_id: UnitId("u".into()),
        parent: None,
        name: "main".into(),
        qualified_name: "u::main".into(),
        signature: None,
        kind: DeclarationKind::Function,
        visibility: Visibility::Private,
        api_eligibility: ApiEligibility::Ineligible,
        evidence: declaration_evidence,
    });
    for family in [
        FactFamily::Sources,
        FactFamily::Declarations,
        FactFamily::Occurrences,
        FactFamily::Dependencies,
        FactFamily::References,
        FactFamily::Implementations,
        FactFamily::Calls,
        FactFamily::Decisions,
        FactFamily::Structure,
        FactFamily::Documentation,
        FactFamily::Effects,
        FactFamily::Tests,
        FactFamily::FrameworkDeclarations,
        FactFamily::FrameworkWiring,
        FactFamily::ExecutionCoverage,
    ] {
        let supported = matches!(
            family,
            FactFamily::Sources | FactFamily::Declarations | FactFamily::Dependencies
        );
        snapshot.coverage.push(Coverage {
            family,
            configuration_id: snapshot.source.configuration.id.clone(),
            unit_ids: vec![UnitId("u".into())],
            status: if supported {
                Completeness::Complete
            } else {
                Completeness::Unsupported
            },
            gaps: if supported { vec![] } else { vec![gap()] },
            applicability_reason: None,
        });
    }
    snapshot
}
fn refresh(s: &mut FactSnapshot) {
    s.source.configuration.id = source_identity::configuration_id(&s.source.configuration).unwrap();
    s.source.id = source_identity::snapshot_id(&s.source).unwrap();
    for u in &mut s.units {
        u.evidence.snapshot_id = s.source.id.clone();
        u.evidence.configuration_id = s.source.configuration.id.clone();
    }
    for d in &mut s.declarations {
        d.evidence.snapshot_id = s.source.id.clone();
        d.evidence.configuration_id = s.source.configuration.id.clone();
    }
    for c in &mut s.coverage {
        c.configuration_id = s.source.configuration.id.clone();
    }
}
fn range() -> SourceRange {
    SourceRange {
        path: "src/main.rs".into(),
        start_byte: 0,
        end_byte: 5,
        start_line: 0,
        start_column: 0,
        end_line: 0,
        end_column: 5,
    }
}
fn gap() -> Gap {
    Gap {
        code: GapCode::UnsupportedCapability,
        reason: "not collected in this profile".into(),
        location: None,
    }
}
fn invalid(s: &FactSnapshot, code: GapCode) {
    let gaps = validate_snapshot(s).expect_err("invalid facts must refuse");
    assert!(gaps.iter().any(|g| g.code == code), "{gaps:?}");
}

#[test]
fn valid_complete_families_and_partial_evidence_are_admitted() {
    let mut s = fixture();
    validate_snapshot(&s).unwrap();
    s.coverage[3].status = Completeness::Partial;
    s.coverage[3].gaps = vec![gap()];
    validate_snapshot(&s).unwrap();
    s.units.clear();
    s.declarations.clear();
    for c in &mut s.coverage {
        c.unit_ids.clear();
    }
    validate_snapshot(&s).unwrap();
}
#[test]
fn stale_evidence_has_a_stable_typed_refusal() {
    let mut s = fixture();
    s.units[0].evidence.snapshot_id = SnapshotId("b".repeat(64));
    assert_eq!(
        validate_snapshot(&s),
        Err(vec![Gap {
            code: GapCode::StaleEvidence,
            reason: "stale evidence: e-unit".into(),
            location: None
        }])
    );
}
#[test]
fn missing_overlap_and_outside_coverage_never_mean_zero() {
    let mut s = fixture();
    s.coverage.pop();
    invalid(&s, GapCode::InvalidFact);
    let mut s = fixture();
    s.coverage.push(s.coverage[0].clone());
    invalid(&s, GapCode::InvalidFact);
    let mut s = fixture();
    s.coverage[0].unit_ids = vec![UnitId("unknown".into())];
    invalid(&s, GapCode::InvalidFact);
    let mut s = fixture();
    s.coverage[0].unit_ids.clear();
    invalid(&s, GapCode::InvalidFact);
    let mut s = fixture();
    s.coverage[4].status = Completeness::Complete;
    s.coverage[4].gaps.clear();
    invalid(&s, GapCode::InvalidFact);
}
#[test]
fn dangling_duplicate_and_cycle_references_refuse() {
    let mut s = fixture();
    s.declarations[0].unit_id = UnitId("missing".into());
    invalid(&s, GapCode::InvalidFact);
    let mut s = fixture();
    s.units.push(s.units[0].clone());
    invalid(&s, GapCode::InvalidFact);
    let mut s = fixture();
    s.declarations[0].parent = Some(s.declarations[0].id.clone());
    invalid(&s, GapCode::InvalidFact);
    let mut s = fixture();
    s.declarations[0].evidence.id = s.units[0].evidence.id.clone();
    invalid(&s, GapCode::InvalidFact);
}
#[test]
fn invalid_locations_and_line_partitions_refuse_without_overflow() {
    for start in [-1, i64::MAX] {
        let mut s = fixture();
        s.declarations[0]
            .evidence
            .location
            .as_mut()
            .unwrap()
            .start_byte = start;
        invalid(&s, GapCode::InvalidFact);
    }
    let mut s = fixture();
    s.declarations[0]
        .evidence
        .location
        .as_mut()
        .unwrap()
        .end_column = i64::MAX;
    invalid(&s, GapCode::InvalidFact);
    let mut s = fixture();
    s.source.files[0].lines[0].end_byte = 4;
    refresh(&mut s);
    invalid(&s, GapCode::InvalidFact);
    let mut s = fixture();
    s.declarations[0]
        .evidence
        .location
        .as_mut()
        .unwrap()
        .end_line = 1;
    s.declarations[0]
        .evidence
        .location
        .as_mut()
        .unwrap()
        .end_column = 0;
    validate_snapshot(&s).unwrap();
}
#[test]
fn identities_and_configuration_views_are_checked() {
    let mut s = fixture();
    s.source.id = SnapshotId("b".repeat(64));
    invalid(&s, GapCode::IdentityMismatch);
    let mut s = fixture();
    s.source.configuration.configuration_files = s.source.files.clone();
    refresh(&mut s);
    validate_snapshot(&s).unwrap();
    s.source.configuration.configuration_files[0].origin = SourceOrigin::TrackedClean;
    refresh(&mut s);
    invalid(&s, GapCode::InvalidFact);
}
#[test]
fn resolution_and_kind_payloads_cannot_fabricate_facts() {
    let mut s = fixture();
    let mut e = s.units[0].evidence.clone();
    e.id = FactId("e-call".into());
    e.resolution = Resolution::SyntacticCandidate;
    e.gaps = vec![gap()];
    s.calls.push(Call {
        caller: DeclarationId("d".into()),
        callee: Some(DeclarationId("d".into())),
        candidates: vec![],
        dynamic_dispatch: false,
        evidence: e,
    });
    invalid(&s, GapCode::InvalidFact);
    let mut s = fixture();
    let mut e = s.units[0].evidence.clone();
    e.id = FactId("e-structure".into());
    s.structure.push(StructureObservation {
        owner: DeclarationId("d".into()),
        kind: StructureKind::TestCase,
        parent_observation: None,
        test_kind: None,
        text: None,
        evidence: e,
    });
    invalid(&s, GapCode::InvalidFact);
}
#[test]
fn typed_resources_are_bounded() {
    let mut s = fixture();
    s.source.files[0].byte_length = i64::MAX;
    invalid(&s, GapCode::InvalidFact);
}

#[test]
fn failed_tools_and_out_of_selection_files_cannot_look_complete() {
    let mut s = fixture();
    s.mode = CollectionMode::Semantic;
    s.coverage[4].status = Completeness::Complete;
    s.coverage[4].gaps.clear();
    s.tools.push(ToolObservation {
        tool: "adapter".into(),
        version: "1".into(),
        host_runtime_version: None,
        configuration_id: s.source.configuration.id.clone(),
        snapshot_id: s.source.id.clone(),
        elapsed_milliseconds: 0,
        exit_code: Some(1),
        timed_out: false,
        diagnostics: vec!["failure".into()],
    });
    invalid(&s, GapCode::InvalidFact);
    let mut s = fixture();
    s.source.selection.exclude_paths.push("src".into());
    refresh(&mut s);
    invalid(&s, GapCode::InvalidFact);
    let mut s = fixture();
    s.source.selection.languages = vec![Language::Go];
    refresh(&mut s);
    invalid(&s, GapCode::InvalidFact);
}
#[test]
fn rich_relationship_families_and_framework_payloads_are_checked() {
    let mut s = fixture();
    let mut e = s.units[0].evidence.clone();
    e.id = FactId("e-occurrence".into());
    s.occurrences.push(Occurrence {
        owner: Some(DeclarationId("missing".into())),
        spelling: "x".into(),
        role: OccurrenceRole::Read,
        evidence: e.clone(),
    });
    assert_eq!(
        validate_snapshot(&s),
        Err(vec![Gap {
            code: GapCode::InvalidFact,
            reason: "unknown occurrence owner: missing".into(),
            location: None
        }])
    );
    s.occurrences[0].owner = Some(DeclarationId("d".into()));
    e.id = FactId("e-ref".into());
    s.references.push(Reference {
        occurrence_id: FactId("e-occurrence".into()),
        target: Some(DeclarationId("d".into())),
        candidates: vec![],
        evidence: e.clone(),
    });
    validate_snapshot(&s).unwrap();
    s.references[0].occurrence_id = FactId("e-unit".into());
    invalid(&s, GapCode::InvalidFact);
    let mut s = fixture();
    e.snapshot_id = s.source.id.clone();
    e.configuration_id = s.source.configuration.id.clone();
    e.id = FactId("framework".into());
    s.framework.push(FrameworkFact {
        kind: FrameworkKind::InjectionBinding,
        declaration: DeclarationId("d".into()),
        target: Some(DeclarationId("d".into())),
        annotation_name: "Inject".into(),
        qualifiers: vec![],
        scope: None,
        produced_type: None,
        http_method: None,
        resource_path: None,
        method_path: None,
        transaction_mode: None,
        configuration_key: None,
        configuration_profile: None,
        evidence: e,
    });
    invalid(&s, GapCode::InvalidFact);
    s.framework[0].evidence.basis = ResolutionBasis::MatchedBuildEvidence;
    validate_snapshot(&s).unwrap();
}
#[test]
fn all_family_statuses_have_honest_coverage_semantics() {
    let mut s = fixture();
    s.coverage[3].status = Completeness::Partial;
    invalid(&s, GapCode::InvalidFact);
    s.coverage[3].gaps = vec![gap()];
    validate_snapshot(&s).unwrap();
    s.coverage[3].status = Completeness::NotApplicable;
    s.coverage[3].gaps.clear();
    invalid(&s, GapCode::InvalidFact);
    s.coverage[3].applicability_reason = Some("no dependencies apply".into());
    validate_snapshot(&s).unwrap();
    s.coverage[3].gaps = vec![Gap {
        code: GapCode::MissingTool,
        reason: "tool absent".into(),
        location: None,
    }];
    invalid(&s, GapCode::InvalidFact);
    let mut s = fixture();
    s.declarations.clear();
    s.units.clear();
    for c in &mut s.coverage {
        c.unit_ids.clear();
    }
    s.coverage.pop();
    assert_eq!(
        validate_snapshot(&s),
        Err(vec![Gap {
            code: GapCode::InvalidFact,
            reason: "missing coverage: ExecutionCoverage:<empty>".into(),
            location: None
        }])
    );
}
