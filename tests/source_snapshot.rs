use codegate::{bindings::lines::parse_source, collection::collect_source, semantic_model::*};
use std::{
    fs,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};
static SERIAL: AtomicU64 = AtomicU64::new(0);
fn project() -> PathBuf {
    let p = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .join(format!(
            "source-fixture-{}-{}",
            std::process::id(),
            SERIAL.fetch_add(1, Ordering::Relaxed)
        ));
    fs::create_dir(&p).unwrap();
    p
}
fn put(root: &Path, path: &str, text: &str) {
    let p = root.join(path);
    fs::create_dir_all(p.parent().unwrap()).unwrap();
    fs::write(p, text).unwrap();
}
fn request(root: &Path) -> CollectionRequest {
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
#[test]
fn selected_dirty_untracked_bytes_configuration_and_manifests_define_identity() {
    let root = project();
    put(&root, "src/lib.rs", "pub fn value() {}\n");
    put(
        &root,
        "Cargo.toml",
        "[package]\nname='fixture'\nversion='0.1.0'\n",
    );
    let req = request(&root);
    let first = collect_source(&req).unwrap();
    assert_eq!(first.snapshot.files.len(), 1);
    assert_eq!(first.snapshot.manifests.len(), 1);
    assert_eq!(first.contents.len(), 2);
    assert_eq!(first.snapshot.manifests[0].module, ".");
    assert_eq!(collect_source(&req).unwrap().snapshot.id, first.snapshot.id);
    put(&root, "unrelated.txt", "ignored");
    assert_eq!(collect_source(&req).unwrap().snapshot.id, first.snapshot.id);
    put(&root, "src/lib.rs", "pub fn changed() {}\n");
    assert_ne!(collect_source(&req).unwrap().snapshot.id, first.snapshot.id);
    let changed = collect_source(&req).unwrap();
    put(
        &root,
        "Cargo.toml",
        "[package]\nname='fixture'\nversion='0.2.0'\n",
    );
    assert_ne!(
        collect_source(&req).unwrap().snapshot.configuration.id,
        changed.snapshot.configuration.id
    );
    let mut cfg = req.clone();
    cfg.configuration.go_build_tags.push("tag".into());
    assert_ne!(
        collect_source(&cfg).unwrap().snapshot.id,
        collect_source(&req).unwrap().snapshot.id
    );
    cfg.configuration.id = ConfigurationId("stale".into());
    assert!(collect_source(&cfg).is_err());
}
#[test]
fn three_syntax_classifiers_preserve_physical_bytes_and_comment_lookalikes() {
    for (language, text) in [
        (
            Language::Rust,
            "// é\nfn a(){let s=\"/* not comment */\";} // tail\n\n",
        ),
        (
            Language::Go,
            "// é\npackage p // tail\nvar x = \"/* not comment */\"\n\n",
        ),
        (
            Language::Java,
            "// é\nclass C { String s=\"/* not comment */\"; } // tail\n\n",
        ),
    ] {
        let parsed = parse_source(language, "source", text).unwrap();
        assert!(parsed.gaps.is_empty(), "{:?}", parsed.gaps);
        assert_eq!(parsed.lines[0].classification, LineClass::Comment);
        assert_eq!(parsed.lines[1].classification, LineClass::Mixed);
        assert_eq!(
            parsed.lines.last().unwrap().classification,
            LineClass::Blank
        );
        assert_eq!(parsed.lines.last().unwrap().end_byte, text.len() as i64);
        for (i, line) in parsed.lines.iter().enumerate() {
            assert_eq!(line.number, i as i64);
            if i > 0 {
                assert_eq!(parsed.lines[i - 1].end_byte, line.start_byte)
            }
        }
        assert!(
            parse_source(language, "bad", "{{{ !!!")
                .unwrap()
                .gaps
                .iter()
                .any(|g| g.code == GapCode::SyntaxError)
        );
        assert!(
            parse_source(language, "empty", "")
                .unwrap()
                .lines
                .is_empty()
        );
    }
    let multiline = parse_source(
        Language::Rust,
        "raw.rs",
        "fn f(){ let s=r#\"\n// literal\n\"#; }\n",
    )
    .unwrap();
    assert_eq!(multiline.lines[1].classification, LineClass::Code);
}
#[test]
fn literal_selection_tests_generated_languages_and_skipped_directories() {
    let root = project();
    for (p, t) in [
        ("src/a.rs", "fn a(){}"),
        ("src/a_test.go", "package p"),
        ("tests/test.rs", "fn t(){}"),
        ("src/GTest.java", "class GTest {}"),
        (
            "src/gen.rs",
            "// Code generated by fixture. DO NOT EDIT.\nfn g(){}",
        ),
        ("target/cached.rs", "fn cache(){}"),
    ] {
        put(&root, p, t)
    }
    let mut req = request(&root);
    req.selection.include_tests = false;
    req.selection.include_generated = false;
    req.selection.languages = vec![Language::Rust];
    let result = collect_source(&req).unwrap();
    assert_eq!(
        result
            .snapshot
            .files
            .iter()
            .map(|f| f.path.as_str())
            .collect::<Vec<_>>(),
        vec!["src/a.rs"]
    );
    req.selection.include_paths = vec!["target".into()];
    assert_eq!(
        collect_source(&req).unwrap().snapshot.files[0].path,
        "target/cached.rs"
    );
    req.selection.include_paths = vec!["src".into()];
    req.selection.exclude_paths = vec!["src/a.rs".into()];
    assert!(collect_source(&req).unwrap().snapshot.files.is_empty());
    req.selection.include_paths = vec!["src".into(), "src".into()];
    assert!(collect_source(&req).is_err());
    req = request(&root);
    req.mode = CollectionMode::Semantic;
    assert!(collect_source(&req).is_err());
}
#[test]
fn git_origin_tracks_staged_and_dirty_bytes_without_affecting_identity() {
    let root = project();
    let repo = git2::Repository::init(&root).unwrap();
    put(&root, "a.rs", "fn a(){}\n");
    let before = collect_source(&request(&root)).unwrap();
    assert_eq!(before.snapshot.files[0].origin, SourceOrigin::Untracked);
    let mut index = repo.index().unwrap();
    index.add_path(Path::new("a.rs")).unwrap();
    index.write().unwrap();
    let treeid = index.write_tree().unwrap();
    let tree = repo.find_tree(treeid).unwrap();
    let sig = git2::Signature::now("Fixture", "fixture@example.invalid").unwrap();
    repo.commit(Some("HEAD"), &sig, &sig, "fixture", &tree, &[])
        .unwrap();
    let clean = collect_source(&request(&root)).unwrap();
    assert_eq!(clean.snapshot.files[0].origin, SourceOrigin::TrackedClean);
    assert_eq!(clean.snapshot.id, before.snapshot.id);
    put(&root, "a.rs", "fn dirty(){}\n");
    assert_eq!(
        collect_source(&request(&root)).unwrap().snapshot.files[0].origin,
        SourceOrigin::TrackedDirty
    );
    index.add_path(Path::new("a.rs")).unwrap();
    index.write().unwrap();
    put(&root, "a.rs", "fn a(){}\n");
    assert_eq!(
        collect_source(&request(&root)).unwrap().snapshot.files[0].origin,
        SourceOrigin::TrackedDirty
    );
}
#[test]
fn static_manifests_preserve_hierarchy_and_report_dynamic_selection() {
    let root = project();
    put(&root, "go.work", "go 1.23\nuse (\n ./service\n)\n");
    put(
        &root,
        "service/go.mod",
        "module example.test/service\ngo 1.23\n",
    );
    put(&root, "service/a.go", "package service\n");
    put(
        &root,
        "pom.xml",
        "<project><modules><module>child</module></modules></project>",
    );
    put(
        &root,
        "child/pom.xml",
        "<project><parent><relativePath>../pom.xml</relativePath></parent></project>",
    );
    put(&root, "child/C.java", "class C {}\n");
    put(
        &root,
        "settings.gradle",
        "include ':web'\ninclude(computedName)\n",
    );
    put(&root, "web/build.gradle", "plugins { id 'java' }\n");
    put(&root, "web/W.java", "class W {}\n");
    let result = collect_source(&request(&root)).unwrap();
    assert_eq!(result.snapshot.files.len(), 3);
    assert!(
        result
            .snapshot
            .manifests
            .iter()
            .any(|m| m.path == "child/pom.xml" && m.parent_module.as_deref() == Some("."))
    );
    assert!(
        result
            .snapshot
            .manifests
            .iter()
            .any(|m| m.path == "service/go.mod" && m.parent_module.as_deref() == Some("."))
    );
    assert!(
        result
            .gaps
            .iter()
            .any(|g| g.code == GapCode::MissingBuildSelection)
    );
    assert_eq!(
        result.contents.len(),
        result.snapshot.files.len() + result.snapshot.configuration.configuration_files.len()
    );
}
#[test]
fn confinement_bounds_and_invalid_input_refuse_instead_of_empty_success() {
    let root = project();
    put(&root, "a.rs", "fn a(){}");
    let mut req = request(&root);
    req.selection.include_paths = vec!["../outside".into()];
    assert!(collect_source(&req).is_err());
    req.selection.include_paths = vec!["/etc".into()];
    assert!(collect_source(&req).is_err());
    req = request(&root);
    fs::write(root.join("a.rs"), [255]).unwrap();
    assert!(collect_source(&req).is_err());
    fs::write(root.join("a.rs"), vec![b' '; 4 * 1024 * 1024 + 1]).unwrap();
    assert!(collect_source(&req).is_err());
}
#[cfg(unix)]
#[test]
fn root_and_nested_symlinks_never_escape() {
    use std::os::unix::fs::symlink;
    let root = project();
    let outside = project();
    put(&outside, "leak.rs", "fn secret(){}");
    symlink(&outside, root.join("linked")).unwrap();
    assert!(collect_source(&request(&root)).is_err());
    assert!(collect_source(&request(&root.join("linked"))).is_err());
    let mut req = request(&root);
    req.selection.exclude_paths = vec!["linked".into()];
    assert!(collect_source(&req).unwrap().snapshot.files.is_empty());
}

#[test]
fn generated_markers_inside_strings_do_not_exclude_source() {
    let root = project();
    put(
        &root,
        "a.rs",
        "fn f(){ let s=r#\"\n// Code generated by fixture. DO NOT EDIT.\n\"#; }\n",
    );
    let mut req = request(&root);
    req.selection.include_generated = false;
    let got = collect_source(&req).unwrap();
    assert_eq!(got.snapshot.files.len(), 1);
    assert_eq!(
        got.snapshot.files[0].classification,
        SourceClass::Production
    );
}
#[test]
fn quoted_go_paths_and_conditional_gradle_includes_have_honest_selection() {
    let root = project();
    put(&root, "go.work", "go 1.23\nuse \"./service\"\n");
    put(
        &root,
        "service/go.mod",
        "module \"example.test/service\"\ngo 1.23\n",
    );
    put(&root, "service/a.go", "package service\n");
    let first = collect_source(&request(&root)).unwrap();
    assert!(first.gaps.is_empty(), "{:?}", first.gaps);
    assert_eq!(
        first
            .snapshot
            .manifests
            .iter()
            .find(|m| m.path == "service/go.mod")
            .unwrap()
            .parent_module
            .as_deref(),
        Some(".")
    );
    put(
        &root,
        "settings.gradle",
        "if (enabled) { include ':web' }\n",
    );
    put(&root, "web/build.gradle", "plugins { id 'java' }\n");
    let conditional = collect_source(&request(&root)).unwrap();
    assert!(
        conditional
            .gaps
            .iter()
            .any(|g| g.code == GapCode::MissingBuildSelection)
    );
}
#[test]
fn corrupt_git_and_asserted_configuration_metadata_are_never_trusted() {
    let root = project();
    put(&root, "a.rs", "fn a(){}\n");
    put(&root, "Cargo.toml", "[package]\nname='x'\n");
    let initial = collect_source(&request(&root)).unwrap();
    let mut req = request(&root);
    req.configuration.configuration_files =
        initial.snapshot.configuration.configuration_files.clone();
    req.configuration.configuration_files[0].content_sha256 = "0".repeat(64);
    assert!(
        collect_source(&req)
            .unwrap_err()
            .iter()
            .any(|g| g.code == GapCode::IdentityMismatch)
    );
    fs::write(root.join(".git"), "not git metadata").unwrap();
    assert!(collect_source(&request(&root)).is_err());
}

#[test]
fn aggregate_bytes_and_file_inventory_are_bounded() {
    let root = project();
    let block = " ".repeat(4 * 1024 * 1024);
    for i in 0..17 {
        put(&root, &format!("config{i}/application.properties"), &block)
    }
    let gap = collect_source(&request(&root)).unwrap_err();
    assert!(
        gap.iter()
            .any(|g| g.reason == "selected set exceeds 64 MiB limit")
    );
    let root = project();
    for i in 0..10_001 {
        put(&root, &format!("f{i:05}.rs"), "")
    }
    let gap = collect_source(&request(&root)).unwrap_err();
    assert!(
        gap.iter()
            .any(|g| g.reason == "selected set exceeds 10000 file limit")
    );
}
#[cfg(unix)]
#[test]
fn referenced_manifest_symlinks_are_refused_and_configuration_claims_cannot_panic() {
    use std::os::unix::fs::symlink;
    let root = project();
    let outside = project();
    put(&root, "Cargo.toml", "[workspace]\nmembers=['hidden']\n");
    put(&root, "a.rs", "fn a(){}");
    put(&outside, "Cargo.toml", "[package]\nname='outside'\n");
    fs::create_dir(root.join("hidden")).unwrap();
    symlink(outside.join("Cargo.toml"), root.join("hidden/Cargo.toml")).unwrap();
    let mut req = request(&root);
    req.selection.include_paths = vec!["a.rs".into()];
    assert!(collect_source(&req).is_err());
    let root = project();
    put(&root, "a.rs", "fn a(){}");
    let initial = collect_source(&request(&root)).unwrap();
    let mut req = request(&root);
    let mut file = initial.snapshot.files[0].clone();
    file.classification = SourceClass::BuildConfiguration;
    file.language = None;
    req.configuration.configuration_files.push(file);
    let result = collect_source(&req).unwrap();
    assert!(result.snapshot.files.is_empty());
    assert_eq!(result.snapshot.configuration.configuration_files.len(), 1);
}

#[test]
fn workspace_root_members_do_not_invent_self_parent_cycles() {
    let root = project();
    put(&root, "go.work", "go 1.23\nuse .\n");
    put(&root, "go.mod", "module example.test/root\ngo 1.23\n");
    put(&root, "a.go", "package root\n");
    let result = collect_source(&request(&root)).unwrap();
    assert!(result.gaps.is_empty(), "{:?}", result.gaps);
    assert!(
        result
            .snapshot
            .manifests
            .iter()
            .all(|m| m.parent_module.is_none())
    );
}

#[test]
fn manifest_references_cannot_select_repository_metadata() {
    let root = project();
    git2::Repository::init(&root).unwrap();
    put(
        &root,
        "Cargo.toml",
        "[workspace]\nmembers=['.git/hidden']\n",
    );
    put(
        &root,
        ".git/hidden/Cargo.toml",
        "[package]\nname='metadata'\n",
    );
    let result = collect_source(&request(&root));
    assert!(result.is_err(), "manifest selected repository metadata");
}

fn adversary_project(name: &str) -> PathBuf {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(".scratch/unit/adversary-inputs")
        .join(format!("{name}-{}", std::process::id()));
    fs::create_dir_all(&path).unwrap();
    path
}

#[test]
fn adversary_selected_source_preserves_ancestor_cargo_configuration() {
    let root = adversary_project("cargo-config");
    put(
        &root,
        "Cargo.toml",
        "[package]\nname='fixture'\nversion='0.1.0'\n",
    );
    put(&root, "src/lib.rs", "pub fn fixture() {}\n");
    put(
        &root,
        ".cargo/config.toml",
        "[build]\nrustflags=['--cfg=before']\n",
    );
    let mut req = request(&root);
    req.selection.include_paths = vec!["src".into()];
    let before = collect_source(&req).unwrap();
    put(
        &root,
        ".cargo/config.toml",
        "[build]\nrustflags=['--cfg=after']\n",
    );
    let after = collect_source(&req).unwrap();
    assert_ne!(
        before.snapshot.configuration.id, after.snapshot.configuration.id,
        "changing ancestor Cargo configuration must change the selected source configuration identity"
    );
    assert!(before.contents.contains_key(".cargo/config.toml"));
}

#[test]
fn adversary_maven_cdata_hierarchy_is_observed_or_explicitly_incomplete() {
    let root = adversary_project("maven-cdata");
    put(
        &root,
        "pom.xml",
        r#"<project><modelVersion>4.0.0</modelVersion><groupId>example</groupId><artifactId>parent</artifactId><version>1</version><packaging>pom</packaging><modules><module><![CDATA[child]]></module></modules></project>"#,
    );
    put(
        &root,
        "child/pom.xml",
        r#"<project><modelVersion>4.0.0</modelVersion><parent><groupId>example</groupId><artifactId>parent</artifactId><version>1</version><relativePath><![CDATA[../pom.xml]]></relativePath></parent><artifactId>child</artifactId></project>"#,
    );
    put(&root, "child/src/main/java/Child.java", "class Child {}\n");
    let result = collect_source(&request(&root)).unwrap();
    let child = result
        .snapshot
        .manifests
        .iter()
        .find(|manifest| manifest.path == "child/pom.xml")
        .unwrap();
    assert!(
        child.parent_module.as_deref() == Some(".")
            || result
                .gaps
                .iter()
                .any(|gap| gap.code == GapCode::MissingBuildSelection),
        "valid CDATA module/parent declarations were silently discarded: parent={:?}, gaps={:?}",
        child.parent_module,
        result.gaps
    );
}

#[test]
fn correction_ancestor_configuration_directories_share_discovery_and_excludes() {
    let root = project();
    put(&root, "Cargo.toml", "[package]\nname='fixture'\n");
    put(&root, "nested/src/lib.rs", "fn fixture(){}\n");
    let configs = [
        ".cargo/config.toml",
        ".mvn/maven.config",
        ".mvn/jvm.config",
        "nested/.cargo/config.toml",
    ];
    for path in configs {
        put(&root, path, "before\n")
    }
    let mut req = request(&root);
    req.selection.include_paths = vec!["nested/src".into()];
    let mut previous = collect_source(&req).unwrap();
    for path in configs {
        assert!(previous.contents.contains_key(path), "omitted {path}");
        put(&root, path, "after\n");
        let next = collect_source(&req).unwrap();
        assert_ne!(
            previous.snapshot.configuration.id, next.snapshot.configuration.id,
            "unbound {path}"
        );
        previous = next;
    }
    req.selection.exclude_paths = vec![".cargo".into(), ".mvn".into(), "nested/.cargo".into()];
    let before = collect_source(&req).unwrap();
    for path in configs {
        assert!(!before.contents.contains_key(path));
        put(&root, path, "excluded change\n")
    }
    assert_eq!(
        before.snapshot.id,
        collect_source(&req).unwrap().snapshot.id
    );
}

#[test]
fn correction_maven_mixed_text_cdata_and_whitespace_preserve_literal_hierarchy() {
    let root = project();
    put(
        &root,
        "pom.xml",
        "<project><modules><module> child<![CDATA[ space]]> </module></modules></project>",
    );
    put(
        &root,
        "child space/pom.xml",
        "<project><parent><relativePath> <![CDATA[../]]>pom.xml </relativePath></parent></project>",
    );
    let result = collect_source(&request(&root)).unwrap();
    assert!(result.gaps.is_empty(), "{:?}", result.gaps);
    assert_eq!(
        result
            .snapshot
            .manifests
            .iter()
            .find(|m| m.path == "child space/pom.xml")
            .unwrap()
            .parent_module
            .as_deref(),
        Some(".")
    );
}

#[cfg(unix)]
#[test]
fn correction_discovered_configuration_directories_refuse_symlinks() {
    use std::os::unix::fs::symlink;
    let root = project();
    let outside = project();
    put(&root, "src/lib.rs", "fn fixture(){}\n");
    put(&outside, "config.toml", "[build]\n");
    symlink(&outside, root.join(".cargo")).unwrap();
    let mut req = request(&root);
    req.selection.include_paths = vec!["src".into()];
    assert!(collect_source(&req).is_err());
    req.selection.exclude_paths = vec![".cargo".into()];
    assert!(collect_source(&req).is_ok());
}

#[test]
fn adversary_round2_unclosed_maven_xml_cannot_claim_complete_selection() {
    let root = adversary_project("maven-unclosed");
    put(
        &root,
        "pom.xml",
        "<project><modules><module><![CDATA[child]]></module></modules>",
    );
    put(
        &root,
        "child/pom.xml",
        "<project><artifactId>child</artifactId></project>",
    );
    put(&root, "child/src/main/java/Child.java", "class Child {}\n");
    match collect_source(&request(&root)) {
        Err(gaps) => assert!(!gaps.is_empty()),
        Ok(result) => assert!(
            result
                .gaps
                .iter()
                .any(|gap| gap.code == GapCode::MissingBuildSelection),
            "unclosed Maven XML returned no build-selection gap: {:?}",
            result.gaps
        ),
    }
}

#[test]
fn correction_round2_maven_eof_requires_all_open_elements_closed() {
    for text in [
        "<project>",
        "<project><modules>",
        "<project><modules><module>child",
        "<project><parent>",
        "<project><parent><relativePath><![CDATA[../pom.xml]]>",
    ] {
        let root = project();
        put(&root, "pom.xml", text);
        let result = collect_source(&request(&root)).unwrap();
        assert!(
            result
                .gaps
                .iter()
                .any(|gap| gap.code == GapCode::MissingBuildSelection
                    && gap.reason == "pom.xml: invalid Maven XML"),
            "truncated XML {text:?} lacked an XML completeness gap: {:?}",
            result.gaps
        );
    }
}
