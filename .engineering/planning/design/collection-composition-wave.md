---
format: aep.planning-md/3
id: design:collection-composition-wave
kind: design
status: draft
title: Production collection composition prerequisite wave
relations:
- designs: story:collection-composition-seam
- serves: vision:language-neutral-code-quality
revision: 7
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

## Runtime dispatch

Runtime unit wt-0b07ece78f7b, branch unit/collection-runtime, base 5ab3854e16f97e5683a1dde9221eda98038bb561; target target/, scratch .scratch/unit. Worker compose_runtime owns collection composition, binding registration/helpers, library exports and tests/collection_composition.rs. Coordinator owns ESS registration/projections, native collection conformance and gate in wt-b26eeec7c8ff, target target/, scratch .scratch/composition (correcting the earlier inferred target location). Worktree registry retains exact absolute paths. No shared targets. Runtime worker uses aep:implementor; native runner is coordinator integration work. Stage implementing, with independent adversary required before merge.

## Conformance dispatch and pinned tool

Conformance unit wt-e44661281e9e, branch unit/collection-conformance, base 5ab3854e16f97e5683a1dde9221eda98038bb561, target target/, scratch .scratch/unit. Worker compose_conformance owns ESS component collection, three authored scenarios, native tests/collection_conformance.rs and regenerated semantic projections. Runtime and harness workers have disjoint files. Coordinator retains src/bin/codegate-check.rs and store writes. Gate expects 4 collection cases, 8 foundation cases and 27 evaluator cases with fresh report identities.

Tool discovery observed global ESS 0.51.0; initial validation warned of the version mismatch and remains only discovery evidence. Running toolchain which inside ess-semantic confirmed cached pinned ESS 0.50.0. All actual generation/tests/gate use that cached binary first in PATH; no specification pin upgrade is selected.

## Integrated treatment and verification

Runtime commit 0d3504c and conformance commit 403ec68 are integrated at dc0f2a084eebce244cb6ad2d720f75b8a87539e2. Native worker observed 4/4 real collection ESS cases on three fresh executions and seven named seeded-fault failures. Runtime worker package summaries observed 81 -> 92 passed, zero failures in the final run; formatter and all-target root Clippy exited 0. New public API was absent on the baseline (compiler red), and a reachable cross-slot coverage test additionally went red before ownership checks were added. The first package run failed root-export boundary handling; its correction preserves all original assertions and rejects calls via direct/renamed aliases in shared algorithms.

The named acceptance condition is: the actual generated Collect handler returns source/configuration identities fixed independently from the selected Rust/Go/Java bytes, fifteen honest coverage records, preserved Semantic mode and canonical failed capture, all admitted without fabricated tools or facts. Baseline 2132b5b has no Collect implementation; treatment native responses satisfy all three authored cases. Native suite digest sha256:0d4eb161d607ab3062d0a2607b84d3362856c5570af800fc7eea32701d274de2, model c64e8e2f5e1a2e791ed83c172875fb04a67c095ec27f188220b1d5644654c126. Integration must repeat against final merged bytes before closure.

First adversary runs against dc0f2a0 in the now-idle runtime tree wt-0b07ece78f7b, retaining its isolated target and scratch .scratch/adversary. Runtime worker lease ended before the tree advanced by fast-forward. Agent compose_adversary is test-only and independent of the implementation workers; coordinator owns every store and integration edit. Three-worker ceiling is preserved. Full integration task check runs in coordinator tree, with 30,580,297,728 bytes free and 24,937 MiB available memory observed immediately before launch. Pinned ESS, two Cargo jobs and sccache remain selected.

Per-agent token/tool/duration counters are not exposed by this host; unavailable rather than estimated. Raw commands, outputs and native reports remain in managed scratch until durable evidence extraction and exact cleanup.

## Verification evidence qualification

The initial verification entry compares the absent baseline API with the worker's executed native treatment; its dc0f2a0 reference names the integration destination. It is not evidence that the full integrated gate had already completed when recorded. The final slot-ownership correction was package-tested separately. Exact integrated native reports must be observed and recorded after the currently running gate before story closure; no complete integration verdict is asserted yet.

## Dispatch procedure deviation

This wave sent each bounded brief in the collaboration tool message rather than passing a prewritten brief file as the wave procedure asks. Role procedures, scopes, base commits, managed ids, target/scratch triples and ownership changes are recorded above, and the worker reports preserve the executed commands. The next wave must write its briefs before dispatch and pass those paths. No different role, authority or test threshold is inferred from this deviation.

## Final integration and wave closure

Final task check at e53a4d4 exited 0 after integrating all four adversarial tests. Native evaluator 27/27, foundation 8/8, collection 4/4 and root Rust 97/97 passed. Final suites, reports, runs and per-step exit outputs are retained under verification/collection-composition/final/. Review-result:collection-composition-adversary records the one bounded independent pass verbatim, including its author-normalized public log paths and empty findings block. There are no open adversary findings and no second attack was needed. Original private logs remain retained for archive cleanup.

Confirmed implementation scope includes composition/runtime helpers, public exports, the narrow boundary regression, exact ESS fixture/component/scenario registration, generated semantic projections, native harness and gate, CI evidence upload paths, and current runtime-limit documentation. The scoper's inferred absence of documentation changes was corrected from actual stale text; collection module registration and regression tests were also confirmed. No shared analysis or admission policy was relaxed.

This completes story:collection-composition-seam, not epic:source-collection-baseline or the repository-report goal. Replan next from the three language-source stories against this now-frozen API. PR integration is authorized; publish the reviewed candidate, merge required green checks, archive private evidence and clean exact managed trees. No new release is included. Cost counters remain unavailable in this host.
