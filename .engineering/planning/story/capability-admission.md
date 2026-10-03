---
format: aep.planning-md/3
id: story:capability-admission
kind: story
status: draft
title: Admit rich facts and expose precise capability gaps
relations:
- informed_by: executable-system-specification:semantic-analysis
- depends_on: story:parity-foundations
- decomposes: epic:source-collection-foundation
scope:
- confidence: inferred
  path: src/semantic/admit.rs
- confidence: inferred
  path: src/semantic/mod.rs
- confidence: inferred
  path: tests/semantic_admission.rs
- confidence: inferred
  path: tests/semantic_boundary.rs
revision: 4
---
## Intent

Private wrapper; duplicate IDs, containment cycles, invalid spans/endpoints, unsupported versions and conflicting producer overlap refuse; per-family/scope/configuration coverage; absent and unsupported remain distinct from observed zero; checker modules cannot import collection/bindings, perform IO or branch on language.

## Contract

Typed values: FactSnapshot, Coverage, Evidence, Capability, Gap, in `ess-semantic/domains/semantic.yaml`.
Normative rules: `specifications/semantic-contract.md`; baseline:
`docs/parity-baseline.md`. These are proposed paths until the foundation is integrated.

## Acceptance

Admitting malformed or mismatched evidence returns a stable refusal before any shared checker can observe an admitted graph.

## Named conformance scenarios

- `parity-stale-evidence`
- `parity-dangling-observation`
- `parity-coverage-not-zero`
- `parity-core-boundary`

These are required authored ESS acceptance scenarios, not a claim that they execute
today. The implementing story adds their fixtures/runner assertions; closing evidence
must contain the actual scenario counts and exact inputs. A generated obligation stub
or mock-only external-tool test cannot satisfy acceptance. Preserve the legacy 27.

## Dependencies

`story:parity-foundations`

## Scope

- inferred: `src/semantic/admit.rs` — planned owned implementation/verification surface.
- inferred: `src/semantic/mod.rs` — planned owned implementation/verification surface.
- inferred: `tests/semantic_admission.rs` — planned owned implementation/verification surface.
- inferred: `tests/semantic_boundary.rs` — planned owned implementation/verification surface.

These inferred surfaces reflect the accepted architecture, not existing code.
Validate and revise them before implementation dispatch. Only the coordinator mutates
AEP or shared integration manifests outside the story's scope; adapters register through
the semantic-navigation seam. All authored runnable code/harnesses are Rust (clap
for command lines); Go and Java source remain fixture data.

## Completion evidence

Run targeted cases and the required integration `task check`; retain named real ESS
results, generated drift result and relevant reference comparison outputs. No GitHub
publication or release is included. An unsupported required capability remains a gap.

## First-round profile and acceptance strengthening

Apply specifications/source-baseline.md for empty coverage scope, complete family inventory, internally verifiable range guarantees and producer overlap. Use the coordinator's pure src/source_identity.rs seam without importing collection or bindings. Rust seam: semantic::validate_snapshot(&model::FactSnapshot) -> Result<(), Vec<model::Gap>>; only admission constructs its private admitted wrapper. Preserve semantic-contract refusal distinctions and deterministic order.

The parity-core-boundary acceptance must cover root src/lib.rs and all shared semantic code, including newly added files; it must reject injected IO, binding dependencies and language/producer dispatch. The existing tests/semantics.rs:122 literal three-file test is insufficient and is not accepted as the new proof. The coordinator owns Cargo/module/ESS integration; the worker owns the four original inferred files. Capabilities describe supported operations separately from snapshot Coverage; no complete tools or facts are fabricated.
