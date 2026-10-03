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
- depends_on: story:collection-composition-seam
scope:
- confidence: inferred
  path: .github/workflows/ci.yml
- confidence: inferred
  path: AGENTS.md
- confidence: inferred
  path: README.md
- confidence: inferred
  path: ess-semantic/ess-inputs.yaml
- confidence: inferred
  path: ess-semantic/scenarios/collection/languages/semantic-rust-source-*.yaml
- confidence: inferred
  path: generated/semantic-behavior
- confidence: inferred
  path: generated/semantic-wire
- confidence: inferred
  path: specifications/semantic-contract.md
- confidence: inferred
  path: specifications/semantic-scenarios.md
- confidence: inferred
  path: src/bin/codegate-check.rs
- confidence: inferred
  path: src/bindings/mod.rs
- confidence: cited
  path: src/bindings/rust.rs
- confidence: inferred
  path: src/collection/compose.rs
- confidence: inferred
  path: src/lib.rs
- confidence: inferred
  path: tests/collection_adversary.rs
- confidence: inferred
  path: tests/collection_composition.rs
- confidence: inferred
  path: tests/collection_conformance.rs
- confidence: cited
  path: tests/fixtures/structure/rust/**
- confidence: inferred
  path: tests/freshness.rs
- confidence: inferred
  path: tests/gate_adversary.rs
- confidence: inferred
  path: tests/source_language_conformance.rs
- confidence: cited
  path: tests/source_rust.rs
- confidence: inferred
  path: tests/support/collection_conformance.rs
revision: 9
---
## Intent and contract

Implement the Rust source-only binding over exact collected bytes using Tree-sitter. Existing typed home: ess-semantic/domains/semantic.yaml (Declaration, Occurrence, Dependency, DecisionPoint, StructureObservation, Evidence and Coverage); normative rules: specifications/semantic-contract.md. Reuse the source-snapshot parser/line seam. No cargo execution, build scripts or proc-macro expansion.

## Acceptance

The semantic-rust-source-baseline authored ESS fixture returns expected modules, functions, structs, traits, impl methods and use occurrences with exact source spans, while syntax errors, macros and conditional compilation retain explicit gaps instead of invented resolved relationships.

## Named scenarios

semantic-rust-source-baseline, semantic-rust-source-syntax-error, semantic-rust-source-cfg-macro. Include grouped/aliased use trees, inline modules, receivers, Unicode and nested closures. A syntactic module/use candidate becomes a dependency edge only when the selected declared unit universe proves it; ambiguous/external targets stay unresolved.

## Scope

Derived 2026-10-03 by read-only aep:story-scoper against 821de3626e019c5c8203ea7f8f026b981c7a0d09; corrected by coordinator after the typed design-scope refusal.

- cited: src/bindings/rust.rs, tests/source_rust.rs, tests/fixtures/structure/rust/** are the explicitly assigned unit-owned implementation, test and fixture surfaces. Their existence as future files is not an implementation claim.
- cited: SourceBinding, BindingFacts, retained CollectedSource and shared parser/ID/range/evidence helpers already supply the language extraction seam. The worker consumes them without source IO or external tools.
- inferred: this story also owns the shared coordinator integration paths recorded in its typed scope and design:source-baseline-wave: registration, collection defaults, native target/gate, ESS scenario registration, generated projections when changed and semantic/user documentation. Coordinator ownership does not erase these paths from scheduling.
- inferred: would collide with other language stories at these shared surfaces. The final computed wave output serializes the complete language stories; only separately assigned files within one story may be worked in parallel.
- cited: confidence high for the language unit paths and interface from the story and code; confidence inferred for the full shared edit set until the implementation diff establishes it. Earlier narrow scope was corrected visibly rather than treated as safe concurrency.
- cited: safety fact, level 2 and unproven by execution: src/collection/compose.rs:182 enforces slot ownership and src/semantic/admit.rs validates composed identities, lexical containment and completeness. Read-only scope review ran no builds or tests. All authored running code is Rust; Go and Java are fixture data.

## Dependencies and completion

Depends on story:source-snapshot and story:capability-admission. Unit tests and real authored ESS execution, followed by task check preserving 27 legacy cases. No rust-analyzer completeness claim follows from syntax extraction.

## Production conformance prerequisite


Depends additionally on story:collection-composition-seam. Its real public Collect Rust handler, shared registration and native ESS runner exist before this language unit forks. Coordinator registers this story's authored scenarios through that production path; the language story closes on those actual results, without waiting for downstream first-slice CLI work. Existing Foundation CollectSource returns only SourceSnapshot and cannot substitute for this FactSnapshot witness.
