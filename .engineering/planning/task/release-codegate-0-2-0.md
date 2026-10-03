---
format: aep.planning-md/3
id: task:release-codegate-0-2-0
kind: task
status: archived
title: Release the source collection foundation as Codegate 0.2.0
relations:
- serves: vision:language-neutral-code-quality
- delivers: epic:source-collection-foundation
revision: 6
transitions:
- {from: "draft", to: "proposed", at: "2026-10-03T09:53:17Z", actor: "human:timo", revision: 2, executor: "agent:codegate-release"}
- {from: "proposed", to: "active", at: "2026-10-03T09:53:17Z", actor: "human:timo", revision: 3, executor: "agent:codegate-release"}
- {from: "active", to: "archived", at: "2026-10-03T10:10:35Z", actor: "human:timo", revision: 6, decided_on: {"recorded":{"verification":1}}, executor: "agent:codegate-release"}
---
## Authority and version

The operator requested a release after PR2 delivered the source collection foundation. Release0.2.0 includes those additive Rust library capabilities and reviewed backlog. Preserve the /0.1 evaluator contract and distinguish unimplemented language declaration extraction/public semantic commands from delivered source collection/admission. Initial remote main is f367f9ad622e59a33845a5b22e0681da6b859254, clean; existing tag0.1.0 remains immutable.

## Delivery

Bump only root Cargo package/lock metadata to0.2.0. Merge the release preparation through a bot PR with required checks. The complete repository gate must run on that exact candidate in CI; this invokes the same codegate-check as task check and executes27 evaluator plus8 foundation native ESS scenarios, Rust tests, drift/fmt/clippy/docs checks. Local disk is below the20GiB preflight floor, so do not start a large local build or delete another owner's data. This release introduces no new domain noun or behavior and needs no ESS change.

Fetch current remote main after merge, verify exact candidate tree and required checks, and create a bare annotated0.2.0 tag matching Cargo metadata through the bot. Sign/publish common Gates evidence for the exact tag. Require successful tag gate/security/release artifact jobs, download the Linux archive and SHA256SUMS, verify checksums and CLI --version, scan artifact, then create/upload/publish the bot-owned GitHub Release. Verify the public release, annotated tag target and downloaded checksums before reporting released.

## Completion

One release task, not an implementation decomposition, so no four-critic panel is needed. Completion requires the exact remote-main tag, required checks, published bot-owned GitHub Release and verified Linux archive/checksums. Documentation observation is asynchronous and does not expand this source release into Atlas or Website work. Primary main stays clean; retire this task's managed tree with archive recovery for private evidence. Record evidence through AEP, never hand-edit its store.

Release preparation also updates website/index.html install URLs to the correct bare 0.2.0 tag and distinguishes the delivered Rust collection/admission library from the existing offline evaluator CLI. The unchanged Rust docs builder produces source provenance in CI; remote docs publication remains asynchronous.

## Artifact refusal and supersession

Version preparation PR3 merged to ed8efbae0b8f450affd77863c3c2f92f99fa8b20. Annotated0.2.0 tag object e220bc920ce7f8456d827667c2a91e430752903a points there. Tag jobs37115244932 (security),37115244491 (gate) and37115244553 (Linux build) succeeded; downloaded SHA256SUMS matched. Publication scan of the extracted ELF refused39 personal-paths matches; exact public refusal: artifact breaks39 private rule(s), with details retained privately. No artifact was uploaded and draft release402449475 was never published.

The correction remaps build-time filesystem paths through Rust compiler flags and is released as0.2.1 from a new verified remote-main commit. The existing0.2.0 tag remains immutable. This task is archived as superseded, not implemented. User authorization remains to deliver a verified source release; no scanner bypass or tag replacement is authorized.
