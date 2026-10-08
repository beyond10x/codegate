# Dependency evaluation contract

This is a draft ESS contract for Codegate's first offline dependency-analysis slice.
It uses ESS 0.56.0, `ess/17` direct command responses and `ess-scenario/4` examples.
The ESS system version is `v1`; the application wire formats remain experimental
`codegate-dependency-facts/0.1`, `codegate-dependency-policy/0.1` and
`codegate-dependency-report/0.1`. This does not stabilize the broader fact IR.

## Value graph and ESS boundary

All records are immutable values. A snapshot contains complete unit inventory and
one dependency observation for one source/configuration. Typed containment uses
`List<Unit>` and `List<Edge>`. `UnitId` and `EdgeId` are snapshot-local identities.
They are not persisted ESS entities and have no invented CRUD lifecycle.

ESS `relations:` applies to entities, not these structs. A typed ID field alone
does not verify its target. The Rust admission stage must establish graph-reference
semantics; the named dangling-reference, duplicate-identity and coverage scenarios
exercise that obligation. Struct predicates are not asserted to prove it generally.
The direct-return outcome constrains result shape; authored literal responses make
the first behavior examples executable. Those examples are necessary, not exhaustive.

## Admission semantics

Input JSON is bounded to 4 MiB per document, 10,000 units and 50,000 edges. Reject
duplicate JSON object keys, unknown core fields, malformed JSON, missing required
fields, unknown enum variants and invalid primitive types at the CLI/decoder boundary.
Such structural refusal exits 2 without a success-shaped report. These boundary
cases supplement ESS command scenarios, whose inputs must already be well-typed.

Semantic admission runs in these phases, returning the first phase's diagnostics
sorted by diagnostic-code spelling and subject, with no metrics or findings:

1. Format strings equal the experimental values above (`InvalidFormat`, subjects
   `snapshot.format` then `policy.format` in the same sorted phase).
2. Source/configuration/producer/version and unit/edge identities are nonempty;
   use `EmptyIdentity` with the field path, `unit:<id>` or `edge:<id>`. IDs compare
   exactly; labels are not normalized or interpreted as language instructions.
3. Unit and edge IDs are unique (`DuplicateUnit`, `DuplicateEdge`).
4. Every edge source resolves (`DanglingSource`); exactly one of target and
   unresolved_target is present and nonempty (`InvalidTarget`); every resolved
   target exists (`DanglingTarget`). Subjects are `edge:<id>`.
5. Coverage is consistent (`InvalidCoverage`, subject `coverage`): Complete has no
   gaps and no unresolved destinations; Partial has at least one nonempty gap;
   Unsupported and Failed have at least one nonempty gap and no edges. Empty gap
   strings are always invalid. The complete selected unit inventory is mandatory
   for every status; partial inventory support is deferred.
6. Policy IDs reference existing units; every forbidden tuple has the selected
   dependency kind; no forbidden tuple is duplicated (`InvalidPolicy`, subject
   `policy.forbidden`). Empty expected source/configuration is invalid policy.
7. Expected source and configuration match the snapshot (`SourceMismatch`, subject
   `snapshot.source_id`; `ConfigurationMismatch`, subject
   `snapshot.configuration_id`). Both may be reported in sorted order.

Within a phase accumulate independent defects; do not cascade to later phases.
The example suite mostly isolates defects; the implementation must add mixed-defect
tests for precedence and order. Wire-size/inventory limits are decoder/admission
errors with explicit diagnostics, not silently truncated data.

Source/configuration IDs in this slice are opaque assertions supplied by the producer
and required by the policy. Offline comparison checks equality, not authenticity or
a filesystem hash. Content-addressed collection and multi-producer evidence attachment
are deferred. A report must not claim to have verified source bytes or signed evidence.

## Shared analysis and check semantics

Select only edges of `policy.dependency_kind`. Fan-out is the number of distinct
resolved destination unit IDs for each unit, including self-edges. It is not a count
of import occurrences. Each unit appears once, ordered by exact unit ID.

For Complete coverage, fan-out is an exact nonnegative integer. For Partial or
Unsupported coverage, each unit's metric value is null, not zero or a lower bound
presented as an exact value. Failed coverage yields no metrics.

For each resolved selected edge matching a forbidden tuple, emit one finding with
its edge ID and endpoints. Multiple source occurrences have different edge IDs:
they count once for fan-out but remain separate traceable findings. Findings sort by
edge ID; policy forbidden tuples are copied into the report in source/target/kind
lexical order. The report binds the exact applied policy directly, rather than
claiming a policy digest before canonical hashing is defined.

The metric and checker identities are `dependency.unique-fan-out/1` and
`dependency.forbidden-edge/1`. Language and producer labels never select an algorithm.
No source paths, compiler processes, clocks, environment variables or networks are
read during admission and evaluation. A function that admits untrusted value records
creates a private validated-IR wrapper before analyses/checkers can inspect them.

## Result semantics

Every semantic report echoes source/configuration identity, the canonical policy,
coverage, metric/checker identities and experimental report format. Semantic admission
refusal returns Error with diagnostics and empty metric/finding lists.

| Admitted coverage | Verdict | Metrics | Diagnostic |
| --- | --- | --- | --- |
| Complete, no forbidden edge | Pass | Exact | None |
| Complete, forbidden edge | Fail | Exact | None |
| Partial, no witnessed forbidden edge | Incomplete | Null per unit | PartialCoverage |
| Partial, witnessed forbidden edge | Fail | Null per unit | PartialCoverage |
| Unsupported | Unsupported | Null per unit | UnsupportedCoverage |
| Failed | Error | None | FailedCoverage |

Coverage diagnostic subject is `coverage`. The CLI returns 0 for Pass, 1 for Fail
with Complete coverage, and 2 whenever required evidence cannot complete evaluation,
including Fail with Partial coverage. The latter preserves the witnessed failure
without disguising the incomplete gate. Each scenario expects the full response.

## Generation and conformance

The authored source is `domains/dependency.yaml`, selected by `ess-inputs.yaml`.
Implementation must generate both the Rust behavior interface (`ess generate
synthesize --layout crate`) and wire data library (`ess generate types --all-types`)
into separate owned output directories, with deterministic regeneration checks.
Do not hand-edit generated files or hand-transcribe domain types. Implement the
generated EvaluateBehavior trait; explicit conversion at the generated wire/behavior
boundary must preserve all fields and be round-trip tested. The type generator's
integer runtime obligation must be discharged with integral bounded fan-out output.

The real Rust `ConformanceTarget` must decode each request and call that behavior
implementation, returning its actual report. It must not read expected response
values or special-case scenario names. Run the combined generated/authored suite
through ESS's Rust Runner pinned to the same ESS release. The CLI's built-in
reference runner is not evidence about Codegate. Emit a report tied to the exact
suite and reject zero executed scenarios, unsupported cases and skipped checks.

The `go-facts-same-checker` fixture supplies normalized facts labelled Go. It proves
checker label independence only, not a working Go binding. Real Rust/Go extraction,
FFI resolution, file/declaration facts, cycles, ratios, baselines, content hashing,
waivers and external tool diagnostics remain later slices.

## Validation available now

```console
ess specify validate --path ess
ess verify conform author --path ess --scenarios ess --out <scratch>/authored.json
ess verify conform synthesize --path ess --scenarios ess --out <scratch>/suite.json
```

These validate/compile the contract and examples. They do not establish implementation
conformance. `task check` will be added by the first implementation story and will
run the real evaluator, CLI tests, ESS conformance and generated-contract drift checks.
