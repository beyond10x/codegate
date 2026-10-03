---
format: aep.planning-md/3
id: story:safety-observations
kind: story
status: draft
title: Normalize safety, security and performance observations
relations:
- decomposes: epic:semantic-parity
- informed_by: executable-system-specification:semantic-analysis
- depends_on: story:source-structure
- depends_on: story:portable-metrics
scope:
- confidence: inferred
  path: src/bindings/safety/**
- confidence: inferred
  path: src/semantic/safety.rs
- confidence: inferred
  path: tests/fixtures/safety/**
- confidence: inferred
  path: tests/safety.rs
revision: 3
---
## Intent

Discarded results, abrupt exits, exec/SQL/path, unsafe/weak crypto/reflection, concatenation/allocation/capacity/copies and Go assertions/defer; qualified API mappings and resolution basis; Java/Rust non-applicability justified narrowly, never blanket unsupported=parity.

## Contract

Typed values: Effect, StructureObservation, Measurement, Finding, in `ess-semantic/domains/semantic.yaml`.
Normative rules: `specifications/semantic-contract.md`; baseline:
`docs/parity-baseline.md`. These are proposed paths until the foundation is integrated.

## Acceptance

Shared checks over the safety/performance/security fixture matrix produce evidence-qualified findings for every applicable baseline row without labeling syntactic suspicion as a proven runtime vulnerability.

## Named conformance scenarios

- `parity-safety-matrix`
- `parity-discarded-result-not-error`
- `parity-language-specific-effects`

These are required authored ESS acceptance scenarios, not a claim that they execute
today. The implementing story adds their fixtures/runner assertions; closing evidence
must contain the actual scenario counts and exact inputs. A generated obligation stub
or mock-only external-tool test cannot satisfy acceptance. Preserve the legacy 27.

## Dependencies

`story:source-structure`, `story:portable-metrics`

## Scope

- inferred: `src/bindings/safety/**` — planned owned implementation/verification surface.
- inferred: `src/semantic/safety.rs` — planned owned implementation/verification surface.
- inferred: `tests/safety.rs` — planned owned implementation/verification surface.
- inferred: `tests/fixtures/safety/**` — planned owned implementation/verification surface.

These inferred surfaces reflect the accepted architecture, not existing code.
Validate and revise them before implementation dispatch. Only the coordinator mutates
AEP or shared integration manifests outside the story's scope; adapters register through
the semantic-navigation seam. All authored runnable code/harnesses are Rust (clap
for command lines); Go and Java source remain fixture data.

## Completion evidence

Run targeted cases and the required integration `task check`; retain named real ESS
results, generated drift result and relevant reference comparison outputs. No GitHub
publication or release is included. An unsupported required capability remains a gap.
