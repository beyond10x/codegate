//! Repository gate. Every subprocess gets its own recorded result; no recursive task invocation.
use clap::Parser;
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::{Command, ExitCode},
};
#[derive(Parser)]
#[command(about = "Validate Codegate contracts, generated drift, real conformance and Rust checks")]
struct Args {}
fn step(name: &str, program: &str, args: &[&str], root: &Path) -> Result<(), String> {
    eprintln!("CHECK {name}: {program} {}", args.join(" "));
    let status = Command::new(program)
        .args(args)
        .current_dir(root)
        .env("CARGO_BUILD_JOBS", "2")
        .status()
        .map_err(|e| format!("{name}: {e}"))?;
    eprintln!(
        "CHECK {name}: exit {}",
        status
            .code()
            .map_or_else(|| "signal".into(), |v| v.to_string())
    );
    if status.success() {
        Ok(())
    } else {
        Err(format!("{name} failed"))
    }
}
fn tree(path: &Path) -> Result<BTreeMap<PathBuf, Vec<u8>>, String> {
    fn walk(
        root: &Path,
        path: &Path,
        files: &mut BTreeMap<PathBuf, Vec<u8>>,
    ) -> Result<(), String> {
        for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            let path = entry.path();
            let name = entry.file_name();
            if path.parent() == Some(root)
                && (name == "target" || name == "Cargo.lock" || name == ".ess-output")
            {
                continue;
            }
            if entry.file_type().map_err(|e| e.to_string())?.is_dir() {
                walk(root, &path, files)?;
            } else {
                files.insert(
                    path.strip_prefix(root)
                        .map_err(|e| e.to_string())?
                        .to_path_buf(),
                    fs::read(path).map_err(|e| e.to_string())?,
                );
            }
        }
        Ok(())
    }
    let mut files = BTreeMap::new();
    walk(path, path, &mut files)?;
    Ok(files)
}
fn observed_millis() -> Result<u64, String> {
    u64::try_from(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| e.to_string())?
            .as_millis(),
    )
    .map_err(|e| e.to_string())
}

/// Run the named producer into an output directory that did not exist before this
/// invocation. No report in the conventional target/conformance path is trusted.
fn fresh_conformance(root: &Path, scratch: &Path, expected_suite: &Path) -> Result<(), String> {
    let output = scratch.join("conformance");
    fs::create_dir(&output).map_err(|e| format!("fresh conformance directory: {e}"))?;
    let started = observed_millis()?;
    eprintln!(
        "CHECK real ESS conformance: cargo test --locked -p codegate-cli --test conformance real_evaluator_conforms_to_complete_combined_suite -- --exact --nocapture"
    );
    let status = Command::new("cargo")
        .args([
            "test",
            "--locked",
            "-p",
            "codegate-cli",
            "--test",
            "conformance",
            "real_evaluator_conforms_to_complete_combined_suite",
            "--",
            "--exact",
            "--nocapture",
        ])
        .current_dir(root)
        .env("CARGO_BUILD_JOBS", "2")
        .env("CODEGATE_CONFORMANCE_OUTPUT", &output)
        .status()
        .map_err(|e| e.to_string())?;
    eprintln!("CHECK real ESS conformance: exit {:?}", status.code());
    if !status.success() {
        return Err("real ESS conformance producer failed".into());
    }
    let ended = observed_millis()?;
    let suite_bytes = fs::read(expected_suite).map_err(|e| e.to_string())?;
    let actual_suite = fs::read(output.join("suite.json"))
        .map_err(|e| format!("fresh conformance suite missing: {e}"))?;
    if suite_bytes != actual_suite {
        return Err("real runner did not execute the exact regenerated suite".into());
    }
    let suite: serde_json::Value =
        serde_json::from_slice(&suite_bytes).map_err(|e| e.to_string())?;
    let report: serde_json::Value = serde_json::from_slice(
        &fs::read(output.join("report.json"))
            .map_err(|e| format!("fresh conformance report missing: {e}"))?,
    )
    .map_err(|e| e.to_string())?;
    validate_report(&report, &suite, &suite_bytes, started, ended)?;
    eprintln!(
        "CHECK native ESS report: executed27 passed27 failed0 error0 unsupported0 skipped0, exit0; evidence {}",
        output.display()
    );
    Ok(())
}

fn validate_report(
    report: &serde_json::Value,
    suite: &serde_json::Value,
    suite_bytes: &[u8],
    started: u64,
    ended: u64,
) -> Result<(), String> {
    let digest = format!(
        "sha256:{}",
        Sha256::digest(suite_bytes)
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect::<String>()
    );
    let scenario_ids = suite["scenarios"]
        .as_object()
        .ok_or("suite scenarios missing")?
        .keys()
        .cloned()
        .collect::<Vec<_>>();
    let completed = report["completed_at"]
        .as_u64()
        .ok_or("report completion missing")?;
    if report["counts"]
        != serde_json::json!({"error":0,"failed":0,"passed":27,"skipped":0,"total":27,"unsupported":0})
        || report["execution_status"] != "passed"
        || report["conformance_status"] != "passed"
        || report["format"] != "ess-conformance-report/2"
        || report["producer_profile"] != "rust-scenario-status/1"
        || report["policy"] != "complete-selection/1"
        || report["implementation"] != format!("codegate-offline {}", env!("CARGO_PKG_VERSION"))
        || report["spec_digest"] != suite["provenance"]["spec_digest"]
        || report["specification"]
            != format!(
                "{}/{}",
                suite["provenance"]["system"]
                    .as_str()
                    .ok_or("suite system missing")?,
                suite["provenance"]["specification_version"]
                    .as_str()
                    .ok_or("suite version missing")?
            )
        || report["suite"]["version"] != suite["provenance"]["suite_version"]
        || report["suite"]["digest_profile"] != "sha256-json-bytes/1"
        || report["suite"]["digest"] != digest
        || report["coverage"]["knowledge"] != "complete_inventory"
        || report["coverage"]["counts"] != suite["coverage"]["counts"]
        || report["coverage"]["selection"] != suite["coverage"]["selection"]
        || report["coverage"]["refused"] != serde_json::json!([])
        || report["outcomes"]["passed"] != serde_json::json!(scenario_ids)
        || ["error", "failed", "skipped", "unsupported"]
            .iter()
            .any(|key| report["outcomes"][key] != serde_json::json!([]))
        || !(started..=ended).contains(&completed)
    {
        return Err(
            "native ESS report does not match this invocation's suite, identity, counts or time"
                .into(),
        );
    }
    Ok(())
}
fn create_scratch(root: &Path) -> Result<PathBuf, String> {
    let target = root.join("target");
    fs::create_dir_all(&target).map_err(|e| e.to_string())?;
    let scratch = target.join(format!("codegate-check-{}", std::process::id()));
    fs::create_dir(&scratch).map_err(|e| e.to_string())?;
    Ok(scratch)
}

fn check() -> Result<(), String> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let version = Command::new("ess")
        .arg("--version")
        .output()
        .map_err(|e| e.to_string())?;
    if !version.status.success() || String::from_utf8_lossy(&version.stdout).trim() != "ess 0.50.0"
    {
        return Err("ESS 0.50.0 required for reproducible generation".into());
    }
    step("AEP", "aep", &["plan", "artifact", "validate"], &root)?;
    step(
        "ESS",
        "ess",
        &["specify", "validate", "--path", "ess"],
        &root,
    )?;
    let scratch = create_scratch(&root)?;
    let behavior = scratch.join("behavior");
    let wire = scratch.join("wire");
    let semantic_behavior = scratch.join("semantic-behavior");
    let semantic_wire = scratch.join("semantic-wire");
    let suite = scratch.join("suite.json");
    let b = behavior.to_str().ok_or("non-UTF8 build path")?;
    let w = wire.to_str().ok_or("non-UTF8 build path")?;
    step(
        "generate behavior",
        "ess",
        &[
            "generate",
            "synthesize",
            "--path",
            "ess",
            "--target",
            "rust",
            "--layout",
            "crate",
            "--out",
            b,
        ],
        &root,
    )?;
    step(
        "generate wire",
        "ess",
        &[
            "generate",
            "types",
            "--path",
            "ess",
            "--target",
            "rust",
            "--all-types",
            "--package",
            "codegate-contract",
            "--out",
            w,
        ],
        &root,
    )?;
    step(
        "semantic ESS",
        "ess",
        &["specify", "validate", "--path", "ess-semantic"],
        &root,
    )?;
    step(
        "generate semantic behavior",
        "ess",
        &[
            "generate",
            "synthesize",
            "--path",
            "ess-semantic",
            "--target",
            "rust",
            "--layout",
            "crate",
            "--out",
            semantic_behavior
                .to_str()
                .ok_or("non-UTF8 semantic behavior path")?,
        ],
        &root,
    )?;
    step(
        "generate semantic wire",
        "ess",
        &[
            "generate",
            "types",
            "--path",
            "ess-semantic",
            "--target",
            "rust",
            "--all-types",
            "--package",
            "codegate-semantic-contract",
            "--out",
            semantic_wire
                .to_str()
                .ok_or("non-UTF8 semantic wire path")?,
        ],
        &root,
    )?;
    for path in [&behavior, &wire, &semantic_behavior, &semantic_wire] {
        let manifest = path.join("Cargo.toml");
        step(
            "normalize generated formatting",
            "cargo",
            &[
                "fmt",
                "--manifest-path",
                manifest.to_str().ok_or("non-UTF8 manifest")?,
            ],
            &root,
        )?;
    }
    for (actual, expected) in [
        ("generated/behavior", behavior),
        ("generated/wire", wire),
        ("generated/semantic-behavior", semantic_behavior),
        ("generated/semantic-wire", semantic_wire),
    ] {
        let actual = tree(&root.join(actual))?;
        let expected = tree(&expected)?;
        if actual != expected {
            let differences: Vec<_> = actual
                .keys()
                .chain(expected.keys())
                .filter(|k| actual.get(*k) != expected.get(*k))
                .collect();
            return Err(format!("generated drift: {differences:?}"));
        }
    }
    eprintln!("CHECK generated drift: exit 0");
    eprintln!(
        "CHECK semantic foundation: generated contracts; six public and two internal runtime obligations remain; no semantic runtime conformance claimed"
    );
    step(
        "combined suite",
        "ess",
        &[
            "verify",
            "conform",
            "synthesize",
            "--path",
            "ess",
            "--scenarios",
            "ess",
            "--suite-format",
            "5",
            "--out",
            suite.to_str().ok_or("non-UTF8 suite path")?,
        ],
        &root,
    )?;
    fresh_conformance(&root, &scratch, &suite)?;
    step(
        "Rust tests",
        "cargo",
        &[
            "test",
            "--locked",
            "-p",
            "codegate-cli",
            "--all-targets",
            "--",
            "--nocapture",
        ],
        &root,
    )?;
    step(
        "format root",
        "cargo",
        &["fmt", "--package", "codegate-cli", "--check"],
        &root,
    )?;
    step(
        "Clippy root",
        "cargo",
        &[
            "clippy",
            "--locked",
            "-p",
            "codegate-cli",
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ],
        &root,
    )?;
    for manifest in [
        "generated/behavior/Cargo.toml",
        "generated/wire/Cargo.toml",
        "generated/semantic-behavior/Cargo.toml",
        "generated/semantic-wire/Cargo.toml",
    ] {
        step(
            "test generated",
            "cargo",
            &["test", "--offline", "--manifest-path", manifest],
            &root,
        )?;
        step(
            "format generated",
            "cargo",
            &["fmt", "--manifest-path", manifest, "--check"],
            &root,
        )?;
        let mut lint_args = vec![
            "clippy",
            "--offline",
            "--manifest-path",
            manifest,
            "--all-targets",
            "--",
            "-D",
            "warnings",
        ];
        // ESS 0.50.0 emits a manual, equivalent Default impl for EssPresence<T>.
        // This upstream style lint is scoped to that generated package; all
        // behavior/root warnings and byte-identical regeneration remain enforced.
        if matches!(
            manifest,
            "generated/wire/Cargo.toml" | "generated/semantic-wire/Cargo.toml"
        ) {
            lint_args.extend(["-A", "clippy::derivable_impls"]);
        }
        // ESS 0.50.0 emits an unreachable push when a component publishes no
        // events: both conversion enums are uninhabited. Preserve generated
        // bytes; the exception applies only to that generated crate.
        if manifest == "generated/semantic-behavior/Cargo.toml" {
            lint_args.extend([
                "-A",
                "unreachable_code",
                "-A",
                "clippy::unneeded_struct_pattern",
            ]);
        }
        step("Clippy generated", "cargo", &lint_args, &root)?;
    }
    step(
        "public documentation",
        "cargo",
        &["run", "--locked", "--bin", "codegate-docs", "--", "check"],
        &root,
    )?;
    Ok(())
}
fn main() -> ExitCode {
    Args::parse();
    match check() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("codegate-check: {e}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gate_clean_external_target() {
        let base = std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join(format!("scratch-regression-{}", std::process::id()));
        fs::create_dir(&base).unwrap();
        for layout in ["external", "ordinary"] {
            let root = base.join(layout);
            fs::create_dir(&root).unwrap();
            if layout == "ordinary" {
                fs::create_dir(root.join("target")).unwrap();
            } else {
                assert!(!root.join("target").exists());
            }
            let scratch = create_scratch(&root).expect("gate must create missing target parent");
            assert!(scratch.is_dir());
            assert_eq!(scratch.parent(), Some(root.join("target").as_path()));
            let evidence = scratch.join("report.json");
            fs::write(&evidence, b"retained prior evidence").unwrap();
            assert!(create_scratch(&root).is_err(), "must refuse scratch reuse");
            assert_eq!(fs::read(&evidence).unwrap(), b"retained prior evidence");
        }
        let root = base.join("blocked-parent");
        fs::create_dir(&root).unwrap();
        fs::write(root.join("target"), b"not a directory").unwrap();
        assert!(create_scratch(&root).is_err());
        assert_eq!(fs::read(root.join("target")).unwrap(), b"not a directory");
    }
}
