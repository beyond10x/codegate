// generated from codegate_semantic v1
// model digest 6429b77034506255e96ca6abd75079983e5e0ca68ef4a30f19a0baf8875612dd
// contract digest 53b2681048db3937988b0fb64c9955287e4a7585230662d8b6db2d63142af9a9
// do not edit: regenerate with `ess synthesize --layout crate`

//! semantic — `codegate_semantic.semantic`.
//!
//! Immutable observations and evidence-bounded analysis/navigation values for semantic contract v1; separate from dependency /0.1.
//!
//! Everything this bounded context declares that the synthesis plan marks generated.

/// ApiEligibility — `codegate_semantic.semantic.ApiEligibility`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ApiEligibility {
    /// `Eligible`.
    Eligible,
    /// `Ineligible`.
    Ineligible,
    /// `Unknown`.
    Unknown,
}

/// Assessment — `codegate_semantic.semantic.Assessment`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assessment {
    /// `format` — `String`.
    pub format: String,
    /// `snapshot_id` — `codegate_semantic.semantic.SnapshotId`.
    pub snapshot_id: SnapshotId,
    /// `configuration_id` — `codegate_semantic.semantic.ConfigurationId`.
    pub configuration_id: ConfigurationId,
    /// `policy` — `codegate_semantic.semantic.Policy`.
    pub policy: Policy,
    /// `coverage` — `List<codegate_semantic.semantic.Coverage>`.
    pub coverage: Vec<Coverage>,
    /// `measurements` — `List<codegate_semantic.semantic.Measurement>`.
    pub measurements: Vec<Measurement>,
    /// `findings` — `List<codegate_semantic.semantic.Finding>`.
    pub findings: Vec<Finding>,
    /// `checks` — `List<codegate_semantic.semantic.CheckResult>`.
    pub checks: Vec<CheckResult>,
    /// `scores` — `List<codegate_semantic.semantic.Score>`.
    pub scores: Vec<Score>,
    /// `overall_score` — `Optional<Decimal>`.
    pub overall_score: Option<crate::primitives::Decimal>,
    /// `verdict` — `codegate_semantic.semantic.Verdict`.
    pub verdict: Verdict,
    /// `suggestions` — `List<codegate_semantic.semantic.Suggestion>`.
    pub suggestions: Vec<Suggestion>,
}

/// BuildSelection — `codegate_semantic.semantic.BuildSelection`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BuildSelection {
    /// `id` — `codegate_semantic.semantic.ConfigurationId`.
    pub id: ConfigurationId,
    /// `modules` — `List<String>`.
    pub modules: Vec<String>,
    /// `target` — `Optional<String>`.
    pub target: Option<String>,
    /// `go_build_tags` — `List<String>`.
    pub go_build_tags: Vec<String>,
    /// `rust_features` — `List<String>`.
    pub rust_features: Vec<String>,
    /// `rust_default_features` — `Boolean`.
    pub rust_default_features: bool,
    /// `rust_cfg` — `List<String>`.
    pub rust_cfg: Vec<String>,
    /// `java_target_version` — `Optional<String>`.
    pub java_target_version: Option<String>,
    /// `java_classpath_sha256` — `Optional<String>`.
    pub java_classpath_sha256: Option<String>,
    /// `build_profiles` — `List<String>`.
    pub build_profiles: Vec<String>,
    /// `configuration_files` — `List<codegate_semantic.semantic.SourceFile>`.
    pub configuration_files: Vec<SourceFile>,
}

/// BuildSystem — `codegate_semantic.semantic.BuildSystem`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BuildSystem {
    /// `GoModules`.
    GoModules,
    /// `Cargo`.
    Cargo,
    /// `Maven`.
    Maven,
    /// `Gradle`.
    Gradle,
}

/// Call — `codegate_semantic.semantic.Call`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call {
    /// `caller` — `codegate_semantic.semantic.DeclarationId`.
    pub caller: DeclarationId,
    /// `callee` — `Optional<codegate_semantic.semantic.DeclarationId>`.
    pub callee: Option<DeclarationId>,
    /// `candidates` — `List<codegate_semantic.semantic.DeclarationId>`.
    pub candidates: Vec<DeclarationId>,
    /// `dynamic_dispatch` — `Boolean`.
    pub dynamic_dispatch: bool,
    /// `evidence` — `codegate_semantic.semantic.Evidence`.
    pub evidence: Evidence,
}

/// Capability — `codegate_semantic.semantic.Capability`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Capability {
    /// `id` — `String`.
    pub id: String,
    /// `language` — `codegate_semantic.semantic.Language`.
    pub language: Language,
    /// `family` — `codegate_semantic.semantic.FactFamily`.
    pub family: FactFamily,
    /// `mode` — `codegate_semantic.semantic.CollectionMode`.
    pub mode: CollectionMode,
    /// `configuration_id` — `codegate_semantic.semantic.ConfigurationId`.
    pub configuration_id: ConfigurationId,
    /// `status` — `codegate_semantic.semantic.Completeness`.
    pub status: Completeness,
    /// `required_tools` — `List<String>`.
    pub required_tools: Vec<String>,
    /// `gaps` — `List<codegate_semantic.semantic.Gap>`.
    pub gaps: Vec<Gap>,
    /// `applicability_reason` — `Optional<String>`.
    pub applicability_reason: Option<String>,
}

/// CheckKind — `codegate_semantic.semantic.CheckKind`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CheckKind {
    /// `DependencyDirection`.
    DependencyDirection,
    /// `Layers`.
    Layers,
    /// `FanIn`.
    FanIn,
    /// `FanOut`.
    FanOut,
    /// `Cycles`.
    Cycles,
    /// `TestBoundary`.
    TestBoundary,
    /// `ForbiddenImport`.
    ForbiddenImport,
    /// `ForbiddenCall`.
    ForbiddenCall,
    /// `Effect`.
    Effect,
    /// `UnknownUnit`.
    UnknownUnit,
    /// `MetricThreshold`.
    MetricThreshold,
    /// `Framework`.
    Framework,
}

/// CheckResult — `codegate_semantic.semantic.CheckResult`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckResult {
    /// `rule_id` — `String`.
    pub rule_id: String,
    /// `verdict` — `codegate_semantic.semantic.Verdict`.
    pub verdict: Verdict,
    /// `finding_ids` — `List<String>`.
    pub finding_ids: Vec<String>,
    /// `gaps` — `List<codegate_semantic.semantic.Gap>`.
    pub gaps: Vec<Gap>,
}

/// CollectionMode — `codegate_semantic.semantic.CollectionMode`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectionMode {
    /// `SourceOnly`.
    SourceOnly,
    /// `Semantic`.
    Semantic,
}

/// CollectionRequest — `codegate_semantic.semantic.CollectionRequest`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CollectionRequest {
    /// `root` — `String`.
    pub root: String,
    /// `selection` — `codegate_semantic.semantic.SourceSelection`.
    pub selection: SourceSelection,
    /// `configuration` — `codegate_semantic.semantic.BuildSelection`.
    pub configuration: BuildSelection,
    /// `mode` — `codegate_semantic.semantic.CollectionMode`.
    pub mode: CollectionMode,
    /// `timeout_milliseconds` — `Integer`.
    pub timeout_milliseconds: i64,
}

/// Completeness — `codegate_semantic.semantic.Completeness`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Completeness {
    /// `Complete`.
    Complete,
    /// `Partial`.
    Partial,
    /// `Unsupported`.
    Unsupported,
    /// `Failed`.
    Failed,
    /// `NotApplicable`.
    NotApplicable,
}

/// ConfigurationId — `codegate_semantic.semantic.ConfigurationId`: a distinct wrapper around `String`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigurationId(pub String);

/// Coverage — `codegate_semantic.semantic.Coverage`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Coverage {
    /// `family` — `codegate_semantic.semantic.FactFamily`.
    pub family: FactFamily,
    /// `configuration_id` — `codegate_semantic.semantic.ConfigurationId`.
    pub configuration_id: ConfigurationId,
    /// `unit_ids` — `List<codegate_semantic.semantic.UnitId>`.
    pub unit_ids: Vec<UnitId>,
    /// `status` — `codegate_semantic.semantic.Completeness`.
    pub status: Completeness,
    /// `gaps` — `List<codegate_semantic.semantic.Gap>`.
    pub gaps: Vec<Gap>,
    /// `applicability_reason` — `Optional<String>`.
    pub applicability_reason: Option<String>,
}

/// DecisionKind — `codegate_semantic.semantic.DecisionKind`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecisionKind {
    /// `ConditionalBranch`.
    ConditionalBranch,
    /// `Loop`.
    Loop,
    /// `CaseBranch`.
    CaseBranch,
    /// `CatchBranch`.
    CatchBranch,
    /// `ShortCircuitAnd`.
    ShortCircuitAnd,
    /// `ShortCircuitOr`.
    ShortCircuitOr,
    /// `Ternary`.
    Ternary,
    /// `GoSelectCase`.
    GoSelectCase,
    /// `RustTryPropagation`.
    RustTryPropagation,
}

/// DecisionPoint — `codegate_semantic.semantic.DecisionPoint`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DecisionPoint {
    /// `owner` — `codegate_semantic.semantic.DeclarationId`.
    pub owner: DeclarationId,
    /// `kind` — `codegate_semantic.semantic.DecisionKind`.
    pub kind: DecisionKind,
    /// `evidence` — `codegate_semantic.semantic.Evidence`.
    pub evidence: Evidence,
}

/// Declaration — `codegate_semantic.semantic.Declaration`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Declaration {
    /// `id` — `codegate_semantic.semantic.DeclarationId`.
    pub id: DeclarationId,
    /// `unit_id` — `codegate_semantic.semantic.UnitId`.
    pub unit_id: UnitId,
    /// `parent` — `Optional<codegate_semantic.semantic.DeclarationId>`.
    pub parent: Option<DeclarationId>,
    /// `name` — `String`.
    pub name: String,
    /// `qualified_name` — `String`.
    pub qualified_name: String,
    /// `signature` — `Optional<String>`.
    pub signature: Option<String>,
    /// `kind` — `codegate_semantic.semantic.DeclarationKind`.
    pub kind: DeclarationKind,
    /// `visibility` — `codegate_semantic.semantic.Visibility`.
    pub visibility: Visibility,
    /// `api_eligibility` — `codegate_semantic.semantic.ApiEligibility`.
    pub api_eligibility: ApiEligibility,
    /// `evidence` — `codegate_semantic.semantic.Evidence`.
    pub evidence: Evidence,
}

/// DeclarationId — `codegate_semantic.semantic.DeclarationId`: a distinct wrapper around `String`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclarationId(pub String);

/// DeclarationKind — `codegate_semantic.semantic.DeclarationKind`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeclarationKind {
    /// `Package`.
    Package,
    /// `Module`.
    Module,
    /// `Namespace`.
    Namespace,
    /// `Function`.
    Function,
    /// `Method`.
    Method,
    /// `Constructor`.
    Constructor,
    /// `Type`.
    Type,
    /// `Interface`.
    Interface,
    /// `Trait`.
    Trait,
    /// `Field`.
    Field,
    /// `Variable`.
    Variable,
    /// `Parameter`.
    Parameter,
    /// `Constant`.
    Constant,
}

/// Dependency — `codegate_semantic.semantic.Dependency`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dependency {
    /// `source` — `codegate_semantic.semantic.UnitId`.
    pub source: UnitId,
    /// `target` — `Optional<codegate_semantic.semantic.UnitId>`.
    pub target: Option<UnitId>,
    /// `target_name` — `String`.
    pub target_name: String,
    /// `kind` — `codegate_semantic.semantic.DependencyKind`.
    pub kind: DependencyKind,
    /// `evidence` — `codegate_semantic.semantic.Evidence`.
    pub evidence: Evidence,
}

/// DependencyKind — `codegate_semantic.semantic.DependencyKind`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DependencyKind {
    /// `Runtime`.
    Runtime,
    /// `Build`.
    Build,
    /// `Test`.
    Test,
}

/// Effect — `codegate_semantic.semantic.Effect`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Effect {
    /// `owner` — `codegate_semantic.semantic.DeclarationId`.
    pub owner: DeclarationId,
    /// `kind` — `codegate_semantic.semantic.EffectKind`.
    pub kind: EffectKind,
    /// `related_declaration` — `Optional<codegate_semantic.semantic.DeclarationId>`.
    pub related_declaration: Option<DeclarationId>,
    /// `evidence` — `codegate_semantic.semantic.Evidence`.
    pub evidence: Evidence,
}

/// EffectKind — `codegate_semantic.semantic.EffectKind`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffectKind {
    /// `DiscardedResult`.
    DiscardedResult,
    /// `AbruptExit`.
    AbruptExit,
    /// `ProcessExecution`.
    ProcessExecution,
    /// `SqlConstruction`.
    SqlConstruction,
    /// `PathConstruction`.
    PathConstruction,
    /// `UnsafeOperation`.
    UnsafeOperation,
    /// `WeakCryptography`.
    WeakCryptography,
    /// `Reflection`.
    Reflection,
    /// `Nondeterminism`.
    Nondeterminism,
    /// `GoDefer`.
    GoDefer,
    /// `GoUncheckedTypeAssertion`.
    GoUncheckedTypeAssertion,
    /// `AllocationInLoop`.
    AllocationInLoop,
    /// `StringConcatenationInLoop`.
    StringConcatenationInLoop,
    /// `MissingCapacity`.
    MissingCapacity,
    /// `LargeValueCopy`.
    LargeValueCopy,
}

/// Evidence — `codegate_semantic.semantic.Evidence`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Evidence {
    /// `id` — `codegate_semantic.semantic.FactId`.
    pub id: FactId,
    /// `snapshot_id` — `codegate_semantic.semantic.SnapshotId`.
    pub snapshot_id: SnapshotId,
    /// `configuration_id` — `codegate_semantic.semantic.ConfigurationId`.
    pub configuration_id: ConfigurationId,
    /// `producer` — `String`.
    pub producer: String,
    /// `producer_version` — `String`.
    pub producer_version: String,
    /// `location` — `Optional<codegate_semantic.semantic.SourceRange>`.
    pub location: Option<SourceRange>,
    /// `resolution` — `codegate_semantic.semantic.Resolution`.
    pub resolution: Resolution,
    /// `basis` — `codegate_semantic.semantic.ResolutionBasis`.
    pub basis: ResolutionBasis,
    /// `gaps` — `List<codegate_semantic.semantic.Gap>`.
    pub gaps: Vec<Gap>,
}

/// FactFamily — `codegate_semantic.semantic.FactFamily`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FactFamily {
    /// `Sources`.
    Sources,
    /// `Declarations`.
    Declarations,
    /// `Occurrences`.
    Occurrences,
    /// `Dependencies`.
    Dependencies,
    /// `References`.
    References,
    /// `Implementations`.
    Implementations,
    /// `Calls`.
    Calls,
    /// `Decisions`.
    Decisions,
    /// `Structure`.
    Structure,
    /// `Documentation`.
    Documentation,
    /// `Effects`.
    Effects,
    /// `Tests`.
    Tests,
    /// `FrameworkDeclarations`.
    FrameworkDeclarations,
    /// `FrameworkWiring`.
    FrameworkWiring,
    /// `ExecutionCoverage`.
    ExecutionCoverage,
}

/// FactId — `codegate_semantic.semantic.FactId`: a distinct wrapper around `String`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FactId(pub String);

/// FactSnapshot — `codegate_semantic.semantic.FactSnapshot`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FactSnapshot {
    /// `format` — `String`.
    pub format: String,
    /// `source` — `codegate_semantic.semantic.SourceSnapshot`.
    pub source: SourceSnapshot,
    /// `mode` — `codegate_semantic.semantic.CollectionMode`.
    pub mode: CollectionMode,
    /// `tools` — `List<codegate_semantic.semantic.ToolObservation>`.
    pub tools: Vec<ToolObservation>,
    /// `coverage` — `List<codegate_semantic.semantic.Coverage>`.
    pub coverage: Vec<Coverage>,
    /// `units` — `List<codegate_semantic.semantic.Unit>`.
    pub units: Vec<Unit>,
    /// `declarations` — `List<codegate_semantic.semantic.Declaration>`.
    pub declarations: Vec<Declaration>,
    /// `occurrences` — `List<codegate_semantic.semantic.Occurrence>`.
    pub occurrences: Vec<Occurrence>,
    /// `dependencies` — `List<codegate_semantic.semantic.Dependency>`.
    pub dependencies: Vec<Dependency>,
    /// `references` — `List<codegate_semantic.semantic.Reference>`.
    pub references: Vec<Reference>,
    /// `calls` — `List<codegate_semantic.semantic.Call>`.
    pub calls: Vec<Call>,
    /// `implementations` — `List<codegate_semantic.semantic.Implementation>`.
    pub implementations: Vec<Implementation>,
    /// `decisions` — `List<codegate_semantic.semantic.DecisionPoint>`.
    pub decisions: Vec<DecisionPoint>,
    /// `structure` — `List<codegate_semantic.semantic.StructureObservation>`.
    pub structure: Vec<StructureObservation>,
    /// `effects` — `List<codegate_semantic.semantic.Effect>`.
    pub effects: Vec<Effect>,
    /// `framework` — `List<codegate_semantic.semantic.FrameworkFact>`.
    pub framework: Vec<FrameworkFact>,
}

/// Finding — `codegate_semantic.semantic.Finding`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    /// `id` — `String`.
    pub id: String,
    /// `rule_id` — `String`.
    pub rule_id: String,
    /// `subject` — `String`.
    pub subject: String,
    /// `severity` — `codegate_semantic.semantic.Severity`.
    pub severity: Severity,
    /// `message` — `String`.
    pub message: String,
    /// `evidence_ids` — `List<codegate_semantic.semantic.FactId>`.
    pub evidence_ids: Vec<FactId>,
    /// `exception_reason` — `Optional<String>`.
    pub exception_reason: Option<String>,
    /// `pressure` — `Optional<Decimal>`.
    pub pressure: Option<crate::primitives::Decimal>,
}

/// FrameworkFact — `codegate_semantic.semantic.FrameworkFact`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FrameworkFact {
    /// `kind` — `codegate_semantic.semantic.FrameworkKind`.
    pub kind: FrameworkKind,
    /// `declaration` — `codegate_semantic.semantic.DeclarationId`.
    pub declaration: DeclarationId,
    /// `target` — `Optional<codegate_semantic.semantic.DeclarationId>`.
    pub target: Option<DeclarationId>,
    /// `annotation_name` — `String`.
    pub annotation_name: String,
    /// `qualifiers` — `List<codegate_semantic.semantic.Qualifier>`.
    pub qualifiers: Vec<Qualifier>,
    /// `scope` — `Optional<String>`.
    pub scope: Option<String>,
    /// `produced_type` — `Optional<String>`.
    pub produced_type: Option<String>,
    /// `http_method` — `Optional<String>`.
    pub http_method: Option<String>,
    /// `resource_path` — `Optional<String>`.
    pub resource_path: Option<String>,
    /// `method_path` — `Optional<String>`.
    pub method_path: Option<String>,
    /// `transaction_mode` — `Optional<String>`.
    pub transaction_mode: Option<String>,
    /// `configuration_key` — `Optional<String>`.
    pub configuration_key: Option<String>,
    /// `configuration_profile` — `Optional<String>`.
    pub configuration_profile: Option<String>,
    /// `evidence` — `codegate_semantic.semantic.Evidence`.
    pub evidence: Evidence,
}

/// FrameworkKind — `codegate_semantic.semantic.FrameworkKind`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FrameworkKind {
    /// `Bean`.
    Bean,
    /// `Qualifier`.
    Qualifier,
    /// `Producer`.
    Producer,
    /// `InjectionPoint`.
    InjectionPoint,
    /// `InjectionCandidate`.
    InjectionCandidate,
    /// `InjectionBinding`.
    InjectionBinding,
    /// `RestResource`.
    RestResource,
    /// `RestRoute`.
    RestRoute,
    /// `TransactionDeclaration`.
    TransactionDeclaration,
    /// `ConfigurationReference`.
    ConfigurationReference,
}

/// Gap — `codegate_semantic.semantic.Gap`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Gap {
    /// `code` — `codegate_semantic.semantic.GapCode`.
    pub code: GapCode,
    /// `reason` — `String`.
    pub reason: String,
    /// `location` — `Optional<codegate_semantic.semantic.SourceRange>`.
    pub location: Option<SourceRange>,
}

/// GapCode — `codegate_semantic.semantic.GapCode`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GapCode {
    /// `InvalidFormat`.
    InvalidFormat,
    /// `InvalidFact`.
    InvalidFact,
    /// `InvalidPolicy`.
    InvalidPolicy,
    /// `IdentityMismatch`.
    IdentityMismatch,
    /// `SourceChangedDuringCollection`.
    SourceChangedDuringCollection,
    /// `MissingTool`.
    MissingTool,
    /// `ToolTimeout`.
    ToolTimeout,
    /// `ToolFailure`.
    ToolFailure,
    /// `SyntaxError`.
    SyntaxError,
    /// `MissingBuildSelection`.
    MissingBuildSelection,
    /// `UnknownUnit`.
    UnknownUnit,
    /// `UnresolvedSymbol`.
    UnresolvedSymbol,
    /// `AmbiguousSymbol`.
    AmbiguousSymbol,
    /// `DynamicDispatch`.
    DynamicDispatch,
    /// `MacroExpansion`.
    MacroExpansion,
    /// `ConditionalCompilation`.
    ConditionalCompilation,
    /// `MissingClasspath`.
    MissingClasspath,
    /// `GeneratedSources`.
    GeneratedSources,
    /// `ProgrammaticLookup`.
    ProgrammaticLookup,
    /// `AmbiguousInjection`.
    AmbiguousInjection,
    /// `AugmentationRequired`.
    AugmentationRequired,
    /// `StaleEvidence`.
    StaleEvidence,
    /// `UnsupportedCapability`.
    UnsupportedCapability,
    /// `PartialCollection`.
    PartialCollection,
}

/// Implementation — `codegate_semantic.semantic.Implementation`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Implementation {
    /// `declaration` — `codegate_semantic.semantic.DeclarationId`.
    pub declaration: DeclarationId,
    /// `implementation` — `Optional<codegate_semantic.semantic.DeclarationId>`.
    pub implementation: Option<DeclarationId>,
    /// `candidates` — `List<codegate_semantic.semantic.DeclarationId>`.
    pub candidates: Vec<DeclarationId>,
    /// `evidence` — `codegate_semantic.semantic.Evidence`.
    pub evidence: Evidence,
}

/// Language — `codegate_semantic.semantic.Language`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    /// `Go`.
    Go,
    /// `Rust`.
    Rust,
    /// `Java`.
    Java,
}

/// LineClass — `codegate_semantic.semantic.LineClass`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LineClass {
    /// `Code`.
    Code,
    /// `Comment`.
    Comment,
    /// `Blank`.
    Blank,
    /// `Mixed`.
    Mixed,
}

/// LookupSelector — `codegate_semantic.semantic.LookupSelector`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LookupSelector {
    /// `kind` — `codegate_semantic.semantic.SelectorKind`.
    pub kind: SelectorKind,
    /// `name` — `Optional<String>`.
    pub name: Option<String>,
    /// `position` — `Optional<codegate_semantic.semantic.SourceRange>`.
    pub position: Option<SourceRange>,
    /// `scope_units` — `List<codegate_semantic.semantic.UnitId>`.
    pub scope_units: Vec<UnitId>,
    /// `limit` — `Integer`.
    pub limit: i64,
}

/// Manifest — `codegate_semantic.semantic.Manifest`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Manifest {
    /// `path` — `String`.
    pub path: String,
    /// `content_sha256` — `String`.
    pub content_sha256: String,
    /// `system` — `codegate_semantic.semantic.BuildSystem`.
    pub system: BuildSystem,
    /// `module` — `String`.
    pub module: String,
    /// `parent_module` — `Optional<String>`.
    pub parent_module: Option<String>,
}

/// Measurement — `codegate_semantic.semantic.Measurement`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Measurement {
    /// `subject` — `String`.
    pub subject: String,
    /// `metric` — `codegate_semantic.semantic.MetricSelection`.
    pub metric: MetricSelection,
    /// `value` — `Optional<Decimal>`.
    pub value: Option<crate::primitives::Decimal>,
    /// `numerator` — `Optional<Integer>`.
    pub numerator: Option<i64>,
    /// `denominator` — `Optional<Integer>`.
    pub denominator: Option<i64>,
    /// `unit` — `String`.
    pub unit: String,
    /// `status` — `codegate_semantic.semantic.Completeness`.
    pub status: Completeness,
    /// `evidence_ids` — `List<codegate_semantic.semantic.FactId>`.
    pub evidence_ids: Vec<FactId>,
    /// `gaps` — `List<codegate_semantic.semantic.Gap>`.
    pub gaps: Vec<Gap>,
}

/// MetricAggregation — `codegate_semantic.semantic.MetricAggregation`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricAggregation {
    /// `PerSubject`.
    PerSubject,
    /// `Total`.
    Total,
    /// `Maximum`.
    Maximum,
    /// `Mean`.
    Mean,
    /// `CountAboveThreshold`.
    CountAboveThreshold,
}

/// MetricKind — `codegate_semantic.semantic.MetricKind`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MetricKind {
    /// `FanIn`.
    FanIn,
    /// `FanOut`.
    FanOut,
    /// `FunctionLines`.
    FunctionLines,
    /// `FileLines`.
    FileLines,
    /// `CodeLines`.
    CodeLines,
    /// `Complexity`.
    Complexity,
    /// `Nesting`.
    Nesting,
    /// `Parameters`.
    Parameters,
    /// `Returns`.
    Returns,
    /// `TypeMembers`.
    TypeMembers,
    /// `InterfaceMembers`.
    InterfaceMembers,
    /// `PublicApiSize`.
    PublicApiSize,
    /// `DocumentationRatio`.
    DocumentationRatio,
    /// `DebtMarkers`.
    DebtMarkers,
    /// `NamingSignals`.
    NamingSignals,
    /// `TestCount`.
    TestCount,
    /// `TestSourceRatio`.
    TestSourceRatio,
    /// `GeneratedSourceRatio`.
    GeneratedSourceRatio,
    /// `NondeterminismCount`.
    NondeterminismCount,
    /// `EffectCount`.
    EffectCount,
    /// `ReferenceCount`.
    ReferenceCount,
    /// `CallFanIn`.
    CallFanIn,
    /// `CallFanOut`.
    CallFanOut,
    /// `ImplementationCount`.
    ImplementationCount,
    /// `FunctionCount`.
    FunctionCount,
    /// `TypeCount`.
    TypeCount,
    /// `InterfaceCount`.
    InterfaceCount,
    /// `SymbolCount`.
    SymbolCount,
    /// `FileCount`.
    FileCount,
    /// `BranchDensity`.
    BranchDensity,
    /// `ExecutionCoverage`.
    ExecutionCoverage,
}

/// MetricSelection — `codegate_semantic.semantic.MetricSelection`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetricSelection {
    /// `kind` — `codegate_semantic.semantic.MetricKind`.
    pub kind: MetricKind,
    /// `aggregation` — `codegate_semantic.semantic.MetricAggregation`.
    pub aggregation: MetricAggregation,
    /// `threshold` — `Optional<Decimal>`.
    pub threshold: Option<crate::primitives::Decimal>,
    /// `effect` — `Optional<codegate_semantic.semantic.EffectKind>`.
    pub effect: Option<EffectKind>,
    /// `occurrence_role` — `Optional<codegate_semantic.semantic.OccurrenceRole>`.
    pub occurrence_role: Option<OccurrenceRole>,
    /// `counting_contract` — `String`.
    pub counting_contract: String,
    /// `required_families` — `List<codegate_semantic.semantic.FactFamily>`.
    pub required_families: Vec<FactFamily>,
}

/// NavigationResult — `codegate_semantic.semantic.NavigationResult`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavigationResult {
    /// `format` — `String`.
    pub format: String,
    /// `snapshot_id` — `codegate_semantic.semantic.SnapshotId`.
    pub snapshot_id: SnapshotId,
    /// `configuration_id` — `codegate_semantic.semantic.ConfigurationId`.
    pub configuration_id: ConfigurationId,
    /// `selector` — `codegate_semantic.semantic.LookupSelector`.
    pub selector: LookupSelector,
    /// `query` — `codegate_semantic.semantic.QueryKind`.
    pub query: QueryKind,
    /// `truncated` — `Boolean`.
    pub truncated: bool,
    /// `total_matches` — `Optional<Integer>`.
    pub total_matches: Option<i64>,
    /// `declarations` — `List<codegate_semantic.semantic.Declaration>`.
    pub declarations: Vec<Declaration>,
    /// `evidence` — `List<codegate_semantic.semantic.Evidence>`.
    pub evidence: Vec<Evidence>,
    /// `status` — `codegate_semantic.semantic.Completeness`.
    pub status: Completeness,
    /// `gaps` — `List<codegate_semantic.semantic.Gap>`.
    pub gaps: Vec<Gap>,
}

/// Occurrence — `codegate_semantic.semantic.Occurrence`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Occurrence {
    /// `owner` — `Optional<codegate_semantic.semantic.DeclarationId>`.
    pub owner: Option<DeclarationId>,
    /// `spelling` — `String`.
    pub spelling: String,
    /// `role` — `codegate_semantic.semantic.OccurrenceRole`.
    pub role: OccurrenceRole,
    /// `evidence` — `codegate_semantic.semantic.Evidence`.
    pub evidence: Evidence,
}

/// OccurrenceRole — `codegate_semantic.semantic.OccurrenceRole`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OccurrenceRole {
    /// `Definition`.
    Definition,
    /// `Read`.
    Read,
    /// `Write`.
    Write,
    /// `TypeUse`.
    TypeUse,
    /// `Import`.
    Import,
    /// `Invocation`.
    Invocation,
    /// `Annotation`.
    Annotation,
    /// `DocumentationReference`.
    DocumentationReference,
}

/// Policy — `codegate_semantic.semantic.Policy`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Policy {
    /// `format` — `String`.
    pub format: String,
    /// `id` — `String`.
    pub id: String,
    /// `version` — `String`.
    pub version: String,
    /// `expected_snapshot_id` — `codegate_semantic.semantic.SnapshotId`.
    pub expected_snapshot_id: SnapshotId,
    /// `expected_configuration_id` — `codegate_semantic.semantic.ConfigurationId`.
    pub expected_configuration_id: ConfigurationId,
    /// `evaluation_time` — `Timestamp`.
    pub evaluation_time: crate::primitives::Timestamp,
    /// `metrics` — `List<codegate_semantic.semantic.MetricSelection>`.
    pub metrics: Vec<MetricSelection>,
    /// `rules` — `List<codegate_semantic.semantic.PolicyRule>`.
    pub rules: Vec<PolicyRule>,
    /// `exceptions` — `List<codegate_semantic.semantic.PolicyException>`.
    pub exceptions: Vec<PolicyException>,
    /// `scoring_contract` — `String`.
    pub scoring_contract: String,
    /// `score_components` — `List<codegate_semantic.semantic.ScoreComponent>`.
    pub score_components: Vec<ScoreComponent>,
}

/// PolicyException — `codegate_semantic.semantic.PolicyException`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyException {
    /// `rule_id` — `String`.
    pub rule_id: String,
    /// `subjects` — `List<String>`.
    pub subjects: Vec<String>,
    /// `reason` — `String`.
    pub reason: String,
    /// `owner` — `String`.
    pub owner: String,
    /// `expires_at` — `Timestamp`.
    pub expires_at: crate::primitives::Timestamp,
}

/// PolicyRule — `codegate_semantic.semantic.PolicyRule`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyRule {
    /// `id` — `String`.
    pub id: String,
    /// `kind` — `codegate_semantic.semantic.CheckKind`.
    pub kind: CheckKind,
    /// `subjects` — `List<String>`.
    pub subjects: Vec<String>,
    /// `targets` — `List<String>`.
    pub targets: Vec<String>,
    /// `required_families` — `List<codegate_semantic.semantic.FactFamily>`.
    pub required_families: Vec<FactFamily>,
    /// `metric` — `Optional<codegate_semantic.semantic.MetricSelection>`.
    pub metric: Option<MetricSelection>,
    /// `maximum` — `Optional<Decimal>`.
    pub maximum: Option<crate::primitives::Decimal>,
    /// `effect` — `Optional<codegate_semantic.semantic.EffectKind>`.
    pub effect: Option<EffectKind>,
    /// `dependency_kind` — `Optional<codegate_semantic.semantic.DependencyKind>`.
    pub dependency_kind: Option<DependencyKind>,
    /// `severity` — `codegate_semantic.semantic.Severity`.
    pub severity: Severity,
}

/// Qualifier — `codegate_semantic.semantic.Qualifier`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Qualifier {
    /// `annotation_name` — `String`.
    pub annotation_name: String,
    /// `member_name` — `String`.
    pub member_name: String,
    /// `member_value` — `String`.
    pub member_value: String,
    /// `nonbinding` — `Boolean`.
    pub nonbinding: bool,
}

/// QueryKind — `codegate_semantic.semantic.QueryKind`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryKind {
    /// `Definitions`.
    Definitions,
    /// `References`.
    References,
    /// `Implementations`.
    Implementations,
    /// `Callers`.
    Callers,
    /// `Callees`.
    Callees,
    /// `FrameworkRelationships`.
    FrameworkRelationships,
}

/// Reference — `codegate_semantic.semantic.Reference`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reference {
    /// `occurrence_id` — `codegate_semantic.semantic.FactId`.
    pub occurrence_id: FactId,
    /// `target` — `Optional<codegate_semantic.semantic.DeclarationId>`.
    pub target: Option<DeclarationId>,
    /// `candidates` — `List<codegate_semantic.semantic.DeclarationId>`.
    pub candidates: Vec<DeclarationId>,
    /// `evidence` — `codegate_semantic.semantic.Evidence`.
    pub evidence: Evidence,
}

/// Resolution — `codegate_semantic.semantic.Resolution`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resolution {
    /// `SyntacticCandidate`.
    SyntacticCandidate,
    /// `Resolved`.
    Resolved,
    /// `Unresolved`.
    Unresolved,
}

/// ResolutionBasis — `codegate_semantic.semantic.ResolutionBasis`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResolutionBasis {
    /// `SourceStructure`.
    SourceStructure,
    /// `LanguageServer`.
    LanguageServer,
    /// `DeclaredFramework`.
    DeclaredFramework,
    /// `MatchedBuildEvidence`.
    MatchedBuildEvidence,
}

/// Score — `codegate_semantic.semantic.Score`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Score {
    /// `component_id` — `String`.
    pub component_id: String,
    /// `value` — `Optional<Decimal>`.
    pub value: Option<crate::primitives::Decimal>,
    /// `status` — `codegate_semantic.semantic.Completeness`.
    pub status: Completeness,
    /// `gaps` — `List<codegate_semantic.semantic.Gap>`.
    pub gaps: Vec<Gap>,
}

/// ScoreComponent — `codegate_semantic.semantic.ScoreComponent`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScoreComponent {
    /// `id` — `String`.
    pub id: String,
    /// `rule_ids` — `List<String>`.
    pub rule_ids: Vec<String>,
    /// `weight` — `Decimal`.
    pub weight: crate::primitives::Decimal,
    /// `required` — `Boolean`.
    pub required: bool,
}

/// SelectorKind — `codegate_semantic.semantic.SelectorKind`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectorKind {
    /// `Name`.
    Name,
    /// `QualifiedName`.
    QualifiedName,
    /// `SourcePosition`.
    SourcePosition,
}

/// Severity — `codegate_semantic.semantic.Severity`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// `Info`.
    Info,
    /// `Warning`.
    Warning,
    /// `Error`.
    Error,
}

/// SnapshotId — `codegate_semantic.semantic.SnapshotId`: a distinct wrapper around `String`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SnapshotId(pub String);

/// SourceClass — `codegate_semantic.semantic.SourceClass`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceClass {
    /// `Production`.
    Production,
    /// `Test`.
    Test,
    /// `Generated`.
    Generated,
    /// `GeneratedTest`.
    GeneratedTest,
    /// `BuildConfiguration`.
    BuildConfiguration,
}

/// SourceFile — `codegate_semantic.semantic.SourceFile`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceFile {
    /// `path` — `String`.
    pub path: String,
    /// `content_sha256` — `String`.
    pub content_sha256: String,
    /// `byte_length` — `Integer`.
    pub byte_length: i64,
    /// `language` — `Optional<codegate_semantic.semantic.Language>`.
    pub language: Option<Language>,
    /// `classification` — `codegate_semantic.semantic.SourceClass`.
    pub classification: SourceClass,
    /// `origin` — `codegate_semantic.semantic.SourceOrigin`.
    pub origin: SourceOrigin,
    /// `lines` — `List<codegate_semantic.semantic.SourceLine>`.
    pub lines: Vec<SourceLine>,
}

/// SourceLine — `codegate_semantic.semantic.SourceLine`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceLine {
    /// `number` — `Integer`.
    pub number: i64,
    /// `start_byte` — `Integer`.
    pub start_byte: i64,
    /// `end_byte` — `Integer`.
    pub end_byte: i64,
    /// `classification` — `codegate_semantic.semantic.LineClass`.
    pub classification: LineClass,
}

/// SourceOrigin — `codegate_semantic.semantic.SourceOrigin`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceOrigin {
    /// `TrackedClean`.
    TrackedClean,
    /// `TrackedDirty`.
    TrackedDirty,
    /// `Untracked`.
    Untracked,
    /// `ExternalBuildEvidence`.
    ExternalBuildEvidence,
}

/// SourceRange — `codegate_semantic.semantic.SourceRange`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceRange {
    /// `path` — `String`.
    pub path: String,
    /// `start_byte` — `Integer`.
    pub start_byte: i64,
    /// `end_byte` — `Integer`.
    pub end_byte: i64,
    /// `start_line` — `Integer`.
    pub start_line: i64,
    /// `start_column` — `Integer`.
    pub start_column: i64,
    /// `end_line` — `Integer`.
    pub end_line: i64,
    /// `end_column` — `Integer`.
    pub end_column: i64,
}

/// SourceSelection — `codegate_semantic.semantic.SourceSelection`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceSelection {
    /// `include_paths` — `List<String>`.
    pub include_paths: Vec<String>,
    /// `exclude_paths` — `List<String>`.
    pub exclude_paths: Vec<String>,
    /// `include_tests` — `Boolean`.
    pub include_tests: bool,
    /// `include_generated` — `Boolean`.
    pub include_generated: bool,
    /// `languages` — `List<codegate_semantic.semantic.Language>`.
    pub languages: Vec<Language>,
}

/// SourceSnapshot — `codegate_semantic.semantic.SourceSnapshot`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceSnapshot {
    /// `id` — `codegate_semantic.semantic.SnapshotId`.
    pub id: SnapshotId,
    /// `selection` — `codegate_semantic.semantic.SourceSelection`.
    pub selection: SourceSelection,
    /// `configuration` — `codegate_semantic.semantic.BuildSelection`.
    pub configuration: BuildSelection,
    /// `files` — `List<codegate_semantic.semantic.SourceFile>`.
    pub files: Vec<SourceFile>,
    /// `manifests` — `List<codegate_semantic.semantic.Manifest>`.
    pub manifests: Vec<Manifest>,
}

/// StructureKind — `codegate_semantic.semantic.StructureKind`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StructureKind {
    /// `Body`.
    Body,
    /// `Parameter`.
    Parameter,
    /// `ReturnStatement`.
    ReturnStatement,
    /// `NestingRegion`.
    NestingRegion,
    /// `Field`.
    Field,
    /// `InterfaceMember`.
    InterfaceMember,
    /// `PublicApiMember`.
    PublicApiMember,
    /// `Documentation`.
    Documentation,
    /// `DebtMarker`.
    DebtMarker,
    /// `NamingSignal`.
    NamingSignal,
    /// `Loop`.
    Loop,
    /// `Allocation`.
    Allocation,
    /// `TestCase`.
    TestCase,
}

/// StructureObservation — `codegate_semantic.semantic.StructureObservation`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructureObservation {
    /// `owner` — `codegate_semantic.semantic.DeclarationId`.
    pub owner: DeclarationId,
    /// `kind` — `codegate_semantic.semantic.StructureKind`.
    pub kind: StructureKind,
    /// `parent_observation` — `Optional<codegate_semantic.semantic.FactId>`.
    pub parent_observation: Option<FactId>,
    /// `test_kind` — `Optional<codegate_semantic.semantic.TestKind>`.
    pub test_kind: Option<TestKind>,
    /// `text` — `Optional<String>`.
    pub text: Option<String>,
    /// `evidence` — `codegate_semantic.semantic.Evidence`.
    pub evidence: Evidence,
}

/// Suggestion — `codegate_semantic.semantic.Suggestion`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Suggestion {
    /// `finding_ids` — `List<String>`.
    pub finding_ids: Vec<String>,
    /// `summary` — `String`.
    pub summary: String,
    /// `rationale` — `String`.
    pub rationale: String,
    /// `rank` — `Integer`.
    pub rank: i64,
}

/// TestKind — `codegate_semantic.semantic.TestKind`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestKind {
    /// `Unit`.
    Unit,
    /// `Integration`.
    Integration,
    /// `TableDriven`.
    TableDriven,
    /// `Parameterized`.
    Parameterized,
    /// `Benchmark`.
    Benchmark,
    /// `Fuzz`.
    Fuzz,
}

/// ToolObservation — `codegate_semantic.semantic.ToolObservation`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ToolObservation {
    /// `tool` — `String`.
    pub tool: String,
    /// `version` — `String`.
    pub version: String,
    /// `host_runtime_version` — `Optional<String>`.
    pub host_runtime_version: Option<String>,
    /// `configuration_id` — `codegate_semantic.semantic.ConfigurationId`.
    pub configuration_id: ConfigurationId,
    /// `snapshot_id` — `codegate_semantic.semantic.SnapshotId`.
    pub snapshot_id: SnapshotId,
    /// `elapsed_milliseconds` — `Integer`.
    pub elapsed_milliseconds: i64,
    /// `exit_code` — `Optional<Integer>`.
    pub exit_code: Option<i64>,
    /// `timed_out` — `Boolean`.
    pub timed_out: bool,
    /// `diagnostics` — `List<String>`.
    pub diagnostics: Vec<String>,
}

/// Unit — `codegate_semantic.semantic.Unit`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unit {
    /// `id` — `codegate_semantic.semantic.UnitId`.
    pub id: UnitId,
    /// `name` — `String`.
    pub name: String,
    /// `language` — `codegate_semantic.semantic.Language`.
    pub language: Language,
    /// `paths` — `List<String>`.
    pub paths: Vec<String>,
    /// `evidence` — `codegate_semantic.semantic.Evidence`.
    pub evidence: Evidence,
}

/// UnitId — `codegate_semantic.semantic.UnitId`: a distinct wrapper around `String`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitId(pub String);

/// Verdict — `codegate_semantic.semantic.Verdict`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    /// `Pass`.
    Pass,
    /// `Fail`.
    Fail,
    /// `Incomplete`.
    Incomplete,
    /// `Unsupported`.
    Unsupported,
    /// `Error`.
    Error,
    /// `NotApplicable`.
    NotApplicable,
}

/// Visibility — `codegate_semantic.semantic.Visibility`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Visibility {
    /// `Public`.
    Public,
    /// `Protected`.
    Protected,
    /// `Package`.
    Package,
    /// `Internal`.
    Internal,
    /// `Private`.
    Private,
    /// `Unknown`.
    Unknown,
}

/// Assess — the input of `codegate_semantic.semantic.Assess`.
///
/// Everything it can result in is [`AssessOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assess {
    /// `request` — `codegate_semantic.semantic.CollectionRequest`.
    pub request: CollectionRequest,
    /// `policy` — `codegate_semantic.semantic.Policy`.
    pub policy: Policy,
}

/// Actual typed response of `codegate_semantic.semantic.Assess`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssessResponse {
    /// `report` — `codegate_semantic.semantic.Assessment`.
    pub report: Assessment,
}

/// Everything `codegate_semantic.semantic.Assess` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssessOutcome {
    /// `assessed` — otherwise.
    Assessed,
}

/// Capabilities — the input of `codegate_semantic.semantic.Capabilities`.
///
/// Everything it can result in is [`CapabilitiesOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Capabilities {
    /// `configuration` — `codegate_semantic.semantic.BuildSelection`.
    pub configuration: BuildSelection,
    /// `mode` — `codegate_semantic.semantic.CollectionMode`.
    pub mode: CollectionMode,
}

/// Actual typed response of `codegate_semantic.semantic.Capabilities`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CapabilitiesResponse {
    /// `capabilities` — `List<codegate_semantic.semantic.Capability>`.
    pub capabilities: Vec<Capability>,
}

/// Everything `codegate_semantic.semantic.Capabilities` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CapabilitiesOutcome {
    /// `listed` — otherwise.
    Listed,
}

/// Collect — the input of `codegate_semantic.semantic.Collect`.
///
/// Everything it can result in is [`CollectOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Collect {
    /// `request` — `codegate_semantic.semantic.CollectionRequest`.
    pub request: CollectionRequest,
}

/// Actual typed response of `codegate_semantic.semantic.Collect`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CollectResponse {
    /// `snapshot` — `codegate_semantic.semantic.FactSnapshot`.
    pub snapshot: FactSnapshot,
}

/// Everything `codegate_semantic.semantic.Collect` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CollectOutcome {
    /// `collected` — otherwise.
    Collected,
}

/// CollectSource — the input of `codegate_semantic.semantic.CollectSource`.
///
/// Everything it can result in is [`CollectSourceOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CollectSource {
    /// `request` — `codegate_semantic.semantic.CollectionRequest`.
    pub request: CollectionRequest,
}

/// Actual typed response of `codegate_semantic.semantic.CollectSource`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CollectSourceResponse {
    /// `source` — `Optional<codegate_semantic.semantic.SourceSnapshot>`.
    pub source: Option<SourceSnapshot>,
    /// `gaps` — `List<codegate_semantic.semantic.Gap>`.
    pub gaps: Vec<Gap>,
}

/// Everything `codegate_semantic.semantic.CollectSource` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CollectSourceOutcome {
    /// `source-observed` — otherwise.
    SourceObserved,
}

/// Evaluate — the input of `codegate_semantic.semantic.Evaluate`.
///
/// Everything it can result in is [`EvaluateOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Evaluate {
    /// `snapshot` — `codegate_semantic.semantic.FactSnapshot`.
    pub snapshot: FactSnapshot,
    /// `policy` — `codegate_semantic.semantic.Policy`.
    pub policy: Policy,
}

/// Actual typed response of `codegate_semantic.semantic.Evaluate`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvaluateResponse {
    /// `report` — `codegate_semantic.semantic.Assessment`.
    pub report: Assessment,
}

/// Everything `codegate_semantic.semantic.Evaluate` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvaluateOutcome {
    /// `evaluated` — otherwise.
    Evaluated,
}

/// Lookup — the input of `codegate_semantic.semantic.Lookup`.
///
/// Everything it can result in is [`LookupOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lookup {
    /// `snapshot` — `codegate_semantic.semantic.FactSnapshot`.
    pub snapshot: FactSnapshot,
    /// `selector` — `codegate_semantic.semantic.LookupSelector`.
    pub selector: LookupSelector,
    /// `query` — `codegate_semantic.semantic.QueryKind`.
    pub query: QueryKind,
}

/// Actual typed response of `codegate_semantic.semantic.Lookup`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LookupResponse {
    /// `result` — `codegate_semantic.semantic.NavigationResult`.
    pub result: NavigationResult,
}

/// Everything `codegate_semantic.semantic.Lookup` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LookupOutcome {
    /// `looked-up` — otherwise.
    LookedUp,
}

/// Suggest — the input of `codegate_semantic.semantic.Suggest`.
///
/// Everything it can result in is [`SuggestOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Suggest {
    /// `report` — `codegate_semantic.semantic.Assessment`.
    pub report: Assessment,
}

/// Actual typed response of `codegate_semantic.semantic.Suggest`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SuggestResponse {
    /// `suggestions` — `List<codegate_semantic.semantic.Suggestion>`.
    pub suggestions: Vec<Suggestion>,
}

/// Everything `codegate_semantic.semantic.Suggest` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SuggestOutcome {
    /// `suggested` — otherwise.
    Suggested,
}

/// ValidateSnapshot — the input of `codegate_semantic.semantic.ValidateSnapshot`.
///
/// Everything it can result in is [`ValidateSnapshotOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateSnapshot {
    /// `snapshot` — `codegate_semantic.semantic.FactSnapshot`.
    pub snapshot: FactSnapshot,
}

/// Actual typed response of `codegate_semantic.semantic.ValidateSnapshot`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidateSnapshotResponse {
    /// `accepted` — `Boolean`.
    pub accepted: bool,
    /// `gaps` — `List<codegate_semantic.semantic.Gap>`.
    pub gaps: Vec<Gap>,
}

/// Everything `codegate_semantic.semantic.ValidateSnapshot` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ValidateSnapshotOutcome {
    /// `validation-observed` — otherwise.
    ValidationObserved,
}

/// What this bounded context owes its implementor, as typed seams.
///
/// One trait per obligation in the synthesis plan, each carrying the plan's own contract.
/// [`Unimplemented`](obligations::Unimplemented) satisfies every trait by refusing in the type system, so the workspace builds —
/// and says exactly what it cannot yet do — before a line is hand-written.
pub mod obligations {
    /// The behaviour `codegate_semantic.semantic.Assess` — an implementation obligation.
    ///
    /// Why it is not generated: kept an obligation by a typed response (`response:`).
    ///
    /// Contract: given `codegate_semantic.semantic.Assess` input, decide and enact exactly one outcome — `assessed` otherwise.
    pub trait AssessBehavior {
        /// Decides and enacts exactly one declared outcome of `codegate_semantic.semantic.Assess`.
        ///
        /// `Err` is the typed refusal of an obligation nothing has satisfied; a satisfying
        /// implementation never returns it.
        fn assess(
            &mut self,
            input: super::Assess,
        ) -> Result<super::AssessOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `codegate_semantic.semantic.Capabilities` — an implementation obligation.
    ///
    /// Why it is not generated: kept an obligation by a typed response (`response:`).
    ///
    /// Contract: given `codegate_semantic.semantic.Capabilities` input, decide and enact exactly one outcome — `listed` otherwise.
    pub trait CapabilitiesBehavior {
        /// Decides and enacts exactly one declared outcome of `codegate_semantic.semantic.Capabilities`.
        ///
        /// `Err` is the typed refusal of an obligation nothing has satisfied; a satisfying
        /// implementation never returns it.
        fn capabilities(
            &mut self,
            input: super::Capabilities,
        ) -> Result<super::CapabilitiesOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `codegate_semantic.semantic.Collect` — an implementation obligation.
    ///
    /// Why it is not generated: kept an obligation by a typed response (`response:`).
    ///
    /// Contract: given `codegate_semantic.semantic.Collect` input, decide and enact exactly one outcome — `collected` otherwise.
    pub trait CollectBehavior {
        /// Decides and enacts exactly one declared outcome of `codegate_semantic.semantic.Collect`.
        ///
        /// `Err` is the typed refusal of an obligation nothing has satisfied; a satisfying
        /// implementation never returns it.
        fn collect(
            &mut self,
            input: super::Collect,
        ) -> Result<super::CollectOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `codegate_semantic.semantic.CollectSource` — an implementation obligation.
    ///
    /// Why it is not generated: kept an obligation by a typed response (`response:`).
    ///
    /// Contract: given `codegate_semantic.semantic.CollectSource` input, decide and enact exactly one outcome — `source-observed` otherwise.
    pub trait CollectSourceBehavior {
        /// Decides and enacts exactly one declared outcome of `codegate_semantic.semantic.CollectSource`.
        ///
        /// `Err` is the typed refusal of an obligation nothing has satisfied; a satisfying
        /// implementation never returns it.
        fn collect_source(
            &mut self,
            input: super::CollectSource,
        ) -> Result<super::CollectSourceOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `codegate_semantic.semantic.Evaluate` — an implementation obligation.
    ///
    /// Why it is not generated: kept an obligation by a typed response (`response:`).
    ///
    /// Contract: given `codegate_semantic.semantic.Evaluate` input, decide and enact exactly one outcome — `evaluated` otherwise.
    pub trait EvaluateBehavior {
        /// Decides and enacts exactly one declared outcome of `codegate_semantic.semantic.Evaluate`.
        ///
        /// `Err` is the typed refusal of an obligation nothing has satisfied; a satisfying
        /// implementation never returns it.
        fn evaluate(
            &mut self,
            input: super::Evaluate,
        ) -> Result<super::EvaluateOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `codegate_semantic.semantic.Lookup` — an implementation obligation.
    ///
    /// Why it is not generated: kept an obligation by a typed response (`response:`).
    ///
    /// Contract: given `codegate_semantic.semantic.Lookup` input, decide and enact exactly one outcome — `looked-up` otherwise.
    pub trait LookupBehavior {
        /// Decides and enacts exactly one declared outcome of `codegate_semantic.semantic.Lookup`.
        ///
        /// `Err` is the typed refusal of an obligation nothing has satisfied; a satisfying
        /// implementation never returns it.
        fn lookup(
            &mut self,
            input: super::Lookup,
        ) -> Result<super::LookupOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `codegate_semantic.semantic.Suggest` — an implementation obligation.
    ///
    /// Why it is not generated: kept an obligation by a typed response (`response:`).
    ///
    /// Contract: given `codegate_semantic.semantic.Suggest` input, decide and enact exactly one outcome — `suggested` otherwise.
    pub trait SuggestBehavior {
        /// Decides and enacts exactly one declared outcome of `codegate_semantic.semantic.Suggest`.
        ///
        /// `Err` is the typed refusal of an obligation nothing has satisfied; a satisfying
        /// implementation never returns it.
        fn suggest(
            &mut self,
            input: super::Suggest,
        ) -> Result<super::SuggestOutcome, crate::obligation::UnmetObligation>;
    }

    /// The behaviour `codegate_semantic.semantic.ValidateSnapshot` — an implementation obligation.
    ///
    /// Why it is not generated: kept an obligation by a typed response (`response:`).
    ///
    /// Contract: given `codegate_semantic.semantic.ValidateSnapshot` input, decide and enact exactly one outcome — `validation-observed` otherwise.
    pub trait ValidateSnapshotBehavior {
        /// Decides and enacts exactly one declared outcome of `codegate_semantic.semantic.ValidateSnapshot`.
        ///
        /// `Err` is the typed refusal of an obligation nothing has satisfied; a satisfying
        /// implementation never returns it.
        fn validate_snapshot(
            &mut self,
            input: super::ValidateSnapshot,
        ) -> Result<super::ValidateSnapshotOutcome, crate::obligation::UnmetObligation>;
    }

    /// Every obligation of this bounded context, refused in the type system.
    ///
    /// Each method returns the typed refusal naming what is owed — never a panic, never a guessed
    /// value — so a workspace built on this stub compiles and reports its own gaps.
    pub struct Unimplemented;

    impl AssessBehavior for Unimplemented {
        fn assess(
            &mut self,
            _input: super::Assess,
        ) -> Result<super::AssessOutcome, crate::obligation::UnmetObligation> {
            Err(crate::obligation::UnmetObligation {
                capability: "command behaviour",
                source: "codegate_semantic.semantic.Assess",
            })
        }
    }

    impl CapabilitiesBehavior for Unimplemented {
        fn capabilities(
            &mut self,
            _input: super::Capabilities,
        ) -> Result<super::CapabilitiesOutcome, crate::obligation::UnmetObligation> {
            Err(crate::obligation::UnmetObligation {
                capability: "command behaviour",
                source: "codegate_semantic.semantic.Capabilities",
            })
        }
    }

    impl CollectBehavior for Unimplemented {
        fn collect(
            &mut self,
            _input: super::Collect,
        ) -> Result<super::CollectOutcome, crate::obligation::UnmetObligation> {
            Err(crate::obligation::UnmetObligation {
                capability: "command behaviour",
                source: "codegate_semantic.semantic.Collect",
            })
        }
    }

    impl CollectSourceBehavior for Unimplemented {
        fn collect_source(
            &mut self,
            _input: super::CollectSource,
        ) -> Result<super::CollectSourceOutcome, crate::obligation::UnmetObligation> {
            Err(crate::obligation::UnmetObligation {
                capability: "command behaviour",
                source: "codegate_semantic.semantic.CollectSource",
            })
        }
    }

    impl EvaluateBehavior for Unimplemented {
        fn evaluate(
            &mut self,
            _input: super::Evaluate,
        ) -> Result<super::EvaluateOutcome, crate::obligation::UnmetObligation> {
            Err(crate::obligation::UnmetObligation {
                capability: "command behaviour",
                source: "codegate_semantic.semantic.Evaluate",
            })
        }
    }

    impl LookupBehavior for Unimplemented {
        fn lookup(
            &mut self,
            _input: super::Lookup,
        ) -> Result<super::LookupOutcome, crate::obligation::UnmetObligation> {
            Err(crate::obligation::UnmetObligation {
                capability: "command behaviour",
                source: "codegate_semantic.semantic.Lookup",
            })
        }
    }

    impl SuggestBehavior for Unimplemented {
        fn suggest(
            &mut self,
            _input: super::Suggest,
        ) -> Result<super::SuggestOutcome, crate::obligation::UnmetObligation> {
            Err(crate::obligation::UnmetObligation {
                capability: "command behaviour",
                source: "codegate_semantic.semantic.Suggest",
            })
        }
    }

    impl ValidateSnapshotBehavior for Unimplemented {
        fn validate_snapshot(
            &mut self,
            _input: super::ValidateSnapshot,
        ) -> Result<super::ValidateSnapshotOutcome, crate::obligation::UnmetObligation> {
            Err(crate::obligation::UnmetObligation {
                capability: "command behaviour",
                source: "codegate_semantic.semantic.ValidateSnapshot",
            })
        }
    }
}
