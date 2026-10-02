---
format: aep.planning-md/3
id: review-result:codegate-wave1-adversary-pass1
kind: review-result
status: active
title: 'Wave 1 adversary: three gate and evidence defects'
relations:
- reviews: story:offline-dependency-slice
revision: 1
---
unit: offline-dependency-slice candidate 9900a86b982a9a13d339bb6867e468f087e68f81
verdict: NEEDS-CHANGE
cases: executed 19→21, red 2
origin: introduced 3 / pre-existing 0 / undecided 0
wrote-outside-worktree: none authored; standard tool caches and own lease only
needs-coordinator: route three gate/evidence defects for correction; preserve logs and clean owned scratch after review

## 1. Tests-only change proof

`git --no-pager diff --stat` is empty because both added tests are untracked. `git ls-files --others --exclude-standard` lists exactly:

```text
tests/gate_adversary.rs
tests/provenance_adversary.rs
```

The equivalent `git diff --no-index --stat /dev/null <test>` output is:

```text
 /dev/null => tests/gate_adversary.rs | 122 +++++++++++++++++++++++++++++++++++
 1 file changed, 122 insertions(+)
 /dev/null => tests/provenance_adversary.rs | 42 ++++++++++++++++++++++++++++++
 1 file changed, 42 insertions(+)
```

No implementation, existing test, ESS input or planning file was edited. Scratch copies are explicitly described below. Base 8e4ead994e1ecadedabb05b60098706078cc2504 has no src/bin/codegate-check.rs or tests/conformance.rs; the candidate introduces all three defects. Baseline19 comes from implementation-report.md, not a pre-attack suite run. Original source, tests, documents, generated models and acceptance were read before tests were written. Only the bounded three gate/evidence hypotheses were probed.

## 2. Added cases and isolated outputs

`tests/gate_adversary.rs` generates the same ESS behavior contract twice, applies the prescribed formatter, verifies equal dependency source, then compiles an unchanged copy of the real gate source with an appended test that calls its actual private tree comparator. It asserts the two generated contracts compare equally. No gate source in the checkout is mutated. RED solely on `.ess-output/state.json`; both generated products and probe compile successfully. The local ownership file contains anchor/path/inode state and must not participate in contract drift (nor be retained as portable generated source).

Command (exit101):

```console
RUSTC_WRAPPER=/usr/bin/sccache CARGO_BUILD_JOBS=2 CODEGATE_ADVERSARY_SCRATCH="$PWD/.scratch/wave-001/offline-dependency/adversary-1" cargo test --locked -p codegate-cli --test gate_adversary unchanged_generated_contract_ignores_local_ownership_state -- --exact --nocapture
```

Verbatim first isolated output:

```text
   Compiling codegate-cli v0.1.0 ($HOME/.local/state/worktree/trees/b10x/codegate/codegate-w1-offline-dependency-20261002)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.68s
     Running tests/gate_adversary.rs (target/debug/deps/gate_adversary-ddf7dfd8aca53a2b)

running 1 test

thread 'unchanged_generated_contract_ignores_local_ownership_state' (1564195) panicked at tests/gate_adversary.rs:49:5:
actual gate comparator rejected unchanged contracts:

running 1 test
test ownership_state_is_not_generated_contract_drift ... FAILED

failures:

failures:
    ownership_state_is_not_generated_contract_drift

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s



thread 'ownership_state_is_not_generated_contract_drift' (1565172) panicked at $HOME/.local/state/worktree/trees/b10x/codegate/codegate-w1-offline-dependency-20261002/.scratch/wave-001/offline-dependency/adversary-1/generation-1564190/gate_probe.rs:271:5:
identical ESS contract spuriously drifts: [".ess-output/state.json", ".ess-output/state.json"]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test unchanged_generated_contract_ignores_local_ownership_state ... FAILED

failures:

failures:
    unchanged_generated_contract_ignores_local_ownership_state

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.65s

error: test failed, to rerun pass `-p codegate-cli --test gate_adversary`
```

`tests/provenance_adversary.rs` runs the real conformance target freshly and checks that its emitted `completed_at` belongs to the actual measured run interval. RED: completion is fixed in2023. `Runner::for_suite` in ESS's pinned runner.rs:321 chooses AdvancingClock::default; runner.rs:388 records that synthetic clock as completed_at. This is the native evidence timestamp consumed downstream, not merely a scenario-local clock. The pure evaluator need not gain IO; the conformance harness can supply an observed clock.

Command (exit101):

```console
RUSTC_WRAPPER=/usr/bin/sccache CARGO_BUILD_JOBS=2 cargo test --locked -p codegate-cli --test provenance_adversary native_report_records_observed_completion_time -- --exact --nocapture
```

Verbatim first isolated output:

```text
   Compiling codegate-cli v0.1.0 ($HOME/.local/state/worktree/trees/b10x/codegate/codegate-w1-offline-dependency-20261002)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.98s
     Running tests/provenance_adversary.rs (target/debug/deps/provenance_adversary-460c773ce7bbfb17)

running 1 test

thread 'native_report_records_observed_completion_time' (1587777) panicked at tests/provenance_adversary.rs:11:5:
native evidence completion 1700000005500 is outside actual run [1790944137692, 1790944142673]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test native_report_records_observed_completion_time ... FAILED

failures:

failures:
    native_report_records_observed_completion_time

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.98s

error: test failed, to rerun pass `-p codegate-cli --test provenance_adversary`
```

The third case is a standalone scratch Rust test in `adversary-1/freshness_probe.rs`, separately compiled by rustc --test. It copies the project to `adversary-1/stale-1602098`, omits conformance.rs (models an accidentally lost/disabled adapter test), and copies the old report/suite. To reach the native-report check past the first proven defect, only this scratch copy's gate comparator ignores `.ess-output`; this compensation is explicit and is not a checkout edit. All remaining18 tests, actual generator commands, formatters and Clippy run, with no tool stubs. The unchanged report-admission code accepts the old evidence, prints executed27, and the gate exits0. The assertion requiring rejection turns RED (probe exit101). This is a missing-test negative control: current candidate still runs conformance normally, and current complete gate is first blocked by ownership drift; stale acceptance becomes reachable when that earlier defect is corrected and an adapter test stops executing. It must remain a gate failure in that state.

Command:

```console
rustc --edition=2024 --test .scratch/wave-001/offline-dependency/adversary-1/freshness_probe.rs -o .scratch/wave-001/offline-dependency/adversary-1/freshness_probe
RUSTC_WRAPPER=/usr/bin/sccache CARGO_BUILD_JOBS=2 .scratch/wave-001/offline-dependency/adversary-1/freshness_probe --nocapture
```

Verbatim isolated output:

```text

running 1 test
test stale_report_cannot_replace_missing_conformance_execution has been running for over 60 seconds

thread 'stale_report_cannot_replace_missing_conformance_execution' (1602099) panicked at .scratch/wave-001/offline-dependency/adversary-1/freshness_probe.rs:27:5:
gate accepted stale 27-scenario report despite no conformance test existing; see stale-gate.log
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test stale_report_cannot_replace_missing_conformance_execution ... FAILED

failures:

failures:
    stale_report_cannot_replace_missing_conformance_execution

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 101.02s

```

Complete nested gate output is retained verbatim in `adversary-1/stale-gate.log`. It finished normally, gate0/probe101,101.02seconds. No full gate ran on the actual checkout. The standalone probe was started before the package suite and completed afterward; it is not counted among the21 package tests. Package red2 plus this separately executed red1 are three executable findings.

## 3. Package suite after the two added package cases were individually red

Command (exit101;21 executed,19 passed,2 failed,0 ignored;27 ESS scenarios pass):

```console
RUSTC_WRAPPER=/usr/bin/sccache CARGO_BUILD_JOBS=2 CODEGATE_ADVERSARY_SCRATCH="$PWD/.scratch/wave-001/offline-dependency/adversary-1" cargo test --locked -p codegate-cli --all-targets --no-fail-fast -- --nocapture
```

Verbatim output:

```text
   Compiling codegate-cli v0.1.0 ($HOME/.local/state/worktree/trees/b10x/codegate/codegate-w1-offline-dependency-20261002)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.26s
     Running unittests src/lib.rs (target/debug/deps/codegate-a36e6f786884551d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/codegate-9d160637506b0ae7)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/codegate-check.rs (target/debug/deps/codegate_check-c64d15f27431dfe0)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/boundary.rs (target/debug/deps/boundary-27fc3f9db43a1f99)

running 5 tests
test generated_integer_obligation_is_checked ... ok
test explicit_absence_bridge ... ok
test strict_json_structure ... ok
test all_generated_fields_roundtrip ... ok
test decoder_limits ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.15s

     Running tests/cli.rs (target/debug/deps/cli-2a3b151a142e54f2)

running 3 tests
test cli_structural_refusal_emits_no_report_or_output_file ... ok
test unsupported_policy_format_is_semantic_error ... ok
test cli_reports_full_outcomes_and_coverage_aware_exits_without_tools ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s

     Running tests/conformance.rs (target/debug/deps/conformance-ab8d06112824f6e4)

running 1 test
{
  "completed_at": 1700000005500,
  "conformance_status": "passed",
  "counts": {
    "error": 0,
    "failed": 0,
    "passed": 27,
    "skipped": 0,
    "total": 27,
    "unsupported": 0
  },
  "coverage": {
    "counts": {
      "authored": 26,
      "generated": 1,
      "outside": 0,
      "refused": 0
    },
    "knowledge": "complete_inventory",
    "refused": [],
    "selection": {
      "filter": {
        "kind": "all"
      },
      "origins": "generated_and_authored",
      "scope": {
        "kind": "system"
      }
    }
  },
  "execution_status": "passed",
  "format": "ess-conformance-report/2",
  "implementation": "codegate-offline 0.1.0",
  "outcomes": {
    "error": [],
    "failed": [],
    "passed": [
      "codegate.dependency.Evaluate/outcome/evaluated",
      "codegate.dependency/authored/canonical-order",
      "codegate.dependency/authored/complete-empty-graph",
      "codegate.dependency/authored/complete-isolated-units",
      "codegate.dependency/authored/configuration-mismatch",
      "codegate.dependency/authored/dangling-source",
      "codegate.dependency/authored/dangling-target",
      "codegate.dependency/authored/dependency-kind-selection",
      "codegate.dependency/authored/duplicate-edge",
      "codegate.dependency/authored/duplicate-unit",
      "codegate.dependency/authored/empty-identity",
      "codegate.dependency/authored/explicit-unresolved-target",
      "codegate.dependency/authored/failed-collection",
      "codegate.dependency/authored/forbidden-runtime-edge",
      "codegate.dependency/authored/go-facts-same-checker",
      "codegate.dependency/authored/invalid-policy-reference",
      "codegate.dependency/authored/invalid-target",
      "codegate.dependency/authored/one-runtime-edge",
      "codegate.dependency/authored/parallel-imports-count-once",
      "codegate.dependency/authored/partial-is-not-empty",
      "codegate.dependency/authored/polyglot-no-inferred-edge",
      "codegate.dependency/authored/source-mismatch",
      "codegate.dependency/authored/unknown-ir-version",
      "codegate.dependency/authored/unresolved-cannot-claim-complete",
      "codegate.dependency/authored/unsupported-is-not-empty",
      "codegate.dependency/authored/unsupported-with-observations",
      "codegate.dependency/authored/witnessed-violation-with-gaps"
    ],
    "skipped": [],
    "unsupported": []
  },
  "policy": "complete-selection/1",
  "producer_profile": "rust-scenario-status/1",
  "spec_digest": "e27ccc45957cf4fe2ef83d9362e81d867eb46bff9a72f1fd28e43b69bf53ec93",
  "specification": "codegate/v1",
  "suite": {
    "digest": "sha256:2a9dbfefefb93ab95151c290d37cdc270effb0541e2e2e384cae403205d66755",
    "digest_profile": "sha256-json-bytes/1",
    "version": "ess-conformance/29"
  }
}

test real_evaluator_conforms_to_complete_combined_suite ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.26s

     Running tests/gate_adversary.rs (target/debug/deps/gate_adversary-ddf7dfd8aca53a2b)

running 1 test

thread 'unchanged_generated_contract_ignores_local_ownership_state' (1612151) panicked at tests/gate_adversary.rs:116:5:
actual gate comparator rejected unchanged contracts:

running 1 test
test ownership_state_is_not_generated_contract_drift ... FAILED

failures:

failures:
    ownership_state_is_not_generated_contract_drift

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s



thread 'ownership_state_is_not_generated_contract_drift' (1612544) panicked at $HOME/.local/state/worktree/trees/b10x/codegate/codegate-w1-offline-dependency-20261002/.scratch/wave-001/offline-dependency/adversary-1/generation-1612150/gate_probe.rs:271:5:
identical ESS contract spuriously drifts: [".ess-output/state.json", ".ess-output/state.json"]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test unchanged_generated_contract_ignores_local_ownership_state ... FAILED

failures:

failures:
    unchanged_generated_contract_ignores_local_ownership_state

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.13s

error: test failed, to rerun pass `-p codegate-cli --test gate_adversary`
     Running tests/obligation.rs (target/debug/deps/obligation-653688523cdb0bfd)

running 2 tests
test evaluate_obligation_is_fulfilled ... ok
test trait_adapter_never_leaks_a_previous_response ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/provenance_adversary.rs (target/debug/deps/provenance_adversary-460c773ce7bbfb17)

running 1 test

thread 'native_report_records_observed_completion_time' (1612552) panicked at tests/provenance_adversary.rs:38:5:
native evidence completion 1700000005500 is outside actual run [1790944211340, 1790944211745]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test native_report_records_observed_completion_time ... FAILED

failures:

failures:
    native_report_records_observed_completion_time

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.41s

error: test failed, to rerun pass `-p codegate-cli --test provenance_adversary`
     Running tests/semantics.rs (target/debug/deps/semantics-4bcb7134b24f2c61)

running 8 tests
test core_module_boundary_has_no_io_or_language_dispatch ... ok
test partial_is_never_complete ... ok
test parallel_imports_count_unique_destinations ... ok
test dangling_target_guard ... ok
test labels_and_input_order_do_not_choose_algorithms ... ok
test diagnostic_phase_precedence_and_order ... ok
test semantic_invalid_classes ... ok
test every_authored_literal_matches_real_evaluation ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

error: 2 targets failed:
    `-p codegate-cli --test gate_adversary`
    `-p codegate-cli --test provenance_adversary`
```

## 4. Findings and reachability

| file:line | Verdict | Origin | What was measured | What reaches it |
| --- | --- | --- | --- | --- |
| src/bin/codegate-check.rs:42 | NEEDS-CHANGE | introduced | Actual comparator rejects identical official regenerations only on local ownership metadata. | Every task check regenerates into a fresh directory before comparing the full trees. |
| src/bin/codegate-check.rs:181 | NEEDS-CHANGE | introduced | Isolated real gate returns0 with an old27-pass report after conformance.rs is omitted; its report branch is unchanged. | A previously run checkout where the conformance test is removed/disabled; current ownership-drift blocker must first be fixed. |
| tests/conformance.rs:112 | NEEDS-CHANGE | introduced | Fresh actual run reports completed_at1700000005500 outside measured2026 invocation. | Every conformance invocation and task check emits that native report for durable evidence import. |

The freshness fix should require current execution, e.g. a unique per-run suite/report destination, freshly cleared output or a run nonce linked to the report, and verify complete count inventory. Do not merely trust an existing success-shaped report. No changes to expected ESS responses are required.

## 5. Attacked and not broken

- Read decoder/admission/analysis/check/trait/CLI seams and all existing tests; the existing19 cases still pass, including strict JSON, precedence, count bounds, label independence and CLI coverage-aware exits.
- The real adapter calls the evaluator and returns its actual response without reading fixture expectations or choosing by scenario name; all27 actual ESS scenarios pass.
- Formatter normalization does produce identical generated domain source; the observed false drift is specifically the tool's local ownership record.

## 6. Writes and handoff

No authored path outside the assigned worktree. Commands used standard Cargo caches, installed sccache and worktree's own session lease state; no cache/credential configuration changed. All probe source, artifacts and logs are within assigned `adversary-1`; normal package output is the existing root `target/`. No cleanup was performed.

Exact new isolated build directories (all processes finished):

- `.scratch/wave-001/offline-dependency/adversary-1/stale-1602098/target` —650MiB
- `.scratch/wave-001/offline-dependency/adversary-1/stale-1602098/generated/behavior/target` —16MiB
- `.scratch/wave-001/offline-dependency/adversary-1/stale-1602098/generated/wire/target` —101MiB

Additional scratch generated roots and small compiled probe binaries are beneath `adversary-1/generation-*`; preserve logs before coordinator-owned cleanup. Available disk was20,957,069,312bytes during the bounded running probe; no further build was launched after that observation. Own lease released at handoff.

```findings
- file: src/bin/codegate-check.rs
  line: 42
  category: contract-drift
  severity: blocker
  verdict: NEEDS-CHANGE
  origin: introduced
  message: The generated-contract comparator includes per-directory ESS ownership metadata and rejects identical official regenerations.
- file: src/bin/codegate-check.rs
  line: 181
  category: mutant
  severity: blocker
  verdict: NEEDS-CHANGE
  origin: introduced
  message: Once the earlier ownership-drift defect is compensated, an old passing report satisfies the gate even when its conformance test no longer exists.
- file: tests/conformance.rs
  line: 112
  category: contract-drift
  severity: blocker
  verdict: NEEDS-CHANGE
  origin: introduced
  message: The native conformance evidence records synthetic 2023 completion time instead of the observed execution time.
```
