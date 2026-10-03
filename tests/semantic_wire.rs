use codegate::{semantic_model as b, semantic_wire as wire};
use serde_json::{Value, json};

fn fixture() -> Value {
    let range = json!({"path":"src/é.rs","start_byte":0,"end_byte":5,"start_line":0,"end_line":0,"start_column":0,"end_column":5});
    let gap = json!({"code":"UnresolvedSymbol","reason":"retained reason","location":range});
    let evidence = json!({"id":"e","snapshot_id":"snapshot","configuration_id":"config","producer":"fixture","producer_version":"1","location":range,"resolution":"SyntacticCandidate","basis":"SourceStructure","gaps":[gap]});
    let file = json!({"path":"src/é.rs","content_sha256":"digest","byte_length":5,"language":"Rust","classification":"Production","origin":"TrackedDirty","lines":[{"number":0,"start_byte":0,"end_byte":5,"classification":"Mixed"}]});
    let configuration = json!({"id":"config","modules":["module"],"target":"target","go_build_tags":["integration"],"rust_features":["feature"],"rust_default_features":true,"rust_cfg":["custom"],"java_target_version":"21","java_classpath_sha256":"classpath","build_profiles":["test"],"configuration_files":[file]});
    json!({
        "format":"codegate.semantic-facts/1",
        "source":{"id":"snapshot","selection":{"include_paths":["src"],"exclude_paths":["vendor"],"include_tests":true,"include_generated":false,"languages":["Rust","Go","Java"]},"configuration":configuration,"files":[file],"manifests":[{"path":"Cargo.toml","content_sha256":"manifest","system":"Cargo","module":".","parent_module":"parent"}]},
        "mode":"SourceOnly",
        "tools":[{"tool":"fixture","version":"1","host_runtime_version":"host","configuration_id":"config","snapshot_id":"snapshot","elapsed_milliseconds":3,"exit_code":0,"timed_out":false,"diagnostics":["diagnostic"]}],
        "coverage":[{"family":"Declarations","configuration_id":"config","unit_ids":["unit"],"status":"Partial","gaps":[gap],"applicability_reason":"reason"}],
        "units":[{"id":"unit","name":"module","language":"Rust","paths":["src/é.rs"],"evidence":evidence}],
        "declarations":[{"id":"decl","unit_id":"unit","parent":"parent-decl","name":"é","qualified_name":"module::é","signature":"fn é()","kind":"Function","visibility":"Public","api_eligibility":"Eligible","evidence":evidence}],
        "occurrences":[{"owner":"decl","spelling":"name","role":"Import","evidence":evidence}],
        "dependencies":[{"source":"unit","target":"other-unit","target_name":"other","kind":"Runtime","evidence":evidence}],
        "references":[{"occurrence_id":"occurrence","target":"other-decl","candidates":["candidate"],"evidence":evidence}],
        "calls":[{"caller":"decl","callee":"other-decl","candidates":["candidate"],"dynamic_dispatch":true,"evidence":evidence}],
        "implementations":[{"declaration":"decl","implementation":"other-decl","candidates":["candidate"],"evidence":evidence}],
        "decisions":[{"owner":"decl","kind":"RustTryPropagation","evidence":evidence}],
        "structure":[{"owner":"decl","kind":"TestCase","parent_observation":"parent-evidence","test_kind":"Parameterized","text":"retained text","evidence":evidence}],
        "effects":[{"owner":"decl","kind":"UnsafeOperation","related_declaration":"other-decl","evidence":evidence}],
        "framework":[{"kind":"RestRoute","declaration":"decl","target":"other-decl","annotation_name":"Path","qualifiers":[{"annotation_name":"Named","member_name":"value","member_value":"bean","nonbinding":true}],"scope":"ApplicationScoped","produced_type":"Bean","http_method":"GET","resource_path":"/resource","method_path":"/method","transaction_mode":"REQUIRED","configuration_key":"property","configuration_profile":"test","evidence":evidence}]
    })
}

#[test]
fn every_snapshot_family_and_transitive_field_round_trips() {
    let value = fixture();
    let snapshot = wire::decode_snapshot(&serde_json::to_vec(&value).unwrap()).unwrap();
    assert_eq!(snapshot.units[0].language, b::Language::Rust);
    assert_eq!(snapshot.coverage[0].status, b::Completeness::Partial);
    assert_eq!(
        snapshot.decisions[0].kind,
        b::DecisionKind::RustTryPropagation
    );
    assert_eq!(snapshot.effects[0].kind, b::EffectKind::UnsafeOperation);
    assert_eq!(snapshot.framework[0].kind, b::FrameworkKind::RestRoute);
    assert_eq!(
        wire::source_value(&snapshot.source).unwrap(),
        value["source"]
    );
    assert_eq!(
        wire::gaps_value(&snapshot.coverage[0].gaps).unwrap(),
        value["coverage"][0]["gaps"]
    );
    assert_eq!(wire::snapshot_value(&snapshot).unwrap(), value);
}

#[test]
fn requests_and_configuration_have_explicit_generated_bridges() {
    let snapshot = fixture();
    let request = json!({"root":"project","selection":snapshot["source"]["selection"],"configuration":snapshot["source"]["configuration"],"mode":"Semantic","timeout_milliseconds":9223372036854775807_i64});
    let typed = wire::decode_request(&serde_json::to_vec(&request).unwrap()).unwrap();
    assert_eq!(typed.mode, b::CollectionMode::Semantic);
    assert_eq!(typed.timeout_milliseconds, i64::MAX);
    assert_eq!(wire::request_value(&typed).unwrap(), request);
    let configuration = request["configuration"].clone();
    let typed = wire::decode_configuration(&serde_json::to_vec(&configuration).unwrap()).unwrap();
    assert_eq!(wire::configuration_value(&typed).unwrap(), configuration);
}

#[test]
fn optional_null_is_absence_but_required_or_unknown_null_is_refused() {
    let mut value = fixture();
    value["declarations"][0]["parent"] = Value::Null;
    value["source"]["files"][0]["language"] = Value::Null;
    let snapshot = wire::decode_snapshot(&serde_json::to_vec(&value).unwrap()).unwrap();
    assert!(snapshot.declarations[0].parent.is_none());
    assert!(snapshot.source.files[0].language.is_none());
    let projected = wire::snapshot_value(&snapshot).unwrap();
    assert!(projected["declarations"][0].get("parent").is_none());
    assert!(projected["source"]["files"][0].get("language").is_none());
    value["units"][0]["language"] = Value::Null;
    assert!(wire::decode_snapshot(&serde_json::to_vec(&value).unwrap()).is_err());
    let mut value = fixture();
    value["source"]["unexpected"] = Value::Null;
    assert!(wire::decode_snapshot(&serde_json::to_vec(&value).unwrap()).is_err());
}

#[test]
fn malformed_unknown_duplicate_enums_and_integer_overflow_are_refused() {
    let bytes = serde_json::to_string(&fixture()).unwrap();
    for bad in [
        bytes.replacen(
            "\"mode\":\"SourceOnly\"",
            "\"mode\":\"SourceOnly\",\"mode\":\"Semantic\"",
            1,
        ),
        bytes.replacen("\"start_byte\":0", "\"start_byte\":0,\"start_byte\":1", 1),
        format!("{bytes} null"),
    ] {
        assert!(wire::decode_snapshot(bad.as_bytes()).is_err());
    }
    for (pointer, bad) in [
        ("/mode", json!("future")),
        (
            "/source/files/0/byte_length",
            json!(18446744073709551615_u64),
        ),
        ("/source/files/0/byte_length", json!(1.5)),
        ("/source/files/0/byte_length", json!("5")),
        (
            "/source/files/0/byte_length",
            json!({"$serde_json::private::Number":"5"}),
        ),
        ("/source/unexpected", json!(true)),
    ] {
        let mut value = fixture();
        if pointer == "/source/unexpected" {
            value["source"]["unexpected"] = bad;
        } else {
            *value.pointer_mut(pointer).unwrap() = bad;
        }
        assert!(
            wire::decode_snapshot(&serde_json::to_vec(&value).unwrap()).is_err(),
            "{pointer}"
        );
    }
    assert!(wire::decode_snapshot(&vec![b' '; 4 * 1024 * 1024 + 1]).is_err());
    assert!(
        wire::decode_snapshot(format!("{}0{}", "[".repeat(200), "]".repeat(200)).as_bytes())
            .is_err()
    );
}
