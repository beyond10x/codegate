use codegate::{
    Evaluator,
    model::{Evaluate, obligations::EvaluateBehavior},
    wire,
};
use ess_conformance::{
    AdmittedSuite, Clock, CountReport, Ids, Runner, RunnerConfig, report::Status, target::*,
};
use ess_primitives::node::Node;
use std::{collections::BTreeMap, path::PathBuf, process::Command};

struct CodegateTarget;
// Evidence uses observation time. The evaluator and scenario inputs remain pure;
// these scenarios contain no eventual observations that need a synthetic clock.
struct ObservedClock;
impl Clock for ObservedClock {
    fn now(&mut self) -> ess_primitives::time::Timestamp {
        let millis = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock before epoch")
            .as_millis();
        ess_primitives::time::Timestamp::from_epoch_millis(
            u64::try_from(millis).expect("clock overflow"),
        )
    }
}
impl ConformanceTarget for CodegateTarget {
    fn identity(&self) -> Result<ImplementationIdentity, TargetError> {
        Ok(ImplementationIdentity::new(
            "codegate-offline",
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
        if request.command.to_string() != "codegate.dependency.Evaluate" {
            return Err(TargetError::unsupported("command", "unknown command"));
        }
        let decode = |detail: String| TargetError::unavailable("Evaluate", detail);
        let snapshot = serde_json::to_vec(
            request
                .input
                .get("snapshot")
                .ok_or_else(|| decode("missing snapshot".into()))?,
        )
        .map_err(|e| decode(e.to_string()))?;
        let policy = serde_json::to_vec(
            request
                .input
                .get("policy")
                .ok_or_else(|| decode("missing policy".into()))?,
        )
        .map_err(|e| decode(e.to_string()))?;
        let input = Evaluate {
            snapshot: wire::decode_snapshot(&snapshot).map_err(decode)?,
            policy: wire::decode_policy(&policy).map_err(decode)?,
        };
        let mut evaluator = Evaluator::default();
        evaluator
            .evaluate(input)
            .map_err(|e| decode(format!("{e:?}")))?;
        let response = evaluator
            .take_response()
            .ok_or_else(|| decode("missing actual response".into()))?;
        let report = wire::report_value(response.report).map_err(decode)?;
        let actual: Node = serde_json::from_value(report).map_err(|e| decode(e.to_string()))?;
        let mut result = SemanticCommandResult::took(ess_conformance::scenario::OutcomeRef::new(
            request.command,
            "evaluated".parse().unwrap(),
        ));
        result.response = Some(BTreeMap::from([("report".into(), actual)]));
        Ok(result)
    }
    fn query_view(&self, _: SemanticViewRequest) -> Result<SemanticViewResult, TargetError> {
        Err(TargetError::unsupported("query", "no views declared"))
    }
    fn observe_events(
        &self,
        _: EventObservationRequest,
    ) -> Result<Vec<ObservedEvent>, TargetError> {
        Err(TargetError::unsupported("events", "no events declared"))
    }
    fn configure_external_outcome(&self, _: ExternalOutcomeControl) -> Result<(), TargetError> {
        Err(TargetError::unsupported(
            "external",
            "no external outcomes declared",
        ))
    }
    fn redeliver_event(&self, _: RedeliveryRequest) -> Result<(), TargetError> {
        Err(TargetError::unsupported("redelivery", "no events declared"))
    }
}
#[test]
fn real_evaluator_conforms_to_complete_combined_suite() {
    let scratch = std::env::var_os("CODEGATE_CONFORMANCE_OUTPUT")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target/conformance"));
    std::fs::create_dir_all(&scratch).unwrap();
    let suite = scratch.join("suite.json");
    let result = Command::new("ess")
        .args([
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
        ])
        .arg(&suite)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let admitted = AdmittedSuite::from_json(&std::fs::read_to_string(&suite).unwrap()).unwrap();
    let run = Runner::new(
        RunnerConfig::default(),
        ObservedClock,
        Ids::for_suite(admitted.suite()),
    )
    .run_admitted(&admitted, &CodegateTarget);
    let report = CountReport::from_run(&run, &admitted).unwrap();
    let report_text = report.to_canonical_json().unwrap();
    std::fs::write(scratch.join("report.json"), &report_text).unwrap();
    std::fs::write(
        scratch.join("run.json"),
        serde_json::to_vec_pretty(&run.scenarios).unwrap(),
    )
    .unwrap();
    println!("{report_text}");
    assert_eq!(
        run.scenarios.len(),
        27,
        "complete combined suite cardinality changed"
    );
    for scenario in &run.scenarios {
        assert_eq!(scenario.status, Status::Passed, "{scenario:?}");
    }
    let value: serde_json::Value = serde_json::from_str(&report_text).unwrap();
    assert_eq!(value["execution_status"], "passed");
    assert_eq!(value["conformance_status"], "passed");
}
