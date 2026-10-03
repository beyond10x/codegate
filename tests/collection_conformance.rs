//! Real, component-scoped ESS execution over collection and offline admission.
use codegate::semantic_model::obligations::CollectBehavior;
use codegate::{Collector, semantic_model::*, semantic_wire};
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
    FalseComplete,
    ConstantIdentity,
    SemanticDowngrade,
}
struct CollectionTarget {
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
            .ok_or("collection input number is not an exact i64")?
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
impl ConformanceTarget for CollectionTarget {
    fn fixture_values(
        &self,
        _: &ScenarioContext,
        _: &ess_conformance::fixtures::Contract,
    ) -> Result<BTreeMap<String, Node>, TargetError> {
        // A declared pre-execution fixture supplies the generated structural
        // witness. Authored literal inputs are never rewritten.
        let request = serde_json::json!({
            "root": "fixture:tri-language",
            "selection": {"include_paths": ["Main.java", "main.go", "main.rs"], "exclude_paths": [], "include_tests": true, "include_generated": true, "languages": []},
            "configuration": {"modules": [], "go_build_tags": [], "rust_features": [], "rust_default_features": false, "rust_cfg": [], "build_profiles": [], "configuration_files": [], "id": ""},
            "mode": "SourceOnly", "timeout_milliseconds": 10000
        });
        Ok(BTreeMap::from([(
            "source-collection-request".into(),
            serde_json::from_value(request).unwrap(),
        )]))
    }
    fn identity(&self) -> Result<ImplementationIdentity, TargetError> {
        Ok(ImplementationIdentity::new(
            "codegate-source-collection",
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
        let failure = |message: String| TargetError::unavailable("collection", message);
        if request.command.to_string() != "codegate_semantic.semantic.Collect" {
            return Err(TargetError::unsupported(
                "command",
                "outside collection component",
            ));
        }
        let input = input_value(
            request
                .input
                .get("request")
                .ok_or_else(|| failure("missing request".into()))?,
        )
        .map_err(failure)?;
        let mut value =
            semantic_wire::decode_request(&serde_json::to_vec(&input).unwrap()).map_err(failure)?;
        // Only test fixture roots are translated. Selection, mode and build
        // configuration flow unchanged into the production generated obligation.
        if let Some(name) = value.root.strip_prefix("fixture:") {
            if name.contains('/') || name.contains('\\') || name == ".." {
                return Err(failure("invalid fixture identifier".into()));
            }
            value.root = self.fixtures.join(name).to_str().unwrap().into();
        }
        let mut handler = Collector::default();
        let outcome = handler
            .collect(Collect { request: value })
            .map_err(|e| failure(format!("{e:?}")))?;
        assert!(matches!(outcome, CollectOutcome::Collected));
        let response = handler
            .take_response()
            .ok_or_else(|| failure("missing actual collection response".into()))?;
        // Admit the exact returned snapshot, rather than an expected fixture or
        // reconstructed substitute. This includes failed source-capture responses.
        codegate::semantic::validate_snapshot(&response.snapshot)
            .map_err(|gaps| failure(format!("actual snapshot failed admission: {gaps:?}")))?;
        let mut snapshot = semantic_wire::snapshot_value(&response.snapshot).map_err(failure)?;
        match self.fault {
            Fault::FalseComplete => {
                for coverage in snapshot["coverage"].as_array_mut().unwrap() {
                    coverage["status"] = serde_json::json!("Complete");
                    coverage["gaps"] = serde_json::json!([]);
                }
            }
            Fault::ConstantIdentity => snapshot["source"]["id"] = serde_json::json!("0".repeat(64)),
            Fault::SemanticDowngrade => snapshot["mode"] = serde_json::json!("SourceOnly"),
            _ => {}
        }
        let mut result = SemanticCommandResult::took(ess_conformance::scenario::OutcomeRef::new(
            request.command,
            "collected".parse().unwrap(),
        ));
        result.response = Some(if matches!(self.fault, Fault::EmptyResponse) {
            BTreeMap::new()
        } else {
            BTreeMap::from([(
                "snapshot".into(),
                serde_json::from_value::<Node>(snapshot).map_err(|e| failure(e.to_string()))?,
            )])
        });
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
    let directory = root.join("tri-language");
    std::fs::create_dir_all(&directory).unwrap();
    // Fixture hashes and source identities in authored scenarios were derived
    // independently with canonical sorted JSON and sha256sum, not the collector.
    for (name, source) in [
        ("Main.java", "class Main {}\n"),
        ("main.go", "package main\n"),
        ("main.rs", "fn main() {}\n"),
    ] {
        std::fs::write(directory.join(name), source).unwrap();
    }
    root
}
#[test]
fn real_collection_conforms_to_selected_component() {
    let output = std::env::var_os("CODEGATE_COLLECTION_OUTPUT")
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!(
                "target/collection-conformance-{}",
                std::process::id()
            ))
        });
    std::fs::create_dir_all(&output).unwrap();
    let fixtures = prepare_fixtures(&output);
    let target = CollectionTarget {
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
            "collection",
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
        4,
        "three authored and one structural collection cases required"
    );
    let expected_ids = [
        "codegate_semantic.semantic.Collect/outcome/collected",
        "codegate_semantic.semantic/authored/parity-collect-source-only-foundation",
        "codegate_semantic.semantic/authored/parity-collect-semantic-gap",
        "codegate_semantic.semantic/authored/parity-collect-capture-failure",
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
        (
            Fault::EmptyResponse,
            "parity-collect-source-only-foundation",
        ),
        (Fault::EmptyResponse, "parity-collect-capture-failure"),
        (
            Fault::FalseComplete,
            "parity-collect-source-only-foundation",
        ),
        (Fault::FalseComplete, "parity-collect-semantic-gap"),
        (
            Fault::ConstantIdentity,
            "parity-collect-source-only-foundation",
        ),
        (Fault::ConstantIdentity, "parity-collect-capture-failure"),
        (Fault::SemanticDowngrade, "parity-collect-semantic-gap"),
    ] {
        let broken = CollectionTarget {
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
                    Fault::FalseComplete => "coverage",
                    Fault::SemanticDowngrade => "mode",
                    _ => "identity",
                }
            )),
            serde_json::to_vec_pretty(&run.scenarios).unwrap(),
        )
        .unwrap();
    }
}
