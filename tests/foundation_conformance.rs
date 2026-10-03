//! Real, component-scoped ESS execution over collection and offline admission.
use codegate::semantic_model::obligations::{CollectSourceBehavior, ValidateSnapshotBehavior};
use codegate::{foundation::Foundation, semantic_model::*, semantic_wire};
use ess_conformance::{
    AdmittedSuite, Clock, CountReport, Ids, Runner, RunnerConfig, report::Status, target::*,
};
use ess_primitives::node::Node;
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    process::Command,
};

struct ObservedClock;
impl Clock for ObservedClock {
    fn now(&mut self) -> ess_primitives::time::Timestamp {
        let millis = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        ess_primitives::time::Timestamp::from_epoch_millis(u64::try_from(millis).unwrap())
    }
}

#[derive(Clone, Copy)]
enum Fault {
    None,
    EmptyResponse,
    AdmitEverything,
    ConstantIdentity,
}
struct FoundationTarget {
    fixtures: PathBuf,
    fault: Fault,
}
// ESS Number serializes ordinary integral values as binary64. The wire contract
// requires integer tokens, so preserve its exact i64 value explicitly at this
// test transport boundary instead of weakening the product decoder.
fn input_value(node: &Node) -> Result<serde_json::Value, String> {
    Ok(match node {
        Node::Null => serde_json::Value::Null,
        Node::Bool(value) => (*value).into(),
        Node::Text(value) => value.clone().into(),
        Node::Number(value) => value
            .as_i64()
            .ok_or("foundation input number is not an exact i64")?
            .into(),
        Node::Seq(values) => values
            .iter()
            .map(input_value)
            .collect::<Result<Vec<_>, _>>()?
            .into(),
        Node::Map(values) => serde_json::Value::Object(
            values
                .iter()
                .map(|(key, value)| input_value(value).map(|value| (key.clone(), value)))
                .collect::<Result<_, _>>()?,
        ),
    })
}
impl ConformanceTarget for FoundationTarget {
    fn identity(&self) -> Result<ImplementationIdentity, TargetError> {
        Ok(ImplementationIdentity::new(
            "codegate-source-foundation",
            env!("CARGO_PKG_VERSION"),
        ))
    }
    fn begin_scenario(&self, _: &ScenarioContext) -> Result<(), TargetError> {
        Ok(())
    }
    fn end_scenario(&self, _: &ScenarioContext) -> Result<(), TargetError> {
        Ok(())
    }
    fn execute_command(
        &self,
        request: SemanticCommandRequest,
    ) -> Result<SemanticCommandResult, TargetError> {
        let failure = |message: String| TargetError::unavailable("foundation", message);
        let input = |key: &str| -> Result<Vec<u8>, TargetError> {
            serde_json::to_vec(
                &input_value(
                    request
                        .input
                        .get(key)
                        .ok_or_else(|| failure(format!("missing {key}")))?,
                )
                .map_err(failure)?,
            )
            .map_err(|e| failure(e.to_string()))
        };
        let mut handler = Foundation::default();
        let (outcome, mut response) = match request.command.to_string().as_str() {
            "codegate_semantic.semantic.CollectSource" => {
                let mut value =
                    semantic_wire::decode_request(&input("request")?).map_err(failure)?;
                // Fixture URI resolution is test infrastructure. The actual
                // selection/configuration reaches the real collector unchanged.
                if let Some(name) = value.root.strip_prefix("fixture:") {
                    if name.contains('/') || name.contains('\\') || name == ".." {
                        return Err(failure("invalid fixture identifier".into()));
                    }
                    value.root = self.fixtures.join(name).to_str().unwrap().into();
                }
                handler
                    .collect_source(CollectSource { request: value })
                    .map_err(|e| failure(format!("{e:?}")))?;
                let result = handler
                    .take_source_response()
                    .ok_or_else(|| failure("missing actual collection response".into()))?;
                let mut response = BTreeMap::from([(
                    "gaps".into(),
                    semantic_wire::gaps_value(&result.gaps).map_err(failure)?,
                )]);
                if let Some(source) = result.source {
                    response.insert(
                        "source".into(),
                        semantic_wire::source_value(&source).map_err(failure)?,
                    );
                }
                ("source-observed", response)
            }
            "codegate_semantic.semantic.ValidateSnapshot" => {
                let snapshot =
                    semantic_wire::decode_snapshot(&input("snapshot")?).map_err(failure)?;
                handler
                    .validate_snapshot(ValidateSnapshot { snapshot })
                    .map_err(|e| failure(format!("{e:?}")))?;
                let result = handler
                    .take_validation_response()
                    .ok_or_else(|| failure("missing actual admission response".into()))?;
                (
                    "validation-observed",
                    BTreeMap::from([
                        ("accepted".into(), serde_json::json!(result.accepted)),
                        (
                            "gaps".into(),
                            semantic_wire::gaps_value(&result.gaps).map_err(failure)?,
                        ),
                    ]),
                )
            }
            _ => {
                return Err(TargetError::unsupported(
                    "command",
                    "outside source-foundation component",
                ));
            }
        };
        // Test-only seeded faults verify that the authored expectations observe
        // behavior, rather than merely accepting an outcome name.
        match self.fault {
            Fault::None => {}
            Fault::EmptyResponse => response.clear(),
            Fault::AdmitEverything if response.contains_key("accepted") => {
                response.insert("accepted".into(), serde_json::json!(true));
                response.insert("gaps".into(), serde_json::json!([]));
            }
            Fault::ConstantIdentity => {
                if let Some(source) = response.get_mut("source") {
                    source["id"] = serde_json::json!("0".repeat(64));
                }
            }
            _ => {}
        }
        let mut result = SemanticCommandResult::took(ess_conformance::scenario::OutcomeRef::new(
            request.command,
            outcome.parse().unwrap(),
        ));
        result.response = Some(
            response
                .into_iter()
                .map(|(key, value)| {
                    serde_json::from_value::<Node>(value)
                        .map(|value| (key, value))
                        .map_err(|e| failure(e.to_string()))
                })
                .collect::<Result<_, _>>()?,
        );
        Ok(result)
    }
    fn query_view(&self, _: SemanticViewRequest) -> Result<SemanticViewResult, TargetError> {
        Err(TargetError::unsupported("view", "none declared"))
    }
    fn observe_events(
        &self,
        _: EventObservationRequest,
    ) -> Result<Vec<ObservedEvent>, TargetError> {
        Err(TargetError::unsupported("events", "none declared"))
    }
    fn configure_external_outcome(&self, _: ExternalOutcomeControl) -> Result<(), TargetError> {
        Err(TargetError::unsupported("external", "none declared"))
    }
    fn redeliver_event(&self, _: RedeliveryRequest) -> Result<(), TargetError> {
        Err(TargetError::unsupported("redelivery", "none declared"))
    }
}

fn prepare_fixtures(output: &Path) -> PathBuf {
    let root = output.join("fixtures");
    std::fs::create_dir_all(&root).unwrap();
    for (name, source) in [
        ("original", "fn f() {}\n"),
        ("same", "fn f() {}\n"),
        ("changed", "fn g() {}\n"),
    ] {
        let dir = root.join(name);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("main.rs"), source).unwrap();
    }
    for (name, version) in [("manifest", "0.1.0"), ("manifest-changed", "0.2.0")] {
        let dir = root.join(name);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(dir.join("main.rs"), "fn f() {}\n").unwrap();
        std::fs::write(
            dir.join("Cargo.toml"),
            format!("[package]\nname = \"fixture\"\nversion = \"{version}\"\n"),
        )
        .unwrap();
    }
    root
}

#[test]
fn real_foundation_conforms_to_selected_component() {
    let output = std::env::var_os("CODEGATE_FOUNDATION_OUTPUT")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!(
                "target/foundation-conformance-{}",
                std::process::id()
            ))
        });
    std::fs::create_dir_all(&output).unwrap();
    let fixtures = prepare_fixtures(&output);
    let target = FoundationTarget {
        fixtures: fixtures.clone(),
        fault: Fault::None,
    };
    let suite = output.join("suite.json");
    let generated = Command::new("ess")
        .args([
            "verify",
            "conform",
            "synthesize",
            "--path",
            "ess-semantic",
            "--scenarios",
            "ess-semantic",
            "--component",
            "source-foundation",
            "--suite-format",
            "5",
            "--out",
        ])
        .arg(&suite)
        .output()
        .unwrap();
    assert!(
        generated.status.success(),
        "{}",
        String::from_utf8_lossy(&generated.stderr)
    );
    let admitted = AdmittedSuite::from_json(&std::fs::read_to_string(&suite).unwrap()).unwrap();
    let run = Runner::new(
        RunnerConfig::default(),
        ObservedClock,
        Ids::for_suite(admitted.suite()),
    )
    .run_admitted(&admitted, &target);
    let report = CountReport::from_run(&run, &admitted)
        .unwrap()
        .to_canonical_json()
        .unwrap();
    std::fs::write(output.join("report.json"), &report).unwrap();
    std::fs::write(
        output.join("run.json"),
        serde_json::to_vec_pretty(&run.scenarios).unwrap(),
    )
    .unwrap();
    println!("{report}");
    assert_eq!(
        run.scenarios.len(),
        8,
        "six authored and two structural foundation cases required"
    );
    let expected_ids = [
        "codegate_semantic.semantic.CollectSource/outcome/source-observed",
        "codegate_semantic.semantic.ValidateSnapshot/outcome/validation-observed",
        "codegate_semantic.semantic/authored/parity-selected-content-identity",
        "codegate_semantic.semantic/authored/parity-manifest-identity",
        "codegate_semantic.semantic/authored/parity-path-confinement",
        "codegate_semantic.semantic/authored/parity-stale-evidence",
        "codegate_semantic.semantic/authored/parity-dangling-observation",
        "codegate_semantic.semantic/authored/parity-coverage-not-zero",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        run.scenarios
            .iter()
            .map(|case| case.scenario.to_string())
            .collect::<std::collections::BTreeSet<_>>(),
        expected_ids
    );
    for scenario in &run.scenarios {
        assert_eq!(scenario.status, Status::Passed, "{scenario:?}");
    }
    for (fault, expected_case) in [
        (Fault::EmptyResponse, "parity-selected-content-identity"),
        (Fault::AdmitEverything, "parity-stale-evidence"),
        (Fault::ConstantIdentity, "parity-selected-content-identity"),
    ] {
        let broken = FoundationTarget {
            fixtures: fixtures.clone(),
            fault,
        };
        let run = Runner::new(
            RunnerConfig::default(),
            ObservedClock,
            Ids::for_suite(admitted.suite()),
        )
        .run_admitted(&admitted, &broken);
        let matched = run
            .scenarios
            .iter()
            .find(|case| case.scenario.to_string().ends_with(expected_case))
            .unwrap();
        assert_eq!(
            matched.status,
            Status::Failed,
            "seeded fault escaped {expected_case}"
        );
        std::fs::write(
            output.join(format!(
                "mutation-{expected_case}-{}.json",
                match fault {
                    Fault::EmptyResponse => "empty",
                    Fault::AdmitEverything => "admit",
                    _ => "identity",
                }
            )),
            serde_json::to_vec_pretty(&run.scenarios).unwrap(),
        )
        .unwrap();
    }
}
