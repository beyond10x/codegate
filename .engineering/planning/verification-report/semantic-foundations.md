---
format: aep.planning-md/3
id: verification-report:semantic-foundations
kind: verification-report
status: draft
title: Semantic foundation checks and blocked runtime verification
relations:
- verifies: story:parity-foundations
- verifies: executable-system-specification:semantic-analysis
revision: 1
---
## Result

Specification/planning foundations implemented; full integration verification and
runtime parity are not complete. `resource-blocker:semantic-build-capacity` prevents
the required large build. No semantic feature or overall parity claim is made.

## Executed checks

All commands used ESS 0.50.0 from its cached toolchain and Rust 1.98.1. The global
ESS default is 0.51.0, so use the pinned binary/PATH for subsequent gate execution.

| Check | Observed output/result | Exit |
|---|---|---|
| ESS validate legacy | codegate v1 — 2 file(s), 26 scenario(s), valid | 0 |
| ESS validate semantic | codegate_semantic v1 — 2 file(s), valid | 0 |
| ESS compile both specifications | JSON IR emitted | 0 |
| Legacy behavior/wire regeneration + pinned cargo fmt + diff | No differences from generated/behavior and generated/wire | 0 |
| Semantic behavior generation | 84 capabilities: 78 generated, 6 obligation(s), 0 refused | 0 |
| Semantic wire generation | 72 model type(s); types-report retained | 0 |
| Semantic regeneration + pinned cargo fmt + diff | No differences from generated/semantic-behavior and generated/semantic-wire | 0 |
| Legacy conformance synthesis | 27 selected scenario(s), 26 authored source(s), 0 refusal occurrence(s) | 0 |
| Semantic conformance synthesis | 6 selected scenario(s), 0 authored source(s), 0 refusal occurrence(s) | 0 |
| cargo check --locked --bins | Root library/binaries including changed codegate-check compiled | 0 |
| cargo check --offline --manifest-path generated/semantic-behavior/Cargo.toml | New behavior crate compiled | 0 |
| cargo check --offline --manifest-path generated/semantic-wire/Cargo.toml | New wire crate compiled | 0 |
| cargo clippy --locked --bins -- -D warnings | Root binary lint passed | 0 |
| cargo fmt --package codegate-cli --check; generated formatting; git diff --check | No formatting/whitespace defect | 0 |

The small metadata-only compile outputs total about 130 MiB; no large conformance/dev
build was started. `task check` was NOT executed. No new Rust tests, actual legacy
conformance execution or semantic runtime conformance were run. Synthesis is not execution.
Retained suite inputs: `verification/semantic-foundations/{legacy-suite,structural-suite}.json`.
There is deliberately no fabricated conformance report beside the unexecuted semantic suite.
The old ESS source, old generated contracts, Cargo manifest/lock, runtime evaluator and CLI
are unchanged; this establishes source compatibility preservation, not new runtime testing.

## Reference and review evidence

Pinned reference archive commit: 4d1515ea925a0d4018ca59da2de3c31a91187f4e.
SHA256: 81add809411fbaa27671bec42111879972c505f61464a64b916c019c011c2ae4.
`docs/parity-baseline.md` enumerates 38 metrics, 38 findings, 9 violations, four gates,
noncatalogue capabilities, intentional differences, exclusions and requested extensions.
Every applicable runtime capability is still a delivery gap unless the legacy imported
fact evaluator explicitly covers it. No pinned-reference execution comparison was run.

Four critic lenses completed two rounds, retained as parity-*-round-1/2 review-results.
Round 1: acceptance 2, design 1, scope 0, parallel safety 0 findings.
Outcomes: all 3 fixed, 0 no-op, 0 escalated. Round 2: four approve, 0 open findings.
The final three-agent concurrency cap required batching the fourth lens. The fourth
critic had inherited conversation context; treat the panel as four separate perspective
reviews, not a claim that all four had fully blinded inputs. No unavailable model/token/
cost numbers are invented; the harness did not report per-agent usage counters.

## Resume and authority

Recheck at least 20 GiB available (21,474,836,480 bytes) or use an operator-authorized
alternate build filesystem. Keep two Cargo jobs and sccache. From the retained managed
tree, run `PATH=<cached ESS 0.50.0 directory>:$PATH RUSTC_WRAPPER=/usr/bin/sccache task check`.
Fix any failure without weakening counts/expectations. Record native real conformance
and gate evidence before closing parity-foundations. Then re-evaluate AEP dependencies
and select the source-snapshot/capability-admission wave; do not dispatch from stale scopes.

The integration checkout is the handoff for the next implementation session. No bot
commit, merge, GitHub push, tag or release was attempted. Unpublished wanted source is
protected by the worktree archive route; the primary checkout remains unchanged.
