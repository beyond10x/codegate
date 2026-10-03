---
format: aep.planning-md/3
id: task:collection-composition-seam
kind: task
status: archived
title: Supply the production Collect seam before language acceptance
relations:
- decomposes: epic:source-collection-baseline
- depends_on: story:source-snapshot
- depends_on: story:capability-admission
- informed_by: executable-system-specification:semantic-analysis
revision: 3
transitions:
- {from: "draft", to: "archived", at: "2026-10-03T08:32:24Z", actor: "human:timo", revision: 3}
---
## Intent and existing contract

Implement the existing public Collect(CollectionRequest) -> CollectResponse(FactSnapshot, gaps) Rust obligation as the production composition seam before the three language stories close. Typed home: ess-semantic/domains/semantic.yaml:422. No new noun or command is introduced. Depends on source-snapshot and capability-admission.

## Acceptance and named ESS scenarios

`parity-collect-source-only-foundation` invokes the real Collect handler over selected Rust/Go/Java bytes, retains the independently expected source identity, and returns an admitted FactSnapshot with honest Sources coverage and unsupported unimplemented families. `parity-collect-semantic-refusal` proves an explicit Semantic request is refused with the declared tool/capability gap, never downgraded. Add a narrowly selected native ESS collection component and runner over this existing command; preserve 27 legacy and eight foundation cases. Unit-only extraction tests cannot close language stories.

The shared production binding registration and ID/range/evidence helpers operate over retained content. Empty registration truthfully reports unsupported Units/declarations/dependencies; later workers fill their language slot through the same real Collect implementation. No fixture-name dispatch or canned responses. The three language scenarios must reach real production registration before their stories close.

## Scope

Inferred coordinator-owned surfaces: src/collection/compose.rs, src/bindings/mod.rs, src/lib.rs, tests/collection_conformance.rs, ess-semantic/components.yaml, ess-semantic/ess-inputs.yaml, ess-semantic/scenarios/collection/**, src/bin/codegate-check.rs. Regenerate contract projections only if the component change affects them; preserve byte-drift checks. All implementation and harnesses Rust.

## Boundary and order

This prerequisite supplies the Collect Rust handler, common registration and native runner, not the CLI or the later assessment and navigation handlers. source-structure still owns cross-language equivalence and aggregate integration; first-slice owns JSON command exposure, assessment/capabilities and declaration lookup. Coordinator owns scenario registration during language units. Freeze these shared seams before forking the three disjoint language workers.

Superseded by story:collection-composition-seam after CLI correctly refused task-level scheduling scope. No implementation or completion is claimed.
