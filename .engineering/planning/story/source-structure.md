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
  path: tests/fixtures/structure/equivalence
- confidence: inferred
  path: tests/structure.rs
revision: 9
---
## Intent

Integrate the three language baseline bindings over retained collected bytes: units, declarations, definition/import occurrences, declared dependency relationships and the initial Java framework catalogue. Reuse the already delivered syntax-aware line classification. Source-only candidates never become resolved calls; byte-based locations preserve Unicode offsets. Decision/control-flow, body structure, documentation and broader maintainability observations are explicitly delivered by story:source-structural-observations after this baseline and remain Unsupported here.

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

- inferred: tests/structure.rs — shared equivalence integration assertions.
- inferred: tests/fixtures/structure/equivalence/** — equivalent Go/Rust/Java source fixtures.

Production registration moved to story:collection-composition-seam; language implementation stays in its three disjoint children. Coordinator owns ESS scenario registration/runner updates. This supersedes earlier inferred parser ownership; no language worker edits AEP.

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

Language extraction is delegated to story:source-go-baseline, story:source-rust-baseline and story:source-java-baseline. story:collection-composition-seam already supplies production registration and its helpers before those workers. This story reuses that registration to verify cross-language equivalence and aggregate integration in tests/structure.rs and tests/fixtures/structure/equivalence; it does not create registration or reimplement parsers. Earlier broad scope describes the original unsplit design; these narrower entries supersede it. Dependencies include all three language children and the two foundation stories.
