---
format: aep.planning-md/3
id: story:collection-composition-seam
kind: story
status: active
title: Supply the production Collect seam before language acceptance
relations:
- decomposes: epic:source-collection-baseline
- depends_on: story:source-snapshot
- depends_on: story:capability-admission
- informed_by: executable-system-specification:semantic-analysis
- serves: vision:language-neutral-code-quality
scope:
- confidence: cited
  path: .github/workflows/ci.yml
- confidence: inferred
  path: AGENTS.md
- confidence: inferred
  path: README.md
- confidence: cited
  path: ess-semantic/components.yaml
- confidence: cited
  path: ess-semantic/domains/semantic.yaml
- confidence: cited
  path: ess-semantic/ess-inputs.yaml
- confidence: cited
  path: ess-semantic/scenarios/collection/**
- confidence: cited
  path: generated/semantic-behavior
- confidence: cited
  path: generated/semantic-wire
- confidence: inferred
  path: specifications/semantic-contract.md
- confidence: cited
  path: src/bin/codegate-check.rs
- confidence: cited
  path: src/bindings/mod.rs
- confidence: cited
  path: src/collection/compose.rs
- confidence: inferred
  path: src/collection/mod.rs
- confidence: cited
  path: src/lib.rs
- confidence: inferred
  path: tests/collection_composition.rs
- confidence: cited
  path: tests/collection_conformance.rs
- confidence: cited
  path: tests/semantic_boundary.rs
revision: 17
transitions:
- {from: "draft", to: "proposed", at: "2026-10-03T10:35:26Z", actor: "human:timo", revision: 5, decided_on: {"recorded":{"review_outcome":2}}}
- {from: "proposed", to: "active", at: "2026-10-03T10:35:26Z", actor: "human:timo", revision: 6, decided_on: {"recorded":{"review_outcome":2}}}
---
## Intent and existing contract

Implement the existing public Collect(CollectionRequest) -> CollectResponse { snapshot: FactSnapshot } Rust obligation as the production composition seam before the three language stories close. Typed home: ess-semantic/domains/semantic.yaml:438. The only declared outcome is collected; there is no invented rejected outcome or top-level gaps field. Collection failure and unavailable capabilities are carried by the snapshot's existing per-family Coverage.gaps. No new noun or command is introduced. Depends on source-snapshot and capability-admission.

## Acceptance and named ESS scenarios

`parity-collect-source-only-foundation` invokes the real Collect handler over exact selected Rust/Go/Java fixture bytes and asserts independent source/configuration identities. With no language slot registered, units and all observation arrays are empty. Coverage contains exactly one record per each of the fifteen existing FactFamily variants, each with units=[] (the empty population, not a wildcard). Sources is Complete only after complete successful capture, with gaps=[]; every other family is Unsupported with UnsupportedCapability and UnknownUnit gaps explaining unavailable language extraction/unit discovery. There is no Units family. The whole snapshot passes actual admission but is not a complete analysis and cannot yield zero measurements.

`parity-collect-semantic-gap` submits Semantic on the same readable fixture. Snapshot.mode remains Semantic; collection may retain actual source bytes and their identity, but all fourteen non-Sources families remain Unsupported with a precise UnsupportedCapability gap stating that semantic adapters are not implemented. tool_observations=[] because no tool was invoked, and no MissingTool assertion about installed tools is fabricated. Sources only records the actual capture result. The collected outcome is an observation response, never a claim of complete semantic analysis. This is the existing typed representation of an unsatisfied semantic request; no successful source-only fallback is reported.

For a source capture failure, no unread bytes or placeholder files are fabricated: retain a canonical empty observed SourceSnapshot for the requested selection/configuration, put the actual capture gaps in Sources with Failed, and leave every unobserved family Unsupported. Independently recompute the returned empty-content identity and retain the original mode. Add `parity-collect-capture-failure` to assert this response for a missing selected file with a valid relative path, including admission and failed Sources coverage.

The aggregate `collection-composition-baseline` verification report must show all three exact authored cases plus the selected structural Collect case passed against the actual handler, with zero failed/error/unsupported/skipped scenario results, exact suite/model identities, preserved 27 evaluator and eight foundation cases, generated drift and task check green. Unsupported fact coverage inside an expected response is distinct from an unsupported scenario execution.

## Production seam

The shared production binding registration and ID/range/evidence helpers operate over retained content. Empty registration has the exact unsupported coverage above; later workers fill their language slot through the same real Collect implementation. No fixture-name dispatch or canned responses. Each language's authored scenarios must reach real production registration before its story closes. Native scenario selection is component-scoped to existing Collect, not the other five unfinished public commands.

## Scope

Derived 2026-10-03 by independent aep:story-scoper. Cited production surfaces: src/collection/compose.rs, src/bindings/mod.rs, src/lib.rs. Inferred module declaration: src/collection/mod.rs. Cited conformance/gate surfaces: tests/collection_conformance.rs, ess-semantic/components.yaml, ess-semantic/ess-inputs.yaml, ess-semantic/scenarios/collection/**, src/bin/codegate-check.rs. Cited regeneration surfaces: generated/semantic-behavior and generated/semantic-wire. Inferred runtime regressions: tests/collection_composition.rs.

Confidence high: the story names these seams; collect_source at src/collection/mod.rs:648 retains source content, semantic_wire.rs:511 serializes FactSnapshot, and semantic/admit.rs:735 validates coverage independently. Safety evidence level 2, not yet proven by execution. Collides with collection exports, binding registration, semantic projections, and gate/scenario registration.

Corrections to earlier inferred wording: actual fields are Coverage.unit_ids and FactSnapshot.tools. A valid failed capture can return canonical empty observed content; invalid selection/configuration that cannot form a valid identity must produce an explicit invocation error, never normalized into a successful-looking admitted snapshot. No new business outcome is introduced.

## Boundary and order

This prerequisite supplies the Collect Rust handler, common registration and native runner, not the CLI or later assessment/navigation handlers. source-structure owns cross-language equivalence and aggregate integration using existing registration; first-slice owns JSON command exposure, assessment/capabilities and declaration lookup. Coordinator owns scenario registration during language units. Freeze shared seams before forking the three disjoint language workers.

## Documentation scope correction

Coordinator adds AGENTS.md, README.md and specifications/semantic-contract.md to the implementation scope because current text still says all six public handlers are missing (README.md:108, AGENTS.md:35, specifications/semantic-contract.md:5). Their updates must describe only the verified public Collect seam and retain the five unimplemented public handlers and incomplete source-language extraction. This corrects the scoper's inferred no-documentation assumption.

## CI evidence retention

Cited .github/workflows/ci.yml uploads evaluator and foundation native JSON only. Add collection-conformance JSON to the same existing artifact so CI retains the newly required evidence; no new execution language or publication credentials are introduced.

## Structural input witness

ESS 0.50.0 synthesized Collect with placeholder strings for configuration identity/classpath and root (observed by conformance worker in red-suite.json). Those are invalid production inputs, so the runtime correctly refuses them. Add the existing command's fixture_inputs metadata and a Rust fixture provider supplying a valid collection request; retain production validation. No new value, command, outcome or fixture-name dispatch in production is introduced. The structural case remains a shape witness; authored cases provide the actual semantic assertions.

## Public wiring boundary

Runtime package tests observed two false boundary rejections for root public reexports collection::compose::{Collector, collect}. The new implementation lives outside the pure core; the root only exposes the operation. Authorize tests/semantic_boundary.rs to recognize those exact public wiring exports while retaining alias resolution and adding a regression proving an algorithm call through the same alias is still refused. This is a testable distinction, not blanket admission of collection calls. The worker owns this scoped correction and reports the original red package output.
