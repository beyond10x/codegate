use codegate::{
    semantic_model::*,
    source_identity::{configuration_id, snapshot_id},
};
use sha2::{Digest, Sha256};

fn configuration() -> BuildSelection {
    BuildSelection {
        id: ConfigurationId("asserted-id-is-excluded".into()),
        modules: vec!["z".into(), "a".into()],
        target: None,
        go_build_tags: vec!["linux".into(), "feature".into()],
        rust_features: vec![],
        rust_default_features: true,
        rust_cfg: vec![],
        java_target_version: None,
        java_classpath_sha256: None,
        build_profiles: vec![],
        configuration_files: vec![],
    }
}
fn file(path: &str) -> SourceFile {
    SourceFile {
        path: path.into(),
        content_sha256: "a".repeat(64),
        byte_length: 2,
        language: Some(Language::Rust),
        classification: SourceClass::Production,
        origin: SourceOrigin::TrackedClean,
        lines: vec![SourceLine {
            number: 0,
            start_byte: 0,
            end_byte: 2,
            classification: LineClass::Code,
        }],
    }
}
fn source() -> SourceSnapshot {
    let mut configuration = configuration();
    configuration.configuration_files = vec![file("Cargo.toml")];
    configuration.id = configuration_id(&configuration).unwrap();
    SourceSnapshot {
        id: SnapshotId("own-id-is-excluded".into()),
        selection: SourceSelection {
            include_paths: vec![],
            exclude_paths: vec![],
            include_tests: true,
            include_generated: false,
            languages: vec![Language::Rust, Language::Go],
        },
        configuration,
        files: vec![file("z.rs"), file("a.rs")],
        manifests: vec![Manifest {
            path: "Cargo.toml".into(),
            content_sha256: "b".repeat(64),
            system: BuildSystem::Cargo,
            module: ".".into(),
            parent_module: None,
        }],
    }
}
#[test]
fn explicit_canonical_wire_bytes_have_known_hash() {
    // Independent literal fixes optional omission, sorted keys/sets and integer-free configuration shape.
    let expected = br#"{"build_profiles":[],"configuration_files":[],"go_build_tags":["feature","linux"],"modules":["a","z"],"rust_cfg":[],"rust_default_features":true,"rust_features":[]}"#;
    let hash: String = Sha256::digest(expected)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    assert_eq!(configuration_id(&configuration()).unwrap().0, hash);
}
#[test]
fn reordering_and_origin_never_change_identity() {
    let before = source();
    let mut after = before.clone();
    after.id.0 = "another-assertion".into();
    after.files.reverse();
    after.selection.languages.reverse();
    after.configuration.modules.reverse();
    after.configuration.go_build_tags.reverse();
    after.files[0].origin = SourceOrigin::Untracked;
    after.configuration.configuration_files[0].origin = SourceOrigin::TrackedDirty;
    assert_eq!(
        configuration_id(&before.configuration).unwrap(),
        configuration_id(&after.configuration).unwrap()
    );
    assert_eq!(snapshot_id(&before).unwrap(), snapshot_id(&after).unwrap());
}
#[test]
fn selected_bytes_and_configuration_change_identity() {
    let before = source();
    let mut after = before.clone();
    after.files[0].content_sha256 = "c".repeat(64);
    assert_ne!(snapshot_id(&before).unwrap(), snapshot_id(&after).unwrap());
    after = before.clone();
    after.configuration.configuration_files[0].content_sha256 = "d".repeat(64);
    after.configuration.id = configuration_id(&after.configuration).unwrap();
    assert_ne!(before.configuration.id, after.configuration.id);
    assert_ne!(snapshot_id(&before).unwrap(), snapshot_id(&after).unwrap());
}

#[test]
fn snapshot_projection_uses_contract_enums_and_omits_absent_optionals() {
    let mut value = source();
    value.configuration = configuration();
    value.configuration.id = configuration_id(&value.configuration).unwrap();
    value.files = vec![file("東京.rs")];
    let expected = serde_json::json!({
        "configuration": {
            "id": value.configuration.id.0,
            "build_profiles": [], "configuration_files": [], "go_build_tags": ["feature", "linux"],
            "modules": ["a", "z"], "rust_cfg": [], "rust_default_features": true, "rust_features": []
        },
        "files": [{"path": "東京.rs", "content_sha256": "a".repeat(64), "byte_length": 2,
            "classification": "Production", "language": "Rust",
            "lines": [{"number": 0, "start_byte": 0, "end_byte": 2, "classification": "Code"}]}],
        "manifests": [{"path": "Cargo.toml", "content_sha256": "b".repeat(64), "system": "Cargo", "module": "."}],
        "selection": {"include_paths": [], "exclude_paths": [], "include_tests": true,
            "include_generated": false, "languages": ["Go", "Rust"]}
    });
    let expected_bytes = serde_json::to_vec(&expected).unwrap();
    let expected_hash: String = Sha256::digest(expected_bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect();
    assert_eq!(snapshot_id(&value).unwrap().0, expected_hash);
    value.configuration.target = Some(String::new());
    assert_ne!(
        configuration_id(&value.configuration).unwrap(),
        value.configuration.id
    );
}
#[test]
fn duplicates_and_non_normalized_paths_are_refused() {
    let mut value = configuration();
    value.modules.push("z".into());
    assert_eq!(
        configuration_id(&value).unwrap_err().code,
        GapCode::InvalidFact
    );
    for invalid in [
        ".",
        "../outside",
        "/absolute",
        "src//a",
        "src/./a",
        "src\\a",
        "C:/a",
        "src/",
    ] {
        let mut value = source();
        value.selection.include_paths = vec![invalid.into()];
        assert_eq!(
            snapshot_id(&value).unwrap_err().code,
            GapCode::InvalidFact,
            "{invalid}"
        );
    }
    let mut value = source();
    value.files.push(value.files[0].clone());
    assert_eq!(snapshot_id(&value).unwrap_err().code, GapCode::InvalidFact);
    let mut value = source();
    value.selection.languages.push(Language::Rust);
    assert_eq!(snapshot_id(&value).unwrap_err().code, GapCode::InvalidFact);
}

#[test]
fn root_module_identifiers_are_distinct_from_selected_paths() {
    let mut value = source();
    value.configuration.modules = vec![".".into(), "child".into()];
    value.configuration.id = configuration_id(&value.configuration).unwrap();
    value.manifests.push(Manifest {
        path: "child/Cargo.toml".into(),
        content_sha256: "c".repeat(64),
        system: BuildSystem::Cargo,
        module: "child".into(),
        parent_module: Some(".".into()),
    });
    assert!(snapshot_id(&value).is_ok());
    value.configuration.configuration_files[0].path = ".".into();
    assert_eq!(snapshot_id(&value).unwrap_err().code, GapCode::InvalidFact);
}
