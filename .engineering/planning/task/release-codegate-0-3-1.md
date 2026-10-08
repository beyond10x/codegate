---
format: aep.planning-md/3
id: task:release-codegate-0-3-1
kind: task
status: active
title: Release the ESS 0.56.0 regeneration as Codegate 0.3.1
relations:
- serves: vision:language-neutral-code-quality
revision: 4
transitions:
- {from: "draft", to: "proposed", at: "2026-10-08T17:41:28Z", actor: "human:timo", revision: 2, executor: "agent:codegate-release"}
- {from: "proposed", to: "active", at: "2026-10-08T17:41:28Z", actor: "human:timo", revision: 3, executor: "agent:codegate-release"}
---
## Intent and authorization

The operator requested that the generator move from ESS 0.50.0 to the newest release, ESS 0.56.0, and that the next patch release carry it together with the planning and verification records merged after 0.3.0. Release 0.3.1 is cut from the merged preparation commit on remote main. Earlier tags stay immutable. All GitHub writes use the bot.

## Scope

The `requires` line of both ESS input documents, byte-for-byte regeneration of `generated/behavior`, `generated/wire`, `generated/semantic-behavior` and `generated/semantic-wire` with ESS 0.56.0, the gate's ESS version check, the test-only `ess-conformance` and `ess-primitives` git pins (moved to the ESS 0.56.0 tag commit 84ee8d38eb69e3a4507de801fdcb248232ed6e6e, because the 0.50.0 runner refuses the 0.56.0 suite field `scenario_initial_state`), the CI tool pin and checksum, the root Cargo.toml/Cargo.lock version, CHANGELOG.md, website/index.html installation links and contributor text, and governed release evidence. ESS 0.56.0 validated both specifications unchanged and refused nothing; generation changed obligation doc comments and split exact JSON numbers into a default `exact-numbers` feature. No product behavior or ESS semantics change. This is a single release task, not a decomposition requiring four critics.

## Acceptance

Merge preparation by bot PR after task check and required CI checks. The gate must preserve 27 evaluator, eight foundation and four collection native ESS cases plus all root Rust tests, and generated drift must be zero against ESS 0.56.0. Tag exact refreshed remote main with annotated 0.3.1 matching Cargo metadata; publish signed common Gates evidence. Require successful tag checks and Linux release build. Download CI artifacts, verify checksums, smoke-test version and evaluator help, and publish only those verified bytes. Independently download the public assets and verify checksum/byte equality, release author and exact tag. Keep primary clean and retire the exact managed worktree. Documentation publication is asynchronous and not part of this source-release completion boundary.
