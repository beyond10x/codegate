---
format: aep.planning-md/3
id: story:parity-adoption
kind: story
status: draft
title: Verify reference comparisons and advisory repository pilots
relations:
- decomposes: epic:semantic-parity
- informed_by: executable-system-specification:semantic-analysis
- depends_on: story:reporting-parity
scope:
- confidence: inferred
  path: docs/parity-baseline.md
- confidence: inferred
  path: docs/pilots/**
- confidence: inferred
  path: tests/fixtures/mixed/**
- confidence: inferred
  path: tests/fixtures/reference/**
- confidence: inferred
  path: tests/reference_comparison.rs
revision: 4
---
## Intent

Run pinned reference on Go fixtures via external reference tool, normalize deterministic shared outputs, document intentional differences; no changed expected value to conceal a gap; one representative pilot each Go/Rust/Java-Quarkus plus mixed fixture; any unsupported required capability leaves story open.

## Contract

Typed values: Assessment, Capability, Coverage, in `ess-semantic/domains/semantic.yaml`.
Normative rules: `specifications/semantic-contract.md`; baseline:
`docs/parity-baseline.md`. These paths were validated before this story was created.

## Acceptance

The parity-adoption audit artifact contains reconciled evidence for every baseline row and the required Go, Rust, Java/Quarkus and mixed-language advisory pilot results, with zero unsupported required capabilities before any new check becomes required.

## Named conformance scenarios

- `parity-pinned-reference-comparison`
- `parity-mixed-language`
- `parity-advisory-pilots`

These are required authored ESS acceptance scenarios, not a claim that they execute
today. The implementing story adds their fixtures/runner assertions; closing evidence
must contain the actual scenario counts and exact inputs. A generated obligation stub
or mock-only external-tool test cannot satisfy acceptance. Preserve the legacy 27.

## Dependencies

`story:reporting-parity`

## Scope

- inferred: `tests/reference_comparison.rs` — planned owned implementation/verification surface.
- inferred: `tests/fixtures/reference/**` — planned owned implementation/verification surface.
- inferred: `tests/fixtures/mixed/**` — planned owned implementation/verification surface.
- inferred: `docs/parity-baseline.md` — planned owned implementation/verification surface.
- inferred: `docs/pilots/**` — planned owned implementation/verification surface.

These inferred surfaces reflect the accepted architecture, not existing code.
Validate and revise them before implementation dispatch. Only the coordinator mutates
AEP or shared integration manifests outside the story's scope; adapters register through
the semantic-navigation seam. All authored runnable code/harnesses are Rust (clap
for command lines); Go and Java source remain fixture data.

## Completion evidence

Run targeted cases and the required integration `task check`; retain named real ESS
results, generated drift result and relevant reference comparison outputs. No GitHub
publication or release is included. An unsupported required capability remains a gap.
