---
format: aep.planning-md/3
id: task:release-codegate-0-2-1
kind: task
status: active
title: Release Codegate 0.2.1 with reproducible artifact paths
relations:
- supersedes: task:release-codegate-0-2-0
- serves: vision:language-neutral-code-quality
- delivers: epic:source-collection-foundation
revision: 3
transitions:
- {from: "draft", to: "proposed", at: "2026-10-03T10:10:35Z", actor: "human:timo", revision: 2, executor: "agent:codegate-release"}
- {from: "proposed", to: "active", at: "2026-10-03T10:10:35Z", actor: "human:timo", revision: 3, executor: "agent:codegate-release"}
---
## Authority and correction

Deliver the operator-requested release as0.2.1 after the0.2.0 artifact scan found embedded CI filesystem paths. Preserve the already-pushed0.2.0 tag and do not publish its draft. Use Rust --remap-path-prefix for the CI home and checkout in the existing release build, then scan the resulting ELF before execution or upload. This corrects product bytes rather than evading the scan.

## Scope

Root Cargo.toml/Cargo.lock version metadata, website/index.html versioned install links, .github/workflows/release-build.yml compiler path remapping, and governed release evidence. No analysis semantics or ESS values change. The existing full remote gate executes27 evaluator plus8 foundation native scenarios and all Rust tests/checks. Local large builds remain prohibited below the20GiB floor; use verified CI output.

## Acceptance

Merge preparation through a bot PR, tag exact remote main with an annotated0.2.1 matching Cargo metadata, publish signed common Gates evidence, verify all required tag checks and Linux artifact job. Download/archive checksum, scan artifact successfully, require CLI --version0.2.1, upload only these verified assets to the bot-owned GitHub Release, then independently download and reverify published assets. Close with a published release and clean primary/managed-worktree cleanup. Documentation publication remains asynchronous. A single release correction is not a decomposition requiring four critics.
