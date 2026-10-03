---
format: aep.planning-md/3
id: story:rust-semantics
kind: story
status: draft
title: Resolve Rust navigation with rust-analyzer
relations:
- decomposes: epic:semantic-parity
- informed_by: executable-system-specification:semantic-analysis
- depends_on: story:semantic-navigation
scope:
- confidence: inferred
  path: src/semantic_tools/rust.rs
- confidence: inferred
  path: tests/fixtures/rust_semantics/**
- confidence: inferred
  path: tests/rust_semantics.rs
revision: 3
---
## Intent

Cargo target/features/default features/cfg selection; workspace and generated inputs identity; macros/trait dispatch explicit evidence limits; installed version retained; fixture verification includes changed selection; authored implementation and Rust fixtures follow repository Rust rule.

## Contract

Typed values: BuildSelection, ToolObservation, Reference, Call, Implementation, in `ess-semantic/domains/semantic.yaml`.
Normative rules: `specifications/semantic-contract.md`; baseline:
`docs/parity-baseline.md`. These are proposed paths until the foundation is integrated.

## Acceptance

A real installed rust-analyzer session resolves the selected Cargo fixture navigation while cfg/features and unexpanded macros retain their specified completeness outcomes.

## Named conformance scenarios

- `parity-rust-analyzer-navigation`
- `parity-rust-cfg-features`
- `parity-rust-macro-gap`

These are required authored ESS acceptance scenarios, not a claim that they execute
today. The implementing story adds their fixtures/runner assertions; closing evidence
must contain the actual scenario counts and exact inputs. A generated obligation stub
or mock-only external-tool test cannot satisfy acceptance. Preserve the legacy 27.

## Dependencies

`story:semantic-navigation`

## Scope

- inferred: `src/semantic_tools/rust.rs` — planned owned implementation/verification surface.
- inferred: `tests/rust_semantics.rs` — planned owned implementation/verification surface.
- inferred: `tests/fixtures/rust_semantics/**` — planned owned implementation/verification surface.

These inferred surfaces reflect the accepted architecture, not existing code.
Validate and revise them before implementation dispatch. Only the coordinator mutates
AEP or shared integration manifests outside the story's scope; adapters register through
the semantic-navigation seam. All authored runnable code/harnesses are Rust (clap
for command lines); Go and Java source remain fixture data.

## Completion evidence

Run targeted cases and the required integration `task check`; retain named real ESS
results, generated drift result and relevant reference comparison outputs. No GitHub
publication or release is included. An unsupported required capability remains a gap.
