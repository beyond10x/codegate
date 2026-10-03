---
format: aep.planning-md/3
id: story:capability-admission
kind: story
status: implemented
title: Admit rich facts and expose precise capability gaps
relations:
- informed_by: executable-system-specification:semantic-analysis
- depends_on: story:parity-foundations
- decomposes: epic:source-collection-foundation
- serves: vision:language-neutral-code-quality
scope:
- confidence: cited
  path: src/semantic/admit.rs
- confidence: cited
  path: src/semantic/mod.rs
- confidence: cited
  path: tests/semantic_admission.rs
- confidence: cited
  path: tests/semantic_boundary.rs
revision: 10
transitions:
- {from: "draft", to: "proposed", at: "2026-10-03T07:36:51Z", actor: "human:timo", revision: 6, executor: "agent:codegate-collection"}
- {from: "proposed", to: "active", at: "2026-10-03T07:36:51Z", actor: "human:timo", revision: 7, executor: "agent:codegate-collection"}
- {from: "active", to: "implemented", at: "2026-10-03T08:42:20Z", actor: "human:timo", revision: 8, decided_on: {"recorded":{"test_result":1,"review_outcome":4,"verification":1,"ess_conformance_coverage_v1":1}}, executor: "agent:codegate-collection"}
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

- cited: src/semantic/admit.rs — actual phased rich-fact admission and private wrapper construction (a68963c).
- cited: src/semantic/mod.rs — actual offline validate_snapshot facade.
- cited: tests/semantic_admission.rs —11 executed product admission tests.
- cited: tests/semantic_boundary.rs —8 executed architectural guard tests with retained negative probes.

All four original inferred surfaces were confirmed by verification-report:admission-unit. Shared lib.rs registration was reconciled by the coordinator and is recorded under source-snapshot integration scope; no concurrent worker owns that shared file.

## Completion evidence

Run targeted cases and the required integration `task check`; retain named real ESS
results, generated drift result and relevant reference comparison outputs. No GitHub
publication or release is included. An unsupported required capability remains a gap.

## First-round profile and acceptance strengthening

Apply specifications/source-baseline.md for empty coverage scope, complete family inventory, internally verifiable range guarantees and producer overlap. Use the coordinator's pure src/source_identity.rs seam without importing collection or bindings. Rust seam: semantic::validate_snapshot(&model::FactSnapshot) -> Result<(), Vec<model::Gap>>; only admission constructs its private admitted wrapper. Preserve semantic-contract refusal distinctions and deterministic order.

The parity-core-boundary acceptance must cover root src/lib.rs and all shared semantic code, including newly added files; it must reject injected IO, binding dependencies and language/producer dispatch. The existing tests/semantics.rs:122 literal three-file test is insufficient and is not accepted as the new proof. The coordinator owns Cargo/module/ESS integration; the worker owns the four original inferred files. Capabilities describe supported operations separately from snapshot Coverage; no complete tools or facts are fabricated.

## Foundation conformance seam

The conformance review identified that the six public commands do not type the intermediate helpers: Collect returns FactSnapshot and Evaluate returns Assessment. Before implementation dispatch, extend ESS with internal CollectSource and ValidateSnapshot operations, using existing source/fact/gap value types, and a source-foundation component. Native ESS targets call actual helpers. Six foundation behavioral cases execute in that component; parity-core-boundary is a separate architectural Rust test with injected violations, not a fabricated semantic command outcome. The six public runtime obligations and specification lifecycle remain incomplete. Coordinator owns the ESS/generated integration; implementation workers own their previously scoped runtime files.
