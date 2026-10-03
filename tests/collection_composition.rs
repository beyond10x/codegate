use codegate::semantic_model::obligations::CollectBehavior;
use codegate::{
    Collector, collect, semantic::validate_snapshot, semantic_model::*, source_identity,
};
use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};
static SERIAL: AtomicU64 = AtomicU64::new(0);
fn request() -> CollectionRequest {
    let root: PathBuf = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join(format!(
            "composition-{}-{}",
            std::process::id(),
            SERIAL.fetch_add(1, Ordering::Relaxed)
        ));
    fs::create_dir(&root).unwrap();
    for (name, text) in [
        ("lib.rs", "pub fn value() {}\n"),
        ("main.go", "package main\nfunc value() {}\n"),
        ("Value.java", "class Value {}\n"),
        ("Cargo.toml", "[package]\nname='fixture'\nversion='0.1.0'\n"),
    ] {
        fs::write(root.join(name), text).unwrap();
    }
    CollectionRequest {
        root: root.to_str().unwrap().into(),
        selection: SourceSelection {
            include_paths: vec![],
            exclude_paths: vec![],
            include_tests: true,
            include_generated: true,
            languages: vec![],
        },
        configuration: BuildSelection {
            id: ConfigurationId(String::new()),
            modules: vec![],
            target: None,
            go_build_tags: vec![],
            rust_features: vec![],
            rust_default_features: true,
            rust_cfg: vec![],
            java_target_version: None,
            java_classpath_sha256: None,
            build_profiles: vec![],
            configuration_files: vec![],
        },
        mode: CollectionMode::SourceOnly,
        timeout_milliseconds: 1000,
    }
}
fn assert_empty_facts(snapshot: &FactSnapshot) {
    assert!(snapshot.units.is_empty());
    assert!(snapshot.declarations.is_empty());
    assert!(snapshot.occurrences.is_empty());
    assert!(snapshot.dependencies.is_empty());
    assert!(snapshot.references.is_empty());
    assert!(snapshot.calls.is_empty());
    assert!(snapshot.implementations.is_empty());
    assert!(snapshot.decisions.is_empty());
    assert!(snapshot.structure.is_empty());
    assert!(snapshot.effects.is_empty());
    assert!(snapshot.framework.is_empty());
    assert!(snapshot.tools.is_empty());
    assert_eq!(snapshot.coverage.len(), 15);
    for coverage in &snapshot.coverage {
        assert!(coverage.unit_ids.is_empty());
        assert_eq!(coverage.configuration_id, snapshot.source.configuration.id);
        if coverage.family != FactFamily::Sources {
            assert_eq!(coverage.status, Completeness::Unsupported);
            assert!(
                coverage
                    .gaps
                    .iter()
                    .any(|g| g.code == GapCode::UnsupportedCapability)
            );
            assert!(coverage.gaps.iter().any(|g| g.code == GapCode::UnknownUnit));
        }
    }
    assert_eq!(
        snapshot.source.id,
        source_identity::snapshot_id(&snapshot.source).unwrap()
    );
    assert_eq!(
        snapshot.source.configuration.id,
        source_identity::configuration_id(&snapshot.source.configuration).unwrap()
    );
    validate_snapshot(snapshot).unwrap();
}
#[test]
fn source_only_composes_real_capture_without_inventing_extraction() {
    let req = request();
    let response = collect(Collect {
        request: req.clone(),
    })
    .unwrap();
    let snapshot = &response.snapshot;
    assert_empty_facts(snapshot);
    assert_eq!(snapshot.source.files.len(), 3);
    assert_eq!(snapshot.source.configuration.configuration_files.len(), 1);
    assert_eq!(snapshot.source.manifests.len(), 1);
    let sources = snapshot
        .coverage
        .iter()
        .find(|c| c.family == FactFamily::Sources)
        .unwrap();
    assert_eq!(sources.status, Completeness::Complete);
    assert!(sources.gaps.is_empty());
    assert_eq!(
        snapshot,
        &collect(Collect {
            request: req.clone()
        })
        .unwrap()
        .snapshot
    );
    fs::write(
        PathBuf::from(&req.root).join("lib.rs"),
        "pub fn changed() {}\n",
    )
    .unwrap();
    let changed = collect(Collect { request: req }).unwrap().snapshot;
    assert_ne!(snapshot.source.id, changed.source.id);
    assert_eq!(
        snapshot.source.configuration.id,
        changed.source.configuration.id
    );
}
#[test]
fn semantic_request_preserves_mode_and_reports_unimplemented_adapters() {
    let mut req = request();
    req.mode = CollectionMode::Semantic;
    let snapshot = collect(Collect { request: req }).unwrap().snapshot;
    assert_empty_facts(&snapshot);
    assert_eq!(snapshot.mode, CollectionMode::Semantic);
    assert_eq!(snapshot.source.files.len(), 3);
    for c in &snapshot.coverage {
        if c.family != FactFamily::Sources {
            assert!(
                c.gaps
                    .iter()
                    .any(|g| g.reason.contains("semantic adapters are not implemented"))
            );
        }
    }
    assert!(
        !snapshot
            .coverage
            .iter()
            .flat_map(|c| &c.gaps)
            .any(|g| g.code == GapCode::MissingTool)
    );
}
#[test]
fn failed_capture_returns_only_observed_empty_content_and_actual_gaps() {
    let mut req = request();
    req.mode = CollectionMode::Semantic;
    req.selection.include_paths = vec!["missing.rs".into()];
    // An asserted descriptor is not evidence that those bytes were observed.
    req.configuration.configuration_files.push(SourceFile {
        path: "Cargo.toml".into(),
        content_sha256: "a".repeat(64),
        byte_length: 1,
        language: None,
        classification: SourceClass::BuildConfiguration,
        origin: SourceOrigin::Untracked,
        lines: vec![SourceLine {
            number: 0,
            start_byte: 0,
            end_byte: 1,
            classification: LineClass::Code,
        }],
    });
    let snapshot = collect(Collect { request: req }).unwrap().snapshot;
    assert_empty_facts(&snapshot);
    assert!(snapshot.source.files.is_empty());
    assert!(snapshot.source.manifests.is_empty());
    assert!(snapshot.source.configuration.configuration_files.is_empty());
    assert_eq!(snapshot.mode, CollectionMode::Semantic);
    assert_eq!(snapshot.source.selection.include_paths, ["missing.rs"]);
    let sources = snapshot
        .coverage
        .iter()
        .find(|c| c.family == FactFamily::Sources)
        .unwrap();
    assert_eq!(sources.status, Completeness::Failed);
    assert!(
        sources
            .gaps
            .iter()
            .any(|g| g.code == GapCode::MissingBuildSelection && g.reason.contains("missing.rs"))
    );
}
#[test]
fn invalid_request_is_an_invocation_error_and_clears_retained_response() {
    let req = request();
    let mut collector = Collector::default();
    assert_eq!(
        collector
            .collect(Collect {
                request: req.clone()
            })
            .unwrap(),
        CollectOutcome::Collected
    );
    let mut invalid = req.clone();
    invalid.selection.include_paths = vec!["../escape.rs".into()];
    assert!(collector.collect(Collect { request: invalid }).is_err());
    assert!(collector.take_response().is_none());
    let mut invalid = req.clone();
    invalid.configuration.java_classpath_sha256 = Some("not-a-hash".into());
    assert!(collect(Collect { request: invalid }).is_err());
    let mut invalid = req;
    invalid.timeout_milliseconds = 0;
    assert!(collect(Collect { request: invalid }).is_err());
}

#[test]
fn partial_capture_retains_every_gap_and_cannot_claim_complete_sources() {
    let req = request();
    fs::write(PathBuf::from(&req.root).join("lib.rs"), "fn broken( {\n").unwrap();
    let captured = codegate::collection::collect_source(&req).unwrap();
    assert!(!captured.gaps.is_empty());
    let snapshot = collect(Collect { request: req }).unwrap().snapshot;
    assert_empty_facts(&snapshot);
    let sources = snapshot
        .coverage
        .iter()
        .find(|c| c.family == FactFamily::Sources)
        .unwrap();
    assert_eq!(sources.status, Completeness::Partial);
    assert_eq!(sources.gaps, captured.gaps);
}

struct RustSlot {
    stale: bool,
}
impl codegate::bindings::SourceBinding for RustSlot {
    fn language(&self) -> Language {
        Language::Rust
    }
    fn collect(
        &self,
        captured: &codegate::collection::CollectedSource,
    ) -> codegate::bindings::BindingFacts {
        use codegate::bindings::*;
        assert_eq!(captured.contents["lib.rs"], "pub fn value() {}\n");
        let mut evidence = source_evidence(
            &captured.snapshot,
            "test-rust-slot",
            "1",
            FactId(stable_id("unit", &["lib.rs"])),
            Some(source_range("lib.rs", &captured.contents["lib.rs"], 0, 17).unwrap()),
        );
        if self.stale {
            evidence.snapshot_id = SnapshotId("stale".into());
        }
        BindingFacts {
            units: vec![Unit {
                id: UnitId("rust-unit".into()),
                name: "rust-unit".into(),
                language: Language::Rust,
                paths: vec!["lib.rs".into()],
                evidence,
            }],
            ..Default::default()
        }
    }
}
#[test]
fn registration_consumes_retained_bytes_and_never_infers_unsupported_family_completion() {
    use codegate::bindings::BindingRegistry;
    let req = request();
    let mut registry = BindingRegistry::default();
    registry.register(RustSlot { stale: false }).unwrap();
    assert!(registry.register(RustSlot { stale: false }).is_err());
    let mut collector = Collector::with_bindings(registry);
    let response = collector
        .collect_response(Collect { request: req })
        .unwrap();
    validate_snapshot(&response.snapshot).unwrap();
    assert_eq!(response.snapshot.units.len(), 1);
    assert_eq!(collector.take_response(), Some(response.clone()));
    assert!(collector.take_response().is_none());
    for coverage in &response.snapshot.coverage {
        assert_eq!(coverage.unit_ids, [UnitId("rust-unit".into())]);
        if coverage.family != FactFamily::Sources {
            assert_eq!(coverage.status, Completeness::Unsupported);
            // Go and Java units remain undiscovered, despite the registered Rust slot.
            assert!(coverage.gaps.iter().any(|g| g.code == GapCode::UnknownUnit));
        }
    }
}
#[test]
fn fabricated_binding_identity_is_refused_before_returning_a_response() {
    let mut registry = codegate::bindings::BindingRegistry::default();
    registry.register(RustSlot { stale: true }).unwrap();
    let mut collector = Collector::with_bindings(registry);
    let gaps = collector
        .collect_response(Collect { request: request() })
        .unwrap_err();
    assert!(gaps.iter().any(|g| g.code == GapCode::StaleEvidence));
    assert!(collector.take_response().is_none());
}
#[test]
fn semantic_mode_does_not_invoke_source_registration_as_an_adapter() {
    let mut registry = codegate::bindings::BindingRegistry::default();
    registry.register(RustSlot { stale: true }).unwrap();
    let mut collector = Collector::with_bindings(registry);
    let mut req = request();
    req.mode = CollectionMode::Semantic;
    let response = collector
        .collect_response(Collect { request: req })
        .unwrap();
    assert_empty_facts(&response.snapshot);
}
#[test]
fn source_helpers_keep_byte_coordinates_and_unambiguous_identity_parts() {
    use codegate::bindings::{source_range, stable_id};
    let text = "é\r\nfn α() {}\n";
    let range = source_range("lib.rs", text, 7, 9).unwrap();
    assert_eq!(
        (
            range.start_line,
            range.start_column,
            range.end_line,
            range.end_column
        ),
        (1, 3, 1, 5)
    );
    let eof = source_range("lib.rs", text, text.len(), text.len()).unwrap();
    assert_eq!((eof.start_line, eof.start_column), (2, 0));
    assert!(source_range("lib.rs", text, 1, 2).is_err());
    assert!(source_range("lib.rs", text, 7, 8).is_err());
    assert!(source_range("lib.rs", text, 2, 1).is_err());
    assert!(source_range("lib.rs", text, 0, text.len() + 1).is_err());
    assert!(source_range("../lib.rs", text, 0, 0).is_err());
    assert_ne!(
        stable_id("unit", &["ab", "c"]),
        stable_id("unit", &["a", "bc"])
    );
    assert_ne!(
        stable_id("unit", &["ab"]),
        stable_id("declaration", &["ab"])
    );
    assert_ne!(stable_id("unit", &["ab"]), stable_id("unit", &["ab", ""]));
    assert_eq!(stable_id("unit", &["ab"]), stable_id("unit", &["ab"]));
}

struct GoSlot;
impl codegate::bindings::SourceBinding for GoSlot {
    fn language(&self) -> Language {
        Language::Go
    }
    fn collect(
        &self,
        source: &codegate::collection::CollectedSource,
    ) -> codegate::bindings::BindingFacts {
        let evidence = codegate::bindings::source_evidence(
            &source.snapshot,
            "go-slot",
            "1",
            FactId("go-unit-evidence".into()),
            None,
        );
        codegate::bindings::BindingFacts {
            units: vec![Unit {
                id: UnitId("go-unit".into()),
                name: "go-unit".into(),
                language: Language::Go,
                paths: vec!["main.go".into()],
                evidence,
            }],
            ..Default::default()
        }
    }
}
struct ForeignClaim(u8);
impl codegate::bindings::SourceBinding for ForeignClaim {
    fn language(&self) -> Language {
        Language::Rust
    }
    fn collect(
        &self,
        source: &codegate::collection::CollectedSource,
    ) -> codegate::bindings::BindingFacts {
        use codegate::bindings::{source_evidence, source_range};
        let mut facts = RustSlot { stale: false }.collect(source);
        match self.0 {
            0 => facts.coverage.push(Coverage {
                family: FactFamily::Declarations,
                configuration_id: source.snapshot.configuration.id.clone(),
                unit_ids: vec![UnitId("go-unit".into())],
                status: Completeness::Complete,
                gaps: vec![],
                applicability_reason: None,
            }),
            1 => facts.declarations.push(Declaration {
                id: DeclarationId("foreign-declaration".into()),
                unit_id: UnitId("go-unit".into()),
                parent: None,
                name: "foreign".into(),
                qualified_name: "foreign".into(),
                signature: None,
                kind: DeclarationKind::Function,
                visibility: Visibility::Private,
                api_eligibility: ApiEligibility::Ineligible,
                evidence: source_evidence(
                    &source.snapshot,
                    "rust-slot",
                    "1",
                    FactId("foreign-evidence".into()),
                    None,
                ),
            }),
            _ => {
                facts.units[0].evidence.location =
                    Some(source_range("main.go", &source.contents["main.go"], 0, 12).unwrap())
            }
        }
        facts
    }
}
#[test]
fn source_slots_cannot_certify_another_languages_units_or_evidence() {
    for claim in 0..3 {
        let mut registry = codegate::bindings::BindingRegistry::default();
        registry.register(GoSlot).unwrap();
        registry.register(ForeignClaim(claim)).unwrap();
        let mut collector = Collector::with_bindings(registry);
        let result = collector.collect_response(Collect { request: request() });
        assert!(result.is_err(), "cross-slot claim {claim} was admitted");
        assert!(collector.take_response().is_none());
    }
}
