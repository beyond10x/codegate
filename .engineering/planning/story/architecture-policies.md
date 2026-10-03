---
format: aep.planning-md/3
id: story:architecture-policies
kind: story
status: draft
title: Evaluate shared architecture policies and exceptions
relations:
- decomposes: epic:semantic-parity
- informed_by: executable-system-specification:semantic-analysis
- depends_on: story:first-slice
- depends_on: story:portable-metrics
scope:
- confidence: inferred
  path: src/semantic/architecture.rs
- confidence: inferred
  path: tests/architecture_policies.rs
- confidence: inferred
  path: tests/fixtures/architecture/**
revision: 3
---
## Intent

Explicit unit roles, allow/deny precedence, required family requirements, reason/owner/expiry scopes and visible waived findings; tests cover witnessed forbidden edges with partial coverage; Go internal and Java/Rust boundary mappings have explicit identities.

## Contract

Typed values: PolicyRule, PolicyException, Dependency, Effect, CheckResult, Finding, in `ess-semantic/domains/semantic.yaml`.
Normative rules: `specifications/semantic-contract.md`; baseline:
`docs/parity-baseline.md`. These are proposed paths until the foundation is integrated.

## Acceptance

Evaluating the architecture scenario matrix produces the expected direction/layer/cycle/fan-in-out/import/call/effect/test-boundary/unknown-unit and exception results from admitted facts.

## Named conformance scenarios

- `parity-architecture-matrix`
- `parity-cycle`
- `parity-exception-expiry`
- `parity-partial-architecture`

These are required authored ESS acceptance scenarios, not a claim that they execute
today. The implementing story adds their fixtures/runner assertions; closing evidence
must contain the actual scenario counts and exact inputs. A generated obligation stub
or mock-only external-tool test cannot satisfy acceptance. Preserve the legacy 27.

## Dependencies

`story:first-slice`, `story:portable-metrics`

## Scope

- inferred: `src/semantic/architecture.rs` — planned owned implementation/verification surface.
- inferred: `tests/architecture_policies.rs` — planned owned implementation/verification surface.
- inferred: `tests/fixtures/architecture/**` — planned owned implementation/verification surface.

These inferred surfaces reflect the accepted architecture, not existing code.
Validate and revise them before implementation dispatch. Only the coordinator mutates
AEP or shared integration manifests outside the story's scope; adapters register through
the semantic-navigation seam. All authored runnable code/harnesses are Rust (clap
for command lines); Go and Java source remain fixture data.

## Completion evidence

Run targeted cases and the required integration `task check`; retain named real ESS
results, generated drift result and relevant reference comparison outputs. No GitHub
publication or release is included. An unsupported required capability remains a gap.
