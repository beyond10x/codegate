---
format: aep.planning-md/3
id: verification-report:gate-scratch-unit
kind: verification-report
status: draft
title: Gate scratch repair unit evidence
relations:
- reviews: task:clean-external-target-gate
revision: 2
---
unit: task:clean-external-target-gate — clean checkout gate scratch allocation
verdict: green
cases: executed 27→28, red 1
origin: n/a
wrote-outside-worktree: $BUILD (exact path retained in private brief)
needs-coordinator: yes — run the clean-checkout full integration gate and independent adversary

## Unit and acceptance

Create the missing repository-local target parent with an external Cargo build target, while retaining exclusive creation of the per-invocation gate evidence directory. The private production allocation helper is exercised with a missing target parent, an existing target parent, an already-populated invocation directory and a nondirectory target path. All previous conformance freshness checks remain unchanged. Full task-check acceptance is reserved for coordinator integration; this report claims package-level verification only.

## Observed diff and confirmed scope

Only src/bin/codegate-check.rs changed: 45 insertions and 4 deletions. The private helper contains the existing allocation logic plus parent creation. Its regression lives beside it in the same assigned source file; tests/gate_adversary.rs and tests/freshness.rs were inspected and run unchanged. No manifests, AEP records, collectors or other owned source files changed. No commits were made.

## Red run

Before the fix, the existing allocation code was extracted unchanged into create_scratch and the new regression was executed. This was a runtime failure, not a compiler failure.

Command: CARGO_TARGET_DIR=$BUILD RUSTC_WRAPPER=/usr/bin/sccache CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 cargo test --locked -p codegate-cli --bin codegate-check gate_clean_external_target -- --nocapture

Exit: 101

```text
   Compiling codegate-cli v0.1.0 ($WORKTREE)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.20s
     Running unittests src/bin/codegate-check.rs ($BUILD/debug/deps/codegate_check-1672523ddec981ba)

running 1 test

thread 'tests::gate_clean_external_target' (1917949) panicked at src/bin/codegate-check.rs:469:49:
gate must create missing target parent: "No such file or directory (os error 2)"
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test tests::gate_clean_external_target ... FAILED

failures:

failures:
    tests::gate_clean_external_target

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `-p codegate-cli --bin codegate-check`
```

## Green runs

Same targeted command: executed 1→1, failed 1→0, exit 0. This lane existed as zero cases on the unmodified base and gained the one regression before its red run.

Full package command: PATH=<pinned ESS 0.50.0>:$PATH CARGO_TARGET_DIR=$BUILD RUSTC_WRAPPER=/usr/bin/sccache CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 cargo test --locked -p codegate-cli

Full package: executed 27→28, exit 0→0. The codegate-check binary unit lane increased 0→1. Other lanes retained their runner-observed counts: docs 5, boundary 5, CLI 3, conformance 1, freshness 1, gate_adversary 1, obligation 2, provenance_adversary 1, semantics 8; library/main/doctest lanes 0. The conformance integration test retains its existing complete27 native suite; package-test counts are separate.

```text
   Compiling codegate-cli v0.1.0 ($WORKTREE)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.30s
     Running unittests src/lib.rs ($BUILD/debug/deps/codegate-bc5058d0cafcdb3f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs ($BUILD/debug/deps/codegate-75714fe6a54523da)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/codegate-check.rs ($BUILD/debug/deps/codegate_check-1672523ddec981ba)

running 1 test
test tests::gate_clean_external_target ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/codegate-docs.rs ($BUILD/debug/deps/codegate_docs-a2d5187a56b274a2)

running 5 tests
test tests::invalid_commits_are_refused ... ok
test tests::authored_site_and_publication_identity_are_valid ... ok
test tests::broken_or_duplicate_anchors_are_refused ... ok
test tests::published_json_example_reaches_the_real_evaluator ... ok
test tests::wrong_routes_assets_and_internal_source_links_are_refused ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/boundary.rs ($BUILD/debug/deps/boundary-0aa2f2c6f25cf5af)

running 5 tests
test generated_integer_obligation_is_checked ... ok
test explicit_absence_bridge ... ok
test strict_json_structure ... ok
test all_generated_fields_roundtrip ... ok
test decoder_limits ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.13s

     Running tests/cli.rs ($BUILD/debug/deps/cli-32c0cb8e9b324d68)

running 3 tests
test cli_structural_refusal_emits_no_report_or_output_file ... ok
test unsupported_policy_format_is_semantic_error ... ok
test cli_reports_full_outcomes_and_coverage_aware_exits_without_tools ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/conformance.rs ($BUILD/debug/deps/conformance-4997597ea84f1696)

running 1 test
test real_evaluator_conforms_to_complete_combined_suite ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s

     Running tests/freshness.rs ($BUILD/debug/deps/freshness-66c44dada4ac6cb0)

running 1 test
test gate_requires_this_invocations_conformance_producer ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.05s

     Running tests/gate_adversary.rs ($BUILD/debug/deps/gate_adversary-68a31bf8ee7c1005)

running 1 test
test unchanged_generated_contract_ignores_local_ownership_state ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.66s

     Running tests/obligation.rs ($BUILD/debug/deps/obligation-1610ec9160fff947)

running 2 tests
test evaluate_obligation_is_fulfilled ... ok
test trait_adapter_never_leaks_a_previous_response ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/provenance_adversary.rs ($BUILD/debug/deps/provenance_adversary-2a0c181e972cead0)

running 1 test
test native_report_records_observed_completion_time ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.24s

     Running tests/semantics.rs ($BUILD/debug/deps/semantics-e95901fafadef02e)

running 8 tests
test core_module_boundary_has_no_io_or_language_dispatch ... ok
test parallel_imports_count_unique_destinations ... ok
test dangling_target_guard ... ok
test partial_is_never_complete ... ok
test diagnostic_phase_precedence_and_order ... ok
test labels_and_input_order_do_not_choose_algorithms ... ok
test semantic_invalid_classes ... ok
test every_authored_literal_matches_real_evaluation ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

   Doc-tests codegate

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

cargo fmt --check: exit 0.

CARGO_TARGET_DIR=$BUILD RUSTC_WRAPPER=/usr/bin/sccache CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 cargo clippy --locked -p codegate-cli --all-targets -- -D warnings: exit 0.

## Deliberate limits

No recursive task-check invocation was added to cargo tests. Coordinator must run the complete gate from a fresh checkout without local target output. No expectation, filter, conformance producer, report binding or generated comparator was weakened. The class fixed is missing parent directories under external Cargo output; ordinary existing-parent operation, occupied invocation paths and nondirectory parents are explicitly witnessed. Symlink policy is unchanged.

## Retained output and handoff

All raw run logs are under $WORKTREE/.scratch/unit/: baseline.log, red.log, green-targeted.log, green-suite.log, fmt.log, clippy.log. Repository-local target output from the preexisting regression suite is retained for coordinator cleanup. External compiler output and the new fixture scratch directories are under $BUILD from the private brief; no other manual external file writes. Cargo and installed sccache manage their standard caches. No build directory or worktree was removed. Coordinator owns adversary, integration, publication and cleanup.

## Full integration witness

At integration 69095e2, verified repository-local target did not exist, then ran task check with external Cargo target. Exit 0. Native ESS report: executed27 passed27 failed0 error0 unsupported0 skipped0. AEP, both ESS roots, all four generated drift comparisons, package tests, formatting, Clippy and public documentation checks exited 0. Fresh suite/report/run are retained under the private gate invocation evidence; no conventional stale report was admitted. This closes task:clean-external-target-gate only.
