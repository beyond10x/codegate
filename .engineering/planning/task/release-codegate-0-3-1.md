---
format: aep.planning-md/3
id: task:release-codegate-0-3-1
kind: task
status: implemented
title: Release the ESS 0.56.0 regeneration as Codegate 0.3.1
relations:
- serves: vision:language-neutral-code-quality
revision: 6
transitions:
- {from: "draft", to: "proposed", at: "2026-10-08T17:41:28Z", actor: "human:timo", revision: 2, executor: "agent:codegate-release"}
- {from: "proposed", to: "active", at: "2026-10-08T17:41:28Z", actor: "human:timo", revision: 3, executor: "agent:codegate-release"}
- {from: "active", to: "implemented", at: "2026-10-08T17:59:08Z", actor: "human:timo", revision: 6, decided_on: {"asserted":{"test_result":1,"verification":1,"ess_conformance_coverage_v1":3}}, executor: "agent:codegate-release"}
---
## Intent and authorization

The operator requested that the generator move from ESS 0.50.0 to the newest release, ESS 0.56.0, and that the next patch release carry it together with the planning and verification records merged after 0.3.0. Release 0.3.1 is cut from the merged preparation commit on remote main. Earlier tags stay immutable. All GitHub writes use the bot.

## Scope

The `requires` line of both ESS input documents, byte-for-byte regeneration of `generated/behavior`, `generated/wire`, `generated/semantic-behavior` and `generated/semantic-wire` with ESS 0.56.0, the gate's ESS version check, the test-only `ess-conformance` and `ess-primitives` git pins (moved to the ESS 0.56.0 tag commit 84ee8d38eb69e3a4507de801fdcb248232ed6e6e, because the 0.50.0 runner refuses the 0.56.0 suite field `scenario_initial_state`), the CI tool pin and checksum, the root Cargo.toml/Cargo.lock version, CHANGELOG.md, website/index.html installation links and contributor text, and governed release evidence. ESS 0.56.0 validated both specifications unchanged and refused nothing; generation changed obligation doc comments and split exact JSON numbers into a default `exact-numbers` feature. No product behavior or ESS semantics change. This is a single release task, not a decomposition requiring four critics.

## Acceptance

Merge preparation by bot PR after task check and required CI checks. The gate must preserve 27 evaluator, eight foundation and four collection native ESS cases plus all root Rust tests, and generated drift must be zero against ESS 0.56.0. Tag exact refreshed remote main with annotated 0.3.1 matching Cargo metadata; publish signed common Gates evidence. Require successful tag checks and Linux release build. Download CI artifacts, verify checksums, smoke-test version and evaluator help, and publish only those verified bytes. Independently download the public assets and verify checksum/byte equality, release author and exact tag. Keep primary clean and retire the exact managed worktree. Documentation publication is asynchronous and not part of this source-release completion boundary.

## Verified publication

Codegate 0.3.1 is published at https://github.com/beyond10x/codegate/releases/tag/0.3.1 (release 407117482, published 2026-10-08T17:58:44Z, draft false, latest). Release author and both asset uploaders are b10x-bot[bot]. Preparation PR 10 merged through the bot App as 9cc3a6fa7a1695d12fcbdcb552de6df91ecab19f; its tree equals the checked candidate 19f11fd17bf38f4cf13847b8e2d51841b03be28f. The annotated tag 0.3.1, tagged by b10x-bot[bot], points to 9cc3a6fa7a1695d12fcbdcb552de6df91ecab19f, exact remote main at tagging, and matches Cargo metadata 0.3.1.

## Verification evidence

Local task check passed on the candidate with Rust 1.98.1 and ESS 0.56.0: 27 evaluator, eight source-foundation and four collection native cases executed and passed with zero failed, error, unsupported or skipped; 97 root Rust tests ran and equal the listed set; generated drift was zero; formatting, Clippy and documentation checks passed. PR checks Gate, Build documentation and common Security and privacy succeeded. Tag runs Repository checks 37820148762, Shared source gates 37820149617 and Release artifacts 37820148661 succeeded; main runs Repository checks 37820020902, Shared source gates 37820021843, Documentation validation 37820020628 and Documentation site 37820350591 succeeded. Signed common Gates evidence for the tag scanned 52 commits and was published as check run 113458900016.

The downloaded CI archive passed SHA256SUMS; its ELF passed the artifact scan before execution (ELF SHA256 8d57f035adea37e6712c15a2d17ab23cb741579dace09a2dc9f58578639c98d9). The executable printed codegate 0.3.1 and evaluate help. Both publicly downloaded assets are byte-equal to the CI bytes and pass the checksum check. Archive SHA256: 99afebcf6a5e1dc74a990d873e7f157a2ad25667170ce3225a3401a528774fea.

## Completion boundary

The release worktree was finished with its build cache discarded and its remainder archived. Documentation publication is asynchronous and is not claimed here.
