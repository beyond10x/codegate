//! Source-language structure producers over retained bytes.
pub mod lines;
use crate::{collection::CollectedSource, semantic_model::*};
use sha2::{Digest, Sha256};

/// Observations from one language binding. Coverage names only the units it observed;
/// absent families are filled as unsupported by the collection composition layer.
#[derive(Default)]
pub struct BindingFacts {
    pub units: Vec<Unit>,
    pub coverage: Vec<Coverage>,
    pub declarations: Vec<Declaration>,
    pub occurrences: Vec<Occurrence>,
    pub dependencies: Vec<Dependency>,
    pub references: Vec<Reference>,
    pub calls: Vec<Call>,
    pub implementations: Vec<Implementation>,
    pub decisions: Vec<DecisionPoint>,
    pub structure: Vec<StructureObservation>,
    pub effects: Vec<Effect>,
    pub framework: Vec<FrameworkFact>,
}

/// A source binding sees exactly the captured source and configuration bytes.
/// It produces observations; shared analysis and policy remain outside this interface.
pub trait SourceBinding {
    fn language(&self) -> Language;
    fn collect(&self, source: &CollectedSource) -> BindingFacts;
}

/// Explicit language slots; no language extraction is registered by default yet.
#[derive(Default)]
pub struct BindingRegistry {
    pub(crate) bindings: Vec<Box<dyn SourceBinding>>,
}
impl BindingRegistry {
    pub fn register(&mut self, binding: impl SourceBinding + 'static) -> Result<(), Gap> {
        if self
            .bindings
            .iter()
            .any(|b| b.language() == binding.language())
        {
            return Err(invalid("duplicate source binding language"));
        }
        self.bindings.push(Box::new(binding));
        // Registry order must not make collection output depend on installation order.
        self.bindings.sort_by_key(|b| match b.language() {
            Language::Go => 0,
            Language::Java => 1,
            Language::Rust => 2,
        });
        Ok(())
    }
}
fn invalid(reason: &str) -> Gap {
    Gap {
        code: GapCode::InvalidFact,
        reason: reason.into(),
        location: None,
    }
}

/// Domain-separated, length-delimited IDs; concatenation boundaries cannot collide.
pub fn stable_id(namespace: &str, parts: &[&str]) -> String {
    let mut hash = Sha256::new();
    for value in std::iter::once(namespace).chain(parts.iter().copied()) {
        hash.update((value.len() as u64).to_be_bytes());
        hash.update(value.as_bytes());
    }
    hash.finalize().iter().map(|b| format!("{b:02x}")).collect()
}

/// Zero-based rows and UTF-8 byte columns for a half-open range in retained text.
/// Invalid UTF-8 boundaries and out-of-bounds coordinates are refused, never rounded.
pub fn source_range(path: &str, text: &str, start: usize, end: usize) -> Result<SourceRange, Gap> {
    if !crate::collection::compose::relative_path(path)
        || start > end
        || end > text.len()
        || !text.is_char_boundary(start)
        || !text.is_char_boundary(end)
    {
        return Err(invalid("invalid retained source range"));
    }
    let position = |offset: usize| {
        let prefix = &text.as_bytes()[..offset];
        let line = prefix.iter().filter(|b| **b == b'\n').count();
        let column = prefix
            .iter()
            .rposition(|b| *b == b'\n')
            .map_or(offset, |i| offset - i - 1);
        (line as i64, column as i64)
    };
    let (start_line, start_column) = position(start);
    let (end_line, end_column) = position(end);
    Ok(SourceRange {
        path: path.into(),
        start_byte: start as i64,
        end_byte: end as i64,
        start_line,
        start_column,
        end_line,
        end_column,
    })
}

/// Source observations start as syntactic candidates. A producer must explicitly
/// supply stronger evidence before declaring a relationship resolved.
pub fn source_evidence(
    source: &SourceSnapshot,
    producer: &str,
    producer_version: &str,
    id: FactId,
    location: Option<SourceRange>,
) -> Evidence {
    Evidence {
        id,
        snapshot_id: source.id.clone(),
        configuration_id: source.configuration.id.clone(),
        producer: producer.into(),
        producer_version: producer_version.into(),
        location,
        resolution: Resolution::SyntacticCandidate,
        basis: ResolutionBasis::SourceStructure,
        gaps: vec![],
    }
}
