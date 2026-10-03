---
format: aep.planning-md/3
id: verification-report:semantic-wire-unit
kind: verification-report
status: draft
title: Semantic wire bridge unit evidence
relations:
- reviews: story:source-snapshot
revision: 1
---
unit: coordinator-owned semantic wire prerequisite
verdict: green
cases: targeted executed 4→4, red 3; final package executed 32 (pre-change package count not measured in this tree)
origin: n/a
wrote-outside-worktree: $BUILD (exact path in private handoff)
needs-coordinator: yes — integrate two product files and retain/drop local dependency wiring deliberately

## Delivered seam

Pure generated semantic behavior/wire conversion for all FactSnapshot transitive values and CollectionRequest/BuildSelection. Encoders use borrowed inputs: snapshot_value, request_value, source_value, configuration_value and gaps_value. Decoders enforce a 4 MiB byte bound, serde recursion bound, strict duplicate-key refusal, generated unknown-field/enum/required-field checks and exact signed-i64 conversion. Explicit optional null means absent only for fields the generated schema marks optional. No CLI handler, runtime collection, semantic admission, Decimal assessment or score contract is claimed.

## Owned files and setup

Product files: src/semantic_wire.rs and tests/semantic_wire.rs. Local setup only: two root Cargo.toml path dependencies (codegate-semantic-behavior and codegate-semantic-contract), their Cargo.lock additions, and src/lib.rs semantic_model export/semantic_wire module. No existing wire.rs or generated source was modified. A scratch-only Rust helper emitted explicit macro invocations from generated declarations, then these checked source invocations were reviewed; it is not a runtime dependency or committed harness.

## Red evidence

Command: CARGO_TARGET_DIR=$BUILD RUSTC_WRAPPER=/usr/bin/sccache CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 cargo test -p codegate-cli --test semantic_wire

The positive cases executed against explicit refusal stubs before implementing the bridge. Exit 101, 1 passed and 3 failed:

```text
     Locking 2 packages to latest Rust 1.98 compatible versions
      Adding codegate-semantic-contract v0.0.0 ($WORKTREE/generated/semantic-wire)
      Adding codegate_semantic v1.0.0 ($WORKTREE/generated/semantic-behavior)
   Compiling proc-macro2 v1.0.107
   Compiling quote v1.0.47
   Compiling unicode-ident v1.0.26
   Compiling serde_core v1.0.229
   Compiling syn v3.0.6
   Compiling serde v1.0.229
   Compiling zmij v1.0.23
   Compiling itoa v1.0.18
   Compiling serde_json v1.0.151
   Compiling memchr v2.8.3
   Compiling serde_derive v1.0.229
   Compiling typenum v1.20.1
   Compiling hybrid-array v0.4.15
   Compiling const-oid v0.10.2
   Compiling syn v2.0.119
   Compiling crypto-common v0.2.2
   Compiling block-buffer v0.12.1
   Compiling digest v0.11.3
   Compiling cfg-if v1.0.5
   Compiling cpufeatures v0.3.1
   Compiling utf8parse v0.2.2
   Compiling anstyle-parse v1.0.0
   Compiling sha2 v0.11.0
   Compiling serde_derive_internals v0.29.1
   Compiling anstyle-query v1.1.5
   Compiling equivalent v1.0.2
   Compiling thiserror v2.0.21
   Compiling colorchoice v1.0.5
   Compiling hashbrown v0.17.1
   Compiling schemars v0.8.22
   Compiling is_terminal_polyfill v1.70.2
   Compiling anstyle v1.0.14
   Compiling indexmap v2.14.2
   Compiling anstream v1.0.0
   Compiling schemars_derive v0.8.22
   Compiling thiserror-impl v2.0.21
   Compiling unsafe-libyaml v0.2.11
   Compiling heck v0.5.0
   Compiling ryu v1.0.23
   Compiling clap_lex v1.1.1
   Compiling strsim v0.11.1
   Compiling dyn-clone v1.0.20
   Compiling clap_builder v4.6.7
   Compiling serde_yaml v0.9.34+deprecated
   Compiling clap_derive v4.6.7
   Compiling ess-primitives v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
   Compiling clap v4.6.7
   Compiling codegate-contract v0.0.0 ($WORKTREE/generated/wire)
   Compiling codegate-semantic-contract v0.0.0 ($WORKTREE/generated/semantic-wire)
   Compiling codegate v1.0.0 ($WORKTREE/generated/behavior)
   Compiling pulldown-cmark v0.13.4
   Compiling codegate_semantic v1.0.0 ($WORKTREE/generated/semantic-behavior)
warning: unreachable expression
   --> generated/semantic-behavior/src/system.rs:102:13
    |
102 |             self.published.push(SystemEvent::from(event));
    |             ^^^^^^^^^^^^^^^^^^^^------------------------^
    |             |                   |
    |             |                   any code following this expression is unreachable
    |             unreachable expression
    |
note: this expression has type `SystemEvent`, which is uninhabited
   --> generated/semantic-behavior/src/system.rs:102:33
    |
102 |             self.published.push(SystemEvent::from(event));
    |                                 ^^^^^^^^^^^^^^^^^^^^^^^^
    = note: `#[warn(unreachable_code)]` (part of `#[warn(unused)]`) on by default

warning: `codegate_semantic` (lib) generated 1 warning
   Compiling ess-domain v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
   Compiling codegate-cli v0.1.0 ($WORKTREE)
   Compiling pulldown-cmark-escape v0.11.0
   Compiling unicase v2.9.0
   Compiling bitflags v2.13.2
   Compiling ess-compiler v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
   Compiling ess-gen v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
   Compiling ess-conformance v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1m 03s
     Running tests/semantic_wire.rs ($BUILD/debug/deps/semantic_wire-b89065942efafb95)

running 4 tests
test requests_and_configuration_have_explicit_generated_bridges ... FAILED
test every_snapshot_family_and_transitive_field_round_trips ... FAILED
test optional_null_is_absence_but_required_or_unknown_null_is_refused ... FAILED
test malformed_unknown_duplicate_enums_and_integer_overflow_are_refused ... ok

failures:

---- requests_and_configuration_have_explicit_generated_bridges stdout ----

thread 'requests_and_configuration_have_explicit_generated_bridges' (2421777) panicked at tests/semantic_wire.rs:48:78:
called `Result::unwrap()` on an `Err` value: "not implemented"
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- every_snapshot_family_and_transitive_field_round_trips stdout ----

thread 'every_snapshot_family_and_transitive_field_round_trips' (2421774) panicked at tests/semantic_wire.rs:33:80:
called `Result::unwrap()` on an `Err` value: "not implemented"

---- optional_null_is_absence_but_required_or_unknown_null_is_refused stdout ----

thread 'optional_null_is_absence_but_required_or_unknown_null_is_refused' (2421776) panicked at tests/semantic_wire.rs:62:80:
called `Result::unwrap()` on an `Err` value: "not implemented"


failures:
    every_snapshot_family_and_transitive_field_round_trips
    optional_null_is_absence_but_required_or_unknown_null_is_refused
    requests_and_configuration_have_explicit_generated_bridges

test result: FAILED. 1 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p codegate-cli --test semantic_wire`
```

A second adversarial probe added an object-shaped private serde Number representation at a required integer field. The first implementation accepted it; the retained number-probe.log records 3 passed, 1 failed, exit 101. The duplicate-key visitor now refuses this private representation before Value deserialization; native i64 integers remain supported. This closes object-to-number type confusion rather than merely adding a byte_length special case.

## Green evidence

Full package command: PATH=<pinned ESS 0.50.0>:$PATH CARGO_TARGET_DIR=$BUILD RUSTC_WRAPPER=/usr/bin/sccache CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 cargo test --locked -p codegate-cli

Exit 0, 32 executed across runner summaries. The new semantic_wire lane executes 4→4, failed 3→0. All pre-existing package lanes pass; the nested legacy conformance suite retains complete27 acceptance independently of package-test counts. No before-package count was executed in this particular tree, so no before/after total is invented.

```text
warning: unreachable expression
   --> generated/semantic-behavior/src/system.rs:102:13
    |
102 |             self.published.push(SystemEvent::from(event));
    |             ^^^^^^^^^^^^^^^^^^^^------------------------^
    |             |                   |
    |             |                   any code following this expression is unreachable
    |             unreachable expression
    |
note: this expression has type `SystemEvent`, which is uninhabited
   --> generated/semantic-behavior/src/system.rs:102:33
    |
102 |             self.published.push(SystemEvent::from(event));
    |                                 ^^^^^^^^^^^^^^^^^^^^^^^^
    = note: `#[warn(unreachable_code)]` (part of `#[warn(unused)]`) on by default

warning: `codegate_semantic` (lib) generated 1 warning
   Compiling codegate-cli v0.1.0 ($WORKTREE)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 5.59s
     Running unittests src/lib.rs ($BUILD/debug/deps/codegate-96ca00e4c0566668)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs ($BUILD/debug/deps/codegate-1eec01056742370f)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/codegate-check.rs ($BUILD/debug/deps/codegate_check-dca24a88f159e194)

running 1 test
test tests::gate_clean_external_target ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/codegate-docs.rs ($BUILD/debug/deps/codegate_docs-1c3adcec8bb6798d)

running 5 tests
test tests::invalid_commits_are_refused ... ok
test tests::authored_site_and_publication_identity_are_valid ... ok
test tests::broken_or_duplicate_anchors_are_refused ... ok
test tests::published_json_example_reaches_the_real_evaluator ... ok
test tests::wrong_routes_assets_and_internal_source_links_are_refused ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/boundary.rs ($BUILD/debug/deps/boundary-02748f8fa5544a91)

running 5 tests
test generated_integer_obligation_is_checked ... ok
test explicit_absence_bridge ... ok
test strict_json_structure ... ok
test all_generated_fields_roundtrip ... ok
test decoder_limits ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.11s

     Running tests/cli.rs ($BUILD/debug/deps/cli-455a2a0761724aaf)

running 3 tests
test unsupported_policy_format_is_semantic_error ... ok
test cli_structural_refusal_emits_no_report_or_output_file ... ok
test cli_reports_full_outcomes_and_coverage_aware_exits_without_tools ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s

     Running tests/conformance.rs ($BUILD/debug/deps/conformance-4c9c62dba1f4c730)

running 1 test
test real_evaluator_conforms_to_complete_combined_suite ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.58s

     Running tests/freshness.rs ($BUILD/debug/deps/freshness-cfa2b3659b3b8541)

running 1 test
test gate_requires_this_invocations_conformance_producer ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.59s

     Running tests/gate_adversary.rs ($BUILD/debug/deps/gate_adversary-05b05ed256ae9971)

running 1 test
test unchanged_generated_contract_ignores_local_ownership_state ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 10.05s

     Running tests/obligation.rs ($BUILD/debug/deps/obligation-d4041a5ae2b3220c)

running 2 tests
test evaluate_obligation_is_fulfilled ... ok
test trait_adapter_never_leaks_a_previous_response ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/provenance_adversary.rs ($BUILD/debug/deps/provenance_adversary-18bae59556722bd4)

running 1 test
test native_report_records_observed_completion_time ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.43s

     Running tests/semantic_wire.rs ($BUILD/debug/deps/semantic_wire-b89065942efafb95)

running 4 tests
test requests_and_configuration_have_explicit_generated_bridges ... ok
test every_snapshot_family_and_transitive_field_round_trips ... ok
test optional_null_is_absence_but_required_or_unknown_null_is_refused ... ok
test malformed_unknown_duplicate_enums_and_integer_overflow_are_refused ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running tests/semantics.rs ($BUILD/debug/deps/semantics-91a15d575abb5ad8)

running 8 tests
test core_module_boundary_has_no_io_or_language_dispatch ... ok
test dangling_target_guard ... ok
test parallel_imports_count_unique_destinations ... ok
test partial_is_never_complete ... ok
test labels_and_input_order_do_not_choose_algorithms ... ok
test diagnostic_phase_precedence_and_order ... ok
test semantic_invalid_classes ... ok
test every_authored_literal_matches_real_evaluation ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

   Doc-tests codegate

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

cargo fmt --check: exit 0.
CARGO_TARGET_DIR=$BUILD RUSTC_WRAPPER=/usr/bin/sccache CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 cargo clippy --locked -p codegate-cli --all-targets -- -D warnings: exit 0.
git diff --check: exit 0.

Generated semantic behavior carries a pre-existing unreachable-expression warning in generated/semantic-behavior/src/system.rs:102. This unit did not edit generated bytes or suppress the warning. Root package Clippy passes.

## Limits and retained paths

Structural bridge output still requires semantic admission. The all-fields fixture intentionally contains referentially invalid combinations to establish lossless wire transport, not accepted semantics. Pure schema normalization uses an embedded checked-in generated schema; no runtime filesystem access occurs. No full task check, commits, AEP mutations or public command completion is claimed.

Raw logs: $WORKTREE/.scratch/unit/red.log, number-probe.log, green-targeted.log, green-suite.log, fmt.log and clippy.log. Scratch generator and its intermediate fragments remain under $WORKTREE/.scratch/unit. External build and linked test output remain under $BUILD. Existing tests created ignored repository-local target evidence. Cargo and sccache use standard managed caches; no other manual external file writes. All outputs remain for coordinator cleanup.
