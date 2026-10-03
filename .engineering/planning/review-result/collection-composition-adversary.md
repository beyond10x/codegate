---
format: aep.planning-md/3
id: review-result:collection-composition-adversary
kind: review-result
status: active
title: Bounded collection composition adversary
relations:
- reviews: story:collection-composition-seam
revision: 1
---
unit: story:collection-composition-seam at dc0f2a084eebce244cb6ad2d720f75b8a87539e2 versus 2132b5b
verdict: nothing found
cases: executed 93→97, red 0
origin: introduced 0 / pre-existing 0 / undecided 0
wrote-outside-worktree: none authored; existing tool-managed compiler cache and lease reused
needs-coordinator: full integration gate and disposition of the four added tests

## 1. Change boundary

`git --no-pager diff --stat` returned no tracked changes. `git status --short` returned:

```text
?? tests/collection_adversary.rs
```

The added file is untracked, so the supplementary command was `git --no-pager diff --no-index --stat /dev/null tests/collection_adversary.rs`:

```text
 /dev/null => tests/collection_adversary.rs | 232 +++++++++++++++++++++++++++++
 1 file changed, 232 insertions(+)
```

No original test, implementation, generated contract or planning file was edited. No commit, checkout or AEP command was performed. Scratch, generated fixtures and raw logs are retained in `.scratch/adversary/`; compiler and native evidence output are under the existing `target/`.

## 2. Cases written before execution

All four tests in `tests/collection_adversary.rs` were written before the first test command. Each was run alone with `cargo test --locked -p codegate-cli --test collection_adversary CASE -- --exact --nocapture`. Environment: pinned ESS 0.50.0 first in PATH, `RUSTC_WRAPPER=/usr/bin/sccache CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0`. The observed filesystem had 29 GiB available before compilation; the existing target was reused sequentially.

- `missing_slot_cannot_leave_observed_slot_declarations_complete`: a Rust producer declares complete declaration coverage while selected Go sources have no slot. The actual collector must downgrade the Rust coverage to Partial, retain the precise Rust unit scope, and attach UnknownUnit. Green.
- `registered_slots_are_deterministic_and_stale_configuration_is_rejected`: two registered production slots in opposite orders return equal full snapshots and exact Go/Rust unit order; complete declaration coverage for both selected units is retained; changing the evidence configuration to stale produces StaleEvidence. Green.
- `malformed_requests_are_refused_and_valid_missing_source_retains_failed_coverage`: duplicate languages, escaping module and repository metadata selection each produce InvalidFact. Valid missing-file capture retains Semantic, the independently pinned empty-source identity, Failed Sources and fourteen Unsupported families. Green.
- `syntax_failure_retains_real_identity_and_partial_sources`: malformed Rust remains retained alongside Go with Partial Sources; fixing the actual bytes changes the identity and yields Complete Sources. Green.

There was no red output. The individual test-result blocks below are verbatim excerpts, in execution order; complete compiler output is in the corresponding raw logs.

```text
running 1 test
test missing_slot_cannot_leave_observed_slot_declarations_complete ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.02s

running 1 test
test registered_slots_are_deterministic_and_stale_configuration_is_rejected ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.01s

running 1 test
test malformed_requests_are_refused_and_valid_missing_source_retains_failed_coverage ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.01s

running 1 test
test syntax_failure_retains_real_identity_and_partial_sources ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out; finished in 0.01s

```

Each command exited 0 and executed exactly one test, with three filtered out.

## 3. Subsequent package suite

After all four individual runs: `cargo test --locked -p codegate-cli`, same environment, exit 0. The 93-before count is the coordinator's runtime-worker 92 count plus the newly integrated native collection test; the actual subsequent package run executed 97. No original test was deselected. The suite also executed the actual native evaluator, foundation and collection runners and their seeded-fault checks. This is not a substitute for the coordinator's full integration gate.

The complete suite output follows with the single absolute checkout prefix replaced by `<worktree>` for publication; `.scratch/adversary/package-suite.log` retains the original bytes.

```text
warning: unreachable expression
   --> generated/semantic-behavior/src/system.rs:116:13
    |
116 |             self.published.push(SystemEvent::from(event));
    |             ^^^^^^^^^^^^^^^^^^^^------------------------^
    |             |                   |
    |             |                   any code following this expression is unreachable
    |             unreachable expression
    |
note: this expression has type `SystemEvent`, which is uninhabited
   --> generated/semantic-behavior/src/system.rs:116:33
    |
116 |             self.published.push(SystemEvent::from(event));
    |                                 ^^^^^^^^^^^^^^^^^^^^^^^^
    = note: `#[warn(unreachable_code)]` (part of `#[warn(unused)]`) on by default

warning: unreachable expression
   --> generated/semantic-behavior/src/system.rs:119:13
    |
119 |             self.published.push(SystemEvent::from(event));
    |             ^^^^^^^^^^^^^^^^^^^^------------------------^
    |             |                   |
    |             |                   any code following this expression is unreachable
    |             unreachable expression
    |
note: this expression has type `SystemEvent`, which is uninhabited
   --> generated/semantic-behavior/src/system.rs:119:33
    |
119 |             self.published.push(SystemEvent::from(event));
    |                                 ^^^^^^^^^^^^^^^^^^^^^^^^

warning: `codegate_semantic` (lib) generated 2 warnings
   Compiling codegate-cli v0.2.1 (<worktree>)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.47s
     Running unittests src/lib.rs (target/debug/deps/codegate-375b788e09f29026)

running 1 test
test collection::tests::mutation_after_read_refuses_the_actual_collection_pipeline ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running unittests src/main.rs (target/debug/deps/codegate-e749360e6e04d265)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/codegate-check.rs (target/debug/deps/codegate_check-d6c47b0133e15be5)

running 1 test
test tests::gate_clean_external_target ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running unittests src/bin/codegate-docs.rs (target/debug/deps/codegate_docs-b32e37152d0d8bd8)

running 5 tests
test tests::invalid_commits_are_refused ... ok
test tests::broken_or_duplicate_anchors_are_refused ... ok
test tests::authored_site_and_publication_identity_are_valid ... ok
test tests::published_json_example_reaches_the_real_evaluator ... ok
test tests::wrong_routes_assets_and_internal_source_links_are_refused ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/boundary.rs (target/debug/deps/boundary-a8b3e81c712362f9)

running 5 tests
test generated_integer_obligation_is_checked ... ok
test explicit_absence_bridge ... ok
test strict_json_structure ... ok
test all_generated_fields_roundtrip ... ok
test decoder_limits ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.11s

     Running tests/cli.rs (target/debug/deps/cli-e3e73d6e885e2eb4)

running 3 tests
test cli_structural_refusal_emits_no_report_or_output_file ... ok
test unsupported_policy_format_is_semantic_error ... ok
test cli_reports_full_outcomes_and_coverage_aware_exits_without_tools ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running tests/collection_adversary.rs (target/debug/deps/collection_adversary-a19f26d8e20ed061)

running 4 tests
test malformed_requests_are_refused_and_valid_missing_source_retains_failed_coverage ... ok
test missing_slot_cannot_leave_observed_slot_declarations_complete ... ok
test syntax_failure_retains_real_identity_and_partial_sources ... ok
test registered_slots_are_deterministic_and_stale_configuration_is_rejected ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running tests/collection_composition.rs (target/debug/deps/collection_composition-8634957ca4d66645)

running 10 tests
test source_helpers_keep_byte_coordinates_and_unambiguous_identity_parts ... ok
test failed_capture_returns_only_observed_empty_content_and_actual_gaps ... ok
test fabricated_binding_identity_is_refused_before_returning_a_response ... ok
test semantic_mode_does_not_invoke_source_registration_as_an_adapter ... ok
test invalid_request_is_an_invocation_error_and_clears_retained_response ... ok
test registration_consumes_retained_bytes_and_never_infers_unsupported_family_completion ... ok
test semantic_request_preserves_mode_and_reports_unimplemented_adapters ... ok
test partial_capture_retains_every_gap_and_cannot_claim_complete_sources ... ok
test source_only_composes_real_capture_without_inventing_extraction ... ok
test source_slots_cannot_certify_another_languages_units_or_evidence ... ok

test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running tests/collection_conformance.rs (target/debug/deps/collection_conformance-495a2687fbf6c736)

running 1 test
test real_collection_conforms_to_selected_component ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.35s

     Running tests/conformance.rs (target/debug/deps/conformance-0fb5b4dfc80c7372)

running 1 test
test real_evaluator_conforms_to_complete_combined_suite ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.21s

     Running tests/foundation_conformance.rs (target/debug/deps/foundation_conformance-fd4dfff0636d38af)

running 1 test
test real_foundation_conforms_to_selected_component ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.22s

     Running tests/freshness.rs (target/debug/deps/freshness-a50ca5002bf771ba)

running 1 test
test gate_requires_this_invocations_conformance_producer ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.34s

     Running tests/gate_adversary.rs (target/debug/deps/gate_adversary-399379c8573b56b8)

running 1 test
test unchanged_generated_contract_ignores_local_ownership_state ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.58s

     Running tests/identity_adversary.rs (target/debug/deps/identity_adversary-2d540c836ec8ad54)

running 1 test
test independent_ess_identity_literals_bind_selected_content_and_manifests ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/obligation.rs (target/debug/deps/obligation-9bd2baa4b79e283b)

running 2 tests
test evaluate_obligation_is_fulfilled ... ok
test trait_adapter_never_leaks_a_previous_response ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/provenance_adversary.rs (target/debug/deps/provenance_adversary-dcf674f1bbe3d906)

running 1 test
test native_report_records_observed_completion_time ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.46s

     Running tests/semantic_admission.rs (target/debug/deps/semantic_admission-e4d8e83d88d4254f)

running 11 tests
test stale_evidence_has_a_stable_typed_refusal ... ok
test valid_complete_families_and_partial_evidence_are_admitted ... ok
test typed_resources_are_bounded ... ok
test resolution_and_kind_payloads_cannot_fabricate_facts ... ok
test identities_and_configuration_views_are_checked ... ok
test rich_relationship_families_and_framework_payloads_are_checked ... ok
test dangling_duplicate_and_cycle_references_refuse ... ok
test missing_overlap_and_outside_coverage_never_mean_zero ... ok
test all_family_statuses_have_honest_coverage_semantics ... ok
test invalid_locations_and_line_partitions_refuse_without_overflow ... ok
test failed_tools_and_out_of_selection_files_cannot_look_complete ... ok

test result: ok. 11 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/semantic_boundary.rs (target/debug/deps/semantic_boundary-1c4d0c6250476564)

running 9 tests
test new_and_inline_core_modules_cannot_escape_discovery ... ok
test nested_platform_io_and_thread_context_are_rejected ... ok
test indirect_process_argument_io_is_rejected ... ok
test wiring_projection_and_neutral_admission_are_distinct_from_algorithms ... ok
test public_collection_exports_are_wiring_but_core_calls_remain_forbidden ... ok
test platform_and_thread_aliases_share_the_effect_guard ... ok
test all_non_admitted_dependencies_and_alias_forms_are_rejected ... ok
test injected_io_binding_and_dispatch_are_rejected_by_ast ... ok
test all_shared_modules_and_root_algorithms_obey_boundary ... ok

test result: ok. 9 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.13s

     Running tests/semantic_wire.rs (target/debug/deps/semantic_wire-5867388d243ed368)

running 4 tests
test requests_and_configuration_have_explicit_generated_bridges ... ok
test every_snapshot_family_and_transitive_field_round_trips ... ok
test optional_null_is_absence_but_required_or_unknown_null_is_refused ... ok
test malformed_unknown_duplicate_enums_and_integer_overflow_are_refused ... ok

test result: ok. 4 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s

     Running tests/semantics.rs (target/debug/deps/semantics-fb8dd2218540517c)

running 8 tests
test core_module_boundary_has_no_io_or_language_dispatch ... ok
test dangling_target_guard ... ok
test partial_is_never_complete ... ok
test diagnostic_phase_precedence_and_order ... ok
test labels_and_input_order_do_not_choose_algorithms ... ok
test semantic_invalid_classes ... ok
test parallel_imports_count_unique_destinations ... ok
test every_authored_literal_matches_real_evaluation ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/source_identity.rs (target/debug/deps/source_identity-cd939f2a25e9ae53)

running 6 tests
test explicit_canonical_wire_bytes_have_known_hash ... ok
test root_module_identifiers_are_distinct_from_selected_paths ... ok
test reordering_and_origin_never_change_identity ... ok
test duplicates_and_non_normalized_paths_are_refused ... ok
test selected_bytes_and_configuration_change_identity ... ok
test snapshot_projection_uses_contract_enums_and_omits_absent_optionals ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/source_snapshot.rs (target/debug/deps/source_snapshot-c995f0bdc9b0f767)

running 21 tests
test adversary_maven_cdata_hierarchy_is_observed_or_explicitly_incomplete ... ok
test adversary_selected_source_preserves_ancestor_cargo_configuration ... ok
test confinement_bounds_and_invalid_input_refuse_instead_of_empty_success ... ok
test generated_markers_inside_strings_do_not_exclude_source ... ok
test adversary_round2_unclosed_maven_xml_cannot_claim_complete_selection ... ok
test manifest_references_cannot_select_repository_metadata ... ok
test correction_round2_maven_eof_requires_all_open_elements_closed ... ok
test literal_selection_tests_generated_languages_and_skipped_directories ... ok
test root_and_nested_symlinks_never_escape ... ok
test corrupt_git_and_asserted_configuration_metadata_are_never_trusted ... ok
test correction_maven_mixed_text_cdata_and_whitespace_preserve_literal_hierarchy ... ok
test referenced_manifest_symlinks_are_refused_and_configuration_claims_cannot_panic ... ok
test correction_discovered_configuration_directories_refuse_symlinks ... ok
test three_syntax_classifiers_preserve_physical_bytes_and_comment_lookalikes ... ok
test correction_ancestor_configuration_directories_share_discovery_and_excludes ... ok
test workspace_root_members_do_not_invent_self_parent_cycles ... ok
test static_manifests_preserve_hierarchy_and_report_dynamic_selection ... ok
test selected_dirty_untracked_bytes_configuration_and_manifests_define_identity ... ok
test git_origin_tracks_staged_and_dirty_bytes_without_affecting_identity ... ok
test quoted_go_paths_and_conditional_gradle_includes_have_honest_selection ... ok
test aggregate_bytes_and_file_inventory_are_bounded ... ok

test result: ok. 21 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 6.20s

   Doc-tests codegate

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

## 4. Findings

Nothing found in this bounded pass.

## 5. Attack coverage and limits

Read the complete integrated change, named acceptance, authored response contracts, registration interface, caller wiring, shared admission, collection capture, generated port and native gate integration.

Did not break incomplete discovery handling, stale binding evidence rejection, registry ordering, malformed input refusal, missing-source identity, syntax-error coverage or selection-byte identity with the four added tests.

Existing unchanged package cases also exercised cross-language scope refusal, canonical source helpers, retained response clearing, no semantic downgrade, source capture mutation, semantic boundary aliases, native scenario mutants and invocation freshness.

The native collection harness calls the actual production handler, admits its exact response, compares exact expected responses, checks the four exact scenario IDs and kills its seeded empty-response, false-complete, constant-identity and semantic-downgrade faults. Inspection of the gate shows a fresh output directory, explicit component producer selection and exact regenerated suite/report identity/count/time checks. The complete integration gate remains the coordinator's responsibility; this pass did not separately mutate that new gate plumbing.

Default production registration is empty. Passing this bounded composition suite establishes neither real language extraction nor complete semantic analysis/reporting. No additional speculative findings are returned.

## 6. Paths and retention

No authored file or explicitly selected output directory lies outside the assigned worktree. The prescribed existing sccache and managed lease machinery may update their own tool-managed storage; their configuration was not changed. All new test fixtures and raw logs remain in `.scratch/adversary/`; the package suite additionally wrote its established `target/` test fixtures and native reports. No cleanup was performed. The separate private appendix records the absolute checkout and tool paths for coordination.

```findings
[]
```
