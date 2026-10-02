use codegate::{evaluate, model::*, wire};
use serde_json::{Value, json};
fn fixture(name: &str) -> Value {
    serde_json::from_str(&std::fs::read_to_string(format!("ess/scenarios/{name}.yaml")).unwrap())
        .unwrap()
}
fn run(input: &Value) -> Value {
    wire::report_value(
        evaluate(Evaluate {
            snapshot: wire::decode_snapshot(&serde_json::to_vec(&input["snapshot"]).unwrap())
                .unwrap(),
            policy: wire::decode_policy(&serde_json::to_vec(&input["policy"]).unwrap()).unwrap(),
        })
        .report,
    )
    .unwrap()
}
#[test]
fn every_authored_literal_matches_real_evaluation() {
    let mut count = 0;
    for entry in std::fs::read_dir("ess/scenarios").unwrap() {
        let scenario: Value =
            serde_json::from_slice(&std::fs::read(entry.unwrap().path()).unwrap()).unwrap();
        let step = &scenario["timeline"][0];
        assert_eq!(
            run(&step["input"]),
            step["response"]["report"],
            "{}",
            scenario["scenario"]
        );
        count += 1;
    }
    assert_eq!(count, 26);
}
#[test]
fn dangling_target_guard() {
    let f = fixture("dangling-target");
    assert_eq!(
        run(&f["timeline"][0]["input"])["diagnostics"][0]["code"],
        "DanglingTarget"
    );
}
#[test]
fn partial_is_never_complete() {
    let f = fixture("partial-is-not-empty");
    let report = run(&f["timeline"][0]["input"]);
    assert_eq!(report["verdict"], "Incomplete");
    assert!(report["fan_out"][0]["value"].is_null());
}
#[test]
fn parallel_imports_count_unique_destinations() {
    let f = fixture("parallel-imports-count-once");
    assert_eq!(run(&f["timeline"][0]["input"])["fan_out"][0]["value"], 1);
}
#[test]
fn diagnostic_phase_precedence_and_order() {
    let mut input = fixture("one-runtime-edge")["timeline"][0]["input"].clone();
    input["snapshot"]["format"] = json!("bad");
    input["policy"]["format"] = json!("bad");
    input["snapshot"]["source_id"] = json!("");
    assert_eq!(
        run(&input)["diagnostics"],
        json!([{"code":"InvalidFormat","subject":"policy.format"},{"code":"InvalidFormat","subject":"snapshot.format"}])
    );
    input["snapshot"]["format"] = json!("codegate-dependency-facts/0.1");
    input["policy"]["format"] = json!("codegate-dependency-policy/0.1");
    input["snapshot"]["edges"][0]["target"] = json!("missing");
    assert_eq!(
        run(&input)["diagnostics"],
        json!([{"code":"EmptyIdentity","subject":"snapshot.source_id"}])
    );
    input["snapshot"]["source_id"] = json!("fixture-source");
    input["snapshot"]["edges"][0]["source"] = json!("missing");
    assert_eq!(
        run(&input)["diagnostics"],
        json!([{"code":"DanglingSource","subject":"edge:e1"},{"code":"DanglingTarget","subject":"edge:e1"}])
    );
}
#[test]
fn semantic_invalid_classes() {
    let baseline = fixture("forbidden-runtime-edge")["timeline"][0]["input"].clone();
    for (pointer, value, code) in [
        ("/policy/forbidden/0/kind", json!("Build"), "InvalidPolicy"),
        ("/policy/expected_source_id", json!(""), "InvalidPolicy"),
        ("/snapshot/edges/0/target", Value::Null, "InvalidTarget"),
        ("/snapshot/coverage/gaps", json!([""]), "InvalidCoverage"),
        (
            "/snapshot/coverage",
            json!({"status":"Failed","gaps":["failure"]}),
            "InvalidCoverage",
        ),
    ] {
        let mut input = baseline.clone();
        *input.pointer_mut(pointer).unwrap() = value;
        assert_eq!(run(&input)["diagnostics"][0]["code"], code, "{pointer}");
    }
    let mut input = baseline;
    let duplicate = input["policy"]["forbidden"][0].clone();
    input["policy"]["forbidden"]
        .as_array_mut()
        .unwrap()
        .push(duplicate);
    assert_eq!(run(&input)["diagnostics"][0]["code"], "InvalidPolicy");
}
#[test]
fn labels_and_input_order_do_not_choose_algorithms() {
    let mut input = fixture("canonical-order")["timeline"][0]["input"].clone();
    let expected = run(&input);
    for unit in input["snapshot"]["units"].as_array_mut().unwrap() {
        unit["language"] = json!("arbitrary-no-binding");
    }
    input["snapshot"]["producer"] = json!("unknown-producer");
    input["snapshot"]["units"].as_array_mut().unwrap().reverse();
    input["snapshot"]["edges"].as_array_mut().unwrap().reverse();
    input["policy"]["forbidden"]
        .as_array_mut()
        .unwrap()
        .reverse();
    assert_eq!(run(&input), expected);
}
#[test]
fn core_module_boundary_has_no_io_or_language_dispatch() {
    for path in ["src/admit.rs", "src/analysis.rs", "src/check.rs"] {
        let source = std::fs::read_to_string(path).unwrap();
        for forbidden in [
            "std::fs",
            "std::env",
            "std::net",
            "std::process",
            "SystemTime",
            ".language",
            "binding::",
        ] {
            assert!(!source.contains(forbidden), "{path} contains {forbidden}");
        }
    }
}
