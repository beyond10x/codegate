---
format: aep.planning-md/3
id: story:portable-metrics
kind: story
status: draft
title: Derive portable maintainability measurements
relations:
- decomposes: epic:semantic-parity
- informed_by: executable-system-specification:semantic-analysis
- depends_on: story:first-slice
scope:
- confidence: inferred
  path: src/semantic/metrics.rs
- confidence: inferred
  path: tests/fixtures/metrics/**
- confidence: inferred
  path: tests/portable_metrics.rs
revision: 3
---
## Intent

Cover function/file size, complexity, nesting, params/returns, fields/interface methods, public API/docs/debt/naming, package/units and symbol counts/fan-in/out; distinguish Go-specific decisions and Rust propagation; zero denominators explicit; all relevant baseline catalogue rows covered.

## Contract

Typed values: DecisionPoint, StructureObservation, Measurement, MetricSelection, in `ess-semantic/domains/semantic.yaml`.
Normative rules: `specifications/semantic-contract.md`; baseline:
`docs/parity-baseline.md`. These are proposed paths until the foundation is integrated.

## Acceptance

Shared metric evaluation of equivalent Go/Rust/Java fixtures matches the exact expected counts and rational ratios under the named counting contract.

## Named conformance scenarios

- `parity-metric-equivalence`
- `parity-ratio-aggregation`
- `parity-missing-metric`
- `parity-debt-comment-only`

These are required authored ESS acceptance scenarios, not a claim that they execute
today. The implementing story adds their fixtures/runner assertions; closing evidence
must contain the actual scenario counts and exact inputs. A generated obligation stub
or mock-only external-tool test cannot satisfy acceptance. Preserve the legacy 27.

## Dependencies

`story:first-slice`

## Scope

- inferred: `src/semantic/metrics.rs` — planned owned implementation/verification surface.
- inferred: `tests/portable_metrics.rs` — planned owned implementation/verification surface.
- inferred: `tests/fixtures/metrics/**` — planned owned implementation/verification surface.

These inferred surfaces reflect the accepted architecture, not existing code.
Validate and revise them before implementation dispatch. Only the coordinator mutates
AEP or shared integration manifests outside the story's scope; adapters register through
the semantic-navigation seam. All authored runnable code/harnesses are Rust (clap
for command lines); Go and Java source remain fixture data.

## Completion evidence

Run targeted cases and the required integration `task check`; retain named real ESS
results, generated drift result and relevant reference comparison outputs. No GitHub
publication or release is included. An unsupported required capability remains a gap.
