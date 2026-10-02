---
format: aep.planning-md/3
id: design:first-wave-offline-dependency
kind: design
status: draft
title: 'Wave 1: offline dependency facts and shared checks'
summary: Stage-1 proposal for one scoped implementation unit, awaiting operator approval.
relations:
- designs: story:offline-dependency-slice
- serves: vision:language-neutral-code-quality
revision: 4
---
## Stage and authority

**Skill version 0.19.1** — aep:implementing.

Interactive stage 2 authorized by approval-record:standing-wave-approval. The
operator explicitly approved all upcoming waves on 2026-10-02 and allows up to
three implementation workers; this first wave selects one. The stage-1 proposal
and pre-flight evidence below are retained as the reviewed selection.

The coordinator uses aep:planning for legal draft -> proposed -> active moves.
No literal accepted story status is invented. No push, tag or release is authorized.

## Proposed unit

- Story: `story:offline-dependency-slice`.
- Objective served: `vision:language-neutral-code-quality`.
- Parent: `epic:offline-dependency-evaluation`.
- Result: real offline JSON evaluation of admitted language-neutral dependency facts,
  with shared unique fan-out and forbidden-edge checks, generated contracts, real
  ESS conformance and a Rust gate.
- Scope: ten cited write surfaces returned by aep:story-scoper. Proposed ownership
  is cited; runtime implementation does not exist yet.
- N = 1 despite the authorized cap of 3. The root package, generated interfaces,
  evaluator, CLI and gate form one end-to-end unit with shared Cargo/module wiring.
  Real bindings depend on that contract and stay outside this first wave.

## Computed selection

Selection used `aep plan artifact waves --kind story --status draft --format json`,
not manual pairwise substitution. Its complete output follows verbatim:

```json
{
  "waves": [
    {
      "wave": 1,
      "artifacts": [
        {
          "id": "story:offline-dependency-slice",
          "inferred": false,
          "scope": [
            {
              "confidence": "cited",
              "path": ".gitignore"
            },
            {
              "confidence": "cited",
              "path": "AGENTS.md"
            },
            {
              "confidence": "cited",
              "path": "Cargo.lock"
            },
            {
              "confidence": "cited",
              "path": "Cargo.toml"
            },
            {
              "confidence": "cited",
              "path": "README.md"
            },
            {
              "confidence": "cited",
              "path": "Taskfile.yml"
            },
            {
              "confidence": "cited",
              "path": "generated/"
            },
            {
              "confidence": "cited",
              "path": "rust-toolchain.toml"
            },
            {
              "confidence": "cited",
              "path": "src/"
            },
            {
              "confidence": "cited",
              "path": "tests/"
            }
          ]
        }
      ]
    }
  ],
  "collisions": [],
  "unassessed": [],
  "cycles": []
}
```

`aep plan artifact blocked` returned `nothing is blocked`.
The dependency graph has one decomposes edge to the first epic. The four-critic
decomposition panel is skipped under planning section 7 because fewer than two
children were decomposed; this is not a claim of independent plan approval.
A separate read-only aep:story-scoper pass supplied the scope recorded on the story.

## Specification evidence

ESS 0.50.0 is the chosen first-slice toolchain; no upgrade is required by this
contract. The installed CLI validates ess/17 and direct-return authored scenarios.

```text
codegate v1 — 2 file(s), 26 scenario(s), valid
26 authored scenario(s) from 26 file(s), 0 refusal(s), suite ess-conformance/28, written to .scratch/authored-suite.json
27 scenario(s) (26 authored), 0 refusal(s), written to .scratch/combined-suite.json
18 capabilities: 17 generated, 1 obligation(s), 0 refused
7 artifact(s), written to .scratch/synth-probe
```

The wire type projection generates 16 model types. Its integer-integrality runtime
obligation is explicit in the story. The generated behavior contract has one
unimplemented Evaluate obligation. No test execution against Codegate is claimed.

Model digest at this proposal:
`e27ccc45957cf4fe2ef83d9362e81d867eb46bff9a72f1fd28e43b69bf53ec93`.

Contract digest:
`df71e33e5abc7d133716034004ba68a3a08d1c620351660f3940aa973a3c93b8`.

Both generated contracts and ESS's exact 0.50.0 conformance dependencies compiled in
a temporary no-behavior build probe. That checks feasibility and cost, not semantics.

## Pre-flight measurements and limits

Observed during planning on 2026-10-02:

| Item | Observation / plan |
| --- | --- |
| Base | Local main at 60705dac3f6ddd61a8ad3fca4b0192234f2be961, clean |
| Remote | None configured; all work remains local |
| Old wave worktrees/build directories | None for Codegate; only this current planning checkout |
| AEP | 0.68.0; store aep.project/5; exact protocol pin retained |
| Rust / Cargo | 1.98.1 / 1.98.1 |
| Compiler cache | /usr/bin/sccache available; explicitly set RUSTC_WRAPPER for builds |
| Build parallelism | CARGO_BUILD_JOBS=2; private target directory per tree |
| Generated wire cargo check | Exit 0; 9.439 seconds; 58 MiB target |
| Generated contracts + clap + pinned ESS Runner dependency test build | Exit 0; 82.046 seconds; 469 MiB target |
| Build measurement method | cargo test --no-run on a temporary empty probe depending on both generated crates and ESS 0.50.0; no Codegate implementation, scenarios or behavioral tests executed |
| Available memory at initial observation | Approximately 40 GiB; recheck before launch |
| Free disk | Initially 43 GiB, then 27 GiB; latest exact observation before cleanup 27,008,024,576 bytes |
| Free-space floor | 20 GiB; recheck immediately before dispatch and each returning unit |
| Per-tree build planning allowance | 2 GiB (estimate, not measured runtime peak); one unit plus coordinator integration targets must be separate |
| Model budget | Operator allows up to three implementation workers; proposed one implementation worker, then one adversary |
| Monetary/token budget | Not supplied; no spending amount asserted |

The measured dependency target is not a guarantee of implementation peak storage.
Disk changed substantially during planning. If the floor is not met at approval,
report the number and do not launch; do not delete unrelated workspace data.

After approval, first commit this planning/specification input on the integration
branch and run cheap existing checks (AEP validation, ESS validation and authored/
combined suite compilation) before creating the unit tree. The absence of a root
implementation gate at baseline is explicit; the unit adds it. The whole real gate
runs on integration after the reviewed unit merges.

## Branch and resource ledger

Paths use portable home-relative notation; resolve them explicitly before launch.
These are manager paths already observed or proposed in the manager's established
layout, never authorization to create/remove a tree manually.

| Role | Managed id | Branch | Current head | Stage |
| --- | --- | --- | --- | --- |
| Coordinator | codegate-wave1-plan-20261002 | plan/first-wave (planned stage-2 name wave/001-offline-dependency) | 60705dac3f6ddd61a8ad3fca4b0192234f2be961 plus uncommitted planning/specification | awaiting operator approval |
| Unit | codegate-w1-offline-dependency-20261002 (reserved; not created) | wave/001/offline-dependency | not created | not dispatched |

Coordinator tree:
`$HOME/.local/state/worktree/trees/b10x/codegate/codegate-wave1-plan-20261002`.

Coordinator build:
`$HOME/.local/state/worktree/trees/b10x/codegate/codegate-wave1-plan-20261002/target`.

Coordinator scratch:
`$HOME/.local/state/worktree/trees/b10x/codegate/codegate-wave1-plan-20261002/.scratch/wave-001`.

Proposed unit tree:
`$HOME/.local/state/worktree/trees/b10x/codegate/codegate-w1-offline-dependency-20261002`.

Proposed unit build:
`$HOME/.local/state/worktree/trees/b10x/codegate/codegate-w1-offline-dependency-20261002/target`.

Assigned unit scratch:
`$HOME/.local/state/worktree/trees/b10x/codegate/codegate-w1-offline-dependency-20261002/.scratch/wave-001/offline-dependency`.

Coordinator session: `codex-codegate-wave1-plan-20261002`. Worker session ids and
actual manager-returned paths must be recorded before use. No published branch.

Temporary planning probes live only under coordinator `.scratch/`: contract-probe,
synth-probe and build-cost-probe. The measured targets contract-build and evaluator-build
are disposable after their processes exit and their timing/size evidence is recorded.
Retain suite JSON, generation accounting and small logs for the next coordinator.
Archive the draft tree as local recovery proof; retain the tree for approval/resume.

## Dispatch and review

Planned role sequence: `aep:implementor` then `aep:adversary`, using their installed
reference procedures and a unit brief file written before dispatch. The coordinator
owns every planning-store write. The implementation/adversary workers get their own
lease on the assigned unit tree and release only their lease when handing back.

Host adaptation: this Codex session exposes generic `collaboration.spawn_agent`,
not a native `subagent_type` selector. The role charters will be supplied explicitly
to generic workers; that is a declared deviation from native plugin-agent dispatch.
Use the session model; no ESS-evolution model override applies to Codegate work.

Record red and green commands, executed scenario counts, exact commits and independent
adversary findings. At most two attacks, with the same correction/claim-verification
rules from aep:implementing. No false-green skipped scenarios or mocked expected responses.
Costs unavailable from the host must be marked unavailable, never estimated as usage.

## Approval boundary

Approval authorizes: one planning/specification opening commit, one unit implementation
commit, the unit merge into integration, one closing evidence/store commit, and the
integration merge into local main after the real gate passes. All commits use the
organization bot through the applicable workspace delivery route. It does not authorize
GitHub repository creation, push, tag, release or a later wave.

The opening commit includes the inputs prepared in this stage-1 checkout, so the unit
branches from an exact ESS contract and selected suite. There is no unstated permission
to alter those expectations merely to pass.

## Completion and recovery

After the real whole gate passes, use planning to record evidence and legal story
moves and integrate into local main. Preserve logs and evidence before cleanup.
Because no remote publication is authorized, use the managed worktree archive path
for local recovery rather than requiring a push for cleanup. Verify exact-id cleanup
or give an explicit retained-tree handoff. The initial source repository stays local.

## Deliberate exclusions

Real Rust and Go bindings, source collection, stable general IR, multi-producer merge,
content digest calculation, architecture cycles, richer metrics, baselines, waivers,
security signing, GitHub operations, release and the next wave. A fixture labelled
Go proves that the checker ignores language labels; it does not prove Go extraction.

## Next action

The coordinator records the opening commit and cheap validation, then dispatches
aep:implementor against the assigned managed unit tree and follows with aep:adversary.
Standing approval removes the wave approval stop; release still requires separate authority.

## Stage-1 handoff

The two temporary build processes were observed exiting 0. Their exact disposable
build directories `.scratch/contract-build` and `.scratch/evaluator-build` were
removed after retaining build logs, timings, generation outputs and suite JSON.
No other repository's files or caches were removed.

The planning checkout remains intentionally uncommitted on `plan/first-wave`, with
base main clean at 60705da. This is the stage-1 draft, not a leftover implementation
wave. The next coordinator reuses it and, after approval, creates the opening commit
before dispatch. No artifact has been moved out of draft.

The recovery archive is assigned to
`$HOME/.local/state/worktree/archives/codegate/codegate-wave1-plan-20261002` and is
written after the final planning mutation. Next owner: the coordinator resuming the
operator's approval. No worker worktree or build directory has been created.
