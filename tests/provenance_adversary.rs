use std::{
    process::Command,
    time::{SystemTime, UNIX_EPOCH},
};
#[test]
fn native_report_records_observed_completion_time() {
    let started = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    let run = Command::new("cargo")
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
        .output()
        .unwrap();
    assert!(
        run.status.success(),
        "{}",
        String::from_utf8_lossy(&run.stderr)
    );
    let ended = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    let report: serde_json::Value =
        serde_json::from_slice(&std::fs::read("target/conformance/report.json").unwrap()).unwrap();
    let completed = report["completed_at"].as_u64().unwrap();
    assert!(
        (started..=ended).contains(&completed),
        "native evidence completion {completed} is outside actual run [{started}, {ended}]"
    );
}
