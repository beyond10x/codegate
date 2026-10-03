---
format: aep.planning-md/3
id: story:source-structural-observations
kind: story
status: draft
title: Collect portable structure and decision observations
relations:
- decomposes: epic:semantic-parity
- depends_on: story:source-structure
- informed_by: executable-system-specification:semantic-analysis
scope:
- confidence: inferred
  path: src/bindings/go.rs
- confidence: inferred
  path: src/bindings/java.rs
- confidence: inferred
  path: src/bindings/rust.rs
- confidence: inferred
  path: tests/fixtures/structure/observations/**
- confidence: inferred
  path: tests/structural_observations.rs
revision: 2
---
## Intent and typed contract

Extend the source-only bindings beyond the first declaration/dependency baseline with DecisionPoint, StructureObservation, documentation and test observations already typed in ess-semantic/domains/semantic.yaml. Apply specifications/semantic-contract.md portable counting rules: nested callable ownership, explicit returns, parameters excluding receivers, lexical/control nesting, type/interface members and source documentation. No analysis computes metrics inside bindings.

## Acceptance

Named real ESS scenarios parity-structural-decisions, parity-nested-callable-ownership and parity-portable-observation-equivalence execute through the real collection handler. Equivalent Rust/Go/Java examples produce the prescribed decision/body/parameter/return observations; nested callable bodies do not add observations to enclosing owners. Syntax or effective-selection gaps prevent complete affected families. Preserve all earlier conformance cases and task check.

## Scope

Inferred src/bindings/go.rs, src/bindings/rust.rs, src/bindings/java.rs, tests/structural_observations.rs and tests/fixtures/structure/observations/**. ESS scenarios and generated integration belong to coordinator. All authored executable code is Rust; language source is fixture data.

## Dependency and boundary

Depends on story:source-structure. This restores the original broader extraction obligation explicitly after narrowing the first round to declarations/dependencies. It is a delivery gap until implemented; Unsupported is not parity. Tool-backed resolution and effective framework wiring remain later independent stories.
