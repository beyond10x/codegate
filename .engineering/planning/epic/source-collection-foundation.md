---
format: aep.planning-md/3
id: epic:source-collection-foundation
kind: epic
status: draft
title: Establish source identity and admission for the first collection round
relations:
- decomposes: epic:semantic-parity
- informed_by: executable-system-specification:semantic-analysis
- depends_on: task:clean-external-target-gate
revision: 2
---
## Intent

Deliver the common source identity and admission foundation required by the first Rust, Go and Java collection round. Reuse existing SourceSelection, SourceSnapshot, FactSnapshot, Evidence, Coverage and Gap value contracts in ess-semantic/domains/semantic.yaml; validation on 2026-10-03: `codegate_semantic v1 — 2 file(s), valid`.

## Acceptance

The first-collection-foundation verification report passes the full gate with all 27 legacy scenarios and every required foundation case (parity-selected-content-identity, parity-manifest-identity, parity-path-confinement, parity-stale-evidence, parity-dangling-observation, parity-coverage-not-zero and parity-core-boundary) executed against real collection/admission behavior, with zero failed, errored, unsupported or skipped selected cases and exact suite/model identities retained.

## Children and order

story:source-snapshot and story:capability-admission are the implementation children. The coordinator owns shared Cargo/module wiring and the pure source-identity seam; each child owns separate source/test paths. task:clean-external-target-gate is a prerequisite harness repair owned by the same collection round.

## Boundary

Source-only collection executes no external build tools or language servers. Git provenance may be read by a Rust library. Unavailable effective build selection remains a located gap. Existing /0.1 evaluation is unchanged. Tool-backed resolution, scoring, broad metrics and effective Quarkus wiring remain in epic:semantic-parity after this round.
