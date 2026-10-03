---
format: aep.planning-md/3
id: story:quarkus-relationships
kind: story
status: draft
title: Interpret declared Quarkus relationships and their limits
relations:
- decomposes: epic:semantic-parity
- informed_by: executable-system-specification:semantic-analysis
- depends_on: story:java-semantics
- depends_on: story:architecture-policies
scope:
- confidence: inferred
  path: src/bindings/quarkus.rs
- confidence: inferred
  path: src/semantic/framework.rs
- confidence: inferred
  path: tests/fixtures/quarkus/**
- confidence: inferred
  path: tests/quarkus.rs
revision: 3
---
## Intent

Beans/scopes, qualifiers and nonbinding members, producers/injection points; declared candidates versus effective bindings; programmatic lookup/generated beans; REST class+method path composition; navigation and policy over framework facts; Maven/Gradle Java17/21 fixture selection inherited.

## Contract

Typed values: FrameworkFact, Qualifier, Evidence, NavigationResult, PolicyRule, in `ess-semantic/domains/semantic.yaml`.
Normative rules: `specifications/semantic-contract.md`; baseline:
`docs/parity-baseline.md`. These are proposed paths until the foundation is integrated.

## Acceptance

Quarkus 3 fixtures yield expected declared CDI, REST, transaction and configuration relationships while ambiguous or augmentation-dependent wiring remains incomplete without matching build evidence.

## Named conformance scenarios

- `parity-cdi-qualifier-ambiguity`
- `parity-cdi-producer`
- `parity-rest-path`
- `parity-transactions`
- `parity-config-profiles`
- `parity-augmentation-gap`

These are required authored ESS acceptance scenarios, not a claim that they execute
today. The implementing story adds their fixtures/runner assertions; closing evidence
must contain the actual scenario counts and exact inputs. A generated obligation stub
or mock-only external-tool test cannot satisfy acceptance. Preserve the legacy 27.

## Dependencies

`story:java-semantics`, `story:architecture-policies`

## Scope

- inferred: `src/bindings/quarkus.rs` — planned owned implementation/verification surface.
- inferred: `src/semantic/framework.rs` — planned owned implementation/verification surface.
- inferred: `tests/quarkus.rs` — planned owned implementation/verification surface.
- inferred: `tests/fixtures/quarkus/**` — planned owned implementation/verification surface.

These inferred surfaces reflect the accepted architecture, not existing code.
Validate and revise them before implementation dispatch. Only the coordinator mutates
AEP or shared integration manifests outside the story's scope; adapters register through
the semantic-navigation seam. All authored runnable code/harnesses are Rust (clap
for command lines); Go and Java source remain fixture data.

## Completion evidence

Run targeted cases and the required integration `task check`; retain named real ESS
results, generated drift result and relevant reference comparison outputs. No GitHub
publication or release is included. An unsupported required capability remains a gap.
