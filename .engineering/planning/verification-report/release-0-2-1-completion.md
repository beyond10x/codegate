---
format: aep.planning-md/3
id: verification-report:release-0-2-1-completion
kind: verification-report
status: draft
title: Verified Codegate 0.2.1 release and corrected artifacts
relations:
- verifies: task:release-codegate-0-2-1
revision: 1
---
## Published result

Codegate0.2.1 is published at https://github.com/beyond10x/codegate/releases/tag/0.2.1 (release402455355, published2026-10-03T10:21:49Z, draftfalse, prereleasefalse, latest). Release author and both asset uploaders are b10x-bot[bot]. Preparation PR3 and correction PR4 merged through the bot. Annotated tag object60b7579db271d115e0d36098c518d9e33e63e52a points to49054af00aa006025811e335c9e9d9c7b52d061b, the exact remote main at tagging. Candidate and merged trees matched18a212e639a9fa2f5e2441cb71259484f7ea5da8; GitHub's merge committer is authorized by the verified App merge and bot-only branch ruleset.

## Checks and native evidence

Tag gate37115902540, common security37115902992 and Linux artifact build37115902520 all succeeded for49054af. Commit check-runs additionally show successful Build documentation and the signed local-evidence check. The full remote codegate-check (the same Rust gate invoked by task check) executed all27 evaluator and8 foundation ESS scenarios, plus81 root Rust tests; exact reports identify codegate-offline0.2.1 and codegate-source-foundation0.2.1. All native counts passed with zero failed/error/skipped/unsupported scenarios. Generated drift, formatting, Clippy and documentation checks passed. Local large builds were not started below the repository20GiB disk floor.

Native reports are retained at verification/releases/0.2.1/{evaluator-report,foundation-report}.json. Their exact suite bytes were compared identical to verification/source-foundation/{evaluator,foundation}/suite.json before recording. Six public semantic commands remain outside foundation conformance; this is not full Go/Rust/Java semantic parity.

## Artifact verification

Downloaded CI archive and SHA256SUMS matched. The extracted CLI passed the unchanged artifact scanner before execution/upload:

artifact valid; format=elf64-little-endian; sha256=d3a8154fa63a0c9b898a2bac48ca147b24439e83ffb427e81b14a944fcad3257; input_bytes=1890208; privacy_bytes=1508901; extracted_bytes=1086598; sections=34; scanner_invocations=1

The CLI printed codegate0.2.1 (with the normal separating space) and evaluate help. GNU/Linux archive SHA256:54f012847a9fe7199269538128461d0a3bb9f0666c310a9654261a14c319ecc7. After publication, both public assets were independently downloaded; sha256sum --check passed and byte comparisons matched both original CI assets. Exact checksum text is verification/releases/0.2.1/SHA256SUMS.

## Refusals and corrected inputs

The0.2.0 ELF was withheld by39 personal-path rule matches. The release workflow now uses Rust compiler path remapping for CI home/checkout; the replacement0.2.1 ELF passed the same scan. The0.2.0 tag remains immutable, its unpublished draft402449475 was deleted through the bot, and task:release-codegate-0-2-0 is archived as superseded rather than implemented.

The publish command requires a branch ref: signed tag evidence was published against exact main, followed by a coordinated bot tag push. An upload call also refused an absolute local filename in argv; repository-relative copies of the exact same verified asset bytes were then uploaded through the same bot wrapper. No scanner, identity, branch or required-check policy was bypassed.

## Repository completion

Primary main is clean at the released commit. This record/task closure is integrated through a final metadata-only bot PR; it does not move the release tag or alter product bytes. Managed tree wt-227ffbd51320 retains private logs until its recovery archive and exact cleanup. Documentation source and versioned installation links were updated and validated; live documentation publication is asynchronous and not claimed here.
