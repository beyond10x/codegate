---
format: aep.planning-md/3
id: story:source-rust-baseline
kind: story
status: draft
title: Extract the Rust source-only baseline
relations:
- decomposes: epic:source-collection-baseline
- depends_on: story:source-snapshot
- depends_on: story:capability-admission
- informed_by: executable-system-specification:semantic-analysis
scope:
- confidence: inferred
  path: src/bindings/rust.rs
- confidence: inferred
  path: tests/fixtures/structure/rust/**
- confidence: inferred
  path: tests/source_rust.rs
revision: 2
---
## Intent and contract

Implement the Rust source-only binding over exact collected bytes using Tree-sitter. Existing typed home: ess-semantic/domains/semantic.yaml (Declaration, Occurrence, Dependency, DecisionPoint, StructureObservation, Evidence and Coverage); normative rules: specifications/semantic-contract.md. Reuse the source-snapshot parser/line seam. No cargo execution, build scripts or proc-macro expansion.

## Acceptance

The semantic-rust-source-baseline authored ESS fixture returns expected modules, functions, structs, traits, impl methods and use occurrences with exact source spans, while syntax errors, macros and conditional compilation retain explicit gaps instead of invented resolved relationships.

## Named scenarios

semantic-rust-source-baseline, semantic-rust-source-syntax-error, semantic-rust-source-cfg-macro. Include grouped/aliased use trees, inline modules, receivers, Unicode and nested closures. A syntactic module/use candidate becomes a dependency edge only when the selected declared unit universe proves it; ambiguous/external targets stay unresolved.

## Scope

Inferred: src/bindings/rust.rs, tests/source_rust.rs, tests/fixtures/structure/rust/**. Coordinator owns shared module registration, manifests, public API, ESS scenario registration and conformance runner. This story supplies fixture inputs/expected observations and Rust tests; no AEP edits.

## Dependencies and completion

Depends on story:source-snapshot and story:capability-admission. Unit tests and real authored ESS execution, followed by task check preserving 27 legacy cases. No rust-analyzer completeness claim follows from syntax extraction.
