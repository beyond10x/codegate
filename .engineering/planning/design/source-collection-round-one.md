---
format: aep.planning-md/3
id: design:source-collection-round-one
kind: design
status: draft
title: First source collection round and parallel wave record
relations:
- designs: epic:source-collection-foundation
- designs: epic:source-collection-baseline
revision: 1
---
## Authority and procedure

Interactive run under aep:implementing/aep:wave skill 0.19.1. Operator requested epics and stories for the first source collection round and explicit parallel agent dispatch; standing-wave approval remains recorded. Operator selected up to three workers. This authorizes the opening plan, bounded unit commits, integration merges and closing evidence commits. Existing PR integration authorization is reused; no new release/tag is included.

## Selection

First repair task:clean-external-target-gate, then the disjoint foundation pair story:source-snapshot and story:capability-admission. Next replan from actual evidence before dispatching the Go/Rust/Java source stories. Integration of source-structure and first-slice follows those workers. All serve vision:language-neutral-code-quality through their parents; explicit serves edges are added on activation. Source-only baseline excludes external language servers and later metrics/scoring.

Scopes are inferred but bounded by the two read-only aep:story-scoper reports. Coordinator owns shared identity, manifest/module setup and conformance registration; workers never write AEP or shared integration files. Public AST/value contracts already exist and validate. Gate defect and boundary weakness are recorded in verification-report:post-release-source-review.

## Roles

Use aep:implementor and aep:adversary procedures through this host's generic collaboration agents; it has no subagent_type selector. The four decomposition critic perspectives ran as three child agents plus one local parallel-safety pass due to the three-child concurrency ceiling. Each verdict and each finding outcome is retained; two reports were returned by their authors with normalized paths to avoid publishing personal filesystem information.

## Preflight

Primary Codegate main was clean at 8ebd4a200c8181b0d48221cab5c1fd25f8d30f45. The only Codegate linked tree is this session's ongoing review/integration tree wt-5a8bf1cbcb49, reused rather than nested. There is no previous unfinished collection wave. The ongoing review build is 129 MiB, under /tmp/codegate-review-build.ZX0bU2, and is not abandoned. Before that build tmpfs had 28,318,007,296 bytes available (above the 20 GiB floor). Memory showed 34 GiB available. Prior complete Codegate gate output measured approximately 808 MiB; three isolated targets plus integration fit the tmpfs budget. Use sccache, two Cargo jobs per worker and no incremental cache. Recheck free space before each large build and do not share targets.

## Integration record

Managed id wt-5a8bf1cbcb49; repository-relative task tree identifier is recorded here rather than a personal absolute path. Session codegate-review-20261003. Integration branch wave/source-collection-round1. Build directory /tmp/codegate-review-build.ZX0bU2; scratch .scratch/review. Source root and exact per-worker absolute triples are retained in private ignored briefs and the worktree registry. Stage: reviewed planning; gate prerequisite active; no implementation dispatched yet.

## Computed plan

The following is the complete waves output at selection time, including later deferred stories and every collision; its grouping is not a claim that all those stories are selected for this round.
wave 1
  story:capability-admission (inferred)
  story:source-snapshot (inferred)
wave 2
  story:source-go-baseline (inferred)
  story:source-java-baseline (inferred)
  story:source-rust-baseline (inferred)
wave 3
  story:source-structure (inferred)
wave 4
  story:first-slice (inferred)
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
collision: story:source-snapshot story:source-structure src/bindings/mod.rs (inferred)
11 wave(s), 17 collision(s), 0 unassessed
