---
format: aep.planning-md/3
id: epic:source-collection-baseline
kind: epic
status: active
title: Deliver the first usable Rust Go and Java source-only baseline
relations:
- decomposes: epic:semantic-parity
- depends_on: epic:source-collection-foundation
- informed_by: executable-system-specification:semantic-analysis
revision: 6
transitions:
- {from: "draft", to: "proposed", at: "2026-10-03T10:35:26Z", actor: "human:timo", revision: 5, decided_on: {"recorded":{"review_outcome":2}}}
- {from: "proposed", to: "active", at: "2026-10-03T10:35:27Z", actor: "human:timo", revision: 6, decided_on: {"recorded":{"review_outcome":2}}}
---
## Intent

Deliver a healthy source-only baseline across Rust, Go and Java from the existing semantic contract, with equivalent fixtures and honest limits. This is the first usable collection round, not full semantic parity.

## Acceptance

The first-collection-baseline verification report passes the full gate and the exact authored case inventory for the three language stories, source-structure and first-slice, retaining real handler outputs and suite/model identities with zero failed, errored, unsupported or skipped selected cases, equivalent fan-out/forbidden-dependency results in all three languages, and explicit partial coverage for the named syntax/build/semantic gaps.

## Children and order

First story:collection-composition-seam supplies the existing public Collect Rust obligation, production registration/helpers and component-scoped native runner over the completed foundation. Then three disjoint stories source-go-baseline, source-rust-baseline and source-java-baseline implement their language slots and close on real Collect conformance. source-structure integrates their equivalent observations. first-slice adds the JSON CLI and shared assessment/capabilities/declaration lookup API. Full imported graph navigation remains story:offline-navigation. This repairs the practical acceptance cycle without changing any existing domain value or claiming unsupported language bindings complete.

## Healthy baseline requirements

Exact dirty/untracked content identities, portable byte-based locations, deterministic offline replay, no external process in source-only mode, explicit partial coverage, versioned JSON, all 27 legacy conformance cases, and real new ESS fixtures for the delivered handlers. Rust macros/cfg, Go build constraints, Java classpaths/generated declarations and nonliteral Gradle configuration remain explicit gaps unless static selected evidence proves the specific relationship. Equivalent fixtures must not turn same spelling into resolved relationships. Java includes declared Quarkus annotation observations only.

## Out of scope

Language servers, effective build execution, dynamic call graph completeness, effective CDI wiring, maintainability breadth, scoring and HTML reports. Those remain required later parity capabilities and are not relabeled non-applicable.
