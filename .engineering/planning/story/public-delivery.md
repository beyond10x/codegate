---
format: aep.planning-md/3
id: story:public-delivery
kind: story
status: implemented
title: Deliver the first public Codegate release and project documentation
relations:
- serves: vision:language-neutral-code-quality
scope:
- confidence: cited
  path: .github/
- confidence: cited
  path: Cargo.toml
- confidence: cited
  path: src/
- confidence: cited
  path: website/
revision: 9
transitions:
- {from: "draft", to: "proposed", at: "2026-10-02T13:13:27Z", actor: "human:timo", revision: 6, executor: "agent:codegate-delivery"}
- {from: "proposed", to: "active", at: "2026-10-02T13:13:27Z", actor: "human:timo", revision: 7, executor: "agent:codegate-delivery"}
- {from: "active", to: "implemented", at: "2026-10-03T07:45:48Z", actor: "human:timo", revision: 9, decided_on: {"recorded":{"test_result":2,"deployment_result":1}}, executor: "agent:codegate-collection"}
---
## Context

Operator explicitly requested public documentation like Mantle, common Gates enrollment, bot-authored history, publication notification and the first version release. No remote or tag exists yet. All five existing commits already have the exact bot author and committer. Preserve commit identities unless a measured admission defect requires rewriting; no false claim of an unnecessary authorship rewrite.

## Acceptance

Public beyond10x/codegate exists with bot-only direct history; common Gates policy/signing grants, hooks, selected CI policy and required security check are enrolled; repository correctness, documentation validation and release builds pass for the exact published tag0.1.0; a bot-owned GitHub Release has Linux x86_64 executable archive plus SHA256SUMS; the public /codegate/ site serves matching source and publisher provenance. Record actual URLs, commit/tag, checks and any limitation before claiming publication.

## Scope

Codegate: public website source and Rust clap docs builder, README/AGENTS, Cargo metadata/version CLI, Taskfile and checks, pinned CI/site/release build workflows, license/release notes, governed delivery evidence. Coordinator owns Gates private registry/signer/selected-secret enrollment and GitHub repository/Pages/rules setup through bot API. No credential or private policy is committed to public source. Existing evaluator ESS semantics remain unchanged.

## Authority

This user request explicitly authorizes GitHub repository creation/publication, bot attribution correction as needed, Gates registration, Pages configuration, tagged0.1.0 source release and release artifacts. Standing wave approval remains applicable. All runnable implementation is Rust, CLI clap derive; managed trees only. Integrations use Connectors first; configured adapters have no GitHub capability, so bot-authenticated Gates API is the declared fallback. Baseline historical privacy is audited before public delivery; no refusal is bypassed.

## Release completion reconciled 2026-10-03

PR https://github.com/beyond10x/codegate/pull/1 merged to remote main 8ebd4a200c8181b0d48221cab5c1fd25f8d30f45. Annotated tag 0.1.0 object 775d9e4393597209535c3aa9027ca4a18789886f points to that exact commit. The bot-owned release is public at https://github.com/beyond10x/codegate/releases/tag/0.1.0, published 2026-10-03T01:43:01Z. Linux archive and SHA256SUMS were uploaded and downloaded checksums verified; archive SHA256 1fa12b20dfc0cc57f0d6bce18c68c0b8ccd8d609f43dfeb8f48aa6d9dbe31e1b.

Required tag checks succeeded: common security run37086993620, Gate run37086993142, release artifacts run37086993155. Main docs/site runs37086809592 and37086884075 succeeded. Required-check ruleset24402354 is active and has no bypass actors. The policy secret enrollment was completed using the operator's single explicit gh exception; this grants no ongoing personal-write authority. General bot Secrets access remains unavailable but no longer blocks this completed release.

Live https://beyond10x.github.io/codegate/.well-known/b10x-site.json and .well-known/b10x-docs.json were re-read during collection planning and both identify 8ebd4a200c8181b0d48221cab5c1fd25f8d30f45. Publisher runtime fb4024ef7846729e5456591b9070db3d48c87e64; artifact e01d174ec402f464fef61cd8c07825e66d4b0dbbf577e26f5007ec0f01cabf0f. Primary main is clean at that remote commit; previous release worktrees were retired with recovery proof. Current collection trees belong to subsequent work.

This resolves the earlier historical pending-release text. No new release is part of the collection wave.
