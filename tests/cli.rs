use serde_json::{Value, json};
use std::{path::PathBuf, process::Command};
fn dir(name: &str) -> PathBuf {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target/cli-tests")
        .join(name);
    std::fs::create_dir_all(&path).unwrap();
    path
}
fn fixture(name: &str) -> Value {
    serde_json::from_slice(&std::fs::read(format!("ess/scenarios/{name}.yaml")).unwrap()).unwrap()
}
#[test]
fn cli_reports_full_outcomes_and_coverage_aware_exits_without_tools() {
    for (name, code) in [
        ("one-runtime-edge", 0),
        ("forbidden-runtime-edge", 1),
        ("partial-is-not-empty", 2),
        ("witnessed-violation-with-gaps", 2),
        ("dangling-target", 2),
        ("failed-collection", 2),
    ] {
        let d = dir(name);
        let f = fixture(name);
        let step = &f["timeline"][0];
        std::fs::write(
            d.join("facts.json"),
            serde_json::to_vec(&step["input"]["snapshot"]).unwrap(),
        )
        .unwrap();
        std::fs::write(
            d.join("policy.json"),
            serde_json::to_vec(&step["input"]["policy"]).unwrap(),
        )
        .unwrap();
        let result = Command::new(env!("CARGO_BIN_EXE_codegate"))
            .current_dir(&d)
            .env_clear()
            .args([
                "evaluate",
                "--facts",
                "facts.json",
                "--policy",
                "policy.json",
            ])
            .output()
            .unwrap();
        assert_eq!(
            result.status.code(),
            Some(code),
            "{name}: {}",
            String::from_utf8_lossy(&result.stderr)
        );
        assert_eq!(
            serde_json::from_slice::<Value>(&result.stdout).unwrap(),
            step["response"]["report"]
        );
        let result = Command::new(env!("CARGO_BIN_EXE_codegate"))
            .current_dir(&d)
            .env_clear()
            .args([
                "evaluate",
                "--facts",
                "facts.json",
                "--policy",
                "policy.json",
                "--out",
                "report.json",
            ])
            .output()
            .unwrap();
        assert_eq!(result.status.code(), Some(code));
        assert!(result.stdout.is_empty());
        assert_eq!(
            serde_json::from_slice::<Value>(&std::fs::read(d.join("report.json")).unwrap())
                .unwrap(),
            step["response"]["report"]
        );
    }
}
#[test]
fn cli_structural_refusal_emits_no_report_or_output_file() {
    let d = dir("refusal");
    std::fs::write(d.join("facts.json"), b"{\"format\":1,\"format\":2}").unwrap();
    std::fs::write(d.join("policy.json"), b"{}").unwrap();
    let output = d.join("must-not-create.json");
    if output.exists() {
        std::fs::remove_file(&output).unwrap();
    }
    let result = Command::new(env!("CARGO_BIN_EXE_codegate"))
        .args(["evaluate", "--facts"])
        .arg(d.join("facts.json"))
        .arg("--policy")
        .arg(d.join("policy.json"))
        .arg("--out")
        .arg(&output)
        .output()
        .unwrap();
    assert_eq!(result.status.code(), Some(2));
    assert!(result.stdout.is_empty());
    assert!(String::from_utf8_lossy(&result.stderr).contains("duplicate"));
    assert!(!output.exists());
}
#[test]
fn unsupported_policy_format_is_semantic_error() {
    let d = dir("policy-format");
    let mut f = fixture("one-runtime-edge");
    f["timeline"][0]["input"]["policy"]["format"] = json!("future");
    for (key, name) in [("snapshot", "facts.json"), ("policy", "policy.json")] {
        std::fs::write(
            d.join(name),
            serde_json::to_vec(&f["timeline"][0]["input"][key]).unwrap(),
        )
        .unwrap();
    }
    let result = Command::new(env!("CARGO_BIN_EXE_codegate"))
        .current_dir(&d)
        .args([
            "evaluate",
            "--facts",
            "facts.json",
            "--policy",
            "policy.json",
        ])
        .output()
        .unwrap();
    assert_eq!(result.status.code(), Some(2));
    let report: Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(report["diagnostics"][0]["code"], "InvalidFormat");
}
