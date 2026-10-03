# Semantic contract v1 foundation

Status: generated Rust data and behavior contracts, with source capture, admission
and the public Collect composition implemented. Source-language bindings, semantic
adapters, semantic evaluation and semantic navigation remain delivery gaps. Five
public behavior traits remain runtime obligations. Planned acceptance is
[semantic-scenarios.md](semantic-scenarios.md).
ESS validation and synthesis are not evidence that these runtime requirements pass.

## Boundary and identities

`ess-semantic/` specifies `codegate_semantic v1`, with the immutable value domain
`codegate_semantic.semantic`. It is separate from `ess/`, its dependency `/0.1`
wire contracts and its 27 existing evaluator scenarios. Rich snapshots use
`codegate.semantic-facts/1`; policies use `codegate.semantic-policy/1`; assessments
use `codegate.semantic-assessment/1`; navigation uses `codegate.semantic-navigation/1`.
Unknown format versions are refused. A new meaning or counting rule requires a new
version, never an in-place reinterpretation of stored evidence.

These are values, not persisted entities. IDs identify values inside one snapshot;
containment is typed composition. No lifecycle or persistence service is implied.
The Rust library and JSON CLI implement the same six generated commands: Collect,
Evaluate, Assess, Lookup, Suggest and Capabilities. CLI names are lowercase. Source
editing, executable refactoring, MCP, hosting and Markdown analysis are excluded.

Collect defaults to SourceOnly at the presentation boundary. The typed request
always contains an explicit mode. Semantic mode invokes only requested, installed
tools and preserves tool identity/version, host runtime, selected configuration,
diagnostics, timeout and exit status. Requested semantic evidence never silently
falls back to source-only success. Evaluate and Lookup on imported snapshots and
Suggest on imported reports perform no process execution, filesystem access or
network access. Assess composes Collect and Evaluate against the same snapshot.

Only admission creates the private admitted wrapper. Shared metrics and policies
consume it and normalized facts. They cannot import bindings or IO, parse source,
invoke tools, or branch on language, producer or annotation spelling. Applicability
and language-specific observations are established before shared analysis.

## Internal foundation library commands

`CollectSource` and `ValidateSnapshot` are intermediate Rust library seams, not
additional JSON CLI commands and not substitutes for the six public commands.
The `source-foundation` component accepts only these two internal commands.
Their response fields reuse the semantic value types; existing snapshot and
public command contracts do not change.

`CollectSource` consumes `CollectionRequest` and returns `source-observed` with
`source: Optional<SourceSnapshot>` and `gaps: List<Gap>`. Only SourceOnly is
supported. An explicit Semantic request returns no source and an
UnsupportedCapability gap; it never silently downgrades. Successful bounded
collection returns the actual source snapshot and all observed gaps. A refusal
such as path escape, invalid selection, exceeded bounds or non-UTF-8 selected
content returns no source and nonempty gaps. A source may carry partial evidence,
but its gaps cannot be discarded or translated into complete semantic coverage.
The response does not fabricate units, declarations or other FactSnapshot facts.

`ValidateSnapshot` consumes `FactSnapshot` and returns `validation-observed` with
`accepted: Boolean` and `gaps: List<Gap>`. Acceptance is true exactly when real
admission validation succeeds; on success the response gaps are empty, while
refusal returns false with the actual nonempty admission diagnostics in stable
order. Valid partial snapshots may be accepted without becoming complete; their
existing evidence gaps remain inside the input snapshot. This operation performs
no filesystem, process, network, environment or clock access. It does not expose
or allow callers to construct the private admitted wrapper.

The observed outcomes name the operation completing, not collection completeness
or admission success; response assertions must check the actual source, accepted
flag and gap codes. Native ESS evidence scoped to this component establishes only
the selected foundation behavior, never public handler or full semantic parity.

## Public collection composition

The `collection` component accepts only public `Collect`. Its Rust implementation
captures source bytes, invokes registered source bindings and returns an admitted
FactSnapshot. Before language bindings are registered, units and observation arrays
are empty. Sources coverage reflects capture; the fourteen other fact families are
Unsupported with explicit unavailable-capability and unknown-unit gaps. Empty
coverage scopes describe the empty observed unit population, never a wildcard.

A Semantic request retains that mode in its response. Source capture may succeed,
but absent semantic adapters remain Unsupported and no tool observation is invented.
Valid requests whose capture fails return a canonical empty observed source with
Failed Sources coverage carrying the actual capture diagnostics. Unread files and
configuration-file descriptors are not fabricated. Invalid selection/configuration
that cannot form a valid snapshot is an invocation error, not a collected outcome.

The component's three authored cases and structural Collect case establish this
bounded behavior; they do not establish language extraction, complete assessments,
navigation, reports or semantic-tool support.

## Collection and admission

1. Paths are normalized, relative UTF-8 paths with `/` separators; absolute paths,
   `..`, duplicate normalized paths and escapes outside the selected root are
   refused. Range bytes are zero-based, half-open UTF-8 byte offsets; lines and
   byte columns are zero-based. A tool's UTF-16 positions are converted by its
   adapter against the exact collected bytes. End is not before start and neither
   byte offset exceeds the file's byte length. Position selectors have equal start
   and end offsets. Nested declaration/occurrence ranges must be consistent.
2. Every selected file has SHA-256 of its exact collected bytes, byte length,
   language when applicable, origin, classification and SourceLine observations.
   Each physical line records its zero-based number, half-open byte span (including
   terminating newline if present), and Code/Comment/Blank/Mixed classification.
   Spans partition the file bytes, without an extra empty line after final newline.
   An empty file has no lines. These are syntax observations, not calculated metrics;
   the core counts them without reading source. Tracked dirty and
   untracked selected content count exactly like clean content. Git HEAD is never
   a sufficient source identity. A file changing while tools run yields
   SourceChangedDuringCollection or StaleEvidence and prevents complete admission.
3. Build selection includes modules, target, Go tags, Cargo default-feature choice,
   features/cfg, Java target and classpath digest, profiles and configuration file
   digests. Manifests include their module/parent and content digest. Maven/Gradle
   multi-module hierarchy cannot be flattened into an unexplained classpath.
   JDT LS host JDK is a tool observation independent of Java 17/21 project target.
4. Configuration ID is lowercase SHA-256 of canonical BuildSelection without `id`.
   Snapshot ID is lowercase SHA-256 of canonical SourceSnapshot without `id` and
   without each file's `origin`, including files nested inside configuration.
   Configuration hashing likewise omits configuration-file `origin`. Git tracking
   status is provenance: committing unchanged dirty bytes cannot change identity.
   Canonical JSON recursively sorts object keys lexicographically, uses compact
   UTF-8 JSON with no whitespace, integer decimal notation, and preserves string
   bytes. Sort set-valued selection paths/languages/modules/tags/features/cfg/
   profiles and file/manifest lists by their normalized keys before hashing.
   Duplicate entries are refused. Configuration file contents and manifest bytes
   participate through their content digests. Tool timestamps and elapsed time do
   not participate. Evidence IDs bind snapshot, configuration and source location;
   replay never regenerates identities from the current filesystem.
5. Unit/declaration/fact IDs are nonempty and unique in their respective namespace.
   All owner, parent, target, candidate, occurrence and evidence references exist
   in the admitted snapshot. Parent containment is acyclic. Source ranges refer to
   selected files. Repeated import occurrences remain individual facts but do not
   multiply a graph edge. External units require explicit identified units; an
   unknown target stays unresolved, never an invented successful resolution.
6. Each fact carries snapshot/configuration, producer/version, resolution state,
   resolution basis, optional location and gaps. Every identity matches collection.
   Imported language-server/build evidence with different source or configuration
   is refused as StaleEvidence. A candidate name is not a resolved identity.
   Resolved reference/call/implementation facts require exactly one target and no
   unresolved candidates. Candidates and unresolved facts have no resolved target;
   candidates may name zero or more possible targets and carry a reason.
   Declaration and structure facts are source observations whose basis is
   SourceStructure; their presence establishes syntax, not downstream resolution.
7. Coverage is keyed by fact family, selected configuration and explicit unit set.
   Missing, overlapping contradictory or out-of-selection coverage is refused.
   Complete means the collector visited the full selected scope for that family
   and its declared resolution contract. Unsupported is a delivery gap.
   NotApplicable needs a nonempty semantic reason; missing tooling is never
   non-applicability. Partial and Failed require gaps. Complete cannot coexist
   with unresolved required facts or gaps contradicting that completeness.
   FrameworkDeclarations and FrameworkWiring are separate families.
8. Invalid schema, references, formats, identities, policies or coverage cause
   refusal/Error, not a check Pass. Valid partial evidence remains inspectable;
   a witnessed violation can still Fail while coverage remains partial. Otherwise
   checks missing required evidence yield Incomplete/Unsupported/Error as relevant.
   Missing required facts cannot become measured zero. Empty complete collections
   may produce zero where the metric has a defined empty-set value.
9. Optional fields in value structs do not make every combination meaningful.
   LookupSelector Name/QualifiedName requires only a nonempty name;
   SourcePosition requires only a zero-width range. Reference targets obey rule 6.
   Scope units must exist and a positive limit is required; an empty scope means
   all selected units. MetricThreshold requires metric and maximum; Effect requires effect;
   dependency policies require dependency_kind. Unknown rule subject references,
   duplicate rule IDs, negative maxima/weights or undeclared scoring contracts
   are invalid. Framework payloads are checked by kind as described below.

No generic JSON property bag stands in for source-specific meaning. Additional
semantics require a typed contract extension and its named acceptance scenarios.

## Portable counting contract `codegate.portable-counts/1`

Bindings emit observations; analyses count. Every metric selection declares its
counting contract and required fact families; callers cannot weaken those required
families below this contract. Location deduplication uses owner, kind and byte
range. Nested callable bodies belong to their own declarations and do not add
complexity, returns or nesting to an enclosing callable.

| Metric | Counting rule | Required evidence |
|---|---|---|
| FanIn/FanOut | Count distinct resolved unit neighbors for the selected dependency kind; parallel occurrences count once; self dependency counts once. Unknown required targets make the value absent. | Dependencies, Declarations |
| FileLines | Count admitted physical SourceLine observations. Empty file = 0; final newline does not introduce another line. Includes blank/comment lines. | Sources |
| CodeLines | Count Code and Mixed line observations; exclude Comment and Blank. | Sources |
| FunctionLines | Lines intersecting the callable body half-open range; no body = NotApplicable. Comments/blank lines within the body count. | Sources, Declarations, Structure |
| Complexity | 1 per callable with body plus each ConditionalBranch, Loop, CaseBranch, CatchBranch, ShortCircuitAnd, ShortCircuitOr or Ternary. Default/else/finally do not add. Each non-default switch/match case arm adds once regardless of alternatives in that arm. | Declarations, Decisions |
| Nesting | Maximum number of enclosing control-flow NestingRegion observations in the body; first control region has depth 1; plain lexical blocks do not count. | Declarations, Structure |
| Parameters | Explicit formal input parameters, excluding implicit/explicit language receiver; variadic input counts once. | Declarations, Structure |
| Returns | Explicit return statements, including bare return; implicit result expression/throw/?/panic does not count. | Declarations, Structure |
| TypeMembers | Direct declared fields plus methods/constructors, excluding generated/inherited members unless selected explicitly. | Declarations, Structure |
| InterfaceMembers | Direct declared callable contracts; embedded parent contracts recorded separately and not expanded without resolved evidence. | Declarations, Structure |
| PublicApiSize | Distinct ApiEligibility=Eligible declarations after selection; effective export eligibility is established by bindings/adapters separately from declared visibility. Any required Unknown eligibility makes value absent. | Declarations, Structure |
| DocumentationRatio | API-eligible declarations with attached nonempty documentation / API-eligible declarations. Unknown eligibility is incomplete; empty denominator = NotApplicable, not 1 or 0. | Declarations, Documentation |
| DebtMarkers/NamingSignals | Distinct emitted occurrences for the selected normalized rule; no language guessing by the checker. | Documentation or Structure, respectively |
| TestCount | Distinct normalized Unit/Integration/TableDriven/Parameterized test declarations; Benchmark and Fuzz remain separately identified, excluded from this default count. Table rows/parameters are not distinct unless enumerated by matching build evidence under a separate counting contract. | Tests, Declarations |
| TestSourceRatio | Physical test source lines / physical production source lines after selection. Zero denominator = NotApplicable. | Sources, Tests |
| GeneratedSourceRatio | Generated physical source lines (including GeneratedTest) / all selected physical source lines. Zero denominator = NotApplicable. | Sources |
| NondeterminismCount | Distinct normalized Nondeterminism effects; the metric describes observations, not proof of determinism. | Effects |
| EffectCount | Distinct normalized effects of the required typed effect selector, including discarded-result/crypto/allocation and language-specific effect identities. | Effects |
| ReferenceCount | Distinct resolved reference occurrences, optionally filtered by typed occurrence_role; unresolved required references prevent completeness. | References, Occurrences |
| CallFanIn/CallFanOut | Distinct resolved caller/callee declaration neighbors; multiple sites count once. | Calls, Declarations |
| ImplementationCount | Distinct resolved implementation declaration targets per declared interface/trait/member. | Implementations, Declarations |
| FunctionCount/TypeCount/InterfaceCount/SymbolCount/FileCount | Count respectively Function/Method/Constructor declarations, Type declarations, Interface/Trait declarations, all declarations, and selected language source files. | Declarations, or Sources for FileCount |
| BranchDensity | Portable counted decision points / CodeLines within the selected callable body; empty denominator is NotApplicable. | Decisions, Sources, Declarations, Structure |
| ExecutionCoverage | Requires imported matching measured execution evidence and an independently versioned counting contract. Inventory/ratios never imply this value. No measurement contract ships in this foundation. | ExecutionCoverage |

GoSelectCase, RustTryPropagation and GoDefer retain their own identities. They are
not included in portable complexity or represented by artificial Java equivalents.
A language-specific policy may select them with a separately versioned contract.
A policy cannot request `codegate.portable-counts/1` and silently change counting.
Every TestCase structure observation requires test_kind; other structure kinds have
no test_kind. Ratio measurements retain exact nonnegative numerator/denominator
integers; complete ratios require a positive denominator. Decimal value is the
presentation value rounded half-even to six decimal places. The exact ratio is
authoritative. Integral count measurements have neither ratio field. Incomplete ratios
have no numerator, denominator or value; zero-denominator NotApplicable ratios
retain neither number to avoid mistaking an undefined ratio for a measured value.
MetricAggregation is explicit: PerSubject preserves individual measurements;
Total sums selected defined values; Maximum selects the largest; Mean divides
their sum by count; CountAboveThreshold counts values strictly greater than the
required threshold. Other aggregations have no threshold. Empty Maximum/Mean are
NotApplicable, empty Total/CountAboveThreshold are zero only with complete scope.
Any required incomplete population member makes its aggregate incomplete. Mean
retains exact numerator/denominator when constituent values are integers. Ratios
permit PerSubject and Total only: Total sums numerators and denominators of
compatible, disjoint selected populations, then divides once. It never sums or
averages percentages. Overlapping populations or incompatible counting contracts
refuse aggregation. For example, 1/2 and 9/10 aggregate to 10/12, not 0.7.
EffectCount requires effect; only ReferenceCount may select occurrence_role.
Other metric kinds reject those selector fields. These typed fields determine
operation; counting_contract never hides an operation in an undocumented string.

Equivalent minimal examples (fixture data, not implementation):

| Language | Callable | Parameters / Returns / Complexity / Nesting |
|---|---|---|
| Go | `func f(x int) int { if x > 0 { return x }; return 0 }` | 1 / 2 / 2 / 1 |
| Rust | `fn f(x: i32) -> i32 { if x > 0 { return x; } return 0; }` | 1 / 2 / 2 / 1 |
| Java | `static int f(int x) { if (x > 0) { return x; } return 0; }` | 1 / 2 / 2 / 1 |

Each example has one physical callable body line. An equivalent implicit-return
Rust spelling has fewer Returns under this explicit-statement contract; parity
requires documenting that semantic difference, not manufacturing an occurrence.

## Navigation and Java/Quarkus

Name lookup can return multiple overloads; qualified-name lookup never discards
signature ambiguity. Position lookup resolves declarations/occurrences against the
collected bytes. Results retain evidence basis and gaps. Definitions, references,
implementations, callers and callees traverse admitted facts only. Syntactic call
candidates remain candidates. A gopls call hierarchy is bounded by the adapter's
static-call coverage and cannot claim dynamic-dispatch completeness. Rust macro/cfg
and Java classpath/generated-source gaps survive normalization and querying.
All query kinds honor selected scope_units, stable ordering and positive limit.
Truncated results set truncated=true; total_matches is present only when the full
match count was actually established. Truncation does not erase semantic coverage
gaps. DocumentationReference occurrences stay distinct from executable references.

Framework facts carry declared annotation spelling for evidence; a binding maps it
to FrameworkKind. Bean needs scope; Qualifier/Producer/InjectionPoint carry qualifier
sets, Producer needs produced_type; REST resource/route retains resource_path and
method_path separately, route also requires http_method; TransactionDeclaration
requires transaction_mode; ConfigurationReference requires configuration_key and
optional selected profile. Qualifier member identity includes annotation/member/
value/nonbinding; a marker qualifier has empty member_name/value. Nonbinding
members are recorded but excluded from candidate matching.

InjectionCandidate is a declaration-level possibility. InjectionBinding needs a
resolved target backed by matching build evidence before it claims effective
application wiring. Annotation scanning may resolve declaration references but
cannot establish Quarkus augmentation. Qualifier ambiguity, programmatic lookup,
generated beans and augmentation-dependent wiring remain explicit gaps without
matching build evidence. Producer relationships preserve the producer declaration
and produced type. Declared REST path composition normalizes joining separators;
it is not proof a route is deployed. Transaction facts describe declarations,
not whether an invocation crossed an effective interceptor. Configuration references
preserve key/profile, never secret values. Framework navigation advertises whether
it follows declared relationships or build-established relationships.

## Policies, findings, scores and replay

PolicyRule subjects/targets identify selected declarations/units (layer membership
is an explicit group of unit IDs). DependencyDirection/Layers/ForbiddenImport check
forbidden directed pairs; cycles use strongly connected components including self
loops; test boundaries use selected classification. ForbiddenCall traverses only
resolved call relationships. PolicyException names a rule, exact subjects, a
nonempty reason, nonempty accountable owner and required expiry. Expiry is evaluated against policy evaluation_time,
not wall-clock time. Suppressed witnessed findings remain visible with their reason.

Scoring requires a named versioned scoring contract, positive component weights,
explicit rule membership and declared required components. This foundation types
selection and results but does not invent a score formula; no scoring algorithm is
implemented. Unknown algorithms are refused. Required incomplete components make
overall_score absent. Complete measurements/scores have present values; incomplete,
unsupported, failed and not-applicable values are absent. A zero is a measurement,
not a stand-in for missing evidence. Finding pressure is optional until a registered
scoring contract defines its unit/range. Suggestions contain explanatory prose and
finding references, never an executable edit. Rank is deterministic, starting at 1,
using the selected policy's ordering and stable finding ID as final tie breaker.

Offline replay uses only imported facts and explicit policy/evaluation_time. Output
arrays have deterministic stable ordering by IDs (and source position for occurrence
results); no wall-clock fields or host paths enter assessment/navigation output.
Policy ordering where semantically significant is retained. JSON and HTML present
identical evidence/gaps; neither may replace an absent score with zero or a green gate.

## Generation and current limits

ESS 0.50.0 synthesizes `generated/semantic-behavior` and
`generated/semantic-wire`; generated plans and types-report retain obligations.
Generated code is never edited except deterministic `cargo fmt` under the pinned
Rust toolchain. The generated behavior defaults refuse work: generated != implemented.

ESS stateless command declarations express typed input/output, not these graph,
counting or evidence rules. Those rules require Rust handlers and authored ESS
scenarios. The generated six structural command scenarios assert only outcome and
response shape; they are not the named semantic acceptance suite. Only the selected
Collect structural case runs with its authored composition cases. Existing dependency conformance cannot
be reused as evidence for new semantic behavior. `semantic-scenarios.md` records
planned obligations and expected witnesses, not fabricated passing results.
