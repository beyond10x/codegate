use codegate::{
    model::*,
    wire::{self, Bridge},
};
use codegate_contract as w;
use serde_json::{Value, json};
fn snapshot() -> Value {
    serde_json::from_str::<Value>(include_str!("../ess/scenarios/one-runtime-edge.yaml")).unwrap()["timeline"][0]["input"]["snapshot"].clone()
}
#[test]
fn strict_json_structure() {
    let baseline = snapshot();
    for raw in ["{\"format\":1,\"format\":2}", "{", "null", "[]", "true"] {
        assert!(wire::decode_snapshot(raw.as_bytes()).is_err(), "{raw}");
    }
    for (pointer, value) in [
        ("/units/0/id", json!(3)),
        ("/coverage/status", json!("Unknown")),
        ("/edges/0/target", json!([])),
        ("/edges/0/unresolved_target", json!(42)),
        ("/format", Value::Null),
    ] {
        let mut v = baseline.clone();
        *v.pointer_mut(pointer).unwrap() = value;
        assert!(
            wire::decode_snapshot(&serde_json::to_vec(&v).unwrap()).is_err(),
            "{pointer}"
        );
    }
    let mut v = baseline.clone();
    v["units"][0]["extra"] = json!(true);
    assert!(wire::decode_snapshot(&serde_json::to_vec(&v).unwrap()).is_err());
    let mut v = baseline;
    v.as_object_mut().unwrap().remove("producer");
    assert!(wire::decode_snapshot(&serde_json::to_vec(&v).unwrap()).is_err());
    let raw = serde_json::to_string(&snapshot())
        .unwrap()
        .replace("\"id\":\"core\"", "\"id\":\"core\",\"id\":\"other\"");
    assert!(
        wire::decode_snapshot(raw.as_bytes())
            .unwrap_err()
            .contains("duplicate")
    );
}
#[test]
fn explicit_absence_bridge() {
    let mut v = snapshot();
    let null = wire::decode_snapshot(&serde_json::to_vec(&v).unwrap()).unwrap();
    v["edges"][0]
        .as_object_mut()
        .unwrap()
        .remove("unresolved_target");
    assert_eq!(
        wire::decode_snapshot(&serde_json::to_vec(&v).unwrap()).unwrap(),
        null
    );
    let projected = w::CodegateDependencyFactSnapshot::from_behavior(null.clone());
    assert_eq!(projected.into_behavior().unwrap(), null);
}
#[test]
fn decoder_limits() {
    assert!(
        wire::decode_snapshot(&vec![b' '; wire::MAX_DOCUMENT_BYTES + 1])
            .unwrap_err()
            .contains("4 MiB")
    );
    let mut v = snapshot();
    v["units"] = json!(vec![
        json!({"id":"x","language":"rust"});
        wire::MAX_UNITS + 1
    ]);
    assert!(
        wire::decode_snapshot(&serde_json::to_vec(&v).unwrap())
            .unwrap_err()
            .contains("unit limit")
    );
    let mut v = snapshot();
    v["edges"] = json!(vec![json!({}); wire::MAX_EDGES + 1]);
    assert!(
        wire::decode_snapshot(&serde_json::to_vec(&v).unwrap())
            .unwrap_err()
            .contains("edge limit")
    );
}
#[test]
fn generated_integer_obligation_is_checked() {
    for invalid in ["1.5", "-1", "10001", "9223372036854775808"] {
        let n: serde_json::Number = serde_json::from_str(invalid).unwrap();
        assert!(
            <serde_json::Number as Bridge<i64>>::into_behavior(n).is_err(),
            "{invalid}"
        );
    }
    for valid in [0, 1, 10000] {
        let n = <serde_json::Number as Bridge<i64>>::from_behavior(valid);
        assert_eq!(n.into_behavior().unwrap(), valid);
    }
}
#[test]
fn all_generated_fields_roundtrip() {
    for entry in std::fs::read_dir("ess/scenarios").unwrap() {
        let f: Value =
            serde_json::from_slice(&std::fs::read(entry.unwrap().path()).unwrap()).unwrap();
        let i = &f["timeline"][0]["input"];
        let s = wire::decode_snapshot(&serde_json::to_vec(&i["snapshot"]).unwrap()).unwrap();
        assert_eq!(
            w::CodegateDependencyFactSnapshot::from_behavior(s.clone())
                .into_behavior()
                .unwrap(),
            s
        );
        let p = wire::decode_policy(&serde_json::to_vec(&i["policy"]).unwrap()).unwrap();
        assert_eq!(
            w::CodegateDependencyPolicy::from_behavior(p.clone())
                .into_behavior()
                .unwrap(),
            p
        );
        let report = codegate::evaluate(Evaluate {
            snapshot: s,
            policy: p,
        })
        .report;
        assert_eq!(
            w::CodegateDependencyEvaluation::from_behavior(report.clone())
                .into_behavior()
                .unwrap(),
            report
        );
    }
}
