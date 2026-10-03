---
format: aep.planning-md/3
id: executable-system-specification:semantic-analysis
kind: executable-system-specification
status: validated
title: Source-bound semantic analysis and navigation contracts
relations:
- specifies: vision:language-neutral-code-quality
model_digest: 78424588ff9f681ee0fb4662ee035a99c049cca6429cc1636f12d70420a138b2
revision: 3
transitions:
- {from: "draft", to: "validated", at: "2026-10-02T23:56:21Z", actor: "human:timo", revision: 2, decided_on: {"recorded":{"verification":1}}}
---
## Contract

`ess-semantic/system.yaml`, `ess-semantic/ess-inputs.yaml` and
`ess-semantic/domains/semantic.yaml` define codegate_semantic v1 using ESS 0.50.0.
The new values and six command interfaces cover source snapshots/configuration,
declarations/occurrences/dependencies/references/calls/implementations, decisions,
effects, framework facts, measurements, coverage, policies, assessment/navigation,
scoring, suggestions and capabilities. They are immutable value records, not invented
persistent entities. Existing dependency /0.1 source and generated crates remain separate.

## Normative semantics

`specifications/semantic-contract.md` states source identities, exact counting,
resolution bases, coverage and admission rules. `specifications/semantic-scenarios.md`
names future authored runtime scenarios. `docs/parity-baseline.md` binds this contract
to the pinned reference inventory and intentional semantic differences.

## Generated products

`generated/semantic-behavior` and `generated/semantic-wire` are emitted by ESS and
formatted by pinned rustfmt. Synthesis plans/runtime obligations are retained.
Command behavior traits remain explicit obligations: generation is not implementation.
No hand-written model mirrors these generated types.

## Verification boundary

ESS validation, compile and deterministic regeneration prove the model and products.
The synthesized structural command suite is not an authored real-runtime parity suite.
Do not move this artifact to conforming until real target conformance evidence for the
exact compiled model is recorded. The dependency domain retains its existing 27 cases.
