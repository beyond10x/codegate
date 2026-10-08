// generated from codegate v1
// model digest e27ccc45957cf4fe2ef83d9362e81d867eb46bff9a72f1fd28e43b69bf53ec93
// contract digest df71e33e5abc7d133716034004ba68a3a08d1c620351660f3940aa973a3c93b8
// do not edit: regenerate with `ess synthesize --layout crate`

//! dependency — `codegate.dependency`.
//!
//! Immutable dependency fact values and an offline, language-independent evaluation.
//!
//! Everything this bounded context declares that the synthesis plan marks generated.

/// Coverage — `codegate.dependency.Coverage`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Coverage {
    /// `status` — `codegate.dependency.CoverageStatus`.
    pub status: CoverageStatus,
    /// `gaps` — `List<String>`.
    pub gaps: Vec<String>,
}

/// CoverageStatus — `codegate.dependency.CoverageStatus`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoverageStatus {
    /// `Complete`.
    Complete,
    /// `Partial`.
    Partial,
    /// `Unsupported`.
    Unsupported,
    /// `Failed`.
    Failed,
}

/// DependencyKind — `codegate.dependency.DependencyKind`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DependencyKind {
    /// `Runtime`.
    Runtime,
    /// `Build`.
    Build,
    /// `Test`.
    Test,
}

/// Diagnostic — `codegate.dependency.Diagnostic`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    /// `code` — `codegate.dependency.DiagnosticCode`.
    pub code: DiagnosticCode,
    /// `subject` — `String`.
    pub subject: String,
}

/// DiagnosticCode — `codegate.dependency.DiagnosticCode`: one of a closed set of names.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticCode {
    /// `InvalidFormat`.
    InvalidFormat,
    /// `EmptyIdentity`.
    EmptyIdentity,
    /// `DuplicateUnit`.
    DuplicateUnit,
    /// `DuplicateEdge`.
    DuplicateEdge,
    /// `DanglingSource`.
    DanglingSource,
    /// `DanglingTarget`.
    DanglingTarget,
    /// `InvalidTarget`.
    InvalidTarget,
    /// `InvalidCoverage`.
    InvalidCoverage,
    /// `InvalidPolicy`.
    InvalidPolicy,
    /// `SourceMismatch`.
    SourceMismatch,
    /// `ConfigurationMismatch`.
    ConfigurationMismatch,
    /// `PartialCoverage`.
    PartialCoverage,
    /// `UnsupportedCoverage`.
    UnsupportedCoverage,
    /// `FailedCoverage`.
    FailedCoverage,
}

/// Edge — `codegate.dependency.Edge`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Edge {
    /// `id` — `codegate.dependency.EdgeId`.
    pub id: EdgeId,
    /// `source` — `codegate.dependency.UnitId`.
    pub source: UnitId,
    /// `target` — `Optional<codegate.dependency.UnitId>`.
    pub target: Option<UnitId>,
    /// `unresolved_target` — `Optional<String>`.
    pub unresolved_target: Option<String>,
    /// `kind` — `codegate.dependency.DependencyKind`.
    pub kind: DependencyKind,
}

/// EdgeId — `codegate.dependency.EdgeId`: a distinct wrapper around `String`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EdgeId(pub String);

/// Evaluation — `codegate.dependency.Evaluation`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Evaluation {
    /// `format` — `String`.
    pub format: String,
    /// `source_id` — `String`.
    pub source_id: String,
    /// `configuration_id` — `String`.
    pub configuration_id: String,
    /// `policy` — `codegate.dependency.Policy`.
    pub policy: Policy,
    /// `metric_id` — `String`.
    pub metric_id: String,
    /// `checker_id` — `String`.
    pub checker_id: String,
    /// `coverage` — `codegate.dependency.CoverageStatus`.
    pub coverage: CoverageStatus,
    /// `verdict` — `codegate.dependency.Verdict`.
    pub verdict: Verdict,
    /// `fan_out` — `List<codegate.dependency.FanOut>`.
    pub fan_out: Vec<FanOut>,
    /// `findings` — `List<codegate.dependency.Finding>`.
    pub findings: Vec<Finding>,
    /// `diagnostics` — `List<codegate.dependency.Diagnostic>`.
    pub diagnostics: Vec<Diagnostic>,
}

/// FactSnapshot — `codegate.dependency.FactSnapshot`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FactSnapshot {
    /// `format` — `String`.
    pub format: String,
    /// `source_id` — `String`.
    pub source_id: String,
    /// `configuration_id` — `String`.
    pub configuration_id: String,
    /// `producer` — `String`.
    pub producer: String,
    /// `producer_version` — `String`.
    pub producer_version: String,
    /// `units` — `List<codegate.dependency.Unit>`.
    pub units: Vec<Unit>,
    /// `edges` — `List<codegate.dependency.Edge>`.
    pub edges: Vec<Edge>,
    /// `coverage` — `codegate.dependency.Coverage`.
    pub coverage: Coverage,
}

/// FanOut — `codegate.dependency.FanOut`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FanOut {
    /// `unit_id` — `codegate.dependency.UnitId`.
    pub unit_id: UnitId,
    /// `value` — `Optional<Integer>`.
    pub value: Option<i64>,
}

/// Finding — `codegate.dependency.Finding`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    /// `edge_id` — `codegate.dependency.EdgeId`.
    pub edge_id: EdgeId,
    /// `source` — `codegate.dependency.UnitId`.
    pub source: UnitId,
    /// `target` — `codegate.dependency.UnitId`.
    pub target: UnitId,
    /// `kind` — `codegate.dependency.DependencyKind`.
    pub kind: DependencyKind,
}

/// ForbiddenDependency — `codegate.dependency.ForbiddenDependency`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForbiddenDependency {
    /// `source` — `codegate.dependency.UnitId`.
    pub source: UnitId,
    /// `target` — `codegate.dependency.UnitId`.
    pub target: UnitId,
    /// `kind` — `codegate.dependency.DependencyKind`.
    pub kind: DependencyKind,
}

/// Policy — `codegate.dependency.Policy`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Policy {
    /// `format` — `String`.
    pub format: String,
    /// `expected_source_id` — `String`.
    pub expected_source_id: String,
    /// `expected_configuration_id` — `String`.
    pub expected_configuration_id: String,
    /// `dependency_kind` — `codegate.dependency.DependencyKind`.
    pub dependency_kind: DependencyKind,
    /// `forbidden` — `List<codegate.dependency.ForbiddenDependency>`.
    pub forbidden: Vec<ForbiddenDependency>,
}

/// Unit — `codegate.dependency.Unit`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unit {
    /// `id` — `codegate.dependency.UnitId`.
    pub id: UnitId,
    /// `language` — `String`.
    pub language: String,
}

/// UnitId — `codegate.dependency.UnitId`: a distinct wrapper around `String`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitId(pub String);

/// Verdict — `codegate.dependency.Verdict`: one of a closed set of names.
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
}

/// Evaluate — the input of `codegate.dependency.Evaluate`.
///
/// Everything it can result in is [`EvaluateOutcome`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Evaluate {
    /// `snapshot` — `codegate.dependency.FactSnapshot`.
    pub snapshot: FactSnapshot,
    /// `policy` — `codegate.dependency.Policy`.
    pub policy: Policy,
}

/// Actual typed response of `codegate.dependency.Evaluate`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvaluateResponse {
    /// `report` — `codegate.dependency.Evaluation`.
    pub report: Evaluation,
}

/// Everything `codegate.dependency.Evaluate` can result in — one variant per declared outcome.
///
/// An infrastructure failure is deliberately not in here: a refusal is a fact about the domain,
/// a transport fault is a fact about the run, and conflating the two is what the declared
/// outcomes exist to prevent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EvaluateOutcome {
    /// `evaluated` — otherwise.
    Evaluated,
}

/// What this bounded context owes its implementor, as typed seams.
///
/// One trait per obligation in the synthesis plan, each carrying the plan's own contract.
/// [`Unimplemented`](obligations::Unimplemented) satisfies every trait by refusing in the type system, so the workspace builds —
/// and says exactly what it cannot yet do — before a line is hand-written.
pub mod obligations {
    /// The behaviour `codegate.dependency.Evaluate` — an implementation obligation.
    ///
    /// Why it is not generated: kept an obligation by a typed response (`response:`).
    ///
    /// Contract: given `codegate.dependency.Evaluate` input, decide and enact exactly one outcome. Declared outcomes (declaration order, not selection precedence): `evaluated` otherwise.
    pub trait EvaluateBehavior {
        /// Decides and enacts exactly one declared outcome of `codegate.dependency.Evaluate`.
        ///
        /// `Err` is the typed refusal of an obligation nothing has satisfied; a satisfying
        /// implementation never returns it.
        fn evaluate(
            &mut self,
            input: super::Evaluate,
        ) -> Result<super::EvaluateOutcome, crate::obligation::UnmetObligation>;
    }

    /// Every obligation of this bounded context, refused in the type system.
    ///
    /// Each method returns the typed refusal naming what is owed — never a panic, never a guessed
    /// value — so a workspace built on this stub compiles and reports its own gaps.
    pub struct Unimplemented;

    impl EvaluateBehavior for Unimplemented {
        fn evaluate(
            &mut self,
            _input: super::Evaluate,
        ) -> Result<super::EvaluateOutcome, crate::obligation::UnmetObligation> {
            Err(crate::obligation::UnmetObligation {
                capability: "command behaviour",
                source: "codegate.dependency.Evaluate",
            })
        }
    }
}
