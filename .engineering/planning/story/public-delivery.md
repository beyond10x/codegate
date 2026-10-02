---
format: aep.planning-md/3
id: story:public-delivery
kind: story
status: active
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
revision: 7
transitions:
- {from: "draft", to: "proposed", at: "2026-10-02T13:13:27Z", actor: "human:timo", revision: 6, executor: "agent:codegate-delivery"}
- {from: "proposed", to: "active", at: "2026-10-02T13:13:27Z", actor: "human:timo", revision: 7, executor: "agent:codegate-delivery"}
---
## Context

Operator explicitly requested public documentation like Mantle, common Gates enrollment, bot-authored history, publication notification and the first version release. No remote or tag exists yet. All five existing commits already have the exact bot author and committer. Preserve commit identities unless a measured admission defect requires rewriting; no false claim of an unnecessary authorship rewrite.

## Acceptance

Public beyond10x/codegate exists with bot-only direct history; common Gates policy/signing grants, hooks, selected CI policy and required security check are enrolled; repository correctness, documentation validation and release builds pass for the exact published tag0.1.0; a bot-owned GitHub Release has Linux x86_64 executable archive plus SHA256SUMS; the public /codegate/ site serves matching source and publisher provenance. Record actual URLs, commit/tag, checks and any limitation before claiming publication.

## Scope

Codegate: public website source and Rust clap docs builder, README/AGENTS, Cargo metadata/version CLI, Taskfile and checks, pinned CI/site/release build workflows, license/release notes, governed delivery evidence. Coordinator owns Gates private registry/signer/selected-secret enrollment and GitHub repository/Pages/rules setup through bot API. No credential or private policy is committed to public source. Existing evaluator ESS semantics remain unchanged.

## Authority

This user request explicitly authorizes GitHub repository creation/publication, bot attribution correction as needed, Gates registration, Pages configuration, tagged0.1.0 source release and release artifacts. Standing wave approval remains applicable. All runnable implementation is Rust, CLI clap derive; managed trees only. Integrations use Connectors first; configured adapters have no GitHub capability, so bot-authenticated Gates API is the declared fallback. Baseline historical privacy is audited before public delivery; no refusal is bypassed.
