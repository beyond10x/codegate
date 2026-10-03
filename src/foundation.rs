//! Internal library obligations. Public semantic commands are separate work.
use crate::{collection, semantic, semantic_model::*};
use codegate_semantic_behavior::obligation::UnmetObligation;

/// Retains the actual typed result of each generated foundation obligation.
#[derive(Default)]
pub struct Foundation {
    source_response: Option<CollectSourceResponse>,
    validation_response: Option<ValidateSnapshotResponse>,
}

impl Foundation {
    pub fn take_source_response(&mut self) -> Option<CollectSourceResponse> {
        self.source_response.take()
    }

    pub fn take_validation_response(&mut self) -> Option<ValidateSnapshotResponse> {
        self.validation_response.take()
    }
}

impl obligations::CollectSourceBehavior for Foundation {
    fn collect_source(
        &mut self,
        input: CollectSource,
    ) -> Result<CollectSourceOutcome, UnmetObligation> {
        self.source_response = Some(match collection::collect_source(&input.request) {
            Ok(collected) => CollectSourceResponse {
                source: Some(collected.snapshot),
                gaps: collected.gaps,
            },
            Err(gaps) => CollectSourceResponse { source: None, gaps },
        });
        Ok(CollectSourceOutcome::SourceObserved)
    }
}

impl obligations::ValidateSnapshotBehavior for Foundation {
    fn validate_snapshot(
        &mut self,
        input: ValidateSnapshot,
    ) -> Result<ValidateSnapshotOutcome, UnmetObligation> {
        self.validation_response = Some(match semantic::validate_snapshot(&input.snapshot) {
            Ok(()) => ValidateSnapshotResponse {
                accepted: true,
                gaps: vec![],
            },
            Err(gaps) => ValidateSnapshotResponse {
                accepted: false,
                gaps,
            },
        });
        Ok(ValidateSnapshotOutcome::ValidationObserved)
    }
}
