---
format: aep.planning-md/3
id: story:java-semantics
kind: story
status: draft
title: Resolve Java navigation with JDT LS and build selection
relations:
- decomposes: epic:semantic-parity
- informed_by: executable-system-specification:semantic-analysis
- depends_on: story:semantic-navigation
scope:
- confidence: inferred
  path: src/semantic_tools/java.rs
- confidence: inferred
  path: tests/fixtures/java_semantics/**
- confidence: inferred
  path: tests/java_semantics.rs
revision: 3
---
## Intent

Resolve classpaths and selected profiles, no source scanning claim of effective resolution; include both build systems at both target versions (four-case matrix); generated sources and missing classpath yield explicit gaps; require real JDT session evidence; no silent host/project JDK conflation.

## Contract

Typed values: BuildSelection, Manifest, ToolObservation, Reference, Call, Implementation, in `ess-semantic/domains/semantic.yaml`.
Normative rules: `specifications/semantic-contract.md`; baseline:
`docs/parity-baseline.md`. These are proposed paths until the foundation is integrated.

## Acceptance

A real installed JDT LS session resolves Java overload/reference/call fixtures for Maven and Gradle multi-module projects targeting Java 17 and 21 with the supported host JDK recorded independently.

## Named conformance scenarios

- `parity-jdtls-maven17`
- `parity-jdtls-gradle21`
- `parity-java-overload`
- `parity-java-generated-gap`

These are required authored ESS acceptance scenarios, not a claim that they execute
today. The implementing story adds their fixtures/runner assertions; closing evidence
must contain the actual scenario counts and exact inputs. A generated obligation stub
or mock-only external-tool test cannot satisfy acceptance. Preserve the legacy 27.

## Dependencies

`story:semantic-navigation`

## Scope

- inferred: `src/semantic_tools/java.rs` — planned owned implementation/verification surface.
- inferred: `tests/java_semantics.rs` — planned owned implementation/verification surface.
- inferred: `tests/fixtures/java_semantics/**` — planned owned implementation/verification surface.

These inferred surfaces reflect the accepted architecture, not existing code.
Validate and revise them before implementation dispatch. Only the coordinator mutates
AEP or shared integration manifests outside the story's scope; adapters register through
the semantic-navigation seam. All authored runnable code/harnesses are Rust (clap
for command lines); Go and Java source remain fixture data.

## Completion evidence

Run targeted cases and the required integration `task check`; retain named real ESS
results, generated drift result and relevant reference comparison outputs. No GitHub
publication or release is included. An unsupported required capability remains a gap.
