---
format: aep.planning-md/3
id: verification-report:codegate-wave1
kind: verification-report
status: draft
title: 'Offline dependency evaluator: executed gate and conformance evidence'
relations:
- verifies: story:offline-dependency-slice
- verifies: executable-system-specification:dependency-evaluation
revision: 1
---
# Wave 001 verification evidence

Candidate: `4bf8fa591888698fdf23194d93daebb81bbb4e18`; integration branch `wave/001-offline-dependency`.
Whole gate: `RUSTC_WRAPPER=/usr/bin/sccache CARGO_BUILD_JOBS=2 task check`, observed exit 0. All printed gate steps exited 0. Root package executed 22 Rust tests; native ESS executed 27 scenarios, all passed, zero failed/errored/unsupported/skipped. Generated packages compile and lint; their zero-test summaries are not counted as extra behavioral coverage.

The first adversary pass found three gate/evidence defects; the second pass found none after correction. Immutable reports are `review-result:codegate-wave1-adversary-pass1` and `review-result:codegate-wave1-adversary-pass2`. Store comparison returned carried0/new0/resolved3. The direct evaluator CLI claim was independently compared against a literal authored fixture, recorded on the wave page.

Native report and exact suite retained under coordinator recovery archive `.scratch/wave-001/final-native/`; unit raw logs/probe sources retained under unit recovery archive `.scratch/wave-001/offline-dependency/`. Archives are at `$HOME/.local/state/worktree/archives/codegate/<managed-id>/`. Logs below are copied verbatim from their producers; implementation narrative is an agent report, not independent program evidence. Host did not expose token/tool-use totals or per-agent wall duration; those costs are unavailable.

## Native integration report

```json
{
  "completed_at": 1790945576970,
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

```

## Complete integration gate output

```text
task: [check] cargo run --locked --bin codegate-check
   Compiling proc-macro2 v1.0.107
   Compiling quote v1.0.47
   Compiling unicode-ident v1.0.26
   Compiling typenum v1.20.1
   Compiling serde_core v1.0.229
   Compiling syn v3.0.6
   Compiling hybrid-array v0.4.15
   Compiling utf8parse v0.2.2
   Compiling zmij v1.0.23
   Compiling anstyle-parse v1.0.0
   Compiling serde_json v1.0.151
   Compiling colorchoice v1.0.5
   Compiling serde v1.0.229
   Compiling is_terminal_polyfill v1.70.2
   Compiling anstyle-query v1.1.5
   Compiling anstyle v1.0.14
   Compiling anstream v1.0.0
   Compiling serde_derive v1.0.229
   Compiling crypto-common v0.2.2
   Compiling block-buffer v0.12.1
   Compiling memchr v2.8.3
   Compiling clap_lex v1.1.1
   Compiling strsim v0.11.1
   Compiling itoa v1.0.18
   Compiling heck v0.5.0
   Compiling const-oid v0.10.2
   Compiling digest v0.11.3
   Compiling clap_derive v4.6.7
   Compiling clap_builder v4.6.7
   Compiling cpufeatures v0.3.1
   Compiling cfg-if v1.0.5
   Compiling sha2 v0.11.0
   Compiling clap v4.6.7
   Compiling codegate v1.0.0 ($HOME/.local/state/worktree/trees/b10x/codegate/codegate-wave1-plan-20261002/generated/behavior)
   Compiling codegate-contract v0.0.0 ($HOME/.local/state/worktree/trees/b10x/codegate/codegate-wave1-plan-20261002/generated/wire)
   Compiling codegate-cli v0.1.0 ($HOME/.local/state/worktree/trees/b10x/codegate/codegate-wave1-plan-20261002)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 19.40s
     Running `target/debug/codegate-check`
CHECK AEP: aep plan artifact validate
9 file(s) in $HOME/.local/state/worktree/trees/b10x/codegate/codegate-wave1-plan-20261002/.engineering/planning: 9 artifact(s)
valid
CHECK AEP: exit 0
CHECK ESS: ess specify validate --path ess
codegate v1 — 2 file(s), 26 scenario(s), valid
CHECK ESS: exit 0
CHECK generate behavior: ess generate synthesize --path ess --target rust --layout crate --out $HOME/.local/state/worktree/trees/b10x/codegate/codegate-wave1-plan-20261002/target/codegate-check-2083868/behavior
18 capabilities: 17 generated, 1 obligation(s), 0 refused
7 artifact(s), written to $HOME/.local/state/worktree/trees/b10x/codegate/codegate-wave1-plan-20261002/target/codegate-check-2083868/behavior
CHECK generate behavior: exit 0
CHECK generate wire: ess generate types --path ess --target rust --all-types --package codegate-contract --out $HOME/.local/state/worktree/trees/b10x/codegate/codegate-wave1-plan-20261002/target/codegate-check-2083868/wire
16 model type(s), written to $HOME/.local/state/worktree/trees/b10x/codegate/codegate-wave1-plan-20261002/target/codegate-check-2083868/wire; runtime obligations in types-report.json
CHECK generate wire: exit 0
CHECK normalize generated formatting: cargo fmt --manifest-path $HOME/.local/state/worktree/trees/b10x/codegate/codegate-wave1-plan-20261002/target/codegate-check-2083868/behavior/Cargo.toml
CHECK normalize generated formatting: exit 0
CHECK normalize generated formatting: cargo fmt --manifest-path $HOME/.local/state/worktree/trees/b10x/codegate/codegate-wave1-plan-20261002/target/codegate-check-2083868/wire/Cargo.toml
CHECK normalize generated formatting: exit 0
CHECK generated drift: exit 0
CHECK combined suite: ess verify conform synthesize --path ess --scenarios ess --suite-format 5 --out $HOME/.local/state/worktree/trees/b10x/codegate/codegate-wave1-plan-20261002/target/codegate-check-2083868/suite.json
27 selected scenario(s), 26 authored source(s), 0 refusal occurrence(s)
CHECK combined suite: exit 0
CHECK real ESS conformance: cargo test --locked -p codegate-cli --test conformance real_evaluator_conforms_to_complete_combined_suite -- --exact --nocapture
   Compiling serde_json v1.0.151
   Compiling memchr v2.8.3
   Compiling syn v2.0.119
   Compiling serde_derive_internals v0.29.1
   Compiling schemars v0.8.22
   Compiling thiserror v2.0.21
   Compiling equivalent v1.0.2
   Compiling hashbrown v0.17.1
   Compiling indexmap v2.14.2
   Compiling schemars_derive v0.8.22
   Compiling thiserror-impl v2.0.21
   Compiling unsafe-libyaml v0.2.11
   Compiling dyn-clone v1.0.20
   Compiling ryu v1.0.23
   Compiling serde_yaml v0.9.34+deprecated
   Compiling pulldown-cmark v0.13.4
   Compiling ess-primitives v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
   Compiling codegate-contract v0.0.0 ($HOME/.local/state/worktree/trees/b10x/codegate/codegate-wave1-plan-20261002/generated/wire)
   Compiling unicase v2.9.0
   Compiling pulldown-cmark-escape v0.11.0
   Compiling bitflags v2.13.2
   Compiling codegate-cli v0.1.0 ($HOME/.local/state/worktree/trees/b10x/codegate/codegate-wave1-plan-20261002)
   Compiling ess-domain v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
   Compiling ess-compiler v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
   Compiling ess-gen v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
   Compiling ess-conformance v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2m 19s
     Running tests/conformance.rs (target/debug/deps/conformance-c7a9fb2bcbc4979a)

running 1 test
{
  "completed_at": 1790945576970,
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

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.17s

CHECK real ESS conformance: exit Some(0)
CHECK native ESS report: executed27 passed27 failed0 error0 unsupported0 skipped0, exit0; evidence $HOME/.local/state/worktree/trees/b10x/codegate/codegate-wave1-plan-20261002/target/codegate-check-2083868/conformance
CHECK Rust tests: cargo test --locked -p codegate-cli --all-targets -- --nocapture
    Blocking waiting for file lock on package cache
   Compiling codegate-cli v0.1.0 ($HOME/.local/state/worktree/trees/b10x/codegate/codegate-wave1-plan-20261002)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 17.27s
     Running unittests src/lib.rs (target/debug/deps/codegate-4656fda71df7915a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/codegate-d35365d64892493d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/codegate-check.rs (target/debug/deps/codegate_check-1fe97699a372e6fd)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/boundary.rs (target/debug/deps/boundary-9bdf51d047dbc681)

running 5 tests
test generated_integer_obligation_is_checked ... ok
test explicit_absence_bridge ... ok
test strict_json_structure ... ok
test all_generated_fields_roundtrip ... ok
test decoder_limits ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.21s

     Running tests/cli.rs (target/debug/deps/cli-fac19e6bfdfe86f0)

running 3 tests
test cli_structural_refusal_emits_no_report_or_output_file ... ok
test unsupported_policy_format_is_semantic_error ... ok
test cli_reports_full_outcomes_and_coverage_aware_exits_without_tools ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running tests/conformance.rs (target/debug/deps/conformance-c7a9fb2bcbc4979a)

running 1 test
{
  "completed_at": 1790945595377,
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

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.39s

     Running tests/freshness.rs (target/debug/deps/freshness-30aa3b1d3fcb79c8)

running 1 test
test gate_requires_this_invocations_conformance_producer ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.71s

     Running tests/gate_adversary.rs (target/debug/deps/gate_adversary-5c075ac973e2fdad)

running 1 test
test unchanged_generated_contract_ignores_local_ownership_state ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 5.73s

     Running tests/obligation.rs (target/debug/deps/obligation-0a26cb3fdf8672a5)

running 2 tests
test evaluate_obligation_is_fulfilled ... ok
test trait_adapter_never_leaks_a_previous_response ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/provenance_adversary.rs (target/debug/deps/provenance_adversary-fddccb4beaab5353)

running 1 test
test native_report_records_observed_completion_time ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.59s

     Running tests/semantics.rs (target/debug/deps/semantics-8b592e45badae6b0)

running 8 tests
test core_module_boundary_has_no_io_or_language_dispatch ... ok
test dangling_target_guard ... ok
test partial_is_never_complete ... ok
test diagnostic_phase_precedence_and_order ... ok
test labels_and_input_order_do_not_choose_algorithms ... ok
test parallel_imports_count_unique_destinations ... ok
test semantic_invalid_classes ... ok
test every_authored_literal_matches_real_evaluation ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

CHECK Rust tests: exit 0
CHECK format root: cargo fmt --package codegate-cli --check
CHECK format root: exit 0
CHECK Clippy root: cargo clippy --locked -p codegate-cli --all-targets -- -D warnings
    Checking serde_core v1.0.229
    Checking memchr v2.8.3
    Checking itoa v1.0.18
    Checking zmij v1.0.23
    Checking typenum v1.20.1
    Checking hybrid-array v0.4.15
    Checking crypto-common v0.2.2
    Checking block-buffer v0.12.1
    Checking const-oid v0.10.2
    Checking digest v0.11.3
    Checking utf8parse v0.2.2
    Checking hashbrown v0.17.1
    Checking serde v1.0.229
    Checking serde_json v1.0.151
    Checking cpufeatures v0.3.1
    Checking equivalent v1.0.2
    Checking cfg-if v1.0.5
    Checking sha2 v0.11.0
    Checking indexmap v2.14.2
    Checking anstyle-parse v1.0.0
    Checking unsafe-libyaml v0.2.11
    Checking is_terminal_polyfill v1.70.2
    Checking anstyle-query v1.1.5
    Checking ryu v1.0.23
    Checking dyn-clone v1.0.20
    Checking anstyle v1.0.14
    Checking colorchoice v1.0.5
    Checking serde_yaml v0.9.34+deprecated
    Checking anstream v1.0.0
    Checking schemars v0.8.22
    Checking thiserror v2.0.21
    Checking strsim v0.11.1
    Checking clap_lex v1.1.1
    Checking ess-primitives v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
    Checking clap_builder v4.6.7
    Checking clap v4.6.7
    Checking codegate-contract v0.0.0 ($HOME/.local/state/worktree/trees/b10x/codegate/codegate-wave1-plan-20261002/generated/wire)
    Checking ess-domain v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
    Checking codegate v1.0.0 ($HOME/.local/state/worktree/trees/b10x/codegate/codegate-wave1-plan-20261002/generated/behavior)
    Checking unicase v2.9.0
    Checking bitflags v2.13.2
    Checking pulldown-cmark-escape v0.11.0
    Checking pulldown-cmark v0.13.4
    Checking codegate-cli v0.1.0 ($HOME/.local/state/worktree/trees/b10x/codegate/codegate-wave1-plan-20261002)
    Checking ess-compiler v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
    Checking ess-gen v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
    Checking ess-conformance v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 42.11s
CHECK Clippy root: exit 0
CHECK test generated: cargo test --offline --manifest-path generated/behavior/Cargo.toml
   Compiling codegate v1.0.0 ($HOME/.local/state/worktree/trees/b10x/codegate/codegate-wave1-plan-20261002/generated/behavior)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.01s
     Running unittests src/lib.rs (generated/behavior/target/debug/deps/codegate-0bbb6a616bf3d2e9)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests codegate

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

CHECK test generated: exit 0
CHECK format generated: cargo fmt --manifest-path generated/behavior/Cargo.toml --check
CHECK format generated: exit 0
CHECK Clippy generated: cargo clippy --offline --manifest-path generated/behavior/Cargo.toml --all-targets -- -D warnings
    Checking codegate v1.0.0 ($HOME/.local/state/worktree/trees/b10x/codegate/codegate-wave1-plan-20261002/generated/behavior)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.68s
CHECK Clippy generated: exit 0
CHECK test generated: cargo test --offline --manifest-path generated/wire/Cargo.toml
     Locking 11 packages to latest compatible versions
   Compiling proc-macro2 v1.0.107
   Compiling quote v1.0.47
   Compiling unicode-ident v1.0.26
   Compiling serde_core v1.0.229
   Compiling zmij v1.0.23
   Compiling syn v3.0.6
   Compiling serde_json v1.0.151
   Compiling serde v1.0.229
   Compiling serde_derive v1.0.229
   Compiling memchr v2.8.3
   Compiling itoa v1.0.18
   Compiling codegate-contract v0.0.0 ($HOME/.local/state/worktree/trees/b10x/codegate/codegate-wave1-plan-20261002/generated/wire)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 12.93s
     Running unittests types.rs (generated/wire/target/debug/deps/codegate_contract-5e9fc53dc7373ec6)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests codegate_contract

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

CHECK test generated: exit 0
CHECK format generated: cargo fmt --manifest-path generated/wire/Cargo.toml --check
CHECK format generated: exit 0
CHECK Clippy generated: cargo clippy --offline --manifest-path generated/wire/Cargo.toml --all-targets -- -D warnings -A clippy::derivable_impls
    Checking serde_core v1.0.229
    Checking zmij v1.0.23
    Checking memchr v2.8.3
    Checking itoa v1.0.18
    Checking serde v1.0.229
    Checking serde_json v1.0.151
    Checking codegate-contract v0.0.0 ($HOME/.local/state/worktree/trees/b10x/codegate/codegate-wave1-plan-20261002/generated/wire)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.47s
CHECK Clippy generated: exit 0

```

## Initial implementor report, verbatim

unit:                   story:offline-dependency-slice — offline dependency evaluator
verdict:                green
cases:                  executed unavailable on base → 19 Rust tests; 27 ESS scenarios, red 0
origin:                 n/a
wrote-outside-worktree: tool-managed caches/lease only (listed below); no authored files
needs-coordinator:      no

## 1. Unit and acceptance

Implement all27 combined ESS cases against the actual Rust evaluator and offline CLI,
with strict boundaries, shared checker independence, exact coverage-aware results,
and generated contract/gate integration. Source models are generated, not copied.

Scope inspected: all assigned surfaces were absent/new except README/AGENTS/.gitignore;
no inferred existing mechanism was adopted. Shared-crate ownership inference confirmed.
No depends_on edge was present. No ess/ or .engineering/ mutation occurred.

## 2. Actual diff and untracked surfaces

The tracked diff stat excludes the deliberately untracked new implementation files;
the complete status immediately below lists them. No staging or commits were done.
 .gitignore |  2 ++
 AGENTS.md  | 62 ++++++++++++++++++++++++-------------------
 README.md  | 89 ++++++++++++++++++++++++++++++++++++++++++++------------------
 3 files changed, 101 insertions(+), 52 deletions(-)
 M .gitignore
 M AGENTS.md
 M README.md
?? Cargo.lock
?? Cargo.toml
?? Taskfile.yml
?? generated/
?? rust-toolchain.toml
?? src/
?? tests/

## 3. Red first (verbatim)

The exact base8e4ead994e1ecadedabb05b60098706078cc2504 has no Cargo.toml, src/ or tests/:
`git ls-tree -r --name-only <base> Cargo.toml src tests` returned empty. There is no
base executable runner summary and no fabricated baseline count. An initial dependency
name typo was corrected before the behavioral red below; that compile/setup error is
not counted as red evidence.

Command: RUSTC_WRAPPER=/usr/bin/sccache CARGO_BUILD_JOBS=2 cargo test -p codegate-cli --test obligation
Exit:101. Generated Unimplemented was reached by a compiling executable adapter.
    Updating crates.io index
    Updating git repository `https://github.com/beyond10x/ess`
     Locking 62 packages to latest Rust 1.98.1 compatible versions
   Compiling proc-macro2 v1.0.107
   Compiling unicode-ident v1.0.26
   Compiling quote v1.0.47
   Compiling serde_core v1.0.229
   Compiling syn v3.0.6
   Compiling zmij v1.0.23
   Compiling serde v1.0.229
   Compiling itoa v1.0.18
   Compiling serde_json v1.0.151
   Compiling memchr v2.8.3
   Compiling serde_derive v1.0.229
   Compiling syn v2.0.119
   Compiling typenum v1.20.1
   Compiling hybrid-array v0.4.15
   Compiling serde_derive_internals v0.29.1
   Compiling equivalent v1.0.2
   Compiling hashbrown v0.17.1
   Compiling thiserror v2.0.21
   Compiling schemars v0.8.22
   Compiling indexmap v2.14.2
   Compiling schemars_derive v0.8.22
   Compiling thiserror-impl v2.0.21
   Compiling ryu v1.0.23
   Compiling utf8parse v0.2.2
   Compiling unsafe-libyaml v0.2.11
   Compiling dyn-clone v1.0.20
   Compiling serde_yaml v0.9.34+deprecated
   Compiling anstyle-parse v1.0.0
   Compiling block-buffer v0.12.1
   Compiling crypto-common v0.2.2
   Compiling const-oid v0.10.2
   Compiling anstyle v1.0.14
   Compiling anstyle-query v1.1.5
   Compiling is_terminal_polyfill v1.70.2
   Compiling colorchoice v1.0.5
   Compiling digest v0.11.3
   Compiling anstream v1.0.0
   Compiling ess-primitives v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
   Compiling heck v0.5.0
   Compiling pulldown-cmark v0.13.4
   Compiling cfg-if v1.0.5
   Compiling strsim v0.11.1
   Compiling clap_lex v1.1.1
   Compiling cpufeatures v0.3.1
   Compiling sha2 v0.11.0
   Compiling clap_builder v4.6.7
   Compiling clap_derive v4.6.7
   Compiling ess-domain v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
   Compiling unicase v2.9.0
   Compiling bitflags v2.13.2
   Compiling pulldown-cmark-escape v0.11.0
   Compiling clap v4.6.7
   Compiling codegate-contract v0.0.0 ($HOME/.local/state/worktree/trees/b10x/codegate/codegate-w1-offline-dependency-20261002/generated/wire)
   Compiling codegate v1.0.0 ($HOME/.local/state/worktree/trees/b10x/codegate/codegate-w1-offline-dependency-20261002/generated/behavior)
   Compiling codegate-cli v0.1.0 ($HOME/.local/state/worktree/trees/b10x/codegate/codegate-w1-offline-dependency-20261002)
   Compiling ess-compiler v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
   Compiling ess-gen v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
   Compiling ess-conformance v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1m 17s
     Running tests/obligation.rs (target/debug/deps/obligation-371e49fa5cd89f1a)

running 1 test
test evaluate_obligation_is_fulfilled ... FAILED

failures:

---- evaluate_obligation_is_fulfilled stdout ----

thread 'evaluate_obligation_is_fulfilled' (1038573) panicked at tests/obligation.rs:9:5:
assertion `left == right` failed
  left: Err(UnmetObligation { capability: "command behaviour", source: "codegate.dependency.Evaluate" })
 right: Ok(Evaluated)
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    evaluate_obligation_is_fulfilled

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `-p codegate-cli --test obligation`

## 4. Green evidence and lanes

Command: RUSTC_WRAPPER=/usr/bin/sccache CARGO_BUILD_JOBS=2 cargo test --locked -p codegate-cli --all-targets -- --nocapture
Exit:0. Runner summaries below count19 Rust tests across five integration lanes.
Base: no executable lane; actual first red obligation lane executed1 (0passed1failed).

- obligation: executed1 red →2 green, exit0 (added repeat-call response isolation).
- boundary: base unavailable →5, exit0.
- CLI: base unavailable →3, exit0.
- semantics: base unavailable →8, exit0 (one checks all26 authored literals).
- native conformance Rust wrapper: base unavailable →1, exit0.
- native ESS: base unavailable →27, passed27 failed0 error0 unsupported0 skipped0.
- root unit targets:0 tests, expected; acceptance is exercised by integration tests.

All counts are taken from the runner summaries, not source annotations.
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
    Blocking waiting for file lock on package cache
   Compiling codegate-cli v0.1.0 ($HOME/.local/state/worktree/trees/b10x/codegate/codegate-w1-offline-dependency-20261002)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.79s
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

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.18s

     Running tests/cli.rs (target/debug/deps/cli-2a3b151a142e54f2)

running 3 tests
test cli_structural_refusal_emits_no_report_or_output_file ... ok
test unsupported_policy_format_is_semantic_error ... ok
test cli_reports_full_outcomes_and_coverage_aware_exits_without_tools ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

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

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.29s

     Running tests/obligation.rs (target/debug/deps/obligation-653688523cdb0bfd)

running 2 tests
test evaluate_obligation_is_fulfilled ... ok
test trait_adapter_never_leaks_a_previous_response ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/semantics.rs (target/debug/deps/semantics-4bcb7134b24f2c61)

running 8 tests
test core_module_boundary_has_no_io_or_language_dispatch ... ok
test dangling_target_guard ... ok
test partial_is_never_complete ... ok
test parallel_imports_count_unique_destinations ... ok
test labels_and_input_order_do_not_choose_algorithms ... ok
test diagnostic_phase_precedence_and_order ... ok
test semantic_invalid_classes ... ok
test every_authored_literal_matches_real_evaluation ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


Command: cargo fmt --package codegate-cli --check
Exit:0 (no output).
Command: RUSTC_WRAPPER=/usr/bin/sccache CARGO_BUILD_JOBS=2 cargo clippy --locked -p codegate-cli --all-targets -- -D warnings
Exit:0.
    Checking codegate-cli v0.1.0 ($HOME/.local/state/worktree/trees/b10x/codegate/codegate-w1-offline-dependency-20261002)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.53s

Generated commands (each run sequentially, independent target directories):
cargo test --offline --manifest-path generated/behavior/Cargo.toml
cargo fmt --manifest-path generated/behavior/Cargo.toml --check
cargo clippy --offline --manifest-path generated/behavior/Cargo.toml --all-targets -- -D warnings
All exit0; generated unit/doc lanes0 tests (build/interface validation, not conformance).
   Compiling codegate v1.0.0 ($HOME/.local/state/worktree/trees/b10x/codegate/codegate-w1-offline-dependency-20261002/generated/behavior)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.40s
     Running unittests src/lib.rs (generated/behavior/target/debug/deps/codegate-0bbb6a616bf3d2e9)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests codegate

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

    Checking codegate v1.0.0 ($HOME/.local/state/worktree/trees/b10x/codegate/codegate-w1-offline-dependency-20261002/generated/behavior)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s

cargo test --offline --manifest-path generated/wire/Cargo.toml
cargo fmt --manifest-path generated/wire/Cargo.toml --check
Both exit0; generated unit/doc lanes0 tests.
     Locking 11 packages to latest compatible versions
   Compiling proc-macro2 v1.0.107
   Compiling unicode-ident v1.0.26
   Compiling quote v1.0.47
   Compiling serde_core v1.0.229
   Compiling zmij v1.0.23
   Compiling syn v3.0.6
   Compiling serde v1.0.229
   Compiling serde_json v1.0.151
   Compiling serde_derive v1.0.229
   Compiling itoa v1.0.18
   Compiling memchr v2.8.3
   Compiling codegate-contract v0.0.0 ($HOME/.local/state/worktree/trees/b10x/codegate/codegate-w1-offline-dependency-20261002/generated/wire)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 6.61s
     Running unittests types.rs (generated/wire/target/debug/deps/codegate_contract-5e9fc53dc7373ec6)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests codegate_contract

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s


Initial upstream-only generated wire lint failed (preserved):
    Blocking waiting for file lock on package cache
    Checking serde_core v1.0.229
    Checking zmij v1.0.23
    Checking itoa v1.0.18
    Checking memchr v2.8.3
    Checking serde_json v1.0.151
    Checking serde v1.0.229
    Checking codegate-contract v0.0.0 ($HOME/.local/state/worktree/trees/b10x/codegate/codegate-w1-offline-dependency-20261002/generated/wire)
error: this `impl` can be derived
  --> types.rs:13:1
   |
13 | / impl<T> Default for EssPresence<T> {
14 | |     fn default() -> Self {
15 | |         Self::Absent
16 | |     }
17 | | }
   | |_^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/rust-1.98.0/index.html#derivable_impls
   = note: `-D clippy::derivable-impls` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::derivable_impls)]`
help: replace the manual implementation with a derive attribute and mark the default variant
   |
 8 + #[derive(Default)]
 9 | pub enum EssPresence<T> {
10 ~     #[default]
11 ~     Absent,
   |

error: could not compile `codegate-contract` (lib test) due to 1 previous error
warning: build failed, waiting for other jobs to finish...
error: could not compile `codegate-contract` (lib) due to 1 previous error
Coordinator authorized the narrow generated-wire style allowance; source unchanged.
The ESS0.50 generated types.rs:13 manual Default is equivalent to a derived Default.
cargo clippy --offline --manifest-path generated/wire/Cargo.toml --all-targets -- -D warnings -A clippy::derivable_impls
Exit0. Root/behavior lint unchanged; drift remains byte-identical after pinned rustfmt.
    Checking codegate-contract v0.0.0 ($HOME/.local/state/worktree/trees/b10x/codegate/codegate-w1-offline-dependency-20261002/generated/wire)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.82s

### Negative controls (all compiled and ran1 named case; originals restored)

DanglingTarget guard removed with false condition:
cargo test --locked -p codegate-cli --test semantics dangling_target_guard -- --exact
    Blocking waiting for file lock on build directory
   Compiling codegate-contract v0.0.0 ($HOME/.local/state/worktree/trees/b10x/codegate/codegate-w1-offline-dependency-20261002/generated/wire)
   Compiling codegate v1.0.0 ($HOME/.local/state/worktree/trees/b10x/codegate/codegate-w1-offline-dependency-20261002/generated/behavior)
   Compiling codegate-cli v0.1.0 ($HOME/.local/state/worktree/trees/b10x/codegate/codegate-w1-offline-dependency-20261002)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 7.18s
     Running tests/semantics.rs (target/debug/deps/semantics-4bcb7134b24f2c61)

running 1 test
test dangling_target_guard ... FAILED

failures:

---- dangling_target_guard stdout ----

thread 'dangling_target_guard' (1347546) panicked at tests/semantics.rs:38:5:
assertion `left == right` failed
  left: Null
 right: "DanglingTarget"
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    dangling_target_guard

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

error: test failed, to rerun pass `-p codegate-cli --test semantics`

Partial coverage mapped to Complete and gaps cleared:
cargo test --locked -p codegate-cli --test semantics partial_is_never_complete -- --exact
   Compiling codegate-cli v0.1.0 ($HOME/.local/state/worktree/trees/b10x/codegate/codegate-w1-offline-dependency-20261002)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.59s
     Running tests/semantics.rs (target/debug/deps/semantics-4bcb7134b24f2c61)

running 1 test
test partial_is_never_complete ... FAILED

failures:

---- partial_is_never_complete stdout ----

thread 'partial_is_never_complete' (1351641) panicked at tests/semantics.rs:47:5:
assertion `left == right` failed
  left: String("Pass")
 right: "Incomplete"
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    partial_is_never_complete

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

error: test failed, to rerun pass `-p codegate-cli --test semantics`

Unique destination BTreeSet replaced with occurrence Vec:
cargo test --locked -p codegate-cli --test semantics parallel_imports_count_unique_destinations -- --exact
   Compiling codegate-cli v0.1.0 ($HOME/.local/state/worktree/trees/b10x/codegate/codegate-w1-offline-dependency-20261002)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1.64s
     Running tests/semantics.rs (target/debug/deps/semantics-4bcb7134b24f2c61)

running 1 test
test parallel_imports_count_unique_destinations ... FAILED

failures:

---- parallel_imports_count_unique_destinations stdout ----

thread 'parallel_imports_count_unique_destinations' (1358359) panicked at tests/semantics.rs:53:5:
assertion `left == right` failed
  left: Number(2)
 right: 1
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    parallel_imports_count_unique_destinations

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 7 filtered out; finished in 0.00s

error: test failed, to rerun pass `-p codegate-cli --test semantics`

Each mutant failed behaviorally with exit101,0passed1failed7filtered. The final
unfiltered19-test green run includes all3 restored cases and no ignored tests.

## 5. Deliberate exclusions and limits

No real Rust/Go binding, source hashing, stable broad IR, cycle analysis, publication,
store mutation, commits or cleanup. Full task check is implemented but reserved to the
coordinator's integration pass. Upstream raw generation requires deterministic pinned
rustfmt normalization; it is part of the gate pipeline, not manual generated edits.
The native ESS report uses official --suite-format5 to include complete inventory;
without that flag the legacy default reports inconclusive knowledge despite27passing.
The original actual legacy report remains verbatim in conformance-initial.log.
Current suite/report/run are copied into this scratch directory for retention.

Wire Optional omission/null bridge is restricted to declared optional edge fields and
fanout values. Explicit generated-value conversion and bounded integral fanout are
tested. EvaluateBehavior's outcome-only seam retains one typed actual response in an
invocation-local adapter; repeated calls are tested for stale response leakage.
Passing named scenarios does not claim exhaustive mathematical correctness.

## 6. Outside writes and handoff

No authored log/source/patch was written outside this managed tree. Tools updated
normal shared caches/metadata: $HOME/.cargo/registry/,
$HOME/.cargo/git/, $HOME/.cache/sccache/ and worktree lease metadata under
$HOME/.local/state/worktree/. These are not disposable unit files to delete.
All retained report/logs/suite files are under the assigned .scratch directory.
Coordinator owns target cleanup and worktree finish. No build/check process remains
running at handoff. Own lease is released with session-end after this report.

## Correction report, verbatim

unit:                   story:offline-dependency-slice — adversary pass1 corrections
verdict:                green
cases:                  executed21→22 Rust tests, red2→0; native ESS27passed
origin:                 n/a
wrote-outside-worktree: tool-managed caches/lease only; no authored files
needs-coordinator:      no

## 1. Unit and acceptance

Correct all3 introduced findings in review-result:codegate-wave1-adversary-pass1.
Reviewer assertions are preserved; generated-drift case adds product-mutation checks
and its compile invocation now supplies the gate's direct sha2 dependency. No core
evaluator, ESS scenario or planning-store mutation. No commits/staging performed.

## 2. Actual change
 .gitignore                                |   1 +
 AGENTS.md                                 |  12 ++-
 Cargo.lock                                |   1 +
 Cargo.toml                                |   1 +
 README.md                                 |   9 ++
 generated/behavior/.ess-output/state.json |   1 -
 generated/wire/.ess-output/state.json     |   1 -
 src/bin/codegate-check.rs                 | 152 +++++++++++++++++++++++++-----
 tests/conformance.rs                      |  29 +++++-
 9 files changed, 179 insertions(+), 28 deletions(-)
 M .gitignore
 M AGENTS.md
 M Cargo.lock
 M Cargo.toml
 M README.md
 D generated/behavior/.ess-output/state.json
 D generated/wire/.ess-output/state.json
 M src/bin/codegate-check.rs
 M tests/conformance.rs
?? tests/freshness.rs
?? tests/gate_adversary.rs
?? tests/provenance_adversary.rs

## 3. Red evidence

Read and retained verbatim original report/logs in ../adversary-1/report.md:
- identical generation comparator: executed1,failed1,exit101;
- observed completion timestamp: executed1,failed1,exit101;
- scratch stale-producer attack: executed1,failed1,exit101 (actual gate incorrectly0).
The package after attack executed21,19passed2failed; these are the before figures.
Initial correction compile errors (sha2 LowerHex/raw-string delimiter) were fixed
before behavioral checks and are not claimed as negative-control evidence.

## 4. Corrected green lanes (raw output follows)

Command: RUSTC_WRAPPER=/usr/bin/sccache CARGO_BUILD_JOBS=2 cargo test --locked -p codegate-cli --test gate_adversary --test provenance_adversary --test freshness -- --nocapture
Exit0: original reviewer cases each1failed→1passed; new maintained freshness case1passed.
   Compiling codegate-cli v0.1.0 ($HOME/.local/state/worktree/trees/b10x/codegate/codegate-w1-offline-dependency-20261002)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.37s
     Running tests/freshness.rs (target/debug/deps/freshness-30aa3b1d3fcb79c8)

running 1 test
test gate_requires_this_invocations_conformance_producer ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 6.50s

     Running tests/gate_adversary.rs (target/debug/deps/gate_adversary-5c075ac973e2fdad)

running 1 test
test unchanged_generated_contract_ignores_local_ownership_state ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 17.57s

     Running tests/provenance_adversary.rs (target/debug/deps/provenance_adversary-fddccb4beaab5353)

running 1 test
test native_report_records_observed_completion_time ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.24s


Command: RUSTC_WRAPPER=/usr/bin/sccache CARGO_BUILD_JOBS=2 cargo test --locked -p codegate-cli --all-targets --no-fail-fast -- --nocapture
Exit0: executed21→22,passed19→22,failed2→0,ignored0. Per-lane:
boundary5→5;CLI3→3;conformance1→1;freshness new→1;gate adversary1→1;
obligation2→2;provenance adversary1→1;semantics8→8. No existing case removed/filtered.
Native ESS27→27passed with complete inventory,zero other outcomes. Unit targets0.
   Compiling codegate-cli v0.1.0 ($HOME/.local/state/worktree/trees/b10x/codegate/codegate-w1-offline-dependency-20261002)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.12s
     Running unittests src/lib.rs (target/debug/deps/codegate-4656fda71df7915a)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs (target/debug/deps/codegate-d35365d64892493d)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/codegate-check.rs (target/debug/deps/codegate_check-1fe97699a372e6fd)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/boundary.rs (target/debug/deps/boundary-9bdf51d047dbc681)

running 5 tests
test strict_json_structure ... ok
test generated_integer_obligation_is_checked ... ok
test explicit_absence_bridge ... ok
test all_generated_fields_roundtrip ... ok
test decoder_limits ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.23s

     Running tests/cli.rs (target/debug/deps/cli-fac19e6bfdfe86f0)

running 3 tests
test cli_structural_refusal_emits_no_report_or_output_file ... ok
test unsupported_policy_format_is_semantic_error ... ok
test cli_reports_full_outcomes_and_coverage_aware_exits_without_tools ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.72s

     Running tests/conformance.rs (target/debug/deps/conformance-c7a9fb2bcbc4979a)

running 1 test
{
  "completed_at": 1790944956949,
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

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.74s

     Running tests/freshness.rs (target/debug/deps/freshness-30aa3b1d3fcb79c8)

running 1 test
test gate_requires_this_invocations_conformance_producer ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.59s

     Running tests/gate_adversary.rs (target/debug/deps/gate_adversary-5c075ac973e2fdad)

running 1 test
test unchanged_generated_contract_ignores_local_ownership_state ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 2.22s

     Running tests/obligation.rs (target/debug/deps/obligation-0a26cb3fdf8672a5)

running 2 tests
test evaluate_obligation_is_fulfilled ... ok
test trait_adapter_never_leaks_a_previous_response ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/provenance_adversary.rs (target/debug/deps/provenance_adversary-fddccb4beaab5353)

running 1 test
test native_report_records_observed_completion_time ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.93s

     Running tests/semantics.rs (target/debug/deps/semantics-8b592e45badae6b0)

running 8 tests
test core_module_boundary_has_no_io_or_language_dispatch ... ok
test labels_and_input_order_do_not_choose_algorithms ... ok
test dangling_target_guard ... ok
test parallel_imports_count_unique_destinations ... ok
test diagnostic_phase_precedence_and_order ... ok
test partial_is_never_complete ... ok
test semantic_invalid_classes ... ok
test every_authored_literal_matches_real_evaluation ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s


Command: cargo fmt --package codegate-cli --check
Exit0; no output.
Command: RUSTC_WRAPPER=/usr/bin/sccache CARGO_BUILD_JOBS=2 cargo clippy --locked -p codegate-cli --all-targets -- -D warnings
Exit0.
    Checking codegate-cli v0.1.0 ($HOME/.local/state/worktree/trees/b10x/codegate/codegate-w1-offline-dependency-20261002)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.52s

### Actual maintained freshness probe output

The scratch-compiled production gate stage admits a real fresh evaluator report and
rejects actual missing/ignored/renamed producer Cargo invocations while old complete
reports are present. No production fallback or command stub. An ignored test exists
only inside a deliberately broken temporary Cargo fixture; the maintained root
regression asserts its rejection. Root has zero ignored cases. This directly covers
the original stale-report attack without repeating an entire copied project build.

running 1 test

running 1 test
{
  "completed_at": 1790944958924,
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

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.82s


running 1 test
test real_evaluator_conforms_to_complete_combined_suite ... ignored

test result: ok. 0 passed; 0 failed; 1 ignored; 0 measured; 0 filtered out; finished in 0.00s


running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.00s

test real_fresh_report_is_admitted_and_stale_producers_are_refused ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.78s

CHECK real ESS conformance: cargo test --locked -p codegate-cli --test conformance real_evaluator_conforms_to_complete_combined_suite -- --exact --nocapture
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.10s
     Running tests/conformance.rs (target/debug/deps/conformance-c7a9fb2bcbc4979a)
CHECK real ESS conformance: exit Some(0)
CHECK native ESS report: executed27 passed27 failed0 error0 unsupported0 skipped0, exit0; evidence $HOME/.local/state/worktree/trees/b10x/codegate/codegate-w1-offline-dependency-20261002/target/freshness-regression-1888359/positive/conformance
CHECK real ESS conformance: cargo test --locked -p codegate-cli --test conformance real_evaluator_conforms_to_complete_combined_suite -- --exact --nocapture
error: no test target named `conformance` in `codegate-cli` package
CHECK real ESS conformance: exit Some(101)
missing: refused stale evidence: real ESS conformance producer failed
CHECK real ESS conformance: cargo test --locked -p codegate-cli --test conformance real_evaluator_conforms_to_complete_combined_suite -- --exact --nocapture
   Compiling codegate-cli v0.1.0 ($HOME/.local/state/worktree/trees/b10x/codegate/codegate-w1-offline-dependency-20261002/target/freshness-regression-1888359/ignored)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.64s
     Running tests/conformance.rs (target/debug/deps/conformance-06fc3b606f92d617)
CHECK real ESS conformance: exit Some(0)
ignored: refused stale evidence: fresh conformance suite missing: No such file or directory (os error 2)
CHECK real ESS conformance: cargo test --locked -p codegate-cli --test conformance real_evaluator_conforms_to_complete_combined_suite -- --exact --nocapture
   Compiling codegate-cli v0.1.0 ($HOME/.local/state/worktree/trees/b10x/codegate/codegate-w1-offline-dependency-20261002/target/freshness-regression-1888359/renamed)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.63s
     Running tests/conformance.rs (target/debug/deps/conformance-06fc3b606f92d617)
CHECK real ESS conformance: exit Some(0)
renamed: refused stale evidence: fresh conformance suite missing: No such file or directory (os error 2)

## 5. Fix, class and enumeration

1. Fix: root .ess-output ownership metadata is excluded and its2 committed state
   files removed; ignore rule prevents recommit. Class: local output-management/build
   state is not generated product drift. Enumeration: only root .ess-output,target,
   Cargo.lock are excluded. Product Cargo.toml,PLAN.md,plan.json,src/dependency.rs,
   types-report.json,source.schema.json are actively mutated by the regression and
   each change must still be detected. All other product bytes remain compared;
   nested paths with these reserved names are not silently excluded.
2. Fix: the harness supplies ObservedClock to native Runner rather than its default
   AdvancingClock; completion is measured in the execution interval. Class: durable
   observation time must not use a deterministic example clock. Enumeration: native
   report completion,scenario runner observations and gate invocation interval use
   actual observation time; authored logical input timestamps are unchanged and all27
   scenarios pass. Core evaluation remains untouched/pure.
3. Fix: gate invokes the exact named producer in a newly created private directory;
   existing target/conformance is never read for admission. Class: evidence must be
   produced during the verifying invocation and match its subject. Enumeration:
   missing,ignored,renamed producers all refused; actual current producer admitted.
   Report format,producer profile,policy,implementation,model identity,specification,
   exact suite digest/profile/version,counts,coverage knowledge/counts/selection/
   refusals,passed IDs,nonpass outcomes and completion interval are validated.
   The regression perturbs19 binding fields independently and requires rejection.

No tests or assertions weakened; narrow generated-wire style exception unchanged.
No full task check was run on this real unit; coordinator owns integration gate.
No re-run of unchanged generated package lint needed: generated product bytes unchanged.

## 6. Outside writes, resources and handoff

No authored file outside assigned tree. Normal shared Cargo/sccache caches and
worktree lease metadata only. Logs/native report/suite retained in correction-1.
New reproducible outputs lie under root target/freshness-regression-* (small dummy
Cargo targets plus compiled probe) and target/gate-adversary/generation-*.
Coordinator owns cleanup; no target directory was removed. Own lease released after
report. No build/check process remains running. Targets remain isolated, jobs2.
