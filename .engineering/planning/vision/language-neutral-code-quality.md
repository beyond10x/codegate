---
format: aep.planning-md/3
id: vision:language-neutral-code-quality
kind: vision
status: draft
title: Code quality from language-neutral facts
summary: Any language binding can supply facts to the same shared analyses and checkers.
revision: 2
---
## Intent

Developers and agents should receive explainable code quality and navigation from
language-neutral facts. Bindings observe source/tool semantics, analyses derive
measurements, and common checkers apply explicit policies to admitted facts.
Equivalent supported facts receive equivalent judgments regardless of source language.

## Current outcome

The first Rust library and JSON CLI evaluate dependency facts offline. The governed
record is `epic:offline-dependency-evaluation` (implemented); its 27 required real ESS
conformance scenarios remain the compatibility floor (`AGENTS.md`, `tests/conformance.rs`).
`story:public-delivery` and `task:publish-codegate-0-1-0` retain their separate public
release scope and existing Secrets-permission blocker.

## Accepted next program

The operator's 2026-10-03 plan expands the target to semantic analysis/navigation
across Go, Rust and Java/Quarkus, with capability-by-capability parity against
fluxplane/codegate at `4d1515ea925a0d4018ca59da2de3c31a91187f4e`.
`architecture-design:language-neutral-fact-ir` records the resulting design.

Tree-sitter supplies source structure. Explicit gopls, rust-analyzer and Eclipse JDT LS
adapters supply semantic evidence tied to source/configuration identities. Shared
architecture, maintainability, safety/security/performance and testability analysis,
versioned scoring, advisory suggestions, lookup/references/implementations/call graphs,
JSON and standalone HTML are in scope. Source-only is the default; unsupported and
incomplete required capabilities remain visible delivery gaps.

## Exclusions

Source editing, executable refactoring, MCP, hosted service and Markdown analysis.
Scoring is now included under an explicit versioned policy; it is not a universal
quality grade, and an overall score cannot be complete with missing required evidence.
Test inventory and ratios do not imply execution coverage.

## Evidence boundary

Specification validation is not implementation, generated obligation stubs are not
completed adapters, and a green legacy suite is not proof of new capability parity.
Require a cited parity matrix, real conformance, generated drift checks, deterministic
replay and advisory pilots before requiring new checks. All authored running code is
Rust. The AEP store owns delivery state; no chat or prose-only backlog replaces it.
