use codegate::{
    Collector,
    bindings::{BindingFacts, BindingRegistry, SourceBinding, source_evidence},
    collect,
    collection::CollectedSource,
    semantic::validate_snapshot,
    semantic_model::*,
};
use std::{
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};
static SERIAL: AtomicU64 = AtomicU64::new(0);
fn request() -> CollectionRequest {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(".scratch/adversary")
        .join(format!(
            "fixture-{}-{}",
            std::process::id(),
            SERIAL.fetch_add(1, Ordering::Relaxed)
        ));
    std::fs::create_dir_all(&root).unwrap();
    std::fs::write(root.join("main.rs"), "fn main() {}\n").unwrap();
    std::fs::write(root.join("main.go"), "package main\n").unwrap();
    codegate::semantic_wire::decode_request(&serde_json::to_vec(&serde_json::json!({
        "root":root.to_str().unwrap(), "selection":{"include_paths":[], "exclude_paths":[], "include_tests":true,"include_generated":true,"languages":[]},
        "configuration":{"id":"","modules":[],"go_build_tags":[],"rust_features":[],"rust_default_features":false,"rust_cfg":[],"build_profiles":[],"configuration_files":[]},
        "mode":"SourceOnly","timeout_milliseconds":10000
    })).unwrap()).unwrap()
}
struct Slot {
    language: Language,
    stale: bool,
}
impl SourceBinding for Slot {
    fn language(&self) -> Language {
        self.language
    }
    fn collect(&self, source: &CollectedSource) -> BindingFacts {
        let (name, path) = if self.language == Language::Rust {
            ("rust-unit", "main.rs")
        } else {
            ("go-unit", "main.go")
        };
        let mut evidence = source_evidence(
            &source.snapshot,
            name,
            "1",
            FactId(format!("{name}-evidence")),
            None,
        );
        if self.stale {
            evidence.configuration_id = ConfigurationId("stale".into());
        }
        BindingFacts {
            units: vec![Unit {
                id: UnitId(name.into()),
                name: name.into(),
                language: self.language,
                paths: vec![path.into()],
                evidence,
            }],
            coverage: vec![Coverage {
                family: FactFamily::Declarations,
                configuration_id: source.snapshot.configuration.id.clone(),
                unit_ids: vec![UnitId(name.into())],
                status: Completeness::Complete,
                gaps: vec![],
                applicability_reason: None,
            }],
            ..Default::default()
        }
    }
}
#[test]
fn missing_slot_cannot_leave_observed_slot_declarations_complete() {
    let mut registry = BindingRegistry::default();
    registry
        .register(Slot {
            language: Language::Rust,
            stale: false,
        })
        .unwrap();
    let result = Collector::with_bindings(registry)
        .collect_response(Collect { request: request() })
        .unwrap();
    validate_snapshot(&result.snapshot).unwrap();
    let coverage = result
        .snapshot
        .coverage
        .iter()
        .find(|c| c.family == FactFamily::Declarations)
        .unwrap();
    assert_eq!(coverage.status, Completeness::Partial);
    assert_eq!(coverage.unit_ids, [UnitId("rust-unit".into())]);
    assert!(coverage.gaps.iter().any(|g| g.code == GapCode::UnknownUnit));
    assert_eq!(result.snapshot.source.files.len(), 2);
}
#[test]
fn registered_slots_are_deterministic_and_stale_configuration_is_rejected() {
    let req = request();
    let run = |reverse, stale| {
        let mut registry = BindingRegistry::default();
        let languages = if reverse {
            [Language::Rust, Language::Go]
        } else {
            [Language::Go, Language::Rust]
        };
        for language in languages {
            registry.register(Slot { language, stale }).unwrap();
        }
        Collector::with_bindings(registry).collect_response(Collect {
            request: req.clone(),
        })
    };
    let first = run(false, false).unwrap();
    assert_eq!(first, run(true, false).unwrap());
    assert_eq!(
        first
            .snapshot
            .units
            .iter()
            .map(|u| u.id.0.as_str())
            .collect::<Vec<_>>(),
        ["go-unit", "rust-unit"]
    );
    assert_eq!(
        first
            .snapshot
            .coverage
            .iter()
            .filter(|c| c.family == FactFamily::Declarations && c.status == Completeness::Complete)
            .count(),
        2
    );
    assert!(
        run(false, true)
            .unwrap_err()
            .iter()
            .any(|g| g.code == GapCode::StaleEvidence)
    );
}
#[test]
fn malformed_requests_are_refused_and_valid_missing_source_retains_failed_coverage() {
    let req = request();
    let mut invalid = req.clone();
    invalid.selection.languages = vec![Language::Rust, Language::Rust];
    assert!(
        collect(Collect { request: invalid })
            .unwrap_err()
            .iter()
            .any(|g| g.code == GapCode::InvalidFact)
    );
    let mut invalid = req.clone();
    invalid.configuration.modules = vec!["../escape".into()];
    assert!(
        collect(Collect { request: invalid })
            .unwrap_err()
            .iter()
            .any(|g| g.code == GapCode::InvalidFact)
    );
    let mut invalid = req.clone();
    invalid.selection.include_paths = vec![".git/config".into()];
    assert!(
        collect(Collect { request: invalid })
            .unwrap_err()
            .iter()
            .any(|g| g.code == GapCode::InvalidFact)
    );
    let mut missing = req;
    missing.mode = CollectionMode::Semantic;
    missing.selection.include_paths = vec!["absent.rs".into()];
    let snapshot = collect(Collect { request: missing }).unwrap().snapshot;
    validate_snapshot(&snapshot).unwrap();
    assert_eq!(
        snapshot.source.id.0,
        "d7471773866a9b2d65096de138b604ba8397831746bdc3cc9879512a693bc405"
    );
    assert_eq!(snapshot.mode, CollectionMode::Semantic);
    assert_eq!(
        snapshot
            .coverage
            .iter()
            .find(|c| c.family == FactFamily::Sources)
            .unwrap()
            .status,
        Completeness::Failed
    );
    assert_eq!(
        snapshot
            .coverage
            .iter()
            .filter(|c| c.status == Completeness::Unsupported)
            .count(),
        14
    );
}
#[test]
fn syntax_failure_retains_real_identity_and_partial_sources() {
    let req = request();
    std::fs::write(PathBuf::from(&req.root).join("main.rs"), "fn broken( {\n").unwrap();
    let response = collect(Collect {
        request: req.clone(),
    })
    .unwrap();
    assert_eq!(response.snapshot.source.files.len(), 2);
    let coverage = response
        .snapshot
        .coverage
        .iter()
        .find(|c| c.family == FactFamily::Sources)
        .unwrap();
    assert_eq!(coverage.status, Completeness::Partial);
    assert!(!coverage.gaps.is_empty());
    std::fs::write(
        PathBuf::from(&req.root).join("main.rs"),
        "fn repaired() {}\n",
    )
    .unwrap();
    let repaired = collect(Collect { request: req }).unwrap();
    assert_ne!(response.snapshot.source.id, repaired.snapshot.source.id);
    assert_eq!(
        repaired
            .snapshot
            .coverage
            .iter()
            .find(|c| c.family == FactFamily::Sources)
            .unwrap()
            .status,
        Completeness::Complete
    );
}
