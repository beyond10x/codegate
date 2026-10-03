---
format: aep.planning-md/3
id: design:collection-composition-wave
kind: design
status: draft
title: Production collection composition prerequisite wave
relations:
- designs: story:collection-composition-seam
- serves: vision:language-neutral-code-quality
revision: 1
---
## Authority and selection

AEP implementing skill 0.19.1, wave mode. The active operator goal is that Codegate creates attractive repository reports. The next computed prerequisite is story:collection-composition-seam, serving vision:language-neutral-code-quality. Recorded approval-record:standing-wave-approval authorizes upcoming waves; the operator's three-worker cap and PR integration authorization persist. No additional release is selected.

This wave authorizes an opening planning/specification commit, bounded implementation and harness commits for this one story, integration merges, closing evidence/store commit and bot PR merge to main after the gate. Later language bindings, assessment and HTML reports remain required work rather than being counted complete here.

## Preflight

Observed primary clean main and remote main at 2132b5b26a05ecfbe91940c0def232d08b14fc6e. worktree repo list showed only primary before creating this wave. Prior release tree and its branches were removed with archived evidence. Home filesystem has 36 GiB available; tmpfs has 9.3 GiB, so use isolated home-backed targets with RUSTC_WRAPPER=/usr/bin/sccache, CARGO_BUILD_JOBS=2 and CARGO_INCREMENTAL=0. Previous measured full gate used approximately 808 MiB (design:source-collection-round-one); recheck the 20 GiB floor at build launch. No prior Codegate build is scheduled or reused.

Coordinator tree wt-b26eeec7c8ff, branch wave/collection-composition, scratch .scratch/composition, target .scratch/composition/target. Exact absolute paths and lease are retained in the private worktree registry. Stage: scope review and conformance preparation. Runtime implementation will use its own managed tree and target, while the coordinator owns ESS/component registration, generated projection integration and native conformance gate.

## Roles

This host exposes generic collaboration agents, not plugin subagent_type selectors. Each role loads the exact aep:story-scoper, aep:implementor or aep:adversary procedure. One runtime worker plus coordinator conformance work is bounded within the three-worker cap; the subsequent adversary is independent. All committed executable implementation and harnesses are Rust, with clap for CLIs. Workers never mutate AEP. Scope review runs read-only before implementation dispatch.

## Computed scheduling
wave 1
  story:collection-composition-seam (inferred)
wave 2
  story:source-go-baseline (inferred)
  story:source-java-baseline (inferred)
  story:source-rust-baseline (inferred)
wave 3
  story:source-structure (inferred)
wave 4
  story:first-slice (inferred)
  story:source-structural-observations (inferred)
wave 5
  story:offline-navigation (inferred)
  story:portable-metrics (inferred)
wave 6
  story:architecture-policies (inferred)
  story:safety-observations (inferred)
  story:semantic-navigation (inferred)
  story:testability-signals (inferred)
wave 7
  story:go-semantics (inferred)
  story:java-semantics (inferred)
  story:rust-semantics (inferred)
wave 8
  story:quarkus-relationships (inferred)
wave 9
  story:scoring-suggestions (inferred)
wave 10
  story:reporting-parity (inferred)
wave 11
  story:parity-adoption (inferred)
collision: story:collection-composition-seam story:first-slice src/lib.rs (inferred)
collision: story:collection-composition-seam story:offline-navigation src/lib.rs (inferred)
collision: story:collection-composition-seam story:reporting-parity src/bin/codegate-check.rs (inferred)
collision: story:collection-composition-seam story:scoring-suggestions src/lib.rs (inferred)
collision: story:collection-composition-seam story:semantic-navigation src/lib.rs (inferred)
collision: story:first-slice story:offline-navigation src/lib.rs (inferred)
collision: story:first-slice story:offline-navigation src/main.rs (inferred)
collision: story:first-slice story:reporting-parity src/main.rs (inferred)
collision: story:first-slice story:scoring-suggestions src/lib.rs (inferred)
collision: story:first-slice story:scoring-suggestions src/main.rs (inferred)
collision: story:first-slice story:semantic-navigation src/lib.rs (inferred)
collision: story:first-slice story:semantic-navigation src/main.rs (inferred)
collision: story:offline-navigation story:reporting-parity src/main.rs (inferred)
collision: story:offline-navigation story:scoring-suggestions src/lib.rs (inferred)
collision: story:offline-navigation story:scoring-suggestions src/main.rs (inferred)
collision: story:offline-navigation story:semantic-navigation src/lib.rs (inferred)
collision: story:offline-navigation story:semantic-navigation src/main.rs (inferred)
collision: story:reporting-parity story:scoring-suggestions src/main.rs (inferred)
collision: story:reporting-parity story:semantic-navigation src/main.rs (inferred)
collision: story:scoring-suggestions story:semantic-navigation src/lib.rs (inferred)
collision: story:scoring-suggestions story:semantic-navigation src/main.rs (inferred)
collision: story:source-go-baseline story:source-structural-observations src/bindings/go.rs (inferred)
collision: story:source-java-baseline story:source-structural-observations src/bindings/java.rs (inferred)
collision: story:source-rust-baseline story:source-structural-observations src/bindings/rust.rs (inferred)
11 wave(s), 24 collision(s), 0 unassessed
