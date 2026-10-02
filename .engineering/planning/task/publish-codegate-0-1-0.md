---
format: aep.planning-md/3
id: task:publish-codegate-0-1-0
kind: task
status: active
title: Publish Codegate documentation, Gates enrollment and first release
relations:
- serves: vision:language-neutral-code-quality
- decomposes: story:public-delivery
revision: 5
transitions:
- {from: "draft", to: "proposed", at: "2026-10-02T13:12:49Z", actor: "human:timo", revision: 2, executor: "agent:codegate-delivery"}
- {from: "proposed", to: "active", at: "2026-10-02T13:12:49Z", actor: "human:timo", revision: 3, executor: "agent:codegate-delivery"}
---
## Context

Operator explicitly requested public documentation like Mantle, common Gates enrollment, bot-authored history, publication notification and the first version release. No remote or tag exists yet. All five existing commits already have the exact bot author and committer. Preserve commit identities unless a measured admission defect requires rewriting; no false claim of an unnecessary authorship rewrite.

## Done When

Public beyond10x/codegate exists with bot-only direct history; common Gates policy/signing grants, hooks, selected CI policy and required security check are enrolled; repository correctness, documentation validation and release builds pass for the exact published tag0.1.0; a bot-owned GitHub Release has Linux x86_64 executable archive plus SHA256SUMS; the public /codegate/ site serves matching source and publisher provenance. Record actual URLs, commit/tag, checks and any limitation before claiming publication.

## Scope

Codegate: public website source and Rust clap docs builder, README/AGENTS, Cargo metadata/version CLI, Taskfile and checks, pinned CI/site/release build workflows, license/release notes, governed delivery evidence. Coordinator owns Gates private registry/signer/selected-secret enrollment and GitHub repository/Pages/rules setup through bot API. No credential or private policy is committed to public source. Existing evaluator ESS semantics remain unchanged.

## Authority

This user request explicitly authorizes GitHub repository creation/publication, bot attribution correction as needed, Gates registration, Pages configuration, tagged0.1.0 source release and release artifacts. Standing wave approval remains applicable. All runnable implementation is Rust, CLI clap derive; managed trees only. Integrations use Connectors first; configured adapters have no GitHub capability, so bot-authenticated Gates API is the declared fallback. Baseline historical privacy is audited before public delivery; no refusal is bypassed.

## Delivery progress

Created public GitHub repository `beyond10x/codegate`, numeric id1401762311, through the bot App. Copied Mantle's active App-only branch/tag authority and identity rulesets through the bot API. Required common/repository checks will be enabled after the first successful main run, per Gates adoption procedure.

Private Gates policy change committed/published as18642a4: exact repository id and original empty root56edf02005fb019957b16ecc2f092e0b2e67790e as baseline; existing authorized Mantle signers granted Codegate. This baseline retains all nonempty source commits inside admission. Coordinated hooks installed. No blanket historical exception or baseline advancement excludes the private-path findings.

Bot API GET of the repository Actions secret public key returned403 Forbidden. CI policy-secret provisioning is pending bot Secrets permission or an authorized owner's direct repository-secret setup. The operator was asked for that external action; no personal-account write or alternative credential bypass was attempted.

Public history projection will replace private filesystem prefixes in historical log text with `$HOME/`, preserve product behavior and retained evidence counts, keep all commit authors/committers as the bot, and record the old/new commit map. Original history and unsanitized evidence remain in local recovery archives. This fulfills the explicit history-rewrite/publication request while removing the measured privacy defect rather than exempting it.

## Documentation implementation evidence

The bounded docs worker returned three files: website/index.html, website/styles.css and src/bin/codegate-docs.rs. Its targeted cargo test executed5passed/0failed; examples are decoded and evaluated through the real core, first Pass then forbidden-edge Fail. Targeted Clippy with-Dwarnings, formatter and diff checks exited0. Static validation only; no browser render claimed. Source was copied into the coordinator checkout unchanged. Full integration check remains pending.

Only the worker's stopped528MiB target was removed. Its source and raw logs were archived under the managed id codegate-docs-20261002. Primary implementation evidence and unsanitized original Git history are already retained in the earlier codegate-wave1-plan-20261002 recovery archive. No new source-language binding is introduced by this delivery.
