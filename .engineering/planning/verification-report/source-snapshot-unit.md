---
format: aep.planning-md/3
id: verification-report:source-snapshot-unit
kind: verification-report
status: draft
title: Source snapshot unit and retained corrections
relations:
- verifies: story:source-snapshot
revision: 3
---
unit: story:source-snapshot — selected source and build configuration identities
verdict: green
cases: source_snapshot executed 7→14, initial red 6; mutation unit 1 passed; final package 53 passed
origin: n/a
wrote-outside-worktree: $BUILD (exact private handoff path)
needs-coordinator: yes — native ESS, integration gate, independent adversary and module wiring

## Acceptance and delivered seam

collect_source(&CollectionRequest) returns exact retained UTF-8 contents, SourceSnapshot and explicit static-selection/syntax gaps. parse_source(Language,path,source) returns Tree-sitter tree, physical SourceLine observations and located syntax gaps. No declaration binding, public CLI, semantic adapter or complete fact coverage is claimed. The frozen invalid selector ../outside returns precisely InvalidFact / invalid selection path: ../outside / location None.

Source-language files live only in snapshot.files. Manifest/configuration descriptors live in configuration.configuration_files with BuildConfiguration classification; contents retains every input. Opaque config lines are Blank when whitespace-only and Code otherwise. Module identities use root-relative directories with '.' at root. Literal declared package/module names remain parser metadata separate from these directory identities; exact text remains available to later bindings.

## Confirmed scope and diff

Owned product files: src/collection/mod.rs, src/collection/secure_fs.rs, src/collection/manifests.rs, src/bindings/lines.rs, src/bindings/mod.rs and tests/source_snapshot.rs. Authorized shared setup: src/lib.rs adds only public collection/bindings module declarations. No Cargo, generated, identity, bridge, AEP or unrelated source files changed. No commits made. Fixtures are created by Rust tests under their owned build scratch; Go/Java snippets are fixture data only.

## Red-first evidence

Command: CARGO_TARGET_DIR=$BUILD RUSTC_WRAPPER=/usr/bin/sccache CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 cargo test --locked -p codegate-cli --test source_snapshot

Initial seven authored tests executed against refusal stubs before implementation: exit 101, one passed and six failed at runtime.

```text
    Blocking waiting for file lock on package cache
   Compiling unicode-ident v1.0.26
   Compiling proc-macro2 v1.0.107
   Compiling quote v1.0.47
   Compiling libc v0.2.189
   Compiling syn v3.0.6
   Compiling jobserver v0.1.35
   Compiling find-msvc-tools v0.1.14
   Compiling shlex v2.0.1
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
   Compiling stable_deref_trait v1.2.1
   Compiling itoa v1.0.18
   Compiling yoke v0.8.3
   Compiling equivalent v1.0.2
   Compiling hashbrown v0.17.1
   Compiling indexmap v2.14.2
   Compiling zerovec-derive v0.11.6
   Compiling displaydoc v0.2.7
   Compiling serde_json v1.0.151
   Compiling tree-sitter-language v0.1.8
   Compiling zerovec v0.11.8
   Compiling tinystr v0.8.4
   Compiling writeable v0.6.4
   Compiling litemap v0.8.3
   Compiling typenum v1.20.1
   Compiling icu_locale_core v2.3.0
   Compiling hybrid-array v0.4.15
   Compiling potential_utf v0.1.6
   Compiling zerotrie v0.2.5
   Compiling utf8_iter v1.0.4
   Compiling pkg-config v0.3.34
   Compiling icu_properties_data v2.3.0
   Compiling icu_normalizer_data v2.3.0
   Compiling icu_collections v2.3.0
   Compiling icu_provider v2.3.1
   Compiling vcpkg v0.2.15
   Compiling libz-sys v1.1.29
   Compiling block-buffer v0.12.1
   Compiling crypto-common v0.2.2
   Compiling bitflags v2.13.2
   Compiling smallvec v1.16.2
   Compiling const-oid v0.10.2
   Compiling digest v0.11.3
   Compiling icu_normalizer v2.3.0
   Compiling icu_properties v2.3.0
   Compiling syn v2.0.119
   Compiling cfg-if v1.0.5
   Compiling cpufeatures v0.3.1
   Compiling utf8parse v0.2.2
   Compiling anstyle-parse v1.0.0
   Compiling serde_derive_internals v0.29.1
   Compiling sha2 v0.11.0
   Compiling idna_adapter v1.2.2
   Compiling libgit2-sys v0.18.8+1.9.7
   Compiling aho-corasick v1.1.5
   Compiling regex-syntax v0.8.11
   Compiling is_terminal_polyfill v1.70.2
   Compiling schemars v0.8.22
   Compiling percent-encoding v2.3.2
   Compiling anstyle v1.0.14
   Compiling anstyle-query v1.1.5
   Compiling colorchoice v1.0.5
   Compiling thiserror v2.0.21
   Compiling regex-automata v0.4.18
   Compiling anstream v1.0.0
   Compiling form_urlencoded v1.2.2
   Compiling tree-sitter v0.25.10
   Compiling idna v1.1.0
   Compiling schemars_derive v0.8.22
   Compiling toml_datetime v0.6.11
   Compiling serde_spanned v0.6.9
   Compiling thiserror-impl v2.0.21
   Compiling tree-sitter-java v0.23.5
   Compiling tree-sitter-rust v0.24.2
   Compiling tree-sitter-go v0.25.0
   Compiling heck v0.5.0
   Compiling unsafe-libyaml v0.2.11
   Compiling strsim v0.11.1
   Compiling rustix v1.1.5
   Compiling clap_lex v1.1.1
   Compiling toml_write v0.1.2
   Compiling winnow v0.7.15
   Compiling ryu v1.0.23
   Compiling dyn-clone v1.0.20
   Compiling serde_yaml v0.9.34+deprecated
   Compiling toml_edit v0.22.27
   Compiling clap_builder v4.6.7
   Compiling clap_derive v4.6.7
   Compiling regex v1.13.1
   Compiling url v2.5.8
   Compiling linux-raw-sys v0.12.1
   Compiling log v0.4.34
   Compiling streaming-iterator v0.1.9
   Compiling git2 v0.20.4
   Compiling ess-primitives v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
   Compiling clap v4.6.7
   Compiling toml v0.8.23
   Compiling codegate-semantic-contract v0.0.0 ($WORKTREE/generated/semantic-wire)
   Compiling codegate-contract v0.0.0 ($WORKTREE/generated/wire)
   Compiling quick-xml v0.37.5
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
   Compiling codegate-cli v0.1.0 ($WORKTREE)
   Compiling ess-domain v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
   Compiling unicase v2.9.0
   Compiling pulldown-cmark-escape v0.11.0
   Compiling ess-compiler v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
   Compiling ess-gen v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
   Compiling ess-conformance v0.50.0 (https://github.com/beyond10x/ess?rev=8700d0808e8f3b19711629d8a17afc5281680f58#8700d080)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1m 08s
     Running tests/source_snapshot.rs ($BUILD/debug/deps/source_snapshot-5ddacd419907a488)

running 7 tests
test selected_dirty_untracked_bytes_configuration_and_manifests_define_identity ... FAILED
test root_and_nested_symlinks_never_escape ... FAILED
test three_syntax_classifiers_preserve_physical_bytes_and_comment_lookalikes ... FAILED
test literal_selection_tests_generated_languages_and_skipped_directories ... FAILED
test static_manifests_preserve_hierarchy_and_report_dynamic_selection ... FAILED
test confinement_bounds_and_invalid_input_refuse_instead_of_empty_success ... ok
test git_origin_tracks_staged_and_dirty_bytes_without_affecting_identity ... FAILED

failures:

---- selected_dirty_untracked_bytes_configuration_and_manifests_define_identity stdout ----

thread 'selected_dirty_untracked_bytes_configuration_and_manifests_define_identity' (2786422) panicked at tests/source_snapshot.rs:10:56:
called `Result::unwrap()` on an `Err` value: []

---- root_and_nested_symlinks_never_escape stdout ----

thread 'root_and_nested_symlinks_never_escape' (2786421) panicked at tests/source_snapshot.rs:60:108:
called `Result::unwrap()` on an `Err` value: []
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- three_syntax_classifiers_preserve_physical_bytes_and_comment_lookalikes stdout ----

thread 'three_syntax_classifiers_preserve_physical_bytes_and_comment_lookalikes' (2786424) panicked at tests/source_snapshot.rs:21:51:
called `Result::unwrap()` on an `Err` value: Gap { code: UnsupportedCapability, reason: "not implemented", location: None }

---- literal_selection_tests_generated_languages_and_skipped_directories stdout ----

thread 'literal_selection_tests_generated_languages_and_skipped_directories' (2786420) panicked at tests/source_snapshot.rs:32:34:
called `Result::unwrap()` on an `Err` value: []

---- static_manifests_preserve_hierarchy_and_report_dynamic_selection stdout ----

thread 'static_manifests_preserve_hierarchy_and_report_dynamic_selection' (2786423) panicked at tests/source_snapshot.rs:49:45:
called `Result::unwrap()` on an `Err` value: []

---- git_origin_tracks_staged_and_dirty_bytes_without_affecting_identity stdout ----

thread 'git_origin_tracks_staged_and_dirty_bytes_without_affecting_identity' (2786419) panicked at tests/source_snapshot.rs:40:143:
called `Result::unwrap()` on an `Err` value: []


failures:
    git_origin_tracks_staged_and_dirty_bytes_without_affecting_identity
    literal_selection_tests_generated_languages_and_skipped_directories
    root_and_nested_symlinks_never_escape
    selected_dirty_untracked_bytes_configuration_and_manifests_define_identity
    static_manifests_preserve_hierarchy_and_report_dynamic_selection
    three_syntax_classifiers_preserve_physical_bytes_and_comment_lookalikes

test result: FAILED. 1 passed; 6 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p codegate-cli --test source_snapshot`
```

Additional real red probes retained: robustness-red.log (10 executed, 8 passed, 2 failed) found generated markers inside raw strings being misclassified and quoted Go workspace paths misparsed; root-module-red.log found false self-parent cycles for workspace root membership; metadata-red.log found indirect manifest references could select repository metadata. Fixes use syntax comment nodes, quoted-token handling, directory-identity-aware membership and the same forbidden metadata rule across direct/configuration/manifest selection. No failing assertion was removed.

## Final green evidence

Command: PATH=<ESS 0.50.0>:$PATH CARGO_TARGET_DIR=$BUILD RUSTC_WRAPPER=/usr/bin/sccache CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 cargo test --locked -p codegate-cli

Exit 0; 53 package tests passed across runner summaries. source_snapshot increased 7→14, failed 6→0; the library mutation/removal unit contributes one additional case. The full package baseline was not executed before edits in this tree, so no pre-change package total is asserted. The real existing legacy conformance integration test passes independently with its 27 evaluator scenarios.

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
    Finished `test` profile [unoptimized + debuginfo] target(s) in 4.80s
     Running unittests src/lib.rs ($BUILD/debug/deps/codegate-13f02747b00aad71)

running 1 test
test collection::tests::mutation_after_read_refuses_the_actual_collection_pipeline ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

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
test tests::broken_or_duplicate_anchors_are_refused ... ok
test tests::authored_site_and_publication_identity_are_valid ... ok
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

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.14s

     Running tests/cli.rs ($BUILD/debug/deps/cli-9d10002ea0f5bbfd)

running 3 tests
test cli_structural_refusal_emits_no_report_or_output_file ... ok
test unsupported_policy_format_is_semantic_error ... ok
test cli_reports_full_outcomes_and_coverage_aware_exits_without_tools ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/conformance.rs ($BUILD/debug/deps/conformance-d2c03fe9842dc916)

running 1 test
test real_evaluator_conforms_to_complete_combined_suite ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s

     Running tests/freshness.rs ($BUILD/debug/deps/freshness-f5d37c077f3df08b)

running 1 test
test gate_requires_this_invocations_conformance_producer ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.15s

     Running tests/gate_adversary.rs ($BUILD/debug/deps/gate_adversary-3717d0214c10e009)

running 1 test
test unchanged_generated_contract_ignores_local_ownership_state ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.66s

     Running tests/obligation.rs ($BUILD/debug/deps/obligation-885b816d89ea5c89)

running 2 tests
test evaluate_obligation_is_fulfilled ... ok
test trait_adapter_never_leaks_a_previous_response ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/provenance_adversary.rs ($BUILD/debug/deps/provenance_adversary-a79347a37b559ada)

running 1 test
test native_report_records_observed_completion_time ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.29s

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
test partial_is_never_complete ... ok
test parallel_imports_count_unique_destinations ... ok
test diagnostic_phase_precedence_and_order ... ok
test labels_and_input_order_do_not_choose_algorithms ... ok
test semantic_invalid_classes ... ok
test every_authored_literal_matches_real_evaluation ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/source_identity.rs ($BUILD/debug/deps/source_identity-cacab3891d5875b4)

running 6 tests
test explicit_canonical_wire_bytes_have_known_hash ... ok
test root_module_identifiers_are_distinct_from_selected_paths ... ok
test reordering_and_origin_never_change_identity ... ok
test snapshot_projection_uses_contract_enums_and_omits_absent_optionals ... ok
test duplicates_and_non_normalized_paths_are_refused ... ok
test selected_bytes_and_configuration_change_identity ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/source_snapshot.rs ($BUILD/debug/deps/source_snapshot-5ddacd419907a488)

running 14 tests
test three_syntax_classifiers_preserve_physical_bytes_and_comment_lookalikes ... ok
test generated_markers_inside_strings_do_not_exclude_source ... ok
test root_and_nested_symlinks_never_escape ... ok
test workspace_root_members_do_not_invent_self_parent_cycles ... ok
test corrupt_git_and_asserted_configuration_metadata_are_never_trusted ... ok
test manifest_references_cannot_select_repository_metadata ... ok
test literal_selection_tests_generated_languages_and_skipped_directories ... ok
test quoted_go_paths_and_conditional_gradle_includes_have_honest_selection ... ok
test static_manifests_preserve_hierarchy_and_report_dynamic_selection ... ok
test confinement_bounds_and_invalid_input_refuse_instead_of_empty_success ... ok
test referenced_manifest_symlinks_are_refused_and_configuration_claims_cannot_panic ... ok
test git_origin_tracks_staged_and_dirty_bytes_without_affecting_identity ... ok
test selected_dirty_untracked_bytes_configuration_and_manifests_define_identity ... ok
test aggregate_bytes_and_file_inventory_are_bounded ... ok

test result: ok. 14 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.98s

   Doc-tests codegate

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

cargo fmt --check: exit 0.
CARGO_TARGET_DIR=$BUILD RUSTC_WRAPPER=/usr/bin/sccache CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 cargo clippy --locked -p codegate-cli --all-targets -- -D warnings: exit 0.
git diff --check: exit 0.

## Evidence boundaries and remaining work

The collector executes no processes or network operations. Git provenance reads libgit2 HEAD/index objects and compares exact retained bytes; malformed provenance refuses collection. Filesystem input uses anchored directory handles and NOFOLLOW traversal, including the root and referenced manifests, plus regular-file checks and bounded reads. Exact limits are 4 MiB/file, 10,000 retained files and 64 MiB retained bytes, with an additional 100,000 traversal-entry guard. Symlink and metadata traversal fail instead of escaping.

Static manifest support includes Cargo literal workspaces, Go literal workspaces, Maven literal hierarchy and Gradle literal includes. Dynamic expressions, unresolved properties, unsupported workspace expansion, excluded/missing modules and ambiguous hierarchy retain explicit gaps. No effective classpath/build/runtime relationship is inferred. Source-origin tracking is omitted from canonical identity by the supplied coordinator seam. Syntax-error observations remain gaps, including parser error nodes hidden inside literal ranges.

Opened-file stamps (length, modification time, Unix inode/device/change time) are checked around reads and again before finalization. The private real-pipeline test mutates/removes a captured file at that boundary and proves SourceChangedDuringCollection refusal. This is change detection, not a claim of an atomic multi-file filesystem snapshot. The current confinement backend is tested on Linux/Unix. Effective generated-source roots or arbitrary build execution remain later semantic/build evidence responsibilities.

Native foundation ESS execution and the complete integration task check remain coordinator-owned. No semantic conformance or parity completion is claimed by package tests.

## Retained output and handoff

Raw logs under $WORKTREE/.scratch/unit: red.log, green-targeted.log, robustness-red.log, robustness-green.log, expanded-tests.log, root-module-red.log, metadata-red.log, green-suite.log, clippy.log and fmt.log. All external compiler/fixture output remains under $BUILD; test-owned fixtures include the bounded 68 MiB corpus and 10,001-file inventory. Standard Cargo/sccache caches are tool-managed. Existing package tests also retain ignored repository-local target evidence. No cleanup or publication was performed; coordinator owns review/integration and exact output cleanup.

unit: story:source-snapshot — source adversary correction
verdict: green
cases: package executed 53→58; source_snapshot 14→19; correction red 5
origin: n/a
wrote-outside-worktree: $BUILD (existing exact private handoff target)
needs-coordinator: yes — final independent adversary and native ESS/integration gate

## 1. Acceptance and correction class

Retain applicable build configuration when selecting a source subtree, and preserve literal Maven module/parent text rather than silently dropping valid CDATA. The two unchanged adversary expectations were reproduced red before correction. Original report.md, adversary.md and their original logs remain unchanged.

Discovery class: recognized configuration containers owned by an ancestor of the selected source remain relevant even when they are siblings of that source. The shared owner computation handles both supported containers, .cargo and .mvn, at root and nested project directories. The added mutation table covers .cargo/config.toml, .mvn/maven.config, .mvn/jvm.config and nested/.cargo/config.toml; each retained mutation changes identity. Excluding those directories prevents retention and identity changes. A symlink in the discovered path refuses collection unless explicitly excluded. Existing metadata exclusion, anchored NOFOLLOW access and traversal limits remain in force. This is bounded static discovery, not a claim to interpret arbitrary build configuration locations.

XML class: scalar text may be split between ordinary XML text, CDATA and comments. The parser concatenates decoded text/CDATA into the same scalar accumulator and trims only the completed value. It applies to every scalar already interpreted by the static Maven parser, including module, relativePath, artifactId and unresolved-property/profile checks. The independent CDATA hierarchy case and added mixed-text/CDATA whitespace case both pass. Malformed XML/entity handling and external/dynamic build gaps remain explicit.

## 2. Diff and owned files

Correction changed only src/collection/mod.rs, src/collection/manifests.rs and tests/source_snapshot.rs. Three new class tests supplement the two unchanged adversary tests. All existing implementation remains uncommitted for coordinator integration. No Cargo/generated/AEP/identity/wire edits or commits.

Actual tracked git diff --stat (new owned source/tests are untracked):

```text
 src/lib.rs | 2 ++
 1 file changed, 2 insertions(+)
```

## 3. Runtime red evidence

Both commands use CARGO_TARGET_DIR=$BUILD, RUSTC_WRAPPER=/usr/bin/sccache, CARGO_BUILD_JOBS=2, CARGO_INCREMENTAL=0 and the pinned ESS toolchain on PATH. Log copies below replace only local root/build paths with $WORKTREE/$BUILD.

Command: cargo test --locked -p codegate-cli --test source_snapshot adversary_
Exit: 101. Executed 2; passed 0; failed 2.

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
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.06s
     Running tests/source_snapshot.rs ($BUILD/debug/deps/source_snapshot-5ddacd419907a488)

running 2 tests
test adversary_maven_cdata_hierarchy_is_observed_or_explicitly_incomplete ... FAILED
test adversary_selected_source_preserves_ancestor_cargo_configuration ... FAILED

failures:

---- adversary_maven_cdata_hierarchy_is_observed_or_explicitly_incomplete stdout ----

thread 'adversary_maven_cdata_hierarchy_is_observed_or_explicitly_incomplete' (3395018) panicked at tests/source_snapshot.rs:522:5:
valid CDATA module/parent declarations were silently discarded: parent=None, gaps=[]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- adversary_selected_source_preserves_ancestor_cargo_configuration stdout ----

thread 'adversary_selected_source_preserves_ancestor_cargo_configuration' (3395019) panicked at tests/source_snapshot.rs:494:5:
assertion `left != right` failed: changing ancestor Cargo configuration must change the selected source configuration identity
  left: ConfigurationId("523ea72e5365fd680306cd5c177abd5185ffd54fac458cc42525cf2778d8f7cb")
 right: ConfigurationId("523ea72e5365fd680306cd5c177abd5185ffd54fac458cc42525cf2778d8f7cb")


failures:
    adversary_maven_cdata_hierarchy_is_observed_or_explicitly_incomplete
    adversary_selected_source_preserves_ancestor_cargo_configuration

test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 14 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p codegate-cli --test source_snapshot`
```

Command: cargo test --locked -p codegate-cli --test source_snapshot correction_
Exit: 101. Executed 3; passed 0; failed 3.

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
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.34s
     Running tests/source_snapshot.rs ($BUILD/debug/deps/source_snapshot-5ddacd419907a488)

running 3 tests
test correction_discovered_configuration_directories_refuse_symlinks ... FAILED
test correction_maven_mixed_text_cdata_and_whitespace_preserve_literal_hierarchy ... FAILED
test correction_ancestor_configuration_directories_share_discovery_and_excludes ... FAILED

failures:

---- correction_discovered_configuration_directories_refuse_symlinks stdout ----

thread 'correction_discovered_configuration_directories_refuse_symlinks' (3404486) panicked at tests/source_snapshot.rs:558:250:
assertion failed: collect_source(&req).is_err()
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- correction_maven_mixed_text_cdata_and_whitespace_preserve_literal_hierarchy stdout ----

thread 'correction_maven_mixed_text_cdata_and_whitespace_preserve_literal_hierarchy' (3404487) panicked at tests/source_snapshot.rs:551:57:
[Gap { code: MissingBuildSelection, reason: "declared module is unavailable: child/pom.xml", location: Some(SourceRange { path: "pom.xml", start_byte: 0, end_byte: 0, start_line: 0, start_column: 0, end_line: 0, end_column: 0 }) }, Gap { code: MissingBuildSelection, reason: "cyclic declared module hierarchy", location: Some(SourceRange { path: "child space/pom.xml", start_byte: 0, end_byte: 0, start_line: 0, start_column: 0, end_line: 0, end_column: 0 }) }]

---- correction_ancestor_configuration_directories_share_discovery_and_excludes stdout ----

thread 'correction_ancestor_configuration_directories_share_discovery_and_excludes' (3404485) panicked at tests/source_snapshot.rs:541:26:
omitted .cargo/config.toml


failures:
    correction_ancestor_configuration_directories_share_discovery_and_excludes
    correction_discovered_configuration_directories_refuse_symlinks
    correction_maven_mixed_text_cdata_and_whitespace_preserve_literal_hierarchy

test result: FAILED. 0 passed; 3 failed; 0 ignored; 0 measured; 16 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p codegate-cli --test source_snapshot`
```

## 4. Green verification

Command: cargo test --locked -p codegate-cli
Exit: 0. Package executed 53→58, all passed (sum of runner summary lines; previous same-command run retained in green-suite.log).
Source snapshot lane executed 14→19, exit 0 (intermediate adversary suite executed 16: 14 passed, 2 failed).
Filtered adversary lane executed 2→2, red 2→0, exit 0; no cases added to that filter.
Filtered correction lane executed 3→3, red 3→0, exit 0; all three authored before correction.

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
    Finished `test` profile [unoptimized + debuginfo] target(s) in 2.39s
     Running unittests src/lib.rs ($BUILD/debug/deps/codegate-13f02747b00aad71)

running 1 test
test collection::tests::mutation_after_read_refuses_the_actual_collection_pipeline ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

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
test tests::broken_or_duplicate_anchors_are_refused ... ok
test tests::authored_site_and_publication_identity_are_valid ... ok
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

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.14s

     Running tests/cli.rs ($BUILD/debug/deps/cli-9d10002ea0f5bbfd)

running 3 tests
test cli_structural_refusal_emits_no_report_or_output_file ... ok
test unsupported_policy_format_is_semantic_error ... ok
test cli_reports_full_outcomes_and_coverage_aware_exits_without_tools ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/conformance.rs ($BUILD/debug/deps/conformance-d2c03fe9842dc916)

running 1 test
test real_evaluator_conforms_to_complete_combined_suite ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.20s

     Running tests/freshness.rs ($BUILD/debug/deps/freshness-f5d37c077f3df08b)

running 1 test
test gate_requires_this_invocations_conformance_producer ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.16s

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

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.28s

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
test parallel_imports_count_unique_destinations ... ok
test partial_is_never_complete ... ok
test dangling_target_guard ... ok
test labels_and_input_order_do_not_choose_algorithms ... ok
test diagnostic_phase_precedence_and_order ... ok
test semantic_invalid_classes ... ok
test every_authored_literal_matches_real_evaluation ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/source_identity.rs ($BUILD/debug/deps/source_identity-cacab3891d5875b4)

running 6 tests
test explicit_canonical_wire_bytes_have_known_hash ... ok
test snapshot_projection_uses_contract_enums_and_omits_absent_optionals ... ok
test reordering_and_origin_never_change_identity ... ok
test root_module_identifiers_are_distinct_from_selected_paths ... ok
test duplicates_and_non_normalized_paths_are_refused ... ok
test selected_bytes_and_configuration_change_identity ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/source_snapshot.rs ($BUILD/debug/deps/source_snapshot-5ddacd419907a488)

running 19 tests
test three_syntax_classifiers_preserve_physical_bytes_and_comment_lookalikes ... ok
test generated_markers_inside_strings_do_not_exclude_source ... ok
test root_and_nested_symlinks_never_escape ... ok
test correction_maven_mixed_text_cdata_and_whitespace_preserve_literal_hierarchy ... ok
test correction_discovered_configuration_directories_refuse_symlinks ... ok
test workspace_root_members_do_not_invent_self_parent_cycles ... ok
test corrupt_git_and_asserted_configuration_metadata_are_never_trusted ... ok
test referenced_manifest_symlinks_are_refused_and_configuration_claims_cannot_panic ... ok
test literal_selection_tests_generated_languages_and_skipped_directories ... ok
test static_manifests_preserve_hierarchy_and_report_dynamic_selection ... ok
test manifest_references_cannot_select_repository_metadata ... ok
test quoted_go_paths_and_conditional_gradle_includes_have_honest_selection ... ok
test adversary_maven_cdata_hierarchy_is_observed_or_explicitly_incomplete ... ok
test confinement_bounds_and_invalid_input_refuse_instead_of_empty_success ... ok
test git_origin_tracks_staged_and_dirty_bytes_without_affecting_identity ... ok
test adversary_selected_source_preserves_ancestor_cargo_configuration ... ok
test selected_dirty_untracked_bytes_configuration_and_manifests_define_identity ... ok
test correction_ancestor_configuration_directories_share_discovery_and_excludes ... ok
test aggregate_bytes_and_file_inventory_are_bounded ... ok

test result: ok. 19 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.69s

   Doc-tests codegate

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

cargo fmt --check: exit 0 (correction-fmt.log).
cargo clippy --locked -p codegate-cli --all-targets -- -D warnings: exit 0 (correction-clippy.log).
git diff --check: exit 0.
No root warning suppression. Generated semantic-behavior retains its existing documented uninhabited-event warning.

## 5. Boundaries and handoff

No semantic adapter, declarations, CLI or complete semantic evidence claim added. Static supported manifest/config selection remains the scope. Full native ESS/task check and final independent adversary are coordinator-owned. No publication, source cleanup or build removal performed. Source/test bytes are stable after these checks.

## 6. Retained outputs

All correction logs/report are under $WORKTREE/.scratch/unit: correction-red.log, correction-class-red.log, correction-green-targeted.log, correction-adversary-green.log, correction-class-green.log, correction-green-suite.log, correction-fmt.log, correction-clippy.log, correction-report.md. Existing original worker/adversary evidence is preserved.

Outside worktree: reused $BUILD only, exact path supplied in private handoff: /tmp/codegate-source-worker.AAF0ck. Standard Cargo/sccache caches are tool-managed. No other output path was created for this correction. Coordinator owns target cleanup after review.

unit: story:source-snapshot — final malformed Maven EOF correction
verdict: green
cases: package executed 58→60; source_snapshot 19→21; correction red 2
origin: n/a
wrote-outside-worktree: $BUILD (existing exact private handoff target)
needs-coordinator: yes — integration and native ESS gate; no third adversary campaign

## 1. Acceptance and correction class

A Maven XML stream that reaches EOF with an open element must retain a MissingBuildSelection gap. The unchanged adversary regression was reproduced red before implementation. The parser now checks its complete open-element stack at EOF rather than treating the streaming reader's EOF as document completeness. Existing parser-error handling already covers mismatched/invalid tokens. This small correction applies at every element depth; it does not claim full Maven schema or effective build validation.

Class enumeration: root project, modules container, module scalar, parent container and CDATA relativePath scalar are five truncation points in one table-driven regression. Every one requires the existing path-qualified invalid-Maven-XML gap. Existing valid ordinary text/CDATA hierarchy tests remain green. No adversary expectation was changed.

The initial authored class test used an unqualified reason string. The collector's existing incomplete() helper prefixes pom.xml. After observing this test-author mistake, I corrected only my new test, removed the EOF guard, reran that corrected test red, then restored the same guard. Both attempts are retained; the verified red below is against the final exact assertion.

## 2. Diff and ownership

Round-two correction writes only src/collection/manifests.rs and tests/source_snapshot.rs, plus this separate report/logs. Product change is the EOF stack guard. One class test was added after the independent adversary's one new test. No AEP, commits, Cargo, generated contracts, identities, bridge or public CLI changes.

Actual tracked git diff --stat (owned new source/tests remain untracked):

```text
 src/lib.rs | 2 ++
 1 file changed, 2 insertions(+)
```

## 3. Red-first runtime evidence

Commands use CARGO_TARGET_DIR=$BUILD RUSTC_WRAPPER=/usr/bin/sccache CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 and the pinned ESS toolchain on PATH. Only local root/build labels are replaced below.

cargo test --locked -p codegate-cli --test source_snapshot adversary_round2_
Exit 101; executed 1, failed 1.

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
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running tests/source_snapshot.rs ($BUILD/debug/deps/source_snapshot-5ddacd419907a488)

running 1 test
test adversary_round2_unclosed_maven_xml_cannot_claim_complete_selection ... FAILED

failures:

---- adversary_round2_unclosed_maven_xml_cannot_claim_complete_selection stdout ----

thread 'adversary_round2_unclosed_maven_xml_cannot_claim_complete_selection' (3512951) panicked at tests/source_snapshot.rs:633:23:
unclosed Maven XML returned no build-selection gap: []
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    adversary_round2_unclosed_maven_xml_cannot_claim_complete_selection

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 19 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p codegate-cli --test source_snapshot`
```

cargo test --locked -p codegate-cli --test source_snapshot correction_round2_
Exit 101; executed 1, failed 1. Final exact assertion verified red with guard removed:

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
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.90s
     Running tests/source_snapshot.rs ($BUILD/debug/deps/source_snapshot-5ddacd419907a488)

running 1 test
test correction_round2_maven_eof_requires_all_open_elements_closed ... FAILED

failures:

---- correction_round2_maven_eof_requires_all_open_elements_closed stdout ----

thread 'correction_round2_maven_eof_requires_all_open_elements_closed' (3540535) panicked at tests/source_snapshot.rs:656:9:
truncated XML "<project>" lacked an XML completeness gap: []
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    correction_round2_maven_eof_requires_all_open_elements_closed

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 20 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p codegate-cli --test source_snapshot`
```

## 4. Green verification

cargo test --locked -p codegate-cli
Exit 0. Package executed 58→60, all passed (runner summary-line sums; prior command in correction-green-suite.log). source_snapshot lane executed 19→21, all passed; intermediate independent adversary lane was 20 executed, 19 passed/1 failed. Final filtered round2_ lane executes both retained regressions, 2 passed/0 failed, exit 0 (correction-round2-targeted-green.log).

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
    Finished `test` profile [unoptimized + debuginfo] target(s) in 3.09s
     Running unittests src/lib.rs ($BUILD/debug/deps/codegate-13f02747b00aad71)

running 1 test
test collection::tests::mutation_after_read_refuses_the_actual_collection_pipeline ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

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

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.21s

     Running tests/freshness.rs ($BUILD/debug/deps/freshness-f5d37c077f3df08b)

running 1 test
test gate_requires_this_invocations_conformance_producer ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.11s

     Running tests/gate_adversary.rs ($BUILD/debug/deps/gate_adversary-3717d0214c10e009)

running 1 test
test unchanged_generated_contract_ignores_local_ownership_state ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.69s

     Running tests/obligation.rs ($BUILD/debug/deps/obligation-885b816d89ea5c89)

running 2 tests
test evaluate_obligation_is_fulfilled ... ok
test trait_adapter_never_leaks_a_previous_response ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/provenance_adversary.rs ($BUILD/debug/deps/provenance_adversary-a79347a37b559ada)

running 1 test
test native_report_records_observed_completion_time ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.30s

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
test partial_is_never_complete ... ok
test parallel_imports_count_unique_destinations ... ok
test diagnostic_phase_precedence_and_order ... ok
test labels_and_input_order_do_not_choose_algorithms ... ok
test semantic_invalid_classes ... ok
test every_authored_literal_matches_real_evaluation ... ok

test result: ok. 8 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/source_identity.rs ($BUILD/debug/deps/source_identity-cacab3891d5875b4)

running 6 tests
test explicit_canonical_wire_bytes_have_known_hash ... ok
test root_module_identifiers_are_distinct_from_selected_paths ... ok
test reordering_and_origin_never_change_identity ... ok
test snapshot_projection_uses_contract_enums_and_omits_absent_optionals ... ok
test duplicates_and_non_normalized_paths_are_refused ... ok
test selected_bytes_and_configuration_change_identity ... ok

test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

     Running tests/source_snapshot.rs ($BUILD/debug/deps/source_snapshot-5ddacd419907a488)

running 21 tests
test three_syntax_classifiers_preserve_physical_bytes_and_comment_lookalikes ... ok
test generated_markers_inside_strings_do_not_exclude_source ... ok
test correction_maven_mixed_text_cdata_and_whitespace_preserve_literal_hierarchy ... ok
test workspace_root_members_do_not_invent_self_parent_cycles ... ok
test root_and_nested_symlinks_never_escape ... ok
test corrupt_git_and_asserted_configuration_metadata_are_never_trusted ... ok
test literal_selection_tests_generated_languages_and_skipped_directories ... ok
test manifest_references_cannot_select_repository_metadata ... ok
test correction_round2_maven_eof_requires_all_open_elements_closed ... ok
test quoted_go_paths_and_conditional_gradle_includes_have_honest_selection ... ok
test static_manifests_preserve_hierarchy_and_report_dynamic_selection ... ok
test correction_discovered_configuration_directories_refuse_symlinks ... ok
test referenced_manifest_symlinks_are_refused_and_configuration_claims_cannot_panic ... ok
test adversary_round2_unclosed_maven_xml_cannot_claim_complete_selection ... ok
test confinement_bounds_and_invalid_input_refuse_instead_of_empty_success ... ok
test adversary_selected_source_preserves_ancestor_cargo_configuration ... ok
test adversary_maven_cdata_hierarchy_is_observed_or_explicitly_incomplete ... ok
test git_origin_tracks_staged_and_dirty_bytes_without_affecting_identity ... ok
test selected_dirty_untracked_bytes_configuration_and_manifests_define_identity ... ok
test correction_ancestor_configuration_directories_share_discovery_and_excludes ... ok
test aggregate_bytes_and_file_inventory_are_bounded ... ok

test result: ok. 21 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.56s

   Doc-tests codegate

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

cargo fmt --check: exit 0 (correction-round2-fmt.log).
cargo clippy --locked -p codegate-cli --all-targets -- -D warnings: exit 0 (correction-round2-clippy.log).
git diff --check: exit 0.

## 5. Boundaries

No third attack campaign. Original worker and both adversary reports remain unchanged. No full integration task check or native foundation ESS execution is claimed here. Coordinator owns those checks and subsequent integration/publication. Existing documented generated semantic-behavior warning remains; no new warning suppressions were added.

## 6. Outputs and handoff

Raw evidence is retained under $WORKTREE/.scratch/unit: correction-round2-red.log, correction-round2-class-red.log, correction-round2-class-red-verified.log, correction-round2-source-green.log (intermediate run documenting my reason-prefix test mistake), correction-round2-package-green.log, correction-round2-targeted-green.log, correction-round2-fmt.log, correction-round2-clippy.log, correction-round-2-report.md.

Outside worktree only reused $BUILD: /tmp/codegate-source-worker.AAF0ck. Standard Cargo/sccache caches are tool-managed. No new large target or output tree. No deletion/cleanup performed. Source/test bytes are stable; builds stopped and own lease released for coordinator handoff.
