---
format: aep.planning-md/3
id: story:parity-foundations
kind: story
status: implemented
title: Pin parity baseline and generate semantic contracts
relations:
- decomposes: epic:semantic-parity
- informed_by: executable-system-specification:semantic-analysis
- serves: vision:language-neutral-code-quality
scope:
- confidence: cited
  path: AGENTS.md
- confidence: cited
  path: README.md
- confidence: inferred
  path: docs/parity-baseline.md
- confidence: inferred
  path: ess-semantic/**
- confidence: inferred
  path: generated/semantic-behavior/**
- confidence: inferred
  path: generated/semantic-wire/**
- confidence: inferred
  path: specifications/**
- confidence: inferred
  path: src/bin/codegate-check.rs
revision: 9
transitions:
- {from: "draft", to: "proposed", at: "2026-10-02T23:56:41Z", actor: "human:timo", revision: 3}
- {from: "proposed", to: "active", at: "2026-10-02T23:56:41Z", actor: "human:timo", revision: 4}
- {from: "active", to: "implemented", at: "2026-10-03T00:22:38Z", actor: "human:timo", revision: 7, decided_on: {"recorded":{"test_result":1,"verification":1}}}
---
## Intent

Verify 38 metric IDs, 38 finding IDs, 9 violation IDs, four gates and every noncatalogue navigation/suggestion surface at the pinned reference; keep generated obligations explicit until handlers exist.

## Contract

Typed values: SourceSnapshot, FactSnapshot, all observation and result types, in `ess-semantic/domains/semantic.yaml`.
Normative rules: `specifications/semantic-contract.md`; baseline:
`docs/parity-baseline.md`. These paths were validated before this story was created.

## Acceptance

When task check runs on the new foundation, it verifies exact generated semantic contracts beside the unchanged 27 evaluator cases with zero drift or omitted cases.

## Named conformance scenarios

- `parity-catalog-complete`
- `parity-generated-drift`
- `parity-old-contract-preserved`

These are required authored ESS acceptance scenarios, not a claim that they execute
today. The implementing story adds their fixtures/runner assertions; closing evidence
must contain the actual scenario counts and exact inputs. A generated obligation stub
or mock-only external-tool test cannot satisfy acceptance. Preserve the legacy 27.

## Dependencies

None; specification foundation precedes runtime implementation.

## Scope

- inferred: `ess-semantic/**` — planned owned implementation/verification surface.
- inferred: `generated/semantic-behavior/**` — planned owned implementation/verification surface.
- inferred: `generated/semantic-wire/**` — planned owned implementation/verification surface.
- inferred: `docs/parity-baseline.md` — planned owned implementation/verification surface.
- inferred: `specifications/**` — planned owned implementation/verification surface.
- inferred: `src/bin/codegate-check.rs` — planned owned implementation/verification surface.
- inferred: `tests/semantic_contract.rs` — planned owned implementation/verification surface.

These inferred surfaces reflect the accepted architecture, not existing code.
Validate and revise them before implementation dispatch. Only the coordinator mutates
AEP or shared integration manifests outside the story's scope; adapters register through
the semantic-navigation seam. All authored runnable code/harnesses are Rust (clap
for command lines); Go and Java source remain fixture data.

## Completion evidence

Run targeted cases and the required integration `task check`; retain named real ESS
results, generated drift result and relevant reference comparison outputs. No GitHub
publication or release is included. An unsupported required capability remains a gap.


## Delivered foundation and remaining gate

Validated `ess-semantic/` with ESS 0.50.0 and generated both new Rust crates;
72 wire types and six behavior obligations compile. Revised the vision/architecture,
added the pinned baseline and counting/scenario contracts, and linked 18 stories with
scopes/dependencies. Two critic rounds ended with all four approve and all three
first-round findings fixed. `src/bin/codegate-check.rs` now checks both ESS roots
and drift/quality for all four generated crates. `AGENTS.md` and `README.md` explain
the boundary between generated foundations and unimplemented runtime features.

The added documentation paths are cited scope. `tests/semantic_contract.rs` remains
planned rather than a delivered test; no runtime acceptance scenario is claimed.
`verification-report:semantic-foundations` records exact executed checks and limits.
`resource-blocker:semantic-build-capacity` withholds the full integration test result;
this story stays active until `task check` executes and its actual evidence is recorded.

## Integration verification completed

The authorized PR/release continuation executed the full `task check` with exit 0;
`verification-report:semantic-integration` and its native 27/27 conformance report
supersede the earlier resource-limited verification state. The resource blocker is
cleared; this foundation is implemented, while semantic runtime work remains planned.

For this tooling foundation the named cases are verification identities: catalogue
inventory is reconciled against the pinned support catalogue, generated drift is
executed by codegate-check for all four crates, and compatibility is exercised by
the actual unchanged 27-case evaluator suite. No semantic runtime scenarios are
claimed. The inferred tests/semantic_contract.rs path was not needed: generation,
compilation and exact drift are already checked by the repository gate.
