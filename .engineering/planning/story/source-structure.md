---
format: aep.planning-md/3
id: story:source-structure
kind: story
status: draft
title: Extract Tree-sitter observations in all three languages
relations:
- informed_by: executable-system-specification:semantic-analysis
- depends_on: story:source-snapshot
- depends_on: story:capability-admission
- decomposes: epic:source-collection-baseline
- depends_on: story:source-go-baseline
- depends_on: story:source-rust-baseline
- depends_on: story:source-java-baseline
scope:
- confidence: inferred
  path: src/bindings/mod.rs
- confidence: inferred
  path: tests/fixtures/structure/equivalence
- confidence: inferred
  path: tests/structure.rs
revision: 5
---
## Intent

Use Tree-sitter grammars with pinned Cargo dependencies; declarations/imports/control-flow/comments/annotations; source-only candidates never become resolved calls; Java annotations initial declared facts only; evidence positions byte-based with specified Unicode conversions.

## Contract

Typed values: Declaration, Occurrence, Dependency, DecisionPoint, StructureObservation, FrameworkFact, in `ess-semantic/domains/semantic.yaml`.
Normative rules: `specifications/semantic-contract.md`; baseline:
`docs/parity-baseline.md`. These paths were validated before this story was created.

## Acceptance

Collecting equivalent Go/Rust/Java structure fixtures returns the specified declarations, source spans and syntactic observations with syntax errors reflected in family coverage.

## Named conformance scenarios

- `parity-three-language-structure`
- `parity-syntax-error`
- `parity-candidate-not-resolved`
- `parity-quarkus-annotations`

These are required authored ESS acceptance scenarios, not a claim that they execute
today. The implementing story adds their fixtures/runner assertions; closing evidence
must contain the actual scenario counts and exact inputs. A generated obligation stub
or mock-only external-tool test cannot satisfy acceptance. Preserve the legacy 27.

## Dependencies

`story:source-snapshot`, `story:capability-admission`

## Scope

- inferred: `src/bindings/**` — planned owned implementation/verification surface.
- inferred: `Cargo.toml` — planned owned implementation/verification surface.
- inferred: `Cargo.lock` — planned owned implementation/verification surface.
- inferred: `tests/structure.rs` — planned owned implementation/verification surface.
- inferred: `tests/fixtures/structure/**` — planned owned implementation/verification surface.

These inferred surfaces reflect the accepted architecture, not existing code.
Validate and revise them before implementation dispatch. Only the coordinator mutates
AEP or shared integration manifests outside the story's scope; adapters register through
the semantic-navigation seam. All authored runnable code/harnesses are Rust (clap
for command lines); Go and Java source remain fixture data.

## Completion evidence

Run targeted cases and the required integration `task check`; retain named real ESS
results, generated drift result and relevant reference comparison outputs. No GitHub
publication or release is included. An unsupported required capability remains a gap.


## Existing parser seam

Reuse the Tree-sitter grammar setup and syntax-aware line classifier delivered by
source-snapshot. This story adds declaration/relationship/annotation observations;
it does not supply a classifier required by an earlier prerequisite. Source/configuration
identities are already finalized over complete classified input before this stage.

## First-round decomposition

Language extraction is delegated to story:source-go-baseline, story:source-rust-baseline and story:source-java-baseline. This story now owns shared registration and cross-language equivalence integration in src/bindings/mod.rs, tests/structure.rs and tests/fixtures/structure/equivalence. Earlier broad scope describes the original unsplit design; these narrower typed entries supersede that ownership. It does not reimplement the three workers' parsers. Its dependencies include all three language children and the two foundation stories.
