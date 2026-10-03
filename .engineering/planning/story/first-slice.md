---
format: aep.planning-md/3
id: story:first-slice
kind: story
status: draft
title: Expose collection and shared dependency assessment
relations:
- decomposes: epic:semantic-parity
- informed_by: executable-system-specification:semantic-analysis
- depends_on: story:source-structure
scope:
- confidence: inferred
  path: src/lib.rs
- confidence: inferred
  path: src/main.rs
- confidence: inferred
  path: src/semantic/analysis.rs
- confidence: inferred
  path: src/semantic/check.rs
- confidence: inferred
  path: tests/first_slice.rs
- confidence: inferred
  path: tests/fixtures/first_slice/**
revision: 3
---
## Intent

Wire Rust library and JSON collect/assess/capabilities while retaining /0.1 evaluate; initial Quarkus annotation facts visible; dependencies require declared universe; incomplete resolution cannot produce passing absence checks; first-slice docs demonstrate explicit source-only limitations.

## Contract

Typed values: Dependency, PolicyRule, Measurement, Assessment, CollectionRequest, in `ess-semantic/domains/semantic.yaml`.
Normative rules: `specifications/semantic-contract.md`; baseline:
`docs/parity-baseline.md`. These are proposed paths until the foundation is integrated.

## Acceptance

Assessing equivalent Go/Rust/Java first-slice fixtures with one shared forbidden-dependency policy yields identical normalized unique fan-out and witnessed policy outcomes.

## Named conformance scenarios

- `parity-first-slice-three-languages`
- `parity-forbidden-with-gaps`
- `parity-assess-equals-offline`

These are required authored ESS acceptance scenarios, not a claim that they execute
today. The implementing story adds their fixtures/runner assertions; closing evidence
must contain the actual scenario counts and exact inputs. A generated obligation stub
or mock-only external-tool test cannot satisfy acceptance. Preserve the legacy 27.

## Dependencies

`story:source-structure`

## Scope

- inferred: `src/semantic/analysis.rs` — planned owned implementation/verification surface.
- inferred: `src/semantic/check.rs` — planned owned implementation/verification surface.
- inferred: `src/lib.rs` — planned owned implementation/verification surface.
- inferred: `src/main.rs` — planned owned implementation/verification surface.
- inferred: `tests/first_slice.rs` — planned owned implementation/verification surface.
- inferred: `tests/fixtures/first_slice/**` — planned owned implementation/verification surface.

These inferred surfaces reflect the accepted architecture, not existing code.
Validate and revise them before implementation dispatch. Only the coordinator mutates
AEP or shared integration manifests outside the story's scope; adapters register through
the semantic-navigation seam. All authored runnable code/harnesses are Rust (clap
for command lines); Go and Java source remain fixture data.

## Completion evidence

Run targeted cases and the required integration `task check`; retain named real ESS
results, generated drift result and relevant reference comparison outputs. No GitHub
publication or release is included. An unsupported required capability remains a gap.
