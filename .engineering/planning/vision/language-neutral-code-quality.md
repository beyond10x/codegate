---
format: aep.planning-md/3
id: vision:language-neutral-code-quality
kind: vision
status: draft
title: Code quality from language-neutral facts
summary: Any language binding can supply facts to the same shared analyses and checkers.
revision: 1
---
## Intent

Codegate provides a language-neutral intermediate representation of code facts and
execution evidence. Language bindings extract those facts. Shared analyses calculate
metrics and shared checkers evaluate quality and architecture policies against the
IR, without depending on Rust, Go, or another source language.

A new language becomes usable by implementing the binding contract for supported
fact families. It must not require a language branch in an existing shared checker.
Support is explicit: a binding need not implement every family, and unsupported or
incomplete required facts cannot silently produce a pass.

## Evidence and provenance

Operator direction, 2026-10-02, this design session: results should have a
language-agnostic IR; checkers operate on it; language implementations return facts;
Codegate should process any language once a binding is added. Rust is the initial
implementation language and initial source-language binding.

The earlier research inspected fluxplane/codegate at
4d1515ea925a0d4018ca59da2de3c31a91187f4e. Its generic backend and evidence concepts
are useful prior art; its language-local assessment/scoring and editing scope are
not the proposed ownership boundaries here.

`README.md:3` records the intended pipeline. The initial AEP reverse scan returned
empty source, test, CI and task-target collections. The initial history scan returned
one empty bootstrap commit, zero tags and no existing backlog. This vision is new
operator-requested design, not a claim reverse-engineered from shipped code.

## Desired outcome

An adopter can explain a metric or failed check through the policy, common analysis,
input facts and source/tool evidence that produced it. Two language bindings that
supply equivalent supported facts receive equivalent results from the same checker.
A third language can be added without modifying the core or existing checks when its
facts fit the published contract.

## Scope

A local Rust library and CLI; a versioned fact IR and result IR; binding admission
and capability reporting; shared metric derivation and policy checks; Rust first;
a small Go portability fixture set before treating the first IR as stable.

ESS specifies the domain and behavioral contract; AEP holds design and delivery
work. Existing Gates retains security/privacy checks, signatures and bot delivery.

## Exclusions

An editor, automatic refactoring, a hosted service, a general AST or compiler IR,
an arbitrary plugin runtime, and an overall quality grade are outside the first
version. Supporting every analysis for every language is not a capability claim.

## Status

First draft in an interactive design session. This is a product proposal; no
implementation, review approval, ESS validation or conformance is asserted.
