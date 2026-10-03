---
format: aep.planning-md/3
id: architecture-design:language-neutral-fact-ir
kind: architecture-design
status: draft
title: Language-neutral fact IR, shared analyses, and language bindings
summary: Bindings extract typed facts; common analyses and policy checks consume validated IR without language-specific logic.
relations:
- designs: vision:language-neutral-code-quality
revision: 2
---
## Decision and authority

The operator's 2026-10-03 accepted semantic-parity plan extends the existing vision to
Go, Rust, and Java/Quarkus together. This revision supersedes the former Rust-first,
Go-second scope and deferral of navigation, complexity and scoring. The existing
`codegate-dependency-facts/0.1` and `codegate-dependency-report/0.1` remain independently
versioned and supported. `src/lib.rs:1` and `src/main.rs:1` implement that offline
slice; `AGENTS.md` requires its 27 conformance scenarios to keep passing.

## Target and completion boundary

Deliver a Rust library and clap-derived JSON CLI for `collect`, `assess`, `lookup`,
`suggest`, `capabilities`, and offline `evaluate`, plus standalone HTML reporting.
Assessment, architecture policy, maintainability, safety/security/performance,
testability signals, configurable versioned scoring, advisory suggestions, symbol
lookup, references, implementations, callers and callees are in scope. Complete parity
requires every reference capability to have a tested implementation, explicit
language mapping, or justified non-applicability. Unsupported is an open delivery gap.

Reference: fluxplane/codegate at `4d1515ea925a0d4018ca59da2de3c31a91187f4e`.
The archive downloaded from the pinned GitHub API URL on 2026-10-03 matches the local
reference cache, SHA256 `81add809411fbaa27671bec42111879972c505f61464a64b916c019c011c2ae4`.
Its capabilities will be enumerated with code/test citations in the parity baseline.

Exclude source editing, executable refactoring, MCP, hosted service and Markdown
analysis. These are intentional exclusions, never reasons to exclude an in-scope
analysis. Suggestions are advisory text with evidence, never edits or shell commands.

## Architecture and ownership

1. Source collection selects files and configuration, reads dirty/untracked bytes,
   classifies test/generated inputs, and computes content identities. Include selected
   manifests, lockfiles and build/profile configuration; a Git commit alone is insufficient.
2. Tree-sitter Go, Rust and Java bindings produce declarations, spans, imports,
   annotations, control-flow and syntactic references. They produce observations,
   never policy verdicts, grades or common metric values.
3. Explicit semantic mode invokes installed gopls, rust-analyzer and Eclipse JDT LS.
   Tool versions, exact selected configuration, diagnostics and completeness accompany
   their normalized facts. Requested semantic mode never silently becomes source-only.
4. Quarkus bindings model declared CDI beans, qualifiers, producers, injections,
   REST routes, transaction declarations and configuration references. Effective
   augmentation-dependent wiring requires matching build evidence. Ambiguous injection,
   programmatic lookup and generated beans stay explicit gaps without that evidence.
5. Admission validates all fact identities, spans, endpoints, coverage and evidence
   identities, then creates a private immutable wrapper. Shared queries/analyses/checkers
   accept only that wrapper and explicit policies. They neither import bindings nor
   perform IO nor branch on language/producer. Enforce this dependency boundary in tests.
6. Presentation emits separately versioned JSON and self-contained HTML with facts,
   findings, limits, coverage, scores and advisory suggestions. Escape untrusted source.

Author implementation and harnesses in Rust. Third-party parser grammars, language
servers and toolchains are external dependencies; Go/Java snippets are fixture data.
No dynamic plugin runtime or embedded policy language is required.

## Semantic invariants

- Syntactic candidates, resolved and unresolved relationships have separate identities
  and bases. A matching name alone is not proof of a resolved reference or call.
- Coverage is per fact family, scope and selected build configuration. Complete means
  complete for its declared universe, never a claim about every runtime target.
- Missing/failed/partial required facts cannot yield zero measurements, passing absence
  checks or complete overall scores. Witnessed violations remain failures with gaps shown.
- Imported evidence must match the collected source and configuration identities;
  mismatches are admission refusals. Overlapping conflicting producers also refuse.
- Portable metrics use one versioned counting contract and equivalent Go/Rust/Java
  examples. Language-specific identities (such as Go defer) have explicit applicability.
- Exact rational aggregation uses compatible disjoint populations; no average of ratios.
  Empty denominators produce an explicit eligibility result, never manufactured zero.
- Tool call graphs remain bounded; dynamic dispatch, macros, cfg, build tags, Java
  overloads and generated sources require explicit resolution/coverage evidence.
- Query and evaluation of imported facts run without filesystem, tools or network.
  Determinism applies to identical admitted inputs and selected configuration.
- Policy exceptions require bounded scope, reason, owner and expiry; retain waived
  findings. Required incomplete results take CLI exit 2, observed complete failures 1,
  and complete success 0. Unsupported required analysis is not a successful exit.

## Language and framework mapping

Go uses package/build-tag selection plus gopls; Rust uses Cargo target/features/cfg
plus rust-analyzer; Java uses Maven/Gradle including multi-module classpaths plus JDT LS.
Java 17 and 21 project targets and Quarkus 3 receive fixtures. The JDT LS host JDK is
selected independently of project target and recorded with tool provenance.
Source-only mode does not run build scripts, processors, language servers or tests.
Semantic-mode execution is explicit and bounded by process time/output limits.

Sources checked 2026-10-03: Tree-sitter using-parsers documentation;
https://go.dev/gopls/features/navigation (build-specific references and bounded call hierarchy);
https://github.com/eclipse-jdtls/eclipse.jdt.ls#requirements (host runtime requirements);
https://quarkus.io/guides/cdi-integration/ (augmentation and synthetic components).

## Delivery and verification

The AEP program decomposes into (1) parity baseline and validated ESS foundations,
(2) common first slice across all three languages, (3) semantic navigation adapters,
(4) broad shared assessment and Quarkus interpretation, and (5) parity verification,
JSON/HTML reports and advisory adoption. Every implementation story follows its typed
ESS contract, carries machine-readable scope/dependencies, and names conformance cases.
Four independent decomposition critics review acceptance, design, scope and parallel
safety; preserve both verdict rounds and one outcome per finding.

`task check` must validate both specifications, generated drift, actual ESS conformance,
Rust tests and the core boundary. Keep all 27 existing evaluator scenarios. Add pinned
reference comparisons, equivalent-language metric/policy fixtures, deterministic replay,
malformed syntax, ambiguity, configuration selection, stale evidence, missing tools,
timeouts and partial collection. Tests must distinguish test inventories/ratios from
measured execution coverage. Required checks follow successful advisory repository pilots.

The existing 0.1.0 public-delivery task and its Secrets-access blocker are separate.
Neither its release nor website delivery is a dependency of this program. No source
publication, tag or release is authorized merely by implementation.
