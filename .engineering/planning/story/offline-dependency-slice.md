---
format: aep.planning-md/3
id: story:offline-dependency-slice
kind: story
status: active
title: Evaluate language-neutral dependency facts offline
summary: Implement the generated Evaluate contract, shared dependency analysis, local CLI and real ESS conformance.
relations:
- decomposes: epic:offline-dependency-evaluation
- serves: vision:language-neutral-code-quality
- informed_by: executable-system-specification:dependency-evaluation
scope:
- confidence: cited
  path: .gitignore
- confidence: cited
  path: AGENTS.md
- confidence: cited
  path: Cargo.lock
- confidence: cited
  path: Cargo.toml
- confidence: cited
  path: README.md
- confidence: cited
  path: Taskfile.yml
- confidence: cited
  path: generated/
- confidence: cited
  path: rust-toolchain.toml
- confidence: cited
  path: src/
- confidence: cited
  path: tests/
revision: 14
transitions:
- {from: "draft", to: "proposed", at: "2026-10-02T11:55:38Z", actor: "human:timo", revision: 13, decided_on: {"recorded":{"approval":1}}}
- {from: "proposed", to: "active", at: "2026-10-02T11:55:38Z", actor: "human:timo", revision: 14, decided_on: {"recorded":{"approval":1}}}
---
## Context

The repository has a validated ESS value contract and 26 compiled authored scenarios
but no executable Codegate. `ess/README.md` is the bounded contract for this first
vertical slice. The architecture design requires shared checkers to consume admitted
IR without language-specific branches or source/tool access.

## Acceptance

Given the exact ESS contract and selected 26 authored scenarios, the real Rust
Evaluate implementation and offline JSON CLI produce the full expected responses
with all 27 combined scenarios executed and none failed, errored, unsupported or
skipped, while the boundary and independence regressions below pass under `task check`.

## Named conformance scenarios

`codegate.dependency/authored/` is the namespace for all 26 authored IDs:

- complete-empty-graph; complete-isolated-units; one-runtime-edge
- parallel-imports-count-once; forbidden-runtime-edge; go-facts-same-checker
- polyglot-no-inferred-edge; canonical-order; partial-is-not-empty
- witnessed-violation-with-gaps; unsupported-is-not-empty; failed-collection
- dangling-source; dangling-target; duplicate-unit; duplicate-edge
- invalid-target; explicit-unresolved-target; unresolved-cannot-claim-complete
- source-mismatch; configuration-mismatch; unknown-ir-version
- dependency-kind-selection; empty-identity; invalid-policy-reference
- unsupported-with-observations

The generated scenario adds the declared return-shape obligation. Preserve the full
combined suite; count its actual execution, not only files or selected tests.

## Implementation contract

- All implementation, test harnesses and gate logic are Rust; CLI uses clap derive.
- Generate behavior/model output into `generated/behavior/` with ESS 0.50.0 Rust
  synthesis and wire contracts into `generated/wire/` with Rust type projection.
  Do not hand-transcribe models or edit generated output. Fulfil EvaluateBehavior;
  conversions between the two generated contexts must be explicit and round-trip tested.
- The root package owns `src/lib.rs`, `src/admit.rs`, `src/analysis.rs`,
  `src/check.rs`, `src/wire.rs`, `src/main.rs` and `src/bin/codegate-check.rs`.
  Source-model distinctions stay outside pure analysis/check modules.
- The typed-input entry admits graph references and coverage before constructing a
  private validated wrapper. Only that wrapper reaches the shared analysis/checker.
- `codegate evaluate --facts <file> --policy <file> --out <file>` reads bounded JSON,
  writes the complete deterministic result for semantic outcomes, and returns the
  exit code in `ess/README.md`. Omitted `--out` writes JSON to stdout. Structural
  decoder errors print a diagnostic to stderr and never emit success-shaped output.
- No source tree, binding, clock, environment or network is used by the pure library.
  Tests call the same implementation as the CLI; no scenario-name dispatch or
  reading expected response objects inside the target adapter.
- Native ESS Runner/ConformanceTarget and supporting ESS crates use the exact
  published 0.50.0 commit `8700d0808e8f3b19711629d8a17afc5281680f58` as development
  dependencies. No relative sibling checkout dependency. Commit the Cargo lockfile.
- `tests/conformance.rs` admits and executes the complete generated/authored suite
  against the real behavior; it emits ESS report evidence tied to the exact suite.
  Retain the generation plan/runtime-obligation accounting and discharge the integer
  integrality obligation. Do not substitute the ESS built-in reference runner.
- `Taskfile.yml` calls the Rust gate entry; its steps validate AEP and ESS, regenerate
  contracts/suite into private scratch and check drift, run real conformance and
  meaningful library/CLI tests, check formatting and Clippy. Record each step's exit
  code and actual scenario counts. No recursively invoked task gate and no skipped
  success stubs. The generated subpackages are checked too.

## Boundary and independence regressions

Add cases for duplicate/unknown JSON keys, malformed values, unsupported policy
format, size/count limits, deterministic multi-defect precedence, invalid policy
kind/duplicate tuples, neither target representation, empty coverage gaps and failed
coverage with contradictory edges. Respect exact examples in `ess/README.md`.

Test CLI exits 0, 1 and 2, including a witnessed Fail with Partial coverage returning
2. Run evaluation with no source/tool access, and verify equivalent normalized
Rust/Go-labelled facts use the same shared code. Check the module dependency boundary.

Demonstrate that removing the dangling-target guard, mapping Partial to Complete, or
counting parallel import occurrences as fan-out turns a named test red. Preserve
red-run commands and outputs; do not count an uncompiled/no-tests-selected run as a
behavioral negative control.

## Work surfaces

The first implementation owns the new root `Cargo.toml`, `Cargo.lock`,
`rust-toolchain.toml`, `Taskfile.yml`, `src/`, `tests/`, `generated/`, plus the existing
`.gitignore`, `README.md` and `AGENTS.md` to document actual commands and validation.
`ess/` is an existing input contract: read it, do not change its expected results to
make the implementation pass. Any semantic disagreement returns to the coordinator.
The coordinator alone mutates `.engineering/` and records wave evidence.

## Baseline and claim verification

Base has no root Cargo manifest, codegate binary or implementation. Before treatment,
record that absence and the generated behavior's explicit unmet obligation. After
implementation, feed the same named fixture through the CLI and real conformance
adapter, retaining output and exit status. The demonstrated claim is new executable
behavior, not a performance improvement or a real language binding.

## Out of scope

Everything excluded by the parent epic. The stable IR and real Rust binding remain
future work; no package publication, GitHub repository creation, push, tag or release
is authorized by this story or its proposed wave.

## Scope

Derived 2026-10-02 by `aep:story-scoper`. Every entry distinguishes observed ownership from inference.

- **Primary surface:** new root Rust package, offline evaluator, CLI and real conformance integration — **cited**, `story:offline-dependency-slice`, Implementation contract and Work surfaces.
- **Proposed paths:** `Cargo.toml`, `Cargo.lock`, `rust-toolchain.toml`, `Taskfile.yml`, `src/`, `tests/`, `generated/` — **cited**, story lines 95–96 explicitly assign these paths to this implementation. These are proposed ownership surfaces, not existing implementation.
- **Existing paths to update:** `.gitignore`, `README.md`, `AGENTS.md` — **cited**, story line 97 and inspected working tree. The ignore file currently exists as an untracked planning-worktree file; the README and repository guidance already exist.
- **Symbols:** `EvaluateBehavior`, ESS `Runner` and `ConformanceTarget` — **cited**, story lines 50 and 64–70. The generated behavior and target adapter are future implementation, not currently implemented symbols.
- **Documents:** update the existing README and repository guidance to describe actual commands and validation — **cited**, story line 97.
- **Integration dependencies:** generated behavior and standalone wire types must interoperate through explicit tested conversions; native ESS dependencies must use published commit `8700d0808e8f3b19711629d8a17afc5281680f58`; the Rust gate must validate the complete combined suite and generated packages — **cited**, story lines 48–75.
- **Input-only boundary:** the existing ESS contract and its authored scenarios are read-only inputs for this implementor; semantic disagreements return to the coordinator, who alone writes the planning store and wave evidence — **cited**, story lines 98–100.
- **Confidence:** high for package ownership — **cited**, the story explicitly assigns every proposed and existing write surface, and repository inspection confirms there is no root implementation yet.
- **Would collide with:** any work creating or changing the root Cargo package, toolchain pin, task gate, source modules, tests, generated contracts, ignore rules, README or repository guidance — **inferred**, these surfaces require shared package/module wiring and coordinated generated-contract versions.
- **Safety fact:** shared analysis/checking must receive only the private admitted wrapper and must not access source trees, bindings, clocks, environment or network — **cited**, story lines 55–63; evidence level 2, unproven. The declaration is present, but no implementation exists to verify enforcement.
