---
format: aep.planning-md/3
id: story:source-snapshot
kind: story
status: implemented
title: Collect selected source and build configuration identities
relations:
- informed_by: executable-system-specification:semantic-analysis
- depends_on: story:parity-foundations
- decomposes: epic:source-collection-foundation
- serves: vision:language-neutral-code-quality
scope:
- confidence: cited
  path: .github/workflows/ci.yml
- confidence: cited
  path: AGENTS.md
- confidence: cited
  path: Cargo.lock
- confidence: cited
  path: Cargo.toml
- confidence: cited
  path: README.md
- confidence: cited
  path: ess-semantic/ess-inputs.yaml
- confidence: cited
  path: ess-semantic/scenarios/**
- confidence: cited
  path: specifications/source-baseline.md
- confidence: cited
  path: src/bin/codegate-check.rs
- confidence: cited
  path: src/bindings/lines.rs
- confidence: cited
  path: src/bindings/mod.rs
- confidence: cited
  path: src/collection/**
- confidence: cited
  path: src/foundation.rs
- confidence: cited
  path: src/lib.rs
- confidence: cited
  path: src/semantic_wire.rs
- confidence: cited
  path: src/source_identity.rs
- confidence: cited
  path: tests/foundation_conformance.rs
- confidence: cited
  path: tests/identity_adversary.rs
- confidence: cited
  path: tests/semantic_wire.rs
- confidence: cited
  path: tests/source_identity.rs
- confidence: cited
  path: tests/source_snapshot.rs
- confidence: cited
  path: verification/source-foundation/**
revision: 18
transitions:
- {from: "draft", to: "proposed", at: "2026-10-03T07:36:51Z", actor: "human:timo", revision: 8, decided_on: {"recorded":{"review_outcome":1}}, executor: "agent:codegate-collection"}
- {from: "proposed", to: "active", at: "2026-10-03T07:36:51Z", actor: "human:timo", revision: 9, decided_on: {"recorded":{"review_outcome":1}}, executor: "agent:codegate-collection"}
- {from: "active", to: "implemented", at: "2026-10-03T08:42:20Z", actor: "human:timo", revision: 15, decided_on: {"recorded":{"test_result":1,"review_outcome":5,"verification":1,"ess_conformance_coverage_v1":1}}, executor: "agent:codegate-collection"}
---
## Intent

Default source-only reads files without executing build scripts or tools; deterministic ordering and explicit include/exclude/test/generated selection; bounded reads and relative paths; Maven/Gradle module manifests, Cargo and Go selections.

## Contract

Typed values: SourceSelection, SourceSnapshot, SourceFile, Manifest, BuildSelection, in `ess-semantic/domains/semantic.yaml`.
Normative rules: `specifications/semantic-contract.md`; baseline:
`docs/parity-baseline.md`. These paths were validated before this story was created.

## Acceptance

Collecting equivalent selected bytes and configuration produces the same snapshot identity, while any selected dirty/untracked input or relevant build configuration change changes that identity.

## Named conformance scenarios

- `parity-selected-content-identity`
- `parity-manifest-identity`
- `parity-path-confinement`

These are required authored ESS acceptance scenarios, not a claim that they execute
today. The implementing story adds their fixtures/runner assertions; closing evidence
must contain the actual scenario counts and exact inputs. A generated obligation stub
or mock-only external-tool test cannot satisfy acceptance. Preserve the legacy 27.

## Dependencies

`story:parity-foundations`

## Scope

- cited: src/collection/{mod,manifests,secure_fs}.rs — actual bounded collection and static manifest implementation (4c99cdc).
- cited: src/bindings/{mod,lines}.rs — actual shared Tree-sitter physical line parser (4c99cdc).
- cited: tests/source_snapshot.rs —21 actual selection/configuration/confinement/manifest tests; fixtures are built by Rust tests, so the previously inferred tests/fixtures/selection tree was not created.
- cited: src/source_identity.rs, src/semantic_wire.rs, their Rust tests and Cargo.toml/Cargo.lock — coordinator-supplied shared identity/projection/dependency setup.
- cited: src/foundation.rs, src/lib.rs, tests/foundation_conformance.rs, tests/identity_adversary.rs, ess-semantic/scenarios/**, ess-semantic/ess-inputs.yaml and src/bin/codegate-check.rs — coordinator integration and real ESS witnesses (5d5192d).
- cited: specifications/source-baseline.md, AGENTS.md, README.md and verification/source-foundation/** — counting/selection contracts, public limits and actual gate evidence.

Earlier inferred paths were confirmed or corrected by unit report verification-report:source-snapshot-unit. No language declaration extractor or public collection CLI is included.

## Completion evidence

Run targeted cases and the required integration `task check`; retain named real ESS
results, generated drift result and relevant reference comparison outputs. No GitHub
publication or release is included. An unsupported required capability remains a gap.


## Classifier ownership and identity finalization

This story owns the Tree-sitter grammar setup and syntax-aware physical line
classifier in `src/bindings/lines.rs`, plus `Cargo.toml` and `Cargo.lock` updates.
It populates SourceLine categories before finalizing SourceSnapshot identity.
The later source-structure story reuses that parser seam for declarations and other
observations; source-snapshot does not depend on its later implementation. Classifier
fixtures cover comments, multiline literals and parse-error gaps in all three languages.
Git origin is provenance and is omitted from snapshot/configuration identity as the
normative contract specifies; committing unchanged content cannot change identity.
These additional paths are inferred implementation scope, owned by this story.

## First-round profile and ownership

specifications/source-baseline.md pins literal selection semantics, bounds, Rust-library Git provenance, static manifest interpretation, coverage scope, offline range guarantees and scenario IDs. Existing generated value types remain unchanged. Coordinator owns src/source_identity.rs, Cargo.toml/Cargo.lock, src/lib.rs and authored ESS suite registration. The source worker owns src/collection/**, src/bindings/lines.rs and src/bindings/mod.rs plus selection tests. Identity helpers are supplied before worker dispatch, so admission and collection do not depend on each other's unfinished code. Root dependencies and the pure identity seam are the coordinator's part of this story, not a parallel worker surface.

## Foundation conformance seam

The conformance review identified that the six public commands do not type the intermediate helpers: Collect returns FactSnapshot and Evaluate returns Assessment. Before implementation dispatch, extend ESS with internal CollectSource and ValidateSnapshot operations, using existing source/fact/gap value types, and a source-foundation component. Native ESS targets call actual helpers. Six foundation behavioral cases execute in that component; parity-core-boundary is a separate architectural Rust test with injected violations, not a fabricated semantic command outcome. The six public runtime obligations and specification lifecycle remain incomplete. Coordinator owns the ESS/generated integration; implementation workers own their previously scoped runtime files.

## Coordinator protocol seam

The foundation ESS target requires strict generated-value JSON conversion. The coordinator's source-snapshot integration adds src/semantic_wire.rs and tests/semantic_wire.rs, translating existing generated behavior/wire values without redefining domain types. This is a pure serialization seam, not a binding or analysis algorithm. It shares no worker paths with collection/admission. Identity and serialization may map language enum values; shared metrics/check/query algorithms still may not select behavior by language or producer. Root module declarations may register collection, but the existing offline evaluator and all shared algorithms remain free of IO and binding calls.

## Foundation API and conformance ownership

The coordinator also owns src/foundation.rs, tests/foundation_conformance.rs, ess-semantic authored scenarios and src/bin/codegate-check.rs integration. Foundation implements the two generated internal obligation traits by retaining actual helper responses; generated outcome names alone are never evidence. The native ESS target forwards JSON through generated wire conversion and checks literal outputs against real filesystem fixtures. Source-only public Collect/Assess and language bindings remain subsequent stories.
