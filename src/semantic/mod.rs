//! Pure admission for separately versioned rich semantic snapshots.
mod admit;
use crate::semantic_model::{FactSnapshot, Gap};

/// Validate imported facts before shared algorithms can observe them.
/// This checks internal consistency, not authenticity of unavailable source bytes.
pub fn validate_snapshot(snapshot: &FactSnapshot) -> Result<(), Vec<Gap>> {
    admit::admit(snapshot).map(|graph| {
        let _ = graph.snapshot();
    })
}
