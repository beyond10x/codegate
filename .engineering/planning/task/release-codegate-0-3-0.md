---
format: aep.planning-md/3
id: task:release-codegate-0-3-0
kind: task
status: implemented
title: Release the public collection API as Codegate 0.3.0
relations:
- depends_on: story:collection-composition-seam
- serves: vision:language-neutral-code-quality
revision: 5
transitions:
- {from: "draft", to: "proposed", at: "2026-10-03T11:15:29Z", actor: "human:timo", revision: 2, executor: "agent:codegate-release"}
- {from: "proposed", to: "active", at: "2026-10-03T11:15:29Z", actor: "human:timo", revision: 3, executor: "agent:codegate-release"}
- {from: "active", to: "implemented", at: "2026-10-03T11:30:15Z", actor: "human:timo", revision: 5, decided_on: {"recorded":{"test_result":1,"verification":1,"ess_conformance_coverage_v1":3}}, executor: "agent:codegate-release"}
---
## Intent and authorization

The operator requested a new release after the collection composition merge. Release 0.3.0 delivers the additive public Rust Collect API from remote main d09d5c7d2ef50ad2fd5ac20656212a3fcdfb5995. Earlier tags stay immutable. All GitHub writes use the bot. Connectors has no configured GitHub adapter; Gates is the declared fallback.

## Scope

Root Cargo.toml/Cargo.lock version metadata, CHANGELOG.md, website/index.html installation links, and governed release evidence. No product behavior or ESS semantics change. The CLI remains the offline evaluator; language declaration extraction, richer CLI commands and repository HTML reports remain unfinished. This is a single release task, not a decomposition requiring four critics.

## Acceptance

Merge preparation by bot PR after task check and required CI checks. The gate must preserve 27 evaluator, eight foundation and four collection native ESS cases plus all 97 root Rust tests. Tag exact refreshed remote main with annotated 0.3.0 matching Cargo metadata; publish signed common Gates evidence. Require successful tag checks and Linux release build. Download CI artifacts, verify checksums, scan the executable before execution/publication, smoke-test version and evaluator help, and publish only those verified bytes. Independently download the public assets and verify checksum/byte equality, release author and exact tag. Keep primary clean; archive private evidence and retire the exact managed worktree. Documentation publication is asynchronous and not part of this source-release completion boundary.

## Verified publication

Codegate 0.3.0 is published at https://github.com/beyond10x/codegate/releases/tag/0.3.0 (release 402480070, published 2026-10-03T11:28:54Z, draft false). Release author and both asset uploaders are b10x-bot[bot]. Preparation PR7 merged through app/b10x-bot. Annotated tag object 3c144abea1d7db42a3316c1f7307d350b2dae615 points to eabd935285e8adcff3712e59d8dc20ecc09f4737, verified as exact remote main at tagging. The merged tree equals checked preparation candidate fb4c7cf3870c956c374e6c7bbfd9846c315c9021. GitHub's merge committer is covered by the verified App merge action and bot-only branch authority.

## Verification evidence

Local task check passed with pinned Rust 1.98.1 and ESS 0.50.0, all 97 root Rust tests, generated contract drift, formatting, Clippy and documentation checks. Tag gate 37119585981, security 37119586353 and Linux artifact build 37119585858 all succeeded. Main documentation validation 37119474873 succeeded; signed tag evidence was published. Native tag reports, exact suites and run results are retained under verification/releases/0.3.0/{evaluator,foundation,collection}/. They identify version 0.3.0 and respectively 27, eight and four passing scenarios, zero failed/error/skipped/unsupported. These selected components do not establish full semantic parity or language extraction.

The downloaded CI archive passed SHA256SUMS; its ELF passed the private artifact scan before execution or upload (ELF SHA256 fd4d47f7aaa31b6b8fbb863c1e6a79d84fd5461c1cf054758ec0728105b15750). The executable printed codegate 0.3.0 and evaluate help. Both publicly downloaded assets match the CI bytes, and the independent checksum check passed. Archive SHA256: 82fbf70ca27e536de1e5eb4ac3a9efd885dcbb6b367d6bbcc610b2aa0ab34af1. Exact checksum text is verification/releases/0.3.0/SHA256SUMS.

## Resolved delivery interruptions

The initial Gates scan encountered a transient storage quota error. This task preserved native evidence and removed only its own reproducible Cargo targets; the unchanged scan later passed and signed the receipt. No other task's data was removed. A publish invocation with a tag in --remote-ref was refused because that option takes only branches; the documented Gates bot tag push and publish --tag route completed. Upload arguments containing absolute personal paths were refused; the unchanged scanned artifacts were supplied by relative paths to the same bot wrapper and uploaded successfully. No rule, scanner or approval was bypassed.

## Completion boundary

This evidence-only follow-up is integrated through a bot PR without moving the immutable release tag or changing product bytes. The primary checkout stays clean. Private logs and local native evidence are retained through the release worktree recovery archive before its exact-id cleanup. Documentation build and producer checks passed, but live documentation provenance was not inspected; documentation publication is not claimed. Source release completion is independent of Atlas and Website adoption.
