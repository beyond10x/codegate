---
format: aep.planning-md/3
id: story:semantic-navigation
kind: story
status: draft
title: Bound language-server sessions and normalize navigation evidence
relations:
- decomposes: epic:semantic-parity
- informed_by: executable-system-specification:semantic-analysis
- depends_on: story:offline-navigation
scope:
- confidence: inferred
  path: src/lib.rs
- confidence: inferred
  path: src/main.rs
- confidence: inferred
  path: src/semantic_tools/mod.rs
- confidence: inferred
  path: src/semantic_tools/protocol.rs
- confidence: inferred
  path: src/semantic_tools/session.rs
- confidence: inferred
  path: tests/semantic_tools.rs
revision: 4
---
## Intent

Rust LSP framing/session/process harness with fake servers authored in Rust; record server/runtime versions, configuration, diagnostics and exact source identity; shutdown/reap children; bounds on time/output; cancellation/errors do not appear complete; semantic registration owned here.

## Contract

Typed values: ToolObservation, Evidence, Coverage, Reference, Call, Implementation, in `ess-semantic/domains/semantic.yaml`.
Normative rules: `specifications/semantic-contract.md`; baseline:
`docs/parity-baseline.md`. These paths were validated before this story was created.

## Acceptance

Given the healthy deterministic Rust LSP fixture server, explicit semantic collection returns the exact expected resolved relationship and tool/snapshot/configuration evidence, while each separate missing-tool, timeout and partial-response fixture returns its specified incomplete or failed result.

## Named conformance scenarios

- `parity-missing-tool`
- `parity-tool-timeout`
- `parity-partial-tool`
- `parity-stale-tool-result`

These are required authored ESS acceptance scenarios, not a claim that they execute
today. The implementing story adds their fixtures/runner assertions; closing evidence
must contain the actual scenario counts and exact inputs. A generated obligation stub
or mock-only external-tool test cannot satisfy acceptance. Preserve the legacy 27.

## Dependencies

`story:offline-navigation`

## Scope

- inferred: `src/semantic_tools/protocol.rs` — planned owned implementation/verification surface.
- inferred: `src/semantic_tools/session.rs` — planned owned implementation/verification surface.
- inferred: `src/semantic_tools/mod.rs` — planned owned implementation/verification surface.
- inferred: `src/lib.rs` — planned owned implementation/verification surface.
- inferred: `src/main.rs` — planned owned implementation/verification surface.
- inferred: `tests/semantic_tools.rs` — planned owned implementation/verification surface.

These inferred surfaces reflect the accepted architecture, not existing code.
Validate and revise them before implementation dispatch. Only the coordinator mutates
AEP or shared integration manifests outside the story's scope; adapters register through
the semantic-navigation seam. All authored runnable code/harnesses are Rust (clap
for command lines); Go and Java source remain fixture data.

## Completion evidence

Run targeted cases and the required integration `task check`; retain named real ESS
results, generated drift result and relevant reference comparison outputs. No GitHub
publication or release is included. An unsupported required capability remains a gap.
