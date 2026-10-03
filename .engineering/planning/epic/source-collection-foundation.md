---
format: aep.planning-md/3
id: epic:source-collection-foundation
kind: epic
status: implemented
title: Establish source identity and admission for the first collection round
relations:
- decomposes: epic:semantic-parity
- informed_by: executable-system-specification:semantic-analysis
- depends_on: task:clean-external-target-gate
- serves: vision:language-neutral-code-quality
revision: 6
transitions:
- {from: "draft", to: "proposed", at: "2026-10-03T07:36:50Z", actor: "human:timo", revision: 4, decided_on: {"recorded":{"review_outcome":2}}, executor: "agent:codegate-collection"}
- {from: "proposed", to: "active", at: "2026-10-03T07:36:50Z", actor: "human:timo", revision: 5, decided_on: {"recorded":{"review_outcome":2}}, executor: "agent:codegate-collection"}
- {from: "active", to: "implemented", at: "2026-10-03T08:42:20Z", actor: "human:timo", revision: 6, decided_on: {"recorded":{"test_result":1,"review_outcome":2}}, executor: "agent:codegate-collection"}
---
## Intent

Deliver the common source identity and admission foundation required by the first Rust, Go and Java collection round. Reuse existing SourceSelection, SourceSnapshot, FactSnapshot, Evidence, Coverage and Gap value contracts in ess-semantic/domains/semantic.yaml; validation on 2026-10-03: `codegate_semantic v1 — 2 file(s), valid`.

## Acceptance

The first-collection-foundation verification report passes the full gate with all 27 legacy evaluator scenarios and six native ESS foundation cases: parity-selected-content-identity, parity-manifest-identity, parity-path-confinement, parity-stale-evidence, parity-dangling-observation and parity-coverage-not-zero. The separate parity-core-boundary architectural check must also pass its real Rust negative probes. Selected ESS cases have zero failures, errors, unsupported or skipped results and retain exact suite/model identities. The foundation component witnesses only CollectSource and ValidateSnapshot library operations; it does not claim the six public semantic commands or complete semantic parity.

## Children and order

story:source-snapshot and story:capability-admission are the implementation children. The coordinator owns shared Cargo/module wiring and the pure source-identity seam; each child owns separate source/test paths. task:clean-external-target-gate is a prerequisite harness repair owned by the same collection round.

## Boundary

Source-only collection executes no external build tools or language servers. Git provenance may be read by a Rust library. Unavailable effective build selection remains a located gap. Existing /0.1 evaluation is unchanged. Tool-backed resolution, scoring, broad metrics and effective Quarkus wiring remain in epic:semantic-parity after this round.
