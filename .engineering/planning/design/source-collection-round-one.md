---
format: aep.planning-md/3
id: design:source-collection-round-one
kind: design
status: draft
title: First source collection round and parallel wave record
relations:
- designs: epic:source-collection-foundation
- designs: epic:source-collection-baseline
revision: 9
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

## Gate prerequisite dispatch

Unit task:clean-external-target-gate, managed tree wt-bdb368b757ea, base 91dc33be346f6b3b5984743456833f1a875f7c61, target /tmp/codegate-gate-worker.oisTVz, scratch .scratch/unit, private brief .scratch/unit/brief.md. Implementor role runs through collaboration agent scope_snapshot. Stage implementing. Coordinator will commit the unit after independent attack; no worker Git/store writes. Full absolute triples are in the private brief and registry. The whole-round source selection and three-worker limit remain unchanged.

## Foundation conformance seam

The conformance review identified that the six public commands do not type the intermediate helpers: Collect returns FactSnapshot and Evaluate returns Assessment. Before implementation dispatch, extend ESS with internal CollectSource and ValidateSnapshot operations, using existing source/fact/gap value types, and a source-foundation component. Native ESS targets call actual helpers. Six foundation behavioral cases execute in that component; parity-core-boundary is a separate architectural Rust test with injected violations, not a fabricated semantic command outcome. The six public runtime obligations and specification lifecycle remain incomplete. Coordinator owns the ESS/generated integration; implementation workers own their previously scoped runtime files.

## Foundation prerequisite workers

The full clean-checkout gate passed at integration 69095e2, with 27 native evaluator cases, and task:clean-external-target-gate is implemented. Independent adversary found nothing in two attacks. Source-foundation contract was integrated at 80ba11b (worker 9526306); two internal obligations are specified, none claimed implemented yet.

Shared setup runs in separate managed trees before runtime workers fork. Identity/dependency worker: wt-441dc9e84c18, base 9526306, target /tmp/codegate-identity-build.AaEcFJ, scratch .scratch/unit. Wire conversion worker: wt-36b37d910096, base 80ba11b, target /tmp/codegate-wire-worker.PQTRHV, scratch .scratch/unit. Code roles are coordinator-owned setup under source-snapshot; disjoint production files. The admission worker performs a read-only invariant review alongside them. Three worker budget observed. Full paths remain in manager/private dispatch records.

## Runtime foundation dispatch

Common base cceca5125ee726b34e44bc28f02aeeadb6274fe9 integrates canonical identities and strict wire conversion. All inherited release items were reconciled from exact tag/check/artifact/provenance evidence and marked implemented, removing their stale scheduling collision. Store result: wave 1 story:capability-admission and story:source-snapshot; 1 wave(s), 0 collision(s), 0 unassessed.

Collection unit: wt-59fefee16e98, target /tmp/codegate-source-worker.AAF0ck, scratch .scratch/unit, worker scope_snapshot. Admission unit: wt-7d9dea940be5, target /tmp/codegate-admission.g37x8M, scratch .scratch/unit, worker scope_admission. Each owns separate source/test paths; only module declarations are reconciled centrally. Both independently observed more than20GiB free before build. Previous gate/identity/wire disposable targets were removed by exact recorded path after stopped agents/processes were verified; source/logs remain retained. The third worker reviews the next language wave's syntax mappings without edits.

The harness exposes no per-agent token/tool/duration totals, so those costs are unavailable rather than estimated. Raw reports retain command counts and case/exit evidence. No semantic runtime completion is claimed at dispatch.

## Authored foundation witness preparation

Six authored ESS scenarios synthesize successfully with the two generated internal cases: 8 selected scenario(s), 6 authored source(s), 0 refusal occurrence(s). Six public outcomes are outside source-foundation. Multi-act fixture timestamps were corrected after ESS refused equal instants; expected behavior was unchanged. Expected source/configuration identities were computed independently with canonical JSON plus sha256sum, then verified against pure helpers by tests/identity_adversary.rs (1 passed, exit0). Runtime collection/admission conformance remains pending their integration. The native target forwards actual helper results and seeds empty-response, admit-everything and constant-identity faults to require observable failures.

## Final bounded adversary trend

Source collection findings fell from2 to1: carried0, new1, resolved2. Admission boundary findings stayed1 to1: carried0, new1, resolved1. Each second-pass finding was returned for correction with its regression retained. No third adversarial campaign is authorized or claimed. Coordinator verifies correction diffs and the same regressions before integration. Exact CLI comparisons follow; the two admission prose-only originals remain archived, with structured replacement records and an explicit correction withdrawing the unmeasured thread-context bypass inference.

{
  "artifact": "story:source-snapshot",
  "reviews": 12,
  "from": "review-result:source-collection-adversary-round-1",
  "from_reviewer": "unattributed",
  "to": "review-result:source-collection-adversary-round-2",
  "to_reviewer": "unattributed",
  "carried": [],
  "new": [
    {
      "file": "src/collection/manifests.rs",
      "line": 209,
      "category": "boundary",
      "severity": "blocker",
      "verdict": "CONFIRMED",
      "origin": "introduced",
      "message": "An unclosed Maven project element reaches EOF without a build-selection gap, so malformed XML can appear fully collected."
    }
  ],
  "resolved": [
    {
      "file": "src/collection/mod.rs",
      "line": 346,
      "category": "acceptance",
      "severity": "blocker",
      "verdict": "CONFIRMED",
      "origin": "introduced",
      "message": "Selecting a source subdirectory omits ancestor .cargo/config.toml, so changing its build configuration leaves the configuration identity unchanged."
    },
    {
      "file": "src/collection/manifests.rs",
      "line": 205,
      "category": "contract-drift",
      "severity": "blocker",
      "verdict": "CONFIRMED",
      "origin": "introduced",
      "message": "Maven CDATA module and relativePath text is silently ignored, leaving the declared parent absent with no build-selection gap."
    }
  ]
}
{
  "artifact": "story:capability-admission",
  "reviews": 12,
  "from": "review-result:admission-boundary-pass-one",
  "from_reviewer": "unattributed",
  "to": "review-result:admission-boundary-pass-two",
  "to_reviewer": "unattributed",
  "carried": [],
  "new": [
    {
      "file": "tests/semantic_boundary.rs",
      "line": 90,
      "category": "acceptance",
      "severity": "blocker",
      "verdict": "CONFIRMED",
      "origin": "introduced",
      "message": "Architectural guard admits nested std::os platform IO."
    }
  ],
  "resolved": [
    {
      "file": "tests/semantic_boundary.rs",
      "line": 90,
      "category": "acceptance",
      "severity": "blocker",
      "verdict": "CONFIRMED",
      "origin": "introduced",
      "message": "Architectural guard admits clap process-argument IO through a normal dependency."
    }
  ]
}

## Foundation wave closure

Implementation5d5192d passed full integration gate: evaluator27/27 native ESS, foundation8/8 native ESS (six authored/two structural), root81/81 Rust tests, all generated drift/fmt/clippy/docs steps exit0. Source-snapshot, capability-admission and their foundation epic moved to implemented on this actual evidence. See verification-report:first-collection-foundation and verification/source-foundation for exact reports and per-step exits. The report's original raw private evidence is preserved in the still-retained integration tree until managed archive cleanup.

Replanning found and repaired a practical acceptance cycle: source-language stories needed real Collect before downstream first-slice. Newly scoped story:collection-composition-seam now precedes all three language workers. Four critics reviewed this new prerequisite separately; round1 acceptance3/design2 findings were fixed, round2 all approve. No extra code-adversary campaign occurred. Next selection begins with composition, then up to3 disjoint Go/Rust/Java workers; no new language implementation is claimed in this foundation wave. Broader structural observations remain story:source-structural-observations.

Publication through the already authorized bot PR route and exact managed cleanup are pending at this record. No new version/tag is selected; the prior0.1.0 release remains verified separately. Per-agent cost counters unavailable from this host.

## Recomputed scheduling after closure

The exact current scheduling output follows. The selected next prerequisite is collection-composition-seam; language workers follow it. Broader structural observations are not selected for this first baseline even if the full backlog permits them alongside first-slice.

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
