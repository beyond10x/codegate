use codegate::model::obligations::EvaluateBehavior;
use codegate::{Evaluator, model::*};
#[test]
fn evaluate_obligation_is_fulfilled() {
    let input = Evaluate {
        snapshot: FactSnapshot {
            format: "codegate-dependency-facts/0.1".into(),
            source_id: "source".into(),
            configuration_id: "default".into(),
            producer: "fixture".into(),
            producer_version: "1".into(),
            units: vec![],
            edges: vec![],
            coverage: Coverage {
                status: CoverageStatus::Complete,
                gaps: vec![],
            },
        },
        policy: Policy {
            format: "codegate-dependency-policy/0.1".into(),
            expected_source_id: "source".into(),
            expected_configuration_id: "default".into(),
            dependency_kind: DependencyKind::Runtime,
            forbidden: vec![],
        },
    };
    assert_eq!(
        Evaluator::default().evaluate(input),
        Ok(EvaluateOutcome::Evaluated)
    );
}

#[test]
fn trait_adapter_never_leaks_a_previous_response() {
    use codegate::wire;
    let mut evaluator = Evaluator::default();
    for name in [
        "one-runtime-edge",
        "failed-collection",
        "complete-empty-graph",
    ] {
        let f: serde_json::Value =
            serde_json::from_slice(&std::fs::read(format!("ess/scenarios/{name}.yaml")).unwrap())
                .unwrap();
        let input = &f["timeline"][0]["input"];
        evaluator
            .evaluate(Evaluate {
                snapshot: wire::decode_snapshot(&serde_json::to_vec(&input["snapshot"]).unwrap())
                    .unwrap(),
                policy: wire::decode_policy(&serde_json::to_vec(&input["policy"]).unwrap())
                    .unwrap(),
            })
            .unwrap();
        assert_eq!(
            wire::report_value(evaluator.take_response().unwrap().report).unwrap(),
            f["timeline"][0]["response"]["report"]
        );
        assert!(evaluator.take_response().is_none());
    }
}
