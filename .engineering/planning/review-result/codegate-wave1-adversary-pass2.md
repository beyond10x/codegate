---
format: aep.planning-md/3
id: review-result:codegate-wave1-adversary-pass2
kind: review-result
status: active
title: 'Wave 1 second adversary: no remaining finding in bounded correction review'
relations:
- reviews: story:offline-dependency-slice
revision: 1
---
unit: offline-dependency-slice corrected candidate 4bf8fa591888698fdf23194d93daebb81bbb4e18
verdict: nothing found
cases: executed 22→22, red 0
origin: introduced 0 / pre-existing 0 / undecided 0
wrote-outside-worktree: none authored; standard tool caches and own lease only
needs-coordinator: none

## 1. Tests-only bound

`git --no-pager diff --stat` and `git status --short` both produced no output. No source, tests, generated products or planning files changed in this pass. The second bounded pass read the complete correction diff against9900a86, the correction report and first findings. Baseline22 came from the correction report. No full gate or fresh copied dependency build ran.

## 2. Targeted cases before the package suite

The existing first-pass adversarial cases were preserved and rerun first, together with the maintained replacement for the standalone stale-report probe. No new finding warranted another case.

Command (exit0;three selected cases executed):

```console
RUSTC_WRAPPER=/usr/bin/sccache CARGO_BUILD_JOBS=2 CODEGATE_ADVERSARY_SCRATCH="$PWD/.scratch/wave-001/offline-dependency/adversary-2" cargo test --locked -p codegate-cli --test gate_adversary --test provenance_adversary --test freshness -- --nocapture
```

Verbatim output:

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.51s
     Running tests/freshness.rs (target/debug/deps/freshness-30aa3b1d3fcb79c8)

running 1 test
test gate_requires_this_invocations_conformance_producer ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.51s

     Running tests/gate_adversary.rs (target/debug/deps/gate_adversary-5c075ac973e2fdad)

running 1 test
test unchanged_generated_contract_ignores_local_ownership_state ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.59s

     Running tests/provenance_adversary.rs (target/debug/deps/provenance_adversary-fddccb4beaab5353)

running 1 test
test native_report_records_observed_completion_time ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.81s

```

The actual nested freshness output is retained in targeted-freshness-raw.log and suite-freshness-raw.log beside this report. It invokes real Cargo targets and the production fresh_conformance/validate_report functions in a scratch-compiled copy. The intentionally ignored dummy producer is a negative fixture, not an ignored repository test: rejection is asserted. Missing, ignored and renamed producers all refuse retained stale evidence. The actual current evaluator produces accepted evidence;19 independently altered binding fields are rejected. Original generation and observed-time assertions remain intact.

## 3. Package suite afterward

Command (exit0;22 Rust tests passed,none failed or ignored;27 ESS scenarios passed with complete inventory and zero other outcomes):

```console
RUSTC_WRAPPER=/usr/bin/sccache CARGO_BUILD_JOBS=2 CODEGATE_ADVERSARY_SCRATCH="$PWD/.scratch/wave-001/offline-dependency/adversary-2" cargo test --locked -p codegate-cli --all-targets --no-fail-fast -- --nocapture
```

Verbatim output:

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.09s
     Running unittests src/lib.rs (target/debug/deps/codegate-4656fda71df7915a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/codegate-d35365d64892493d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/codegate-check.rs (target/debug/deps/codegate_check-1fe97699a372e6fd)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/boundary.rs (target/debug/deps/boundary-9bdf51d047dbc681)

running 5 tests
test generated_integer_obligation_is_checked ... ok
test explicit_absence_bridge ... ok
test strict_json_structure ... ok
test all_generated_fields_roundtrip ... ok
test decoder_limits ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.13s

     Running tests/cli.rs (target/debug/deps/cli-fac19e6bfdfe86f0)

running 3 tests
test cli_structural_refusal_emits_no_report_or_output_file ... ok
test unsupported_policy_format_is_semantic_error ... ok
test cli_reports_full_outcomes_and_coverage_aware_exits_without_tools ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s

     Running tests/conformance.rs (target/debug/deps/conformance-c7a9fb2bcbc4979a)

running 1 test
{
  "completed_at": 1790945273056,
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

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.41s

     Running tests/freshness.rs (target/debug/deps/freshness-30aa3b1d3fcb79c8)

running 1 test
test gate_requires_this_invocations_conformance_producer ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.66s

     Running tests/gate_adversary.rs (target/debug/deps/gate_adversary-5c075ac973e2fdad)

running 1 test
test unchanged_generated_contract_ignores_local_ownership_state ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.51s

     Running tests/obligation.rs (target/debug/deps/obligation-0a26cb3fdf8672a5)

running 2 tests
test evaluate_obligation_is_fulfilled ... ok
test trait_adapter_never_leaks_a_previous_response ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/provenance_adversary.rs (target/debug/deps/provenance_adversary-fddccb4beaab5353)

running 1 test
test native_report_records_observed_completion_time ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.59s

     Running tests/semantics.rs (target/debug/deps/semantics-8b592e45badae6b0)

running 8 tests
test core_module_boundary_has_no_io_or_language_dispatch ... ok
test labels_and_input_order_do_not_choose_algorithms ... ok
test partial_is_never_complete ... ok
test semantic_invalid_classes ... ok
test dangling_target_guard ... ok
test diagnostic_phase_precedence_and_order ... ok
test parallel_imports_count_unique_destinations ... ok
test every_authored_literal_matches_real_evaluation ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s

```

## 4. Findings

Nothing found in this bounded second pass. The three previous findings did not reproduce against the corrected candidate.

## 5. Boundaries exercised

- src/bin/codegate-check.rs:43 excludes local ownership/build state only at the generated crate root. Two official regenerations compare equal; six generated product mutations still differ. Neither .ess-output ownership file remains tracked.
- src/bin/codegate-check.rs:77 creates a previously absent report directory, invokes the exact conformance producer with that destination, compares regenerated suite bytes and validates identity, counts, coverage, outcome inventory, suite digest and completion interval. Missing/ignored/renamed producer attacks refuse stale evidence; the positive control really runs the evaluator.
- tests/conformance.rs:15 uses ObservedClock only in the harness. The native observed-time regression passes; the pure evaluator was untouched. All27 real scenarios continue to pass.

This is bounded test evidence, not an assertion of exhaustive correctness. The coordinator still owns the complete integration gate.

## 6. Writes and handoff

No authored path outside the assigned worktree. Standard Cargo/sccache and own worktree lease metadata only. No source edits or cleanup. All processes exited0. Raw commands and statuses are in targeted.log/targeted.exit and suite.log/suite.exit; paired native report/suite retained as native-report.json/native-suite.json.

Existing maintained tests created these new disposable paths within the root target directory:

- target/freshness-regression-2009530
- target/freshness-regression-2022032

Generated comparator probes are beneath assigned adversary-2/generation-*; normal incremental compilation stays in root target. Coordinator owns cleanup. Own session lease released at handoff.

```findings
[]
```
