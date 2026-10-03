---
format: aep.planning-md/3
id: review-result:source-collection-adversary-round-1
kind: review-result
status: active
title: Independent source configuration and Maven attacks
relations:
- reviews: story:source-snapshot
revision: 1
---
unit: story:source-snapshot — stable worker tree wt-59fefee16e98 over cceca5125ee726b34e44bc28f02aeeadb6274fe9
verdict: CONFIRMED
cases: source_snapshot executed 14→16, red 2
origin: introduced 2 / pre-existing 0 / undecided 0
wrote-outside-worktree: one reused compiler/fixture output tree, named below
needs-coordinator: return both reachable failures to source implementor before integration

## 1. Diff and ownership

`git --no-pager diff --stat` includes the implementor's pre-existing unstaged module registration:

```text
 src/lib.rs | 2 ++
 1 file changed, 2 insertions(+)
```

The source files and source_snapshot.rs were untracked when this review began, so that Git command alone cannot distinguish ownership. The immediately pre-attack copy compared to the final test file gives:

```text
 tests/source_snapshot.rs | 71 ++++++++++++++++++++++
 1 file changed, 71 insertions(+)
```

Only those 71 test lines were added by this adversary. No existing case was changed or removed. All six implementation files' SHA-256 values match the pre-attack inventory in adversary-source-before.sha256. The non-test Git diff above is inherited from the implementation handoff, not an adversary implementation edit. The worker had explicitly stopped builds and released its lease before the review acquired its own lease.

## 2. Cases written before execution

Two new cases were written together, then each was run alone before the suite. Both compile and fail at their intended runtime assertion.

### Ancestor Cargo configuration

`tests/source_snapshot.rs:494` calls real collect_source with include_paths=['src'], changes only root .cargo/config.toml rustflags, and requires the configuration identity to change. The case also requires the configuration bytes to be retained. Exact single-case command, with BUILD naming the owned reused target:

```console
CARGO_TARGET_DIR=$BUILD RUSTC_WRAPPER=/usr/bin/sccache CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 cargo test --locked -p codegate-cli --test source_snapshot adversary_selected_source_preserves_ancestor_cargo_configuration -- --exact
```

Exit 101. Verbatim failure excerpt from adversary-cargo-config.log:

```text
running 1 test
test adversary_selected_source_preserves_ancestor_cargo_configuration ... FAILED

failures:

---- adversary_selected_source_preserves_ancestor_cargo_configuration stdout ----

thread 'adversary_selected_source_preserves_ancestor_cargo_configuration' (3354238) panicked at tests/source_snapshot.rs:494:5:
assertion `left != right` failed: changing ancestor Cargo configuration must change the selected source configuration identity
  left: ConfigurationId("523ea72e5365fd680306cd5c177abd5185ffd54fac458cc42525cf2778d8f7cb")
 right: ConfigurationId("523ea72e5365fd680306cd5c177abd5185ffd54fac458cc42525cf2778d8f7cb")
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    adversary_selected_source_preserves_ancestor_cargo_configuration

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.01s
```

### Maven CDATA declarations

`tests/source_snapshot.rs:522` supplies valid XML CDATA module and parent relativePath values to real collection and requires either the declared parent hierarchy or an explicit MissingBuildSelection gap.

```console
CARGO_TARGET_DIR=$BUILD RUSTC_WRAPPER=/usr/bin/sccache CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 cargo test --locked -p codegate-cli --test source_snapshot adversary_maven_cdata_hierarchy_is_observed_or_explicitly_incomplete -- --exact
```

Exit 101. Verbatim failure excerpt from adversary-maven-cdata.log:

```text
running 1 test
test adversary_maven_cdata_hierarchy_is_observed_or_explicitly_incomplete ... FAILED

failures:

---- adversary_maven_cdata_hierarchy_is_observed_or_explicitly_incomplete stdout ----

thread 'adversary_maven_cdata_hierarchy_is_observed_or_explicitly_incomplete' (3355423) panicked at tests/source_snapshot.rs:522:5:
valid CDATA module/parent declarations were silently discarded: parent=None, gaps=[]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    adversary_maven_cdata_hierarchy_is_observed_or_explicitly_incomplete

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 15 filtered out; finished in 0.01s
```

## 3. Suite run after the two isolated failures

```console
CARGO_TARGET_DIR=$BUILD RUSTC_WRAPPER=/usr/bin/sccache CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 cargo test --locked -p codegate-cli --test source_snapshot
```

Exit 101. Verbatim test output is below; compiler preamble and raw full output remain in adversary-suite.log.

```text
running 16 tests
test three_syntax_classifiers_preserve_physical_bytes_and_comment_lookalikes ... ok
test generated_markers_inside_strings_do_not_exclude_source ... ok
test root_and_nested_symlinks_never_escape ... ok
test workspace_root_members_do_not_invent_self_parent_cycles ... ok
test corrupt_git_and_asserted_configuration_metadata_are_never_trusted ... ok
test manifest_references_cannot_select_repository_metadata ... ok
test referenced_manifest_symlinks_are_refused_and_configuration_claims_cannot_panic ... ok
test adversary_maven_cdata_hierarchy_is_observed_or_explicitly_incomplete ... FAILED
test literal_selection_tests_generated_languages_and_skipped_directories ... ok
test static_manifests_preserve_hierarchy_and_report_dynamic_selection ... ok
test quoted_go_paths_and_conditional_gradle_includes_have_honest_selection ... ok
test adversary_selected_source_preserves_ancestor_cargo_configuration ... FAILED
test git_origin_tracks_staged_and_dirty_bytes_without_affecting_identity ... ok
test confinement_bounds_and_invalid_input_refuse_instead_of_empty_success ... ok
test selected_dirty_untracked_bytes_configuration_and_manifests_define_identity ... ok
test aggregate_bytes_and_file_inventory_are_bounded ... ok

failures:

---- adversary_maven_cdata_hierarchy_is_observed_or_explicitly_incomplete stdout ----

thread 'adversary_maven_cdata_hierarchy_is_observed_or_explicitly_incomplete' (3356447) panicked at tests/source_snapshot.rs:522:5:
valid CDATA module/parent declarations were silently discarded: parent=None, gaps=[]
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

---- adversary_selected_source_preserves_ancestor_cargo_configuration stdout ----

thread 'adversary_selected_source_preserves_ancestor_cargo_configuration' (3356448) panicked at tests/source_snapshot.rs:494:5:
assertion `left != right` failed: changing ancestor Cargo configuration must change the selected source configuration identity
  left: ConfigurationId("523ea72e5365fd680306cd5c177abd5185ffd54fac458cc42525cf2778d8f7cb")
 right: ConfigurationId("523ea72e5365fd680306cd5c177abd5185ffd54fac458cc42525cf2778d8f7cb")


failures:
    adversary_maven_cdata_hierarchy_is_observed_or_explicitly_incomplete
    adversary_selected_source_preserves_ancestor_cargo_configuration

test result: FAILED. 14 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.48s

error: test failed, to rerun pass `-p codegate-cli --test source_snapshot`
```

## 4. Findings and reachability

| Location | Verdict / origin | Finding | What was measured | What reaches it |
| --- | --- | --- | --- | --- |
| src/collection/mod.rs:346 | CONFIRMED / introduced | Selecting a source subdirectory omits ancestor .cargo/config.toml, so changing its build configuration leaves the configuration identity unchanged. | tests/source_snapshot.rs:494, isolated test exit101, identical before/after identities | Public collect_source with an ordinary Rust project, include_paths=['src'], and standard root Cargo configuration; no mutation hook or synthetic internal state |
| src/collection/manifests.rs:205 | CONFIRMED / introduced | Maven CDATA module and relativePath text is silently ignored, leaving the declared parent absent with no build-selection gap. | tests/source_snapshot.rs:522, isolated test exit101, parent=None and gaps=[] | Public collect_source with well-formed parent/child POM documents using XML CDATA text; no build execution or parser internals invoked |

Both source files are newly introduced by this unit; the base has no collector path to exercise. Fix the traversal's relevant configuration-directory discovery while retaining confinement and excludes. Handle Maven CData consistently with literal text, or retain an explicit gap when interpretation is unsupported. This adversary applied neither fix.

## 5. Bounds of the attack

The two-case budget was spent on identity completeness and static Maven hierarchy. Existing 14 source_snapshot cases remained green, including root/nested/referenced symlink refusal, metadata confinement and aggregate limits. No additional adversarial concurrency claim is made. The implementor's separate deterministic mutation unit test was inspected but not rerun in this bounded suite.

## 6. Output paths

The only outside-worktree output reused was /tmp/codegate-source-worker.AAF0ck (compiler artifacts and existing source_snapshot fixture machinery). No new external build directory was created. New attack fixtures, raw logs, the pre-attack test copy and source hash inventory are inside .scratch/unit/. No implementation files, AEP artifacts or commits were written; coordinator owns cleanup and next implementation pass.

```findings
- file: src/collection/mod.rs
  line: 346
  category: acceptance
  severity: blocker
  verdict: CONFIRMED
  origin: introduced
  message: Selecting a source subdirectory omits ancestor .cargo/config.toml, so changing its build configuration leaves the configuration identity unchanged.
- file: src/collection/manifests.rs
  line: 205
  category: contract-drift
  severity: blocker
  verdict: CONFIRMED
  origin: introduced
  message: Maven CDATA module and relativePath text is silently ignored, leaving the declared parent absent with no build-selection gap.
```
