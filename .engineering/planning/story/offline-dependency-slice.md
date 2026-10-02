---
format: aep.planning-md/3
id: story:offline-dependency-slice
kind: story
status: implemented
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
revision: 18
transitions:
- {from: "draft", to: "proposed", at: "2026-10-02T11:55:38Z", actor: "human:timo", revision: 13, decided_on: {"recorded":{"approval":1}}}
- {from: "proposed", to: "active", at: "2026-10-02T11:55:38Z", actor: "human:timo", revision: 14, decided_on: {"recorded":{"approval":1}}}
- {from: "active", to: "implemented", at: "2026-10-02T12:55:17Z", actor: "human:timo", revision: 18, decided_on: {"recorded":{"test_result":1,"approval":1,"review_outcome":2,"verification":1}}, executor: "agent:codegate-wave001", correlation: "codegate-wave001"}
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

Confirmed by implementation report and candidate `9900a86b982a9a13d339bb6867e468f087e68f81`, then refined during the first adversary correction.

- **Owned implementation surfaces — cited:** `.gitignore`, `AGENTS.md`, `Cargo.lock`, `Cargo.toml`, `README.md`, `Taskfile.yml`, `generated/`, `rust-toolchain.toml`, `src/`, `tests/`. All are present in candidate diff; corrections remain inside these surfaces.
- **Generated seams — cited:** `generated/behavior/src/dependency.rs` owns domain values and EvaluateBehavior; `generated/wire/types.rs` owns serde wire models. `src/wire.rs` explicitly converts them and bridges optional absence/null representation. The implementation fulfills the outcome-only trait through a response-holding adapter around the pure evaluator.
- **Dependency boundary — cited:** `src/admit.rs` constructs the private admitted wrapper; `src/analysis.rs` and `src/check.rs` accept it. No binding exists in this wave. Rust/Go-labelled normalized fixtures exercise the same evaluator.
- **Evidence/gate surfaces — cited:** `tests/conformance.rs` runs native ESS; `src/bin/codegate-check.rs` implements the repository gate; `tests/gate_adversary.rs` and `tests/provenance_adversary.rs` were added by the adversary. The first gate implementation had three measured defects, recorded in `review-result:codegate-wave1-adversary-pass1`; do not infer first-candidate gate success from evaluator success.
- **Read-only input — cited:** `ess/` remained unchanged by implementor/reviewer. Coordinator alone owns `.engineering/` mutations.
- **Correction to initial scope text:** `.gitignore` was untracked at the scoper's earlier observation but was committed in opening input `8e4ead994e1ecadedabb05b60098706078cc2504`; it was an existing tracked file at implementation dispatch. This replaces the stale untracked-file description.
- **Overlap inference confirmed:** root package wiring, CLI, generated contracts, tests and gate had shared surfaces; one implementation unit avoided conflicting ownership. No inferred fix mechanism was assumed without measurement.

## Generated projection normalization

Coordinator probe on the exact opening contract: `cargo fmt --manifest-path .scratch/synth-probe/Cargo.toml --check` exited 1 on generated trait signatures and struct literals. The retained output is `.scratch/wave-001/generated-format-probe.log` in the coordinator recovery archive. This is generator formatting, not behavioral failure.

Clarification: the deterministic projection pipeline is ESS generation followed by the pinned Rust formatter on generated Rust. Drift checks repeat both steps and compare bytes. This permits automated format normalization, never hand-edited generated models. Generated subpackages still undergo actual formatting and lint checks.

Generated wire Optional values use absence, while the authored contract examples use JSON null. A narrow explicit wire/behavior representation bridge handles the declared optional fields; malformed/unknown fields remain refusals. The generated EvaluateBehavior trait returns an outcome only; a per-invocation adapter exposes the generated typed response from the pure evaluator. Tests must show successive invocations do not leak prior responses. These are implementation seams, not changes to ESS expected behavior.

## Coverage-bearing suite generation

The first real native Runner report executed all 27 cases successfully but reported `coverage.knowledge=unknown`, `execution_status=passed`, `conformance_status=inconclusive` for ordinary synthesis. That result is retained as historical evidence, not promoted to complete coverage.

Both coordinator and implementor verified the supported `ess verify conform synthesize --path ess --scenarios ess --suite-format 5 --out <file>` option. Its actual inventory lists the same 26 authored IDs and one generated ID, zero outside and zero refused, and `knowledge=complete_inventory`. Use this official coverage-bearing synthesis option in the gate and conformance adapter; do not invent or patch inventory metadata. Preserve lineage and the native admitted suite/report. Complete declared inventory is not a claim that finite examples exhaust all possible inputs.
