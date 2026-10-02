---
format: aep.planning-md/3
id: epic:offline-dependency-evaluation
kind: epic
status: active
title: First offline language-neutral dependency evaluation
summary: A real evaluator admits experimental fact snapshots and executes shared dependency checks without a language binding.
relations:
- serves: vision:language-neutral-code-quality
- informed_by: architecture-design:language-neutral-fact-ir
- informed_by: executable-system-specification:dependency-evaluation
revision: 3
transitions:
- {from: "draft", to: "proposed", at: "2026-10-02T11:55:38Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-02T11:55:38Z", actor: "human:timo", revision: 3}
---
## Outcome

Deliver the smallest end-to-end Codegate runtime: typed dependency facts enter a
pure admission/evaluation library and a local JSON CLI; shared fan-out and forbidden
edge checks produce traceable results with explicit evidence completeness.

## Evidence

The operator requested the first planning/implementation batch on 2026-10-02.
`.engineering/planning/architecture-design/language-neutral-fact-ir.md` describes
an offline synthetic-binding slice before real extraction. `ess/domains/dependency.yaml`
provides typed homes for every value in this slice, and `ess/README.md` makes admission,
coverage and checker behavior explicit. The 26 authored scenarios compile today;
there is no implementation or executed conformance yet.

## Scope

One cohesive implementation story owns generated contracts, real behavior, JSON CLI,
ESS target adapter, conformance and its Rust gate. It establishes one shared package
surface, so the first proposed wave has one unit even though up to three implementation
workers are available. Separating types, evaluator and tests into concurrent stories
would split one abstraction and require shared Cargo/module wiring before any unit
could demonstrate a complete result.

## Completion evidence

The story's named ESS scenarios execute against the real evaluator and the combined
suite has 27 executed cases, zero failures/errors/unsupported/skips. CLI boundary
checks and mutation-sensitive regressions prove malformed or incomplete evidence
cannot pass. `task check` includes ESS validation, generation drift, real conformance,
Rust tests, formatting, linting and planning validation, without placeholder steps.

## Exclusions

Source collection and real Rust/Go bindings; a stable general IR release; calls,
control flow, source/declaration metrics, dependency cycles, ratios, baselines,
waivers, hosted execution, signing, publishing and release. Facts labelled Go in the
suite demonstrate checker label independence, not a Go implementation.
