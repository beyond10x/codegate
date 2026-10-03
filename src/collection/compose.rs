//! Production composition of exact source capture and registered observations.
use super::{CollectedSource, collect_source, failure};
use crate::{
    bindings::{BindingFacts, BindingRegistry},
    semantic,
    semantic_model::*,
    source_identity,
};
use codegate_semantic_behavior::obligation::UnmetObligation;

pub const FACT_FAMILIES: [FactFamily; 15] = [
    FactFamily::Sources,
    FactFamily::Declarations,
    FactFamily::Occurrences,
    FactFamily::Dependencies,
    FactFamily::References,
    FactFamily::Implementations,
    FactFamily::Calls,
    FactFamily::Decisions,
    FactFamily::Structure,
    FactFamily::Documentation,
    FactFamily::Effects,
    FactFamily::Tests,
    FactFamily::FrameworkDeclarations,
    FactFamily::FrameworkWiring,
    FactFamily::ExecutionCoverage,
];

pub(crate) fn relative_path(path: &str) -> bool {
    super::valid_path(path)
}

fn unsupported(
    mode: CollectionMode,
    configuration_id: &ConfigurationId,
    family: FactFamily,
    unit_ids: Vec<UnitId>,
    unknown_units: bool,
) -> Coverage {
    let mut gaps = vec![failure(
        GapCode::UnsupportedCapability,
        if mode == CollectionMode::Semantic {
            "semantic adapters are not implemented"
        } else {
            "source binding is not implemented for this fact family"
        },
    )];
    if unknown_units {
        gaps.push(failure(
            GapCode::UnknownUnit,
            "unit discovery is unavailable for selected sources",
        ));
    }
    Coverage {
        family,
        configuration_id: configuration_id.clone(),
        unit_ids,
        status: Completeness::Unsupported,
        gaps,
        applicability_reason: None,
    }
}
fn empty_snapshot(source: SourceSnapshot, mode: CollectionMode) -> FactSnapshot {
    FactSnapshot {
        format: "codegate.semantic-facts/1".into(),
        source,
        mode,
        tools: vec![],
        coverage: vec![],
        units: vec![],
        declarations: vec![],
        occurrences: vec![],
        dependencies: vec![],
        references: vec![],
        calls: vec![],
        implementations: vec![],
        decisions: vec![],
        structure: vec![],
        effects: vec![],
        framework: vec![],
    }
}
fn empty_source(
    request: &CollectionRequest,
    asserted_descriptors: bool,
) -> Result<SourceSnapshot, Vec<Gap>> {
    let mut configuration = request.configuration.clone();
    if !asserted_descriptors {
        configuration.configuration_files.clear();
    }
    configuration.modules.sort();
    configuration.go_build_tags.sort();
    configuration.rust_features.sort();
    configuration.rust_cfg.sort();
    configuration.build_profiles.sort();
    configuration
        .configuration_files
        .sort_by(|a, b| a.path.cmp(&b.path));
    configuration.id = source_identity::configuration_id(&configuration).map_err(|g| vec![g])?;
    let mut selection = request.selection.clone();
    selection.include_paths.sort();
    selection.exclude_paths.sort();
    selection.languages.sort_by_key(|l| match l {
        Language::Go => 0,
        Language::Java => 1,
        Language::Rust => 2,
    });
    let mut source = SourceSnapshot {
        id: SnapshotId(String::new()),
        selection,
        configuration,
        files: vec![],
        manifests: vec![],
    };
    source.id = source_identity::snapshot_id(&source).map_err(|g| vec![g])?;
    Ok(source)
}
fn validate_request(request: &CollectionRequest) -> Result<(), Vec<Gap>> {
    let invalid = |reason| vec![failure(GapCode::InvalidFact, reason)];
    if request.root.is_empty()
        || request.root.contains('\0')
        || std::path::Path::new(&request.root).components().any(|p| {
            matches!(
                p,
                std::path::Component::ParentDir | std::path::Component::Prefix(_)
            )
        })
    {
        return Err(invalid("invalid source root"));
    }
    if request.timeout_milliseconds <= 0 {
        return Err(invalid("collection timeout must be positive"));
    }
    if request
        .selection
        .include_paths
        .iter()
        .any(|p| super::metadata_path(p))
        || request
            .configuration
            .modules
            .iter()
            .any(|p| super::metadata_path(p))
        || request
            .configuration
            .configuration_files
            .iter()
            .any(|f| super::metadata_path(&f.path))
    {
        return Err(invalid("repository metadata cannot be selected"));
    }
    // Validate asserted descriptor shape without treating those descriptors as observed
    // bytes. This internal value is never returned as collection output.
    let mut shape = empty_snapshot(empty_source(request, true)?, request.mode);
    for family in FACT_FAMILIES {
        shape.coverage.push(unsupported(
            request.mode,
            &shape.source.configuration.id,
            family,
            vec![],
            true,
        ));
    }
    semantic::validate_snapshot(&shape)
}
fn append(snapshot: &mut FactSnapshot, mut facts: BindingFacts) {
    snapshot.units.append(&mut facts.units);
    snapshot.coverage.append(&mut facts.coverage);
    snapshot.declarations.append(&mut facts.declarations);
    snapshot.occurrences.append(&mut facts.occurrences);
    snapshot.dependencies.append(&mut facts.dependencies);
    snapshot.references.append(&mut facts.references);
    snapshot.calls.append(&mut facts.calls);
    snapshot.implementations.append(&mut facts.implementations);
    snapshot.decisions.append(&mut facts.decisions);
    snapshot.structure.append(&mut facts.structure);
    snapshot.effects.append(&mut facts.effects);
    snapshot.framework.append(&mut facts.framework);
}
// A producer owns the source end of each observation. The only cross-slot edge
// currently supported is a dependency target; admission still validates that target.
fn binding_scope(f: &BindingFacts, language: Language, source: &SourceSnapshot) -> bool {
    let units: std::collections::BTreeSet<_> = f.units.iter().map(|u| &u.id.0).collect();
    let declarations: std::collections::BTreeSet<_> =
        f.declarations.iter().map(|d| &d.id.0).collect();
    let occurrences: std::collections::BTreeSet<_> =
        f.occurrences.iter().map(|o| &o.evidence.id.0).collect();
    let owns = |d: &DeclarationId| declarations.contains(&d.0);
    let optional = |d: &Option<DeclarationId>| d.as_ref().is_none_or(owns);
    let location = |r: &SourceRange| {
        source
            .files
            .iter()
            .any(|s| s.path == r.path && s.language == Some(language))
            || source
                .configuration
                .configuration_files
                .iter()
                .any(|s| s.path == r.path)
    };
    let evidence = f
        .units
        .iter()
        .map(|v| &v.evidence)
        .chain(f.declarations.iter().map(|v| &v.evidence))
        .chain(f.occurrences.iter().map(|v| &v.evidence))
        .chain(f.dependencies.iter().map(|v| &v.evidence))
        .chain(f.references.iter().map(|v| &v.evidence))
        .chain(f.calls.iter().map(|v| &v.evidence))
        .chain(f.implementations.iter().map(|v| &v.evidence))
        .chain(f.decisions.iter().map(|v| &v.evidence))
        .chain(f.structure.iter().map(|v| &v.evidence))
        .chain(f.effects.iter().map(|v| &v.evidence))
        .chain(f.framework.iter().map(|v| &v.evidence));
    evidence
        .clone()
        .all(|e| e.location.as_ref().is_none_or(location))
        && evidence
            .flat_map(|e| &e.gaps)
            .chain(f.coverage.iter().flat_map(|c| &c.gaps))
            .all(|g| g.location.as_ref().is_none_or(location))
        && f.units.iter().all(|u| {
            u.language == language
                && !u.paths.is_empty()
                && u.paths.iter().all(|p| {
                    source
                        .files
                        .iter()
                        .any(|s| &s.path == p && s.language == Some(language))
                })
        })
        && f.coverage.iter().all(|c| {
            c.family != FactFamily::Sources && c.unit_ids.iter().all(|u| units.contains(&u.0))
        })
        && f.declarations
            .iter()
            .all(|d| units.contains(&d.unit_id.0) && optional(&d.parent))
        && f.occurrences.iter().all(|o| optional(&o.owner))
        && f.dependencies.iter().all(|d| units.contains(&d.source.0))
        && f.references.iter().all(|r| {
            occurrences.contains(&r.occurrence_id.0)
                && optional(&r.target)
                && r.candidates.iter().all(owns)
        })
        && f.calls
            .iter()
            .all(|c| owns(&c.caller) && optional(&c.callee) && c.candidates.iter().all(owns))
        && f.implementations.iter().all(|i| {
            owns(&i.declaration) && optional(&i.implementation) && i.candidates.iter().all(owns)
        })
        && f.decisions.iter().all(|d| owns(&d.owner))
        && f.structure.iter().all(|s| owns(&s.owner))
        && f.effects
            .iter()
            .all(|e| owns(&e.owner) && optional(&e.related_declaration))
        && f.framework
            .iter()
            .all(|f| owns(&f.declaration) && optional(&f.target))
}
fn compose(
    request: &CollectionRequest,
    registry: &BindingRegistry,
) -> Result<CollectResponse, Vec<Gap>> {
    validate_request(request)?;
    // An explicit internal source-only capture never changes the requested analysis mode.
    let mut capture_request = request.clone();
    capture_request.mode = CollectionMode::SourceOnly;
    let (captured, sources_status, sources_gaps) = match collect_source(&capture_request) {
        Ok(source) => {
            let status = if source.gaps.is_empty() {
                Completeness::Complete
            } else {
                Completeness::Partial
            };
            let gaps = source.gaps.clone();
            (source, status, gaps)
        }
        Err(gaps) => (
            CollectedSource {
                snapshot: empty_source(request, false)?,
                contents: Default::default(),
                gaps: gaps.clone(),
            },
            Completeness::Failed,
            gaps,
        ),
    };
    let mut snapshot = empty_snapshot(captured.snapshot.clone(), request.mode);
    if sources_status != Completeness::Failed && request.mode == CollectionMode::SourceOnly {
        for binding in &registry.bindings {
            if !captured
                .snapshot
                .files
                .iter()
                .any(|f| f.language == Some(binding.language()))
            {
                continue;
            }
            let facts = binding.collect(&captured);
            // A language slot cannot claim collection coverage or another slot's units.
            if !binding_scope(&facts, binding.language(), &captured.snapshot) {
                return Err(vec![failure(
                    GapCode::InvalidFact,
                    "source binding exceeded its language or coverage scope",
                )]);
            }
            append(&mut snapshot, facts);
        }
    }
    let unit_ids: Vec<_> = snapshot.units.iter().map(|u| u.id.clone()).collect();
    let unknown_units = snapshot.units.is_empty()
        || captured
            .snapshot
            .files
            .iter()
            .filter(|f| f.language.is_some())
            .any(|f| !snapshot.units.iter().any(|u| u.paths.contains(&f.path)));
    snapshot.coverage.push(Coverage {
        family: FactFamily::Sources,
        configuration_id: snapshot.source.configuration.id.clone(),
        unit_ids: unit_ids.clone(),
        status: sources_status,
        gaps: sources_gaps,
        applicability_reason: None,
    });
    for family in FACT_FAMILIES
        .into_iter()
        .filter(|f| *f != FactFamily::Sources)
    {
        let missing: Vec<_> = unit_ids
            .iter()
            .filter(|id| {
                !snapshot
                    .coverage
                    .iter()
                    .any(|c| c.family == family && c.unit_ids.contains(id))
            })
            .cloned()
            .collect();
        if !missing.is_empty()
            || (unit_ids.is_empty() && !snapshot.coverage.iter().any(|c| c.family == family))
        {
            snapshot.coverage.push(unsupported(
                request.mode,
                &snapshot.source.configuration.id,
                family,
                missing,
                unknown_units,
            ));
        }
        // Observed units cannot imply complete discovery of every selected source.
        if unknown_units {
            for coverage in snapshot.coverage.iter_mut().filter(|c| c.family == family) {
                if !coverage.gaps.iter().any(|g| g.code == GapCode::UnknownUnit) {
                    coverage.gaps.push(failure(
                        GapCode::UnknownUnit,
                        "unit discovery is unavailable for selected sources",
                    ));
                }
                if matches!(
                    coverage.status,
                    Completeness::Complete | Completeness::NotApplicable
                ) {
                    coverage.status = Completeness::Partial;
                    coverage.applicability_reason = None;
                }
            }
        }
    }
    semantic::validate_snapshot(&snapshot)?;
    Ok(CollectResponse { snapshot })
}

/// Collect through the production registry. Invalid requests or invalid binding output
/// are invocation errors; valid capture failures are represented by Failed Sources.
pub fn collect(input: Collect) -> Result<CollectResponse, Vec<Gap>> {
    compose(&input.request, &BindingRegistry::default())
}

/// Adapter for the generated outcome-only obligation, retaining its typed response.
#[derive(Default)]
pub struct Collector {
    registry: BindingRegistry,
    response: Option<CollectResponse>,
}
impl Collector {
    pub fn with_bindings(registry: BindingRegistry) -> Self {
        Self {
            registry,
            response: None,
        }
    }
    pub fn take_response(&mut self) -> Option<CollectResponse> {
        self.response.take()
    }
    pub fn collect_response(&mut self, input: Collect) -> Result<CollectResponse, Vec<Gap>> {
        self.response = None;
        let response = compose(&input.request, &self.registry)?;
        self.response = Some(response.clone());
        Ok(response)
    }
}
impl obligations::CollectBehavior for Collector {
    fn collect(&mut self, input: Collect) -> Result<CollectOutcome, UnmetObligation> {
        self.collect_response(input)
            .map(|_| CollectOutcome::Collected)
            .map_err(|_| UnmetObligation {
                capability: "command behaviour",
                source: "codegate_semantic.semantic.Collect",
            })
    }
}
