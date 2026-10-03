---
format: aep.planning-md/3
id: story:source-snapshot
kind: story
status: draft
title: Collect selected source and build configuration identities
relations:
- informed_by: executable-system-specification:semantic-analysis
- depends_on: story:parity-foundations
- decomposes: epic:source-collection-foundation
scope:
- confidence: inferred
  path: Cargo.lock
- confidence: inferred
  path: Cargo.toml
- confidence: inferred
  path: specifications/source-baseline.md
- confidence: inferred
  path: src/bindings/lines.rs
- confidence: inferred
  path: src/bindings/mod.rs
- confidence: inferred
  path: src/collection/**
- confidence: inferred
  path: src/source_identity.rs
- confidence: inferred
  path: tests/fixtures/selection/**
- confidence: inferred
  path: tests/source_snapshot.rs
revision: 7
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

- inferred: `src/collection/**` — planned owned implementation/verification surface.
- inferred: `tests/source_snapshot.rs` — planned owned implementation/verification surface.
- inferred: `tests/fixtures/selection/**` — planned owned implementation/verification surface.

These inferred surfaces reflect the accepted architecture, not existing code.
Validate and revise them before implementation dispatch. Only the coordinator mutates
AEP or shared integration manifests outside the story's scope; adapters register through
the semantic-navigation seam. All authored runnable code/harnesses are Rust (clap
for command lines); Go and Java source remain fixture data.

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
