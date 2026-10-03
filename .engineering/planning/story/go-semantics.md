---
format: aep.planning-md/3
id: story:go-semantics
kind: story
status: draft
title: Resolve Go navigation with gopls
relations:
- decomposes: epic:semantic-parity
- informed_by: executable-system-specification:semantic-analysis
- depends_on: story:semantic-navigation
scope:
- confidence: inferred
  path: src/semantic_tools/go.rs
- confidence: inferred
  path: tests/fixtures/go_semantics/**
- confidence: inferred
  path: tests/go_semantics.rs
revision: 3
---
## Intent

Go module/workspace identity, platform and build tags; no cross-build reference completeness claim; references, implementations and call hierarchy evidence; recorded installed version; real tool fixture run required, fake-server tests alone cannot close story.

## Contract

Typed values: BuildSelection, ToolObservation, Reference, Call, Implementation, in `ess-semantic/domains/semantic.yaml`.
Normative rules: `specifications/semantic-contract.md`; baseline:
`docs/parity-baseline.md`. These are proposed paths until the foundation is integrated.

## Acceptance

A real installed gopls session resolves the named Go fixture references/implementations/calls under selected build tags while documenting dynamic-call omissions as gaps.

## Named conformance scenarios

- `parity-gopls-navigation`
- `parity-go-build-tags`
- `parity-go-dynamic-calls`

These are required authored ESS acceptance scenarios, not a claim that they execute
today. The implementing story adds their fixtures/runner assertions; closing evidence
must contain the actual scenario counts and exact inputs. A generated obligation stub
or mock-only external-tool test cannot satisfy acceptance. Preserve the legacy 27.

## Dependencies

`story:semantic-navigation`

## Scope

- inferred: `src/semantic_tools/go.rs` — planned owned implementation/verification surface.
- inferred: `tests/go_semantics.rs` — planned owned implementation/verification surface.
- inferred: `tests/fixtures/go_semantics/**` — planned owned implementation/verification surface.

These inferred surfaces reflect the accepted architecture, not existing code.
Validate and revise them before implementation dispatch. Only the coordinator mutates
AEP or shared integration manifests outside the story's scope; adapters register through
the semantic-navigation seam. All authored runnable code/harnesses are Rust (clap
for command lines); Go and Java source remain fixture data.

## Completion evidence

Run targeted cases and the required integration `task check`; retain named real ESS
results, generated drift result and relevant reference comparison outputs. No GitHub
publication or release is included. An unsupported required capability remains a gap.
