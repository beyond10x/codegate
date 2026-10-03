---
format: aep.planning-md/3
id: task:release-codegate-0-3-0
kind: task
status: active
title: Release the public collection API as Codegate 0.3.0
relations:
- depends_on: story:collection-composition-seam
- serves: vision:language-neutral-code-quality
revision: 3
transitions:
- {from: "draft", to: "proposed", at: "2026-10-03T11:15:29Z", actor: "human:timo", revision: 2, executor: "agent:codegate-release"}
- {from: "proposed", to: "active", at: "2026-10-03T11:15:29Z", actor: "human:timo", revision: 3, executor: "agent:codegate-release"}
---
## Intent and authorization

The operator requested a new release after the collection composition merge. Release 0.3.0 delivers the additive public Rust Collect API from remote main d09d5c7d2ef50ad2fd5ac20656212a3fcdfb5995. Earlier tags stay immutable. All GitHub writes use the bot. Connectors has no configured GitHub adapter; Gates is the declared fallback.

## Scope

Root Cargo.toml/Cargo.lock version metadata, CHANGELOG.md, website/index.html installation links, and governed release evidence. No product behavior or ESS semantics change. The CLI remains the offline evaluator; language declaration extraction, richer CLI commands and repository HTML reports remain unfinished. This is a single release task, not a decomposition requiring four critics.

## Acceptance

Merge preparation by bot PR after task check and required CI checks. The gate must preserve 27 evaluator, eight foundation and four collection native ESS cases plus all 97 root Rust tests. Tag exact refreshed remote main with annotated 0.3.0 matching Cargo metadata; publish signed common Gates evidence. Require successful tag checks and Linux release build. Download CI artifacts, verify checksums, scan the executable before execution/publication, smoke-test version and evaluator help, and publish only those verified bytes. Independently download the public assets and verify checksum/byte equality, release author and exact tag. Keep primary clean; archive private evidence and retire the exact managed worktree. Documentation publication is asynchronous and not part of this source-release completion boundary.
