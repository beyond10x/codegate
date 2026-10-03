---
format: aep.planning-md/3
id: story:offline-navigation
kind: story
status: draft
title: Query admitted declarations and relationships offline
relations:
- decomposes: epic:semantic-parity
- informed_by: executable-system-specification:semantic-analysis
- depends_on: story:first-slice
scope:
- confidence: inferred
  path: src/lib.rs
- confidence: inferred
  path: src/main.rs
- confidence: inferred
  path: src/semantic/query.rs
- confidence: inferred
  path: tests/fixtures/navigation/**
- confidence: inferred
  path: tests/navigation.rs
revision: 3
---
## Intent

Deterministic ordering, scope/limit, explicit ambiguity, candidates versus resolved, resolution evidence, empty complete versus unavailable; references include read/write/import/doc and invocations; no invented cross-language edges.

## Contract

Typed values: LookupSelector, NavigationResult, Reference, Call, Implementation, in `ess-semantic/domains/semantic.yaml`.
Normative rules: `specifications/semantic-contract.md`; baseline:
`docs/parity-baseline.md`. These are proposed paths until the foundation is integrated.

## Acceptance

Lookup over imported admitted facts returns expected definitions/references/implementations/callers/callees for name, qualified name and position selectors without invoking a source collector or tool.

## Named conformance scenarios

- `parity-offline-lookup`
- `parity-ambiguous-name`
- `parity-unicode-position`
- `parity-query-truncation`

These are required authored ESS acceptance scenarios, not a claim that they execute
today. The implementing story adds their fixtures/runner assertions; closing evidence
must contain the actual scenario counts and exact inputs. A generated obligation stub
or mock-only external-tool test cannot satisfy acceptance. Preserve the legacy 27.

## Dependencies

`story:first-slice`

## Scope

- inferred: `src/semantic/query.rs` — planned owned implementation/verification surface.
- inferred: `src/lib.rs` — planned owned implementation/verification surface.
- inferred: `src/main.rs` — planned owned implementation/verification surface.
- inferred: `tests/navigation.rs` — planned owned implementation/verification surface.
- inferred: `tests/fixtures/navigation/**` — planned owned implementation/verification surface.

These inferred surfaces reflect the accepted architecture, not existing code.
Validate and revise them before implementation dispatch. Only the coordinator mutates
AEP or shared integration manifests outside the story's scope; adapters register through
the semantic-navigation seam. All authored runnable code/harnesses are Rust (clap
for command lines); Go and Java source remain fixture data.

## Completion evidence

Run targeted cases and the required integration `task check`; retain named real ESS
results, generated drift result and relevant reference comparison outputs. No GitHub
publication or release is included. An unsupported required capability remains a gap.
