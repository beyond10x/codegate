---
format: aep.planning-md/3
id: executable-system-specification:dependency-evaluation
kind: executable-system-specification
status: draft
title: Immutable dependency facts and offline evaluation
summary: Typed value-graph contract with 26 authored direct-response conformance scenarios.
relations:
- derived_from: architecture-design:language-neutral-fact-ir
revision: 1
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
