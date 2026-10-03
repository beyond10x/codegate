---
format: aep.planning-md/3
id: story:reporting-parity
kind: story
status: draft
title: Deliver versioned JSON and standalone HTML reports
relations:
- decomposes: epic:semantic-parity
- informed_by: executable-system-specification:semantic-analysis
- depends_on: story:scoring-suggestions
- depends_on: story:go-semantics
- depends_on: story:rust-semantics
scope:
- confidence: inferred
  path: ess-semantic/scenarios/**
- confidence: inferred
  path: src/bin/codegate-check.rs
- confidence: inferred
  path: src/main.rs
- confidence: inferred
  path: src/report/**
- confidence: inferred
  path: tests/conformance_semantic.rs
- confidence: inferred
  path: tests/reporting.rs
revision: 3
---
## Intent

Compact/full/summary/top units and fail-on selection; deterministic offline replay with source/tools absent; imported evidence refusal; integrate real ESS semantic scenarios through task check alongside old 27; unknown version refuses; no remote report dependencies.

## Contract

Typed values: Assessment, NavigationResult, Capability, Suggestion, in `ess-semantic/domains/semantic.yaml`.
Normative rules: `specifications/semantic-contract.md`; baseline:
`docs/parity-baseline.md`. These are proposed paths until the foundation is integrated.

## Acceptance

The report conformance matrix produces deterministic versioned JSON and escaped standalone HTML containing exact evidence, capability gaps, limits, gate statuses, ranked findings and scores for admitted inputs.

## Named conformance scenarios

- `parity-report-json-html`
- `parity-html-escaping`
- `parity-offline-replay`
- `parity-exit-codes`

These are required authored ESS acceptance scenarios, not a claim that they execute
today. The implementing story adds their fixtures/runner assertions; closing evidence
must contain the actual scenario counts and exact inputs. A generated obligation stub
or mock-only external-tool test cannot satisfy acceptance. Preserve the legacy 27.

## Dependencies

`story:scoring-suggestions`, `story:go-semantics`, `story:rust-semantics`

## Scope

- inferred: `src/report/**` — planned owned implementation/verification surface.
- inferred: `src/main.rs` — planned owned implementation/verification surface.
- inferred: `tests/reporting.rs` — planned owned implementation/verification surface.
- inferred: `tests/conformance_semantic.rs` — planned owned implementation/verification surface.
- inferred: `src/bin/codegate-check.rs` — planned owned implementation/verification surface.
- inferred: `ess-semantic/scenarios/**` — planned owned implementation/verification surface.

These inferred surfaces reflect the accepted architecture, not existing code.
Validate and revise them before implementation dispatch. Only the coordinator mutates
AEP or shared integration manifests outside the story's scope; adapters register through
the semantic-navigation seam. All authored runnable code/harnesses are Rust (clap
for command lines); Go and Java source remain fixture data.

## Completion evidence

Run targeted cases and the required integration `task check`; retain named real ESS
results, generated drift result and relevant reference comparison outputs. No GitHub
publication or release is included. An unsupported required capability remains a gap.
