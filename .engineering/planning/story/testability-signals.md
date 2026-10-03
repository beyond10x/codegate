---
format: aep.planning-md/3
id: story:testability-signals
kind: story
status: draft
title: Measure testability inventory without fabricating coverage
relations:
- decomposes: epic:semantic-parity
- informed_by: executable-system-specification:semantic-analysis
- depends_on: story:source-structure
- depends_on: story:portable-metrics
scope:
- confidence: inferred
  path: src/bindings/testability/**
- confidence: inferred
  path: src/semantic/testability.rs
- confidence: inferred
  path: tests/fixtures/testability/**
- confidence: inferred
  path: tests/testability.rs
revision: 3
---
## Intent

Go test conventions; Rust test attributes; Java JUnit-style fixture mappings; classifier provenance; ratio aggregation from disjoint populations; absence of selected required facts cannot count as zero.

## Contract

Typed values: SourceClass, StructureObservation, Effect, Measurement, in `ess-semantic/domains/semantic.yaml`.
Normative rules: `specifications/semantic-contract.md`; baseline:
`docs/parity-baseline.md`. These are proposed paths until the foundation is integrated.

## Acceptance

Testability fixture evaluation reports expected tests, benchmarks/fuzz/parameterized shapes, source/generated ratios and nondeterminism observations while leaving execution coverage unavailable without execution evidence.

## Named conformance scenarios

- `parity-test-inventory`
- `parity-generated-ratio`
- `parity-nondeterminism`
- `parity-inventory-not-coverage`

These are required authored ESS acceptance scenarios, not a claim that they execute
today. The implementing story adds their fixtures/runner assertions; closing evidence
must contain the actual scenario counts and exact inputs. A generated obligation stub
or mock-only external-tool test cannot satisfy acceptance. Preserve the legacy 27.

## Dependencies

`story:source-structure`, `story:portable-metrics`

## Scope

- inferred: `src/bindings/testability/**` — planned owned implementation/verification surface.
- inferred: `src/semantic/testability.rs` — planned owned implementation/verification surface.
- inferred: `tests/testability.rs` — planned owned implementation/verification surface.
- inferred: `tests/fixtures/testability/**` — planned owned implementation/verification surface.

These inferred surfaces reflect the accepted architecture, not existing code.
Validate and revise them before implementation dispatch. Only the coordinator mutates
AEP or shared integration manifests outside the story's scope; adapters register through
the semantic-navigation seam. All authored runnable code/harnesses are Rust (clap
for command lines); Go and Java source remain fixture data.

## Completion evidence

Run targeted cases and the required integration `task check`; retain named real ESS
results, generated drift result and relevant reference comparison outputs. No GitHub
publication or release is included. An unsupported required capability remains a gap.
