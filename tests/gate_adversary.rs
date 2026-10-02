//! Probe the actual private gate comparator in a scratch-compiled copy, without changing it.
use std::{fs, path::PathBuf, process::Command};

#[test]
fn unchanged_generated_contract_ignores_local_ownership_state() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let base = std::env::var_os("CODEGATE_ADVERSARY_SCRATCH")
        .map(PathBuf::from)
        .unwrap_or_else(|| root.join("target/gate-adversary"));
    let scratch = base.join(format!("generation-{}", std::process::id()));
    fs::create_dir_all(&scratch).unwrap();
    for name in ["first", "second"] {
        let out = scratch.join(name);
        let result = Command::new("ess")
            .args([
                "generate",
                "synthesize",
                "--path",
                "ess",
                "--target",
                "rust",
                "--layout",
                "crate",
                "--out",
            ])
            .arg(&out)
            .current_dir(&root)
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        let result = Command::new("cargo")
            .args(["fmt", "--manifest-path"])
            .arg(out.join("Cargo.toml"))
            .current_dir(&root)
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
    }
    assert_eq!(
        fs::read(scratch.join("first/src/dependency.rs")).unwrap(),
        fs::read(scratch.join("second/src/dependency.rs")).unwrap()
    );
    let mut source = fs::read_to_string(root.join("src/bin/codegate-check.rs")).unwrap();
    source.push_str(r#"
#[test]
fn ownership_state_is_not_generated_contract_drift() {
    let root = std::path::PathBuf::from(std::env::var_os("GATE_PROBE_ROOT").unwrap());
    let first = tree(&root.join("first")).unwrap();
    let second = tree(&root.join("second")).unwrap();
    let differences: Vec<_> = first.keys().chain(second.keys()).filter(|key| first.get(*key) != second.get(*key)).collect();
    assert!(differences.is_empty(), "identical ESS contract spuriously drifts: {differences:?}");
    // Product drift remains strict after excluding directory-local ownership state.
    for relative in ["Cargo.toml", "PLAN.md", "plan.json", "src/dependency.rs", "types-report.json", "source.schema.json"] {
        let path=root.join("second").join(relative);
        let original=std::fs::read(&path).ok();
        std::fs::write(&path,b"changed generated product").unwrap();
        assert_ne!(first,tree(&root.join("second")).unwrap(),"silently ignored product {relative}");
        match original {Some(bytes)=>std::fs::write(path,bytes).unwrap(),None=>std::fs::remove_file(path).unwrap()}
    }
}
"#);
    let source_path = scratch.join("gate_probe.rs");
    fs::write(&source_path, source).unwrap();
    let deps = std::env::current_exe()
        .unwrap()
        .parent()
        .unwrap()
        .to_path_buf();
    let dependency = |name: &str| -> PathBuf {
        fs::read_dir(&deps)
            .unwrap()
            .map(|e| e.unwrap().path())
            .find(|p| {
                p.file_name()
                    .unwrap()
                    .to_string_lossy()
                    .starts_with(&format!("lib{name}-"))
                    && p.extension().is_some_and(|x| x == "rlib")
            })
            .unwrap()
    };
    let binary = scratch.join("gate_probe");
    let compile = Command::new("rustc")
        .args(["--edition=2024", "--test"])
        .arg(&source_path)
        .arg("-L")
        .arg(format!("dependency={}", deps.display()))
        .arg("--extern")
        .arg(format!("clap={}", dependency("clap").display()))
        .arg("--extern")
        .arg(format!("serde_json={}", dependency("serde_json").display()))
        .arg("--extern")
        .arg(format!("sha2={}", dependency("sha2").display()))
        .arg("-o")
        .arg(&binary)
        .env("CARGO_MANIFEST_DIR", &root)
        .output()
        .unwrap();
    assert!(
        compile.status.success(),
        "compile-only failure: {}",
        String::from_utf8_lossy(&compile.stderr)
    );
    let result = Command::new(&binary)
        .args([
            "ownership_state_is_not_generated_contract_drift",
            "--exact",
            "--nocapture",
        ])
        .env("GATE_PROBE_ROOT", &scratch)
        .output()
        .unwrap();
    fs::write(
        scratch.join("output.log"),
        [&result.stdout[..], &result.stderr[..]].concat(),
    )
    .unwrap();
    assert!(
        result.status.success(),
        "actual gate comparator rejected unchanged contracts:\n{}\n{}",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
}
