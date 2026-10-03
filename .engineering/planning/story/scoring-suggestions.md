---
format: aep.planning-md/3
id: story:scoring-suggestions
kind: story
status: draft
title: Compute complete-evidence scores and advisory suggestions
relations:
- decomposes: epic:semantic-parity
- informed_by: executable-system-specification:semantic-analysis
- depends_on: story:architecture-policies
- depends_on: story:safety-observations
- depends_on: story:testability-signals
- depends_on: story:quarkus-relationships
scope:
- confidence: inferred
  path: src/lib.rs
- confidence: inferred
  path: src/main.rs
- confidence: inferred
  path: src/semantic/scoring.rs
- confidence: inferred
  path: src/semantic/suggest.rs
- confidence: inferred
  path: tests/scoring.rs
- confidence: inferred
  path: tests/suggestions.rs
revision: 3
---
## Intent

Four reference gate summaries; severity/LOC weights, findings pressure; unused symbols/extract/parameter objects/boolean flags/high fan-in/debt suggestions; add suggest command; suggestions contain no executable operations; preserve intentional differences from go-architecture-v1.

## Contract

Typed values: Policy, ScoreComponent, Score, Finding, Suggestion, Assessment, in `ess-semantic/domains/semantic.yaml`.
Normative rules: `specifications/semantic-contract.md`; baseline:
`docs/parity-baseline.md`. These are proposed paths until the foundation is integrated.

## Acceptance

A selected versioned scoring policy produces deterministic ranked findings, pressure and advisory suggestions, with no complete overall score when any required component lacks evidence.

## Named conformance scenarios

- `parity-score-required-gap`
- `parity-score-policy-version`
- `parity-suggestion-ranking`
- `parity-advice-never-edits`

These are required authored ESS acceptance scenarios, not a claim that they execute
today. The implementing story adds their fixtures/runner assertions; closing evidence
must contain the actual scenario counts and exact inputs. A generated obligation stub
or mock-only external-tool test cannot satisfy acceptance. Preserve the legacy 27.

## Dependencies

`story:architecture-policies`, `story:safety-observations`, `story:testability-signals`, `story:quarkus-relationships`

## Scope

- inferred: `src/semantic/scoring.rs` — planned owned implementation/verification surface.
- inferred: `src/semantic/suggest.rs` — planned owned implementation/verification surface.
- inferred: `src/lib.rs` — planned owned implementation/verification surface.
- inferred: `src/main.rs` — planned owned implementation/verification surface.
- inferred: `tests/scoring.rs` — planned owned implementation/verification surface.
- inferred: `tests/suggestions.rs` — planned owned implementation/verification surface.

These inferred surfaces reflect the accepted architecture, not existing code.
Validate and revise them before implementation dispatch. Only the coordinator mutates
AEP or shared integration manifests outside the story's scope; adapters register through
the semantic-navigation seam. All authored runnable code/harnesses are Rust (clap
for command lines); Go and Java source remain fixture data.

## Completion evidence

Run targeted cases and the required integration `task check`; retain named real ESS
results, generated drift result and relevant reference comparison outputs. No GitHub
publication or release is included. An unsupported required capability remains a gap.
