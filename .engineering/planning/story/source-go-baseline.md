---
format: aep.planning-md/3
id: story:source-go-baseline
kind: story
status: draft
title: Extract the Go source-only baseline
relations:
- decomposes: epic:source-collection-baseline
- depends_on: story:source-snapshot
- depends_on: story:capability-admission
- informed_by: executable-system-specification:semantic-analysis
scope:
- confidence: inferred
  path: src/bindings/go.rs
- confidence: inferred
  path: tests/fixtures/structure/go/**
- confidence: inferred
  path: tests/source_go.rs
revision: 2
---
## Intent and contract

Implement the Go source-only binding over exact collected bytes using Tree-sitter. Existing typed home: ess-semantic/domains/semantic.yaml (Declaration, Occurrence, Dependency, DecisionPoint, StructureObservation, Evidence and Coverage); normative counting and identity rules: specifications/semantic-contract.md. Reuse the source-snapshot grammar/line seam; no external Go process.

## Acceptance

The semantic-go-source-baseline authored ESS fixture returns expected package, function, method, type and interface declarations, import occurrences and source spans, while syntax errors and build constraints produce explicit gaps and unproved references/calls remain candidates.

## Named scenarios

semantic-go-source-baseline, semantic-go-source-syntax-error, semantic-go-source-build-constraints. Include aliased imports, grouped imports, receivers, Unicode, nested callables, comments and literals. Imports only form resolved dependency edges when the selected declared unit universe proves the target; unavailable external packages remain unresolved.

## Scope

Inferred: src/bindings/go.rs, tests/source_go.rs, tests/fixtures/structure/go/**. Coordinator owns shared module registration, manifests, public API, ESS scenario registration and conformance runner. This story supplies fixture inputs/expected observations and Rust tests; no direct AEP edits.

## Dependencies and completion

Depends on story:source-snapshot and story:capability-admission. Unit tests and real authored ESS execution, followed by complete task check preserving 27 legacy cases. A source-only result never implies gopls resolution or complete dynamic calls.
