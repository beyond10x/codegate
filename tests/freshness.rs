//! Exercise the production gate stage with real Cargo producers and retained stale evidence.
use std::{fs, path::PathBuf, process::Command};

#[test]
fn gate_requires_this_invocations_conformance_producer() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let scratch = root
        .join("target")
        .join(format!("freshness-regression-{}", std::process::id()));
    fs::create_dir_all(&scratch).unwrap();
    let mut source = fs::read_to_string(root.join("src/bin/codegate-check.rs")).unwrap();
    source.push_str(r###"
#[test]
fn real_fresh_report_is_admitted_and_stale_producers_are_refused() {
    let root=PathBuf::from(std::env::var_os("GATE_SOURCE_ROOT").unwrap());
    let scratch=PathBuf::from(std::env::var_os("GATE_PROBE_ROOT").unwrap());
    let suite=scratch.join("expected-suite.json");
    let generated=Command::new("ess").args(["verify","conform","synthesize","--path","ess","--scenarios","ess","--suite-format","5","--out"]).arg(&suite).current_dir(&root).output().unwrap();
    assert!(generated.status.success(),"{}",String::from_utf8_lossy(&generated.stderr));
    let positive=scratch.join("positive");fs::create_dir(&positive).unwrap();
    fresh_conformance(&root,&positive,&suite).expect("actual current evaluator must produce admissible evidence");
    let stale=positive.join("conformance");
    for mode in ["missing","ignored","renamed"] {
        let project=scratch.join(mode);fs::create_dir_all(project.join("src")).unwrap();
        fs::write(project.join("Cargo.toml"),"[package]\nname=\"codegate-cli\"\nversion=\"0.1.0\"\nedition=\"2024\"\n[workspace]\n").unwrap();
        fs::write(project.join("src/lib.rs"),"pub fn marker() {}\n").unwrap();
        if mode!="missing" {
            fs::create_dir(project.join("tests")).unwrap();
            let producer=if mode=="ignored" {"#[test]\n#[ignore]\nfn real_evaluator_conforms_to_complete_combined_suite() {}\n"} else {"#[test]\nfn unrelated_green_test() {}\n"};
            fs::write(project.join("tests/conformance.rs"),producer).unwrap();
        }
        let lock=Command::new("cargo").args(["generate-lockfile","--offline"]).current_dir(&project).output().unwrap();assert!(lock.status.success());
        fs::create_dir_all(project.join("target/conformance")).unwrap();
        for file in ["suite.json","report.json","run.json"] {fs::copy(stale.join(file),project.join("target/conformance").join(file)).unwrap();}
        let private=project.join("target/current-gate");fs::create_dir(&private).unwrap();
        let result=fresh_conformance(&project,&private,&suite);
        assert!(result.is_err(),"{mode} producer accepted stale complete27 report");
        let error=result.unwrap_err();
        if mode=="missing" {assert!(error.contains("producer failed"),"{error}");} else {assert!(error.contains("fresh conformance suite missing"),"{error}");}
        assert_eq!(fs::read(stale.join("report.json")).unwrap(),fs::read(project.join("target/conformance/report.json")).unwrap());
        eprintln!("{mode}: refused stale evidence: {error}");
    }
    // Enumerate all evidence-binding fields against one actual report, not a fabricated success.
    let suite_bytes=fs::read(&suite).unwrap();let suite_value=serde_json::from_slice(&suite_bytes).unwrap();
    let report:serde_json::Value=serde_json::from_slice(&fs::read(stale.join("report.json")).unwrap()).unwrap();
    let completed=report["completed_at"].as_u64().unwrap();
    validate_report(&report,&suite_value,&suite_bytes,completed,completed).unwrap();
    for (pointer,value) in [
        ("/format",serde_json::json!("future")),
        ("/producer_profile",serde_json::json!("wrong")),
        ("/policy",serde_json::json!("partial-selection/1")),
        ("/implementation",serde_json::json!("someone-else 1")),
        ("/spec_digest",serde_json::json!("wrong")),
        ("/specification",serde_json::json!("wrong/v1")),
        ("/suite/digest",serde_json::json!("sha256:wrong")),
        ("/suite/digest_profile",serde_json::json!("wrong")),
        ("/suite/version",serde_json::json!("wrong")),
        ("/counts/passed",serde_json::json!(26)),
        ("/coverage/knowledge",serde_json::json!("unknown")),
        ("/coverage/counts/generated",serde_json::json!(0)),
        ("/coverage/selection/filter/kind",serde_json::json!("partial")),
        ("/coverage/refused",serde_json::json!(["refusal"])),
        ("/outcomes/passed",serde_json::json!([])),
        ("/outcomes/skipped",serde_json::json!(["skipped"])),
        ("/execution_status",serde_json::json!("failed")),
        ("/conformance_status",serde_json::json!("inconclusive")),
        ("/completed_at",serde_json::json!(0))
    ] {
        let mut changed=report.clone();*changed.pointer_mut(pointer).unwrap()=value;
        assert!(validate_report(&changed,&suite_value,&suite_bytes,completed,completed).is_err(),"unbound {pointer}");
    }
}
"###);
    let source_path = scratch.join("freshness_probe.rs");
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
                    && p.extension().is_some_and(|e| e == "rlib")
            })
            .unwrap()
    };
    let binary = scratch.join("freshness_probe");
    let mut compiler = Command::new("rustc");
    compiler
        .args(["--edition=2024", "--test"])
        .arg(&source_path)
        .arg("-L")
        .arg(format!("dependency={}", deps.display()));
    for name in ["clap", "serde_json", "sha2"] {
        compiler
            .arg("--extern")
            .arg(format!("{name}={}", dependency(name).display()));
    }
    let compile = compiler
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
            "real_fresh_report_is_admitted_and_stale_producers_are_refused",
            "--exact",
            "--nocapture",
        ])
        .env("GATE_SOURCE_ROOT", &root)
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
        "{}\n{}",
        String::from_utf8_lossy(&result.stdout),
        String::from_utf8_lossy(&result.stderr)
    );
}
