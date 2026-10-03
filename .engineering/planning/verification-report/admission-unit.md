---
format: aep.planning-md/3
id: verification-report:admission-unit
kind: verification-report
status: draft
title: Rich admission unit and two corrections
relations:
- verifies: story:capability-admission
revision: 3
---
unit: story:capability-admission — Admit rich facts and expose precise capability gaps
verdict: green
cases: executed unmeasured-base→53, red 8 runtime assertions across three recorded red runs
origin: n/a
wrote-outside-worktree: $BUILD (exact private path in coordinator brief), standard Cargo/sccache caches
needs-coordinator: native foundation ESS integration, full task check, independent adversary and final shared module wiring

## Unit and scope

Malformed, stale and contradictory rich snapshots are refused before a private admitted wrapper can reach shared algorithms. Only src/semantic/admit.rs constructs Admitted. All four inferred owned paths were absent at dispatch and are now implemented: src/semantic/admit.rs, src/semantic/mod.rs, tests/semantic_admission.rs and tests/semantic_boundary.rs. The explicitly permitted src/lib.rs change adds public semantic module registration. No AEP, manifests, generated files, collection files or commits changed.

Actual `git diff --stat` (new files remain untracked until coordinator stages them):

```text
 src/lib.rs | 1 +
 1 file changed, 1 insertion(+)
```

New files are the four owned paths listed above; the status output exposes all of them. Resource preflight observed 22,866,206,720 available bytes on the assigned build filesystem, above 20 GiB. Build environment throughout: CARGO_TARGET_DIR=$BUILD, RUSTC_WRAPPER=/usr/bin/sccache, CARGO_BUILD_JOBS=2, CARGO_INCREMENTAL=0. Distinct external target is expressly assigned by the coordinator and repository workflow.

## Red runs

A compiling refusing stub preceded admission implementation. Command: `cargo test --locked -p codegate-cli --test semantic_admission`, exit 101, executed8 with4 runtime failures. AST scanner initially returned no violations before its implementation: `cargo test --locked -p codegate-cli --test semantic_boundary`, exit101, executed4 with3 runtime failures. Additional tool-failure/selection case ran before those guards: `cargo test --locked -p codegate-cli --test semantic_admission failed_tools_and_out_of_selection_files_cannot_look_complete -- --exact`, exit101, executed1 with1 runtime failure. These failures are assertions, not compilation errors. Raw complete logs below normalize only personal checkout/build paths.

### red.log

```text
   Compiling unicode-ident v1.0.26
   Compiling proc-macro2 v1.0.107
   Compiling quote v1.0.47
   Compiling libc v0.2.189
   Compiling syn v3.0.6
   Compiling jobserver v0.1.35
   Compiling shlex v2.0.1
   Compiling find-msvc-tools v0.1.14
   Compiling cc v1.6.0
   Compiling serde_core v1.0.229
   Compiling synstructure v0.14.0
   Compiling zerofrom-derive v0.1.8
   Compiling serde v1.0.229
   Compiling zmij v1.0.23
   Compiling memchr v2.8.3
   Compiling zerofrom v0.1.8
   Compiling yoke-derive v0.8.4
   Compiling serde_derive v1.0.229
   Compiling itoa v1.0.18
   Compiling stable_deref_trait v1.2.1
   Compiling yoke v0.8.3
   Compiling equivalent v1.0.2
   Compiling hashbrown v0.17.1
   Compiling indexmap v2.14.2
   Compiling zerovec-derive v0.11.6
   Compiling displaydoc v0.2.7
   Compiling zerovec v0.11.8
   Compiling tree-sitter-language v0.1.8
   Compiling serde_json v1.0.151
   Compiling tinystr v0.8.4
   Compiling writeable v0.6.4
   Compiling litemap v0.8.3
   Compiling typenum v1.20.1
   Compiling icu_locale_core v2.3.0
   Compiling hybrid-array v0.4.15
   Compiling potential_utf v0.1.6
   Compiling zerotrie v0.2.5
   Compiling pkg-config v0.3.34
   Compiling utf8_iter v1.0.4
   Compiling icu_properties_data v2.3.0
   Compiling icu_normalizer_data v2.3.0
   Compiling icu_collections v2.3.0
   Compiling icu_provider v2.3.1
   Compiling vcpkg v0.2.15
   Compiling libz-sys v1.1.29
   Compiling crypto-common v0.2.2
   Compiling block-buffer v0.12.1
   Compiling const-oid v0.10.2
   Compiling smallvec v1.16.2
   Compiling bitflags v2.13.2
   Compiling digest v0.11.3
   Compiling icu_normalizer v2.3.0
   Compiling icu_properties v2.3.0
   Compiling syn v2.0.119
   Compiling utf8parse v0.2.2
   Compiling cfg-if v1.0.5
   Compiling cpufeatures v0.3.1
   Compiling serde_derive_internals v0.29.1
   Compiling sha2 v0.11.0
   Compiling anstyle-parse v1.0.0
   Compiling idna_adapter v1.2.2
   Compiling libgit2-sys v0.18.8+1.9.7
   Compiling aho-corasick v1.1.5
   Compiling anstyle v1.0.14
   Compiling percent-encoding v2.3.2
   Compiling schemars v0.8.22
   Compiling thiserror v2.0.21
   Compiling regex-syntax v0.8.11
   Compiling is_terminal_polyfill v1.70.2
   Compiling colorchoice v1.0.5
   Compiling anstyle-query v1.1.5
   Compiling anstream v1.0.0
   Compiling form_urlencoded v1.2.2
   Compiling tree-sitter v0.25.10
   Compiling idna v1.1.0
   Compiling regex-automata v0.4.18
   Compiling schemars_derive v0.8.22
   Compiling toml_datetime v0.6.11
   Compiling serde_spanned v0.6.9
   Compiling thiserror-impl v2.0.21
   Compiling tree-sitter-go v0.25.0
   Compiling tree-sitter-rust v0.24.2
   Compiling tree-sitter-java v0.23.5
   Compiling strsim v0.11.1
   Compiling ryu v1.0.23
   Compiling toml_write v0.1.2
   Compiling heck v0.5.0
   Compiling winnow v0.7.15
   Compiling clap_lex v1.1.1
   Compiling unsafe-libyaml v0.2.11
   Compiling rustix v1.1.5
   Compiling dyn-clone v1.0.20
   Compiling toml_edit v0.22.27
   Compiling serde_yaml v0.9.34+deprecated
   Compiling clap_builder v4.6.7
   Compiling clap_derive v4.6.7
   Compiling regex v1.13.1
   Compiling url v2.5.8
   Compiling streaming-iterator v0.1.9
   Compiling linux-raw-sys v0.12.1
   Compiling log v0.4.34
   Compiling git2 v0.20.4
   Compiling clap v4.6.7
   Compiling ess-primitives v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
   Compiling toml v0.8.23
   Compiling codegate-semantic-contract v0.0.0 ($WORKTREE/generated/semantic-wire)
   Compiling codegate-contract v0.0.0 ($WORKTREE/generated/wire)
   Compiling quick-xml v0.37.5
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
   Compiling codegate v1.0.0 ($WORKTREE/generated/behavior)
   Compiling pulldown-cmark v0.13.4
   Compiling ess-domain v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
   Compiling codegate-cli v0.1.0 ($WORKTREE)
   Compiling unicase v2.9.0
   Compiling pulldown-cmark-escape v0.11.0
   Compiling ess-compiler v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
   Compiling ess-gen v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
   Compiling ess-conformance v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1m 09s
     Running tests/semantic_admission.rs ($BUILD/debug/deps/semantic_admission-3c7c4b39a57f6379)

running 8 tests
test typed_resources_are_bounded ... ok
test valid_complete_families_and_partial_evidence_are_admitted ... FAILED
test stale_evidence_has_a_stable_typed_refusal ... FAILED
test identities_and_configuration_views_are_checked ... FAILED
test resolution_and_kind_payloads_cannot_fabricate_facts ... ok
test missing_overlap_and_outside_coverage_never_mean_zero ... ok
test dangling_duplicate_and_cycle_references_refuse ... ok
test invalid_locations_and_line_partitions_refuse_without_overflow ... FAILED

failures:

---- valid_complete_families_and_partial_evidence_are_admitted stdout ----

thread 'valid_complete_families_and_partial_evidence_are_admitted' (2757576) panicked at tests/semantic_admission.rs:30:47:
called `Result::unwrap()` on an `Err` value: [Gap { code: InvalidFact, reason: "unimplemented admission", location: None }]

---- stale_evidence_has_a_stable_typed_refusal stdout ----

thread 'stale_evidence_has_a_stable_typed_refusal' (2757574) panicked at tests/semantic_admission.rs:37:5:
assertion `left == right` failed
  left: Err([Gap { code: InvalidFact, reason: "unimplemented admission", location: None }])
 right: Err([Gap { code: StaleEvidence, reason: "stale evidence: e-unit", location: None }])

---- identities_and_configuration_views_are_checked stdout ----

thread 'identities_and_configuration_views_are_checked' (2757569) panicked at tests/semantic_admission.rs:26:113:
[Gap { code: InvalidFact, reason: "unimplemented admission", location: None }]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- invalid_locations_and_line_partitions_refuse_without_overflow stdout ----

thread 'invalid_locations_and_line_partitions_refuse_without_overflow' (2757570) panicked at tests/semantic_admission.rs:59:179:
called `Result::unwrap()` on an `Err` value: [Gap { code: InvalidFact, reason: "unimplemented admission", location: None }]


failures:
    identities_and_configuration_views_are_checked
    invalid_locations_and_line_partitions_refuse_without_overflow
    stale_evidence_has_a_stable_typed_refusal
    valid_complete_families_and_partial_evidence_are_admitted

test result: FAILED. 4 passed; 4 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p codegate-cli --test semantic_admission`

```

### red-boundary.log

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
warning: unused imports: `BTreeMap` and `BTreeSet`
 --> tests/semantic_boundary.rs:2:25
  |
2 | use std::{collections::{BTreeMap,BTreeSet},fs,path::{Path,PathBuf}};
  |                         ^^^^^^^^ ^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` (part of `#[warn(unused)]`) on by default

warning: unused imports: `Item`, `UseTree`, `Visit`, and `self`
 --> tests/semantic_boundary.rs:3:19
  |
3 | use syn::{visit::{self,Visit},Item,UseTree};
  |                   ^^^^ ^^^^^  ^^^^ ^^^^^^^

warning: `codegate-cli` (test "semantic_boundary") generated 2 warnings (run `cargo fix --test "semantic_boundary" -p codegate-cli` to apply 2 suggestions)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.16s
     Running tests/semantic_boundary.rs ($BUILD/debug/deps/semantic_boundary-e8ca43ef171983d5)

running 4 tests
test all_shared_modules_and_root_algorithms_obey_boundary ... ok
test injected_io_binding_and_dispatch_are_rejected_by_ast ... FAILED
test wiring_projection_and_neutral_admission_are_distinct_from_algorithms ... FAILED
test new_and_inline_core_modules_cannot_escape_discovery ... FAILED

failures:

---- injected_io_binding_and_dispatch_are_rejected_by_ast stdout ----

thread 'injected_io_binding_and_dispatch_are_rejected_by_ast' (2981720) panicked at tests/semantic_boundary.rs:26:8:
not rejected: fn evaluate(){ std::fs::read("x"); }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- wiring_projection_and_neutral_admission_are_distinct_from_algorithms stdout ----

thread 'wiring_projection_and_neutral_admission_are_distinct_from_algorithms' (2981722) panicked at tests/semantic_boundary.rs:33:5:
assertion failed: !violations("pub mod collection; fn evaluate(){ collection::collect(); }",
            Role::Root).is_empty()

---- new_and_inline_core_modules_cannot_escape_discovery stdout ----

thread 'new_and_inline_core_modules_cannot_escape_discovery' (2981721) panicked at tests/semantic_boundary.rs:43:5:
assertion failed: !scan_tree(&root).unwrap().is_empty()


failures:
    injected_io_binding_and_dispatch_are_rejected_by_ast
    new_and_inline_core_modules_cannot_escape_discovery
    wiring_projection_and_neutral_admission_are_distinct_from_algorithms

test result: FAILED. 1 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

error: test failed, to rerun pass `-p codegate-cli --test semantic_boundary`

```

### red-selection.log

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
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.09s
     Running tests/semantic_admission.rs ($BUILD/debug/deps/semantic_admission-3c7c4b39a57f6379)

running 1 test
test failed_tools_and_out_of_selection_files_cannot_look_complete ... FAILED

failures:

---- failed_tools_and_out_of_selection_files_cannot_look_complete stdout ----

thread 'failed_tools_and_out_of_selection_files_cannot_look_complete' (3084376) panicked at tests/semantic_admission.rs:26:73:
invalid facts must refuse: ()
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    failed_tools_and_out_of_selection_files_cannot_look_complete

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10 filtered out; finished in 0.00s

error: test failed, to rerun pass `-p codegate-cli --test semantic_admission`

```

## Final green verification

Command: `PATH=<pinned ESS 0.50.0>:$PATH cargo test --locked -p codegate-cli`, with the build environment above; exit0. Runner-observed total53, no failures/ignored/skipped Rust tests. The base package total was not run in this tree; do not infer an observed base count by adding results from other checkouts. Admission lane executed8 in its first red run and11 in the final suite; boundary lane executed4 red and4 green, with the previously failing probes now rejecting violations. Other lanes retain runner-observed docs5, boundary5, CLI3, conformance1, freshness1, gate_adversary1, obligation2, provenance_adversary1, semantics8, wire4, source_identity6 and codegate-check1. Library/main/doctest lanes executed0.

Command: `cargo fmt --check`, exit0.
Command: `cargo clippy --locked -p codegate-cli --all-targets -- -D warnings`, exit0 with the assigned build environment. Root warnings remain denied. The known dependency-generated unreachable_code warning is unchanged; no lint exception was added.

Full final package output (paths normalized):
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
    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.66s
     Running unittests src/lib.rs ($BUILD/debug/deps/codegate-13f02747b00aad71)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs ($BUILD/debug/deps/codegate-2f32bdfa27a53d34)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/codegate-check.rs ($BUILD/debug/deps/codegate_check-2f559fc5b688cc09)

running 1 test
test tests::gate_clean_external_target ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/codegate-docs.rs ($BUILD/debug/deps/codegate_docs-e6f1893802ce61c7)

running 5 tests
test tests::invalid_commits_are_refused ... ok
test tests::authored_site_and_publication_identity_are_valid ... ok
test tests::broken_or_duplicate_anchors_are_refused ... ok
test tests::published_json_example_reaches_the_real_evaluator ... ok
test tests::wrong_routes_assets_and_internal_source_links_are_refused ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/boundary.rs ($BUILD/debug/deps/boundary-71b8e0b4adea1e24)

running 5 tests
test generated_integer_obligation_is_checked ... ok
test explicit_absence_bridge ... ok
test strict_json_structure ... ok
test all_generated_fields_roundtrip ... ok
test decoder_limits ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s

     Running tests/cli.rs ($BUILD/debug/deps/cli-9d10002ea0f5bbfd)

running 3 tests
test cli_structural_refusal_emits_no_report_or_output_file ... ok
test unsupported_policy_format_is_semantic_error ... ok
test cli_reports_full_outcomes_and_coverage_aware_exits_without_tools ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/conformance.rs ($BUILD/debug/deps/conformance-d2c03fe9842dc916)

running 1 test
test real_evaluator_conforms_to_complete_combined_suite ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.22s

     Running tests/freshness.rs ($BUILD/debug/deps/freshness-f5d37c077f3df08b)

running 1 test
test gate_requires_this_invocations_conformance_producer ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.22s

     Running tests/gate_adversary.rs ($BUILD/debug/deps/gate_adversary-3717d0214c10e009)

running 1 test
test unchanged_generated_contract_ignores_local_ownership_state ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.70s

     Running tests/obligation.rs ($BUILD/debug/deps/obligation-885b816d89ea5c89)

running 2 tests
test evaluate_obligation_is_fulfilled ... ok
test trait_adapter_never_leaks_a_previous_response ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/provenance_adversary.rs ($BUILD/debug/deps/provenance_adversary-a79347a37b559ada)

running 1 test
test native_report_records_observed_completion_time ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.29s

     Running tests/semantic_admission.rs ($BUILD/debug/deps/semantic_admission-3c7c4b39a57f6379)

running 11 tests
test typed_resources_are_bounded ... ok
test stale_evidence_has_a_stable_typed_refusal ... ok
test valid_complete_families_and_partial_evidence_are_admitted ... ok
test resolution_and_kind_payloads_cannot_fabricate_facts ... ok
test failed_tools_and_out_of_selection_files_cannot_look_complete ... ok
test identities_and_configuration_views_are_checked ... ok
test dangling_duplicate_and_cycle_references_refuse ... ok
test rich_relationship_families_and_framework_payloads_are_checked ... ok
test all_family_statuses_have_honest_coverage_semantics ... ok
test missing_overlap_and_outside_coverage_never_mean_zero ... ok
test invalid_locations_and_line_partitions_refuse_without_overflow ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/semantic_boundary.rs ($BUILD/debug/deps/semantic_boundary-e8ca43ef171983d5)

running 4 tests
test new_and_inline_core_modules_cannot_escape_discovery ... ok
test wiring_projection_and_neutral_admission_are_distinct_from_algorithms ... ok
test injected_io_binding_and_dispatch_are_rejected_by_ast ... ok
test all_shared_modules_and_root_algorithms_obey_boundary ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s

     Running tests/semantic_wire.rs ($BUILD/debug/deps/semantic_wire-edcfcd6e1297f119)

running 4 tests
test requests_and_configuration_have_explicit_generated_bridges ... ok
test every_snapshot_family_and_transitive_field_round_trips ... ok
test optional_null_is_absence_but_required_or_unknown_null_is_refused ... ok
test malformed_unknown_duplicate_enums_and_integer_overflow_are_refused ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/semantics.rs ($BUILD/debug/deps/semantics-efbce059cf065140)

running 8 tests
test core_module_boundary_has_no_io_or_language_dispatch ... ok
test dangling_target_guard ... ok
test parallel_imports_count_unique_destinations ... ok
test partial_is_never_complete ... ok
test diagnostic_phase_precedence_and_order ... ok
test labels_and_input_order_do_not_choose_algorithms ... ok
test semantic_invalid_classes ... ok
test every_authored_literal_matches_real_evaluation ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/source_identity.rs ($BUILD/debug/deps/source_identity-cacab3891d5875b4)

running 6 tests
test explicit_canonical_wire_bytes_have_known_hash ... ok
test root_module_identifiers_are_distinct_from_selected_paths ... ok
test snapshot_projection_uses_contract_enums_and_omits_absent_optionals ... ok
test duplicates_and_non_normalized_paths_are_refused ... ok
test reordering_and_origin_never_change_identity ... ok
test selected_bytes_and_configuration_change_identity ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests codegate

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

## Stable native ESS seam

Rust entry point: `semantic::validate_snapshot(&semantic_model::FactSnapshot) -> Result<(), Vec<Gap>>`. InvalidFormat is checked before resource/identity/graph checks; source identity precedes stale fact/tool checks; relationships precede coverage. Each guard currently returns one stable typed refusal; no clock, filesystem or tool access occurs.

Exact single diagnostics, all with `location: null` when projected as a complete Gap value (the generated wire bridge omits absent optional location):

- Stale tool: `{"code":"StaleEvidence","reason":"stale tool evidence: TOOL","location":null}`, where TOOL is the nonempty ToolObservation.tool.
- Stale fact e-unit: `{"code":"StaleEvidence","reason":"stale evidence: e-unit","location":null}`.
- Missing occurrence owner `missing`: `{"code":"InvalidFact","reason":"unknown occurrence owner: missing","location":null}`.
- Missing ExecutionCoverage in empty unit universe: `{"code":"InvalidFact","reason":"missing coverage: ExecutionCoverage:<empty>","location":null}`.
- Missing family for unit u: reason `missing coverage: FAMILY:u`, where FAMILY uses the contract variant spelling.

Minimal valid empty FactSnapshot: exactformat codegate.semantic-facts/1, SourceOnly, tools and all fact lists empty. Source selection has include_paths/exclude_paths/languages empty, include_tests=true and include_generated=true. BuildSelection has modules/go_build_tags/rust_features/rust_cfg/build_profiles/configuration_files empty, rust_default_features=true, all optionals absent; derive its id through source_identity::configuration_id. Source files/manifests empty; derive id through source_identity::snapshot_id after configuration id. Coverage contains exactly the15 frozen families with unit_ids=[] and matching configuration_id: Sources is Complete with no gaps; every other family is Unsupported with Gap UnsupportedCapability, reason `not collected in this profile`, location absent. This represents complete empty sources with explicitly unavailable other evidence, not global semantic completeness.

## Deliberate limits and integration notes

- No source language collection, metrics, policy evaluation, public generated handlers or ESS harness was implemented here. Unit-test counts are not ESS conformance counts. Coordinator owns the real six-case native foundation suite and complete integration gate.
- Offline admission checks supplied metadata and identity consistency; it cannot authenticate unavailable source bytes. EOF metadata accepts both agreed forms; cross-file logical module/package parents are allowed, while lexical containment is checked where applicable.
- Typed records, coverage membership, line observations and related lists are bounded before expensive traversal. Checked byte arithmetic and iterative containment walks avoid integer overflow and recursive stack growth. Coverage validation indexes owner evidence rather than scanning every fact for every unit record.
- AST tests recursively discover modules from src/lib.rs, handle alias imports, inline/custom-path modules, macros, destructured language fields and root executable code. Projection/admission metadata roles differ from shared algorithms. Explicit collection/bindings/foundation modules are orchestration seams. The scanner is an architectural regression check, not a complete interprocedural Rust effect system; independent negative probes remain valuable. A coordinator clap get_matches probe is pending and was deliberately not preemptively fixed.
- Full task check, independent adversary, native semantic conformance and publication remain coordinator work. No implementation-story closure is claimed by this handoff.

## Retained output

All logs and this report are under $WORKTREE/.scratch/unit; architecture-discovery fixtures are there too. Existing package regressions retain their own ignored target evidence. External build output is exactly $BUILD from the private brief; it is not removed. No worktree or build cleanup was attempted. Coordinator owns integration, publication and cleanup.

unit: story:capability-admission — architecture dependency-policy correction after adversary round1
verdict: green
cases: executed 53→55, red1 independently observed before correction
origin: n/a
wrote-outside-worktree: assigned $BUILD and standard compiler caches
needs-coordinator: second fresh adversary pass, native ESS integration and full gate

## Confirmed finding and correction

review-result:admission-boundary-adversary-round-1 records a real negative-probe failure before this correction: clap::Command::get_matches reaches process arguments indirectly and escaped the direct std::env rule. The coordinator-added indirect_process_argument_io_is_rejected test is retained unchanged except formatting.

Only tests/semantic_boundary.rs changed in this correction. Admission production behavior and the original unit report are unchanged. The scanner now derives every normal dependency name from Cargo.toml, including target-specific dependency tables, and refuses external imports/calls outside an explicit pure dependency allowlist: generated contracts, serde/serde_json and sha2. A future normal dependency starts refused rather than relying on somebody adding its IO API to a denylist. Direct paths, grouped/renamed uses, extern-crate aliases, and macro argument paths use the same policy.

The finding's class is indirect IO or source parsing through external dependencies. The new all_non_admitted_dependencies_and_alias_forms_are_rejected case enumerates every currently non-admitted normal dependency and checks five path/alias forms for each, with a positive serde projection case. It does not assert that arbitrary third-party Rust effects can be inferred from an AST; the allowlist makes the architectural dependency decision explicit.

## Independent red witness

Command: `CARGO_TARGET_DIR=$BUILD RUSTC_WRAPPER=/usr/bin/sccache CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 cargo test --locked -p codegate-cli --test semantic_boundary indirect_process_argument_io_is_rejected -- --exact --nocapture`
Exit101; executed1, failed1. Raw output, paths normalized:
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
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.33s
     Running tests/semantic_boundary.rs ($BUILD/debug/deps/semantic_boundary-e8ca43ef171983d5)

running 1 test

thread 'indirect_process_argument_io_is_rejected' (3352761) panicked at tests/semantic_boundary.rs:499:9:
IO dependency escaped: fn evaluate(){clap::Command::new("x").get_matches();}
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test indirect_process_argument_io_is_rejected ... FAILED

failures:

failures:
    indirect_process_argument_io_is_rejected

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 4 filtered out; finished in 0.00s

error: test failed, to rerun pass `-p codegate-cli --test semantic_boundary`
```

## Verification

All commands use the isolated assigned build target, sccache, two Cargo jobs and CARGO_INCREMENTAL=0. Full package adds pinned ESS0.50.0 to PATH.

- `cargo test --locked -p codegate-cli --test semantic_boundary --test semantic_admission`: exit0, boundary6 and admission11 executed, no failures.
- `cargo test --locked -p codegate-cli`: exit0,55 executed, no failures. Prior recorded package run in report.md executed53; this correction retains one coordinator-added regression and adds one class-enumeration case. Admission11 unchanged. Boundary4→6; all other lanes retain their prior counts.
- `cargo fmt --check`: exit0.
- `cargo clippy --locked -p codegate-cli --all-targets -- -D warnings`: exit0. No lint suppression added.

Full package output, paths normalized:
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
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running unittests src/lib.rs ($BUILD/debug/deps/codegate-13f02747b00aad71)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs ($BUILD/debug/deps/codegate-2f32bdfa27a53d34)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/codegate-check.rs ($BUILD/debug/deps/codegate_check-2f559fc5b688cc09)

running 1 test
test tests::gate_clean_external_target ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/codegate-docs.rs ($BUILD/debug/deps/codegate_docs-e6f1893802ce61c7)

running 5 tests
test tests::invalid_commits_are_refused ... ok
test tests::authored_site_and_publication_identity_are_valid ... ok
test tests::broken_or_duplicate_anchors_are_refused ... ok
test tests::published_json_example_reaches_the_real_evaluator ... ok
test tests::wrong_routes_assets_and_internal_source_links_are_refused ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/boundary.rs ($BUILD/debug/deps/boundary-71b8e0b4adea1e24)

running 5 tests
test generated_integer_obligation_is_checked ... ok
test explicit_absence_bridge ... ok
test strict_json_structure ... ok
test all_generated_fields_roundtrip ... ok
test decoder_limits ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s

     Running tests/cli.rs ($BUILD/debug/deps/cli-9d10002ea0f5bbfd)

running 3 tests
test cli_structural_refusal_emits_no_report_or_output_file ... ok
test unsupported_policy_format_is_semantic_error ... ok
test cli_reports_full_outcomes_and_coverage_aware_exits_without_tools ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/conformance.rs ($BUILD/debug/deps/conformance-d2c03fe9842dc916)

running 1 test
test real_evaluator_conforms_to_complete_combined_suite ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.26s

     Running tests/freshness.rs ($BUILD/debug/deps/freshness-f5d37c077f3df08b)

running 1 test
test gate_requires_this_invocations_conformance_producer ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.03s

     Running tests/gate_adversary.rs ($BUILD/debug/deps/gate_adversary-3717d0214c10e009)

running 1 test
test unchanged_generated_contract_ignores_local_ownership_state ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.65s

     Running tests/obligation.rs ($BUILD/debug/deps/obligation-885b816d89ea5c89)

running 2 tests
test evaluate_obligation_is_fulfilled ... ok
test trait_adapter_never_leaks_a_previous_response ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/provenance_adversary.rs ($BUILD/debug/deps/provenance_adversary-a79347a37b559ada)

running 1 test
test native_report_records_observed_completion_time ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.28s

     Running tests/semantic_admission.rs ($BUILD/debug/deps/semantic_admission-3c7c4b39a57f6379)

running 11 tests
test typed_resources_are_bounded ... ok
test stale_evidence_has_a_stable_typed_refusal ... ok
test valid_complete_families_and_partial_evidence_are_admitted ... ok
test resolution_and_kind_payloads_cannot_fabricate_facts ... ok
test identities_and_configuration_views_are_checked ... ok
test failed_tools_and_out_of_selection_files_cannot_look_complete ... ok
test rich_relationship_families_and_framework_payloads_are_checked ... ok
test dangling_duplicate_and_cycle_references_refuse ... ok
test all_family_statuses_have_honest_coverage_semantics ... ok
test missing_overlap_and_outside_coverage_never_mean_zero ... ok
test invalid_locations_and_line_partitions_refuse_without_overflow ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/semantic_boundary.rs ($BUILD/debug/deps/semantic_boundary-e8ca43ef171983d5)

running 6 tests
test indirect_process_argument_io_is_rejected ... ok
test new_and_inline_core_modules_cannot_escape_discovery ... ok
test wiring_projection_and_neutral_admission_are_distinct_from_algorithms ... ok
test injected_io_binding_and_dispatch_are_rejected_by_ast ... ok
test all_non_admitted_dependencies_and_alias_forms_are_rejected ... ok
test all_shared_modules_and_root_algorithms_obey_boundary ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.08s

     Running tests/semantic_wire.rs ($BUILD/debug/deps/semantic_wire-edcfcd6e1297f119)

running 4 tests
test requests_and_configuration_have_explicit_generated_bridges ... ok
test every_snapshot_family_and_transitive_field_round_trips ... ok
test optional_null_is_absence_but_required_or_unknown_null_is_refused ... ok
test malformed_unknown_duplicate_enums_and_integer_overflow_are_refused ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/semantics.rs ($BUILD/debug/deps/semantics-efbce059cf065140)

running 8 tests
test core_module_boundary_has_no_io_or_language_dispatch ... ok
test dangling_target_guard ... ok
test parallel_imports_count_unique_destinations ... ok
test partial_is_never_complete ... ok
test diagnostic_phase_precedence_and_order ... ok
test labels_and_input_order_do_not_choose_algorithms ... ok
test semantic_invalid_classes ... ok
test every_authored_literal_matches_real_evaluation ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/source_identity.rs ($BUILD/debug/deps/source_identity-cacab3891d5875b4)

running 6 tests
test explicit_canonical_wire_bytes_have_known_hash ... ok
test root_module_identifiers_are_distinct_from_selected_paths ... ok
test snapshot_projection_uses_contract_enums_and_omits_absent_optionals ... ok
test reordering_and_origin_never_change_identity ... ok
test duplicates_and_non_normalized_paths_are_refused ... ok
test selected_bytes_and_configuration_change_identity ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests codegate

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

No full task check, ESS claim, AEP mutation, commit or cleanup was performed. The existing production code checksum remains the unit handoff checksum. All correction logs and this report are under $WORKTREE/.scratch/unit. Own correction lease released for the coordinator's second fresh attack.

unit: story:capability-admission — final architectural platform/context correction
verdict: green
cases: executed 55→57, red1 independently observed before correction
origin: n/a
wrote-outside-worktree: existing assigned $BUILD and standard compiler caches
needs-coordinator: native ESS integration and complete integration gate

## Finding, class and correction

review-result:admission-boundary-adversary-round-2 records the coordinator's failing nested_platform_io_and_thread_context_are_rejected case before correction. The measured failure is its first UnixStream assertion; the runner stops there and does not establish a second thread-current failure. Direct std::thread paths were already denied by the previous code.

Only tests/semantic_boundary.rs changes. The guard now defines one shared set of host-context std namespaces: fs, env, net, process, io, thread and os. Denying std::os as a namespace covers platform-specific nesting without enumerating every operating system or raw-handle/socket implementation. The same rule applies to inspected macro tokens, so thread/platform calls cannot escape through an allowed formatting macro. Alias resolution remains shared with the external-dependency policy.

The original coordinator regression is preserved without weakened expectations. One class-regression case enumerates Unix sockets, Unix filesystem extensions, Windows handles, platform-neutral raw descriptors and thread context through direct, renamed-import and macro forms, with a pure collections control. This is correction verification of the recorded finding, not a third adversarial campaign.

Admission product behavior and previous reports remain unchanged. Source checksum remains b22cfdee67667568a2500ade5eecce31289614310b51addb303fbdf84b16127e for src/semantic/admit.rs.

## Independent red witness

Command: `cargo test --locked -p codegate-cli --test semantic_boundary nested_platform_io_and_thread_context_are_rejected -- --exact --nocapture` with the assigned build environment.
Exit101, executed1, failed1. Raw output, paths normalized:
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
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.38s
     Running tests/semantic_boundary.rs ($BUILD/debug/deps/semantic_boundary-e8ca43ef171983d5)

running 1 test

thread 'nested_platform_io_and_thread_context_are_rejected' (3409140) panicked at tests/semantic_boundary.rs:555:9:
platform effect escaped: fn evaluate(){std::os::unix::net::UnixStream::connect("/tmp/socket");}
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test nested_platform_io_and_thread_context_are_rejected ... FAILED

failures:

failures:
    nested_platform_io_and_thread_context_are_rejected

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 6 filtered out; finished in 0.00s

error: test failed, to rerun pass `-p codegate-cli --test semantic_boundary`
```

## Final verification

Existing warm CARGO_TARGET_DIR=$BUILD, RUSTC_WRAPPER=/usr/bin/sccache, CARGO_BUILD_JOBS=2, CARGO_INCREMENTAL=0 throughout; no new large build or target allocation. Full package uses pinned ESS0.50.0 in PATH.

- `cargo test --locked -p codegate-cli --test semantic_boundary --test semantic_admission`: exit0; boundary8 and admission11 passed.
- `cargo test --locked -p codegate-cli`: exit0;57 passed, no failures. The previous correction package run observed55; boundary6→8, other lanes unchanged.
- `cargo fmt --check`: exit0.
- `cargo clippy --locked -p codegate-cli --all-targets -- -D warnings`: exit0; no lint exception added.

Full package output, paths normalized:
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
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.08s
     Running unittests src/lib.rs ($BUILD/debug/deps/codegate-13f02747b00aad71)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/main.rs ($BUILD/debug/deps/codegate-2f32bdfa27a53d34)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/codegate-check.rs ($BUILD/debug/deps/codegate_check-2f559fc5b688cc09)

running 1 test
test tests::gate_clean_external_target ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/codegate-docs.rs ($BUILD/debug/deps/codegate_docs-e6f1893802ce61c7)

running 5 tests
test tests::invalid_commits_are_refused ... ok
test tests::authored_site_and_publication_identity_are_valid ... ok
test tests::published_json_example_reaches_the_real_evaluator ... ok
test tests::broken_or_duplicate_anchors_are_refused ... ok
test tests::wrong_routes_assets_and_internal_source_links_are_refused ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/boundary.rs ($BUILD/debug/deps/boundary-71b8e0b4adea1e24)

running 5 tests
test strict_json_structure ... ok
test generated_integer_obligation_is_checked ... ok
test explicit_absence_bridge ... ok
test all_generated_fields_roundtrip ... ok
test decoder_limits ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.09s

     Running tests/cli.rs ($BUILD/debug/deps/cli-9d10002ea0f5bbfd)

running 3 tests
test unsupported_policy_format_is_semantic_error ... ok
test cli_structural_refusal_emits_no_report_or_output_file ... ok
test cli_reports_full_outcomes_and_coverage_aware_exits_without_tools ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running tests/conformance.rs ($BUILD/debug/deps/conformance-d2c03fe9842dc916)

running 1 test
test real_evaluator_conforms_to_complete_combined_suite ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.21s

     Running tests/freshness.rs ($BUILD/debug/deps/freshness-f5d37c077f3df08b)

running 1 test
test gate_requires_this_invocations_conformance_producer ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.09s

     Running tests/gate_adversary.rs ($BUILD/debug/deps/gate_adversary-3717d0214c10e009)

running 1 test
test unchanged_generated_contract_ignores_local_ownership_state ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.61s

     Running tests/obligation.rs ($BUILD/debug/deps/obligation-885b816d89ea5c89)

running 2 tests
test evaluate_obligation_is_fulfilled ... ok
test trait_adapter_never_leaks_a_previous_response ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/provenance_adversary.rs ($BUILD/debug/deps/provenance_adversary-a79347a37b559ada)

running 1 test
test native_report_records_observed_completion_time ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.27s

     Running tests/semantic_admission.rs ($BUILD/debug/deps/semantic_admission-3c7c4b39a57f6379)

running 11 tests
test typed_resources_are_bounded ... ok
test stale_evidence_has_a_stable_typed_refusal ... ok
test valid_complete_families_and_partial_evidence_are_admitted ... ok
test resolution_and_kind_payloads_cannot_fabricate_facts ... ok
test identities_and_configuration_views_are_checked ... ok
test failed_tools_and_out_of_selection_files_cannot_look_complete ... ok
test all_family_statuses_have_honest_coverage_semantics ... ok
test dangling_duplicate_and_cycle_references_refuse ... ok
test rich_relationship_families_and_framework_payloads_are_checked ... ok
test missing_overlap_and_outside_coverage_never_mean_zero ... ok
test invalid_locations_and_line_partitions_refuse_without_overflow ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/semantic_boundary.rs ($BUILD/debug/deps/semantic_boundary-e8ca43ef171983d5)

running 8 tests
test nested_platform_io_and_thread_context_are_rejected ... ok
test indirect_process_argument_io_is_rejected ... ok
test new_and_inline_core_modules_cannot_escape_discovery ... ok
test wiring_projection_and_neutral_admission_are_distinct_from_algorithms ... ok
test platform_and_thread_aliases_share_the_effect_guard ... ok
test injected_io_binding_and_dispatch_are_rejected_by_ast ... ok
test all_non_admitted_dependencies_and_alias_forms_are_rejected ... ok
test all_shared_modules_and_root_algorithms_obey_boundary ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.08s

     Running tests/semantic_wire.rs ($BUILD/debug/deps/semantic_wire-edcfcd6e1297f119)

running 4 tests
test requests_and_configuration_have_explicit_generated_bridges ... ok
test every_snapshot_family_and_transitive_field_round_trips ... ok
test optional_null_is_absence_but_required_or_unknown_null_is_refused ... ok
test malformed_unknown_duplicate_enums_and_integer_overflow_are_refused ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running tests/semantics.rs ($BUILD/debug/deps/semantics-efbce059cf065140)

running 8 tests
test core_module_boundary_has_no_io_or_language_dispatch ... ok
test parallel_imports_count_unique_destinations ... ok
test labels_and_input_order_do_not_choose_algorithms ... ok
test diagnostic_phase_precedence_and_order ... ok
test dangling_target_guard ... ok
test partial_is_never_complete ... ok
test semantic_invalid_classes ... ok
test every_authored_literal_matches_real_evaluation ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/source_identity.rs ($BUILD/debug/deps/source_identity-cacab3891d5875b4)

running 6 tests
test explicit_canonical_wire_bytes_have_known_hash ... ok
test snapshot_projection_uses_contract_enums_and_omits_absent_optionals ... ok
test duplicates_and_non_normalized_paths_are_refused ... ok
test reordering_and_origin_never_change_identity ... ok
test root_module_identifiers_are_distinct_from_selected_paths ... ok
test selected_bytes_and_configuration_change_identity ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

   Doc-tests codegate

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

No full task check, new ESS claim, AEP mutation, commit or cleanup. Correction logs and report remain under $WORKTREE/.scratch/unit. Own lease released at stable handoff. Both bounded adversary rounds have recorded findings and retained regression witnesses; this report records the correction, not a fresh independent verdict.
