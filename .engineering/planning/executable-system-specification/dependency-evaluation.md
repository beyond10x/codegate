---
format: aep.planning-md/3
id: executable-system-specification:dependency-evaluation
kind: executable-system-specification
status: conforming
title: Immutable dependency facts and offline evaluation
summary: Typed value-graph contract with 26 authored direct-response conformance scenarios.
relations:
- derived_from: architecture-design:language-neutral-fact-ir
model_digest: e27ccc45957cf4fe2ef83d9362e81d867eb46bff9a72f1fd28e43b69bf53ec93
revision: 5
transitions:
- {from: "draft", to: "validated", at: "2026-10-02T12:53:06Z", actor: "human:timo", revision: 3, executor: "agent:codegate-wave001", correlation: "codegate-wave001"}
- {from: "validated", to: "conforming", at: "2026-10-02T12:53:46Z", actor: "human:timo", revision: 4, decided_on: {"recorded":{"static_analysis":1,"ess_conformance_coverage_v1":1}}, executor: "agent:codegate-wave001", correlation: "codegate-wave001"}
---
## Contract

`ess/system.yaml` and `ess/domains/dependency.yaml` define the first typed slice:
FactSnapshot, Unit, Edge, Coverage, Policy, FanOut, Finding, Diagnostic and Evaluation.
`ess/README.md` specifies admission, common dependency analysis, coverage and verdict
semantics. `ess/ess-inputs.yaml` pins ESS 0.50.0 and selects 26 authored scenarios.

These are immutable values, not persisted entities. Struct containment is typed;
ESS entity relations cannot be attached to structs. Graph references are an explicit
runtime admission obligation checked by named authored scenarios. No entity lifecycle
or compiler-proven referential integrity is claimed.

## Evidence observed in planning

`ess specify validate --path ess`:

    codegate v1 — 2 file(s), 26 scenario(s), valid

`ess verify conform author --path ess --scenarios ess --out .scratch/authored-suite.json`:

    26 authored scenario(s) from 26 file(s), 0 refusal(s), suite ess-conformance/28, written to .scratch/authored-suite.json

`ess verify conform synthesize --path ess --scenarios ess --out .scratch/combined-suite.json`:

    27 scenario(s) (26 authored), 0 refusal(s), written to .scratch/combined-suite.json

`ess generate synthesize --path ess --target rust --layout crate --out .scratch/synth-probe`:

    18 capabilities: 17 generated, 1 obligation(s), 0 refused
    7 artifact(s), written to .scratch/synth-probe

The behavior obligation is Evaluate; generation is not an implementation. The standalone
wire type projection also succeeds with 16 types and one integer-integrality runtime
obligation. No conformance execution or approval is asserted, and the artifact remains
draft under this stage-1-only request.

## Validation corrections

The initial draft was refused with these diagnostics, then corrected to v1 and a
command without an unsupported summary field:

    ess was refused:
      - domains/dependency.yaml: unknown field `summary`, expected one of `name`, `input`, `fixture_inputs`, `response`, `outcomes`, `naming`, `refs`
      - system.yaml: invalid version reference "v0": expected a whole number after `v`, without a leading zero

The application formats remain experimental /0.1; ESS system v1 is not a stable-IR claim.

## Deferred

Actual language extraction, multi-producer merging, broader fact families, canonical
content digests, baseline matching, architectural cycles and policy exceptions.

## First implementation evidence

The first-wave integration gate exited0 at candidate `4bf8fa591888698fdf23194d93daebb81bbb4e18`. Official coverage-bearing synthesis (`--suite-format 5`) selects `ess-conformance/29`, with the same26 authored plus1 generated scenario, complete inventory and zero refusals. Native ESS Runner against the real evaluator executed27/27 passed, zero other outcomes. Report model digest matches this artifact; AEP imported the actual report/2 plus exact suite as ess_conformance_coverage_v1, and moved validated -> conforming on that evidence.

Current report/suite digest and full gate output are in `verification-report:codegate-wave1`. Exact native files are retained in the coordinator worktree recovery archive at `.scratch/wave-001/final-native/`. Conforming means the complete declared selected inventory passed; it does not claim exhaustive correctness for every possible input or an implemented source-language extractor.

The historical planning evidence above remains as observed then. The first ordinary suite report had unknown coverage, and the first implementation report had a synthetic clock; neither is used as final conformance evidence. The official inventory-bearing suite and corrected wall-clock observer are used in the final integration run.
