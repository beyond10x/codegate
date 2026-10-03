//! Independently authored ESS identity literals must match pure identity code.
//! The literals were calculated from canonical JSON using sha256sum, not from
//! collection or source_identity output.
use codegate::{semantic_wire, source_identity};

#[test]
fn independent_ess_identity_literals_bind_selected_content_and_manifests() {
    for text in [
        include_str!("../ess-semantic/scenarios/parity-selected-content-identity.yaml"),
        include_str!("../ess-semantic/scenarios/parity-manifest-identity.yaml"),
    ] {
        let value: serde_json::Value = serde_json::from_str(text).unwrap();
        for act in value["timeline"].as_array().unwrap() {
            let source = &act["response"]["source"];
            // Decode via the actual generated snapshot boundary; admission is
            // deliberately not involved in establishing the identity oracle.
            let mut envelope: serde_json::Value = serde_json::from_str(include_str!(
                "../ess-semantic/scenarios/parity-stale-evidence.yaml"
            ))
            .unwrap();
            let snapshot = &mut envelope["timeline"][0]["input"]["snapshot"];
            snapshot["source"] = source.clone();
            let actual =
                semantic_wire::decode_snapshot(&serde_json::to_vec(snapshot).unwrap()).unwrap();
            assert_eq!(
                source_identity::configuration_id(&actual.source.configuration).unwrap(),
                actual.source.configuration.id
            );
            assert_eq!(
                source_identity::snapshot_id(&actual.source).unwrap(),
                actual.source.id
            );
            let mut changed = actual.source.clone();
            changed.files[0].content_sha256 = "0".repeat(64);
            assert_ne!(
                source_identity::snapshot_id(&changed).unwrap(),
                actual.source.id
            );
            changed = actual.source.clone();
            changed.files[0].origin = codegate::semantic_model::SourceOrigin::TrackedClean;
            assert_eq!(
                source_identity::snapshot_id(&changed).unwrap(),
                actual.source.id
            );
        }
    }
}
