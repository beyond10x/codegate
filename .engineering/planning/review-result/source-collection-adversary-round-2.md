---
format: aep.planning-md/3
id: review-result:source-collection-adversary-round-2
kind: review-result
status: active
title: Final source collection adversary pass
relations:
- reviews: story:source-snapshot
revision: 1
---
unit: story:source-snapshot — corrected worker tree wt-59fefee16e98 over cceca5125ee726b34e44bc28f02aeeadb6274fe9
verdict: CONFIRMED
cases: source_snapshot executed 19→20, red 1
origin: introduced 1 / pre-existing 0 / undecided 0
wrote-outside-worktree: one reused compiler/fixture output tree, named below
needs-coordinator: final-round malformed Maven XML finding remains open; no third adversary campaign

## 1. Diff and ownership

`git --no-pager diff --stat`:

```text
 src/lib.rs | 2 ++
 1 file changed, 2 insertions(+)
```

This is the implementor's inherited unstaged registration; newly authored implementation/tests are still untracked. Comparing the pre-round test copy against the final test file records only this adversary's additions:

```text
 tests/source_snapshot.rs | 27 ++++++++++++++++++++++
 1 file changed, 27 insertions(+)
```

All six implementation hashes matched adversary-round2-source-before.sha256 after execution. No existing case was changed, deleted or weakened. The implementor had stopped its builds and released its lease before this review acquired its own.

## 2. One bounded new attack, written and run before the suite

The corrected XML scalar handling was inspected together with its claim that malformed XML retains gaps. The new case supplies a POM with a complete CDATA module value but an unclosed project element, plus a real child POM and Java source. It calls the public collector and requires either refusal or a MissingBuildSelection gap.

Location: tests/source_snapshot.rs:633. The case is red after compiling successfully.

```console
CARGO_TARGET_DIR=$BUILD RUSTC_WRAPPER=/usr/bin/sccache CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 cargo test --locked -p codegate-cli --test source_snapshot adversary_round2_unclosed_maven_xml_cannot_claim_complete_selection -- --exact
```

Exit 101. Verbatim test output from adversary-round2-unclosed.log:

```text
running 1 test
test adversary_round2_unclosed_maven_xml_cannot_claim_complete_selection ... FAILED

failures:

---- adversary_round2_unclosed_maven_xml_cannot_claim_complete_selection stdout ----

thread 'adversary_round2_unclosed_maven_xml_cannot_claim_complete_selection' (3499541) panicked at tests/source_snapshot.rs:633:23:
unclosed Maven XML returned no build-selection gap: []
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    adversary_round2_unclosed_maven_xml_cannot_claim_complete_selection

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 19 filtered out; finished in 0.01s

error: test failed, to rerun pass `-p codegate-cli --test source_snapshot`
```

## 3. Suite run after the isolated failure

```console
CARGO_TARGET_DIR=$BUILD RUSTC_WRAPPER=/usr/bin/sccache CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 cargo test --locked -p codegate-cli --test source_snapshot
```

Exit 101. Verbatim test output below; raw compiler preamble is retained in adversary-round2-suite.log.

```text
running 20 tests
test three_syntax_classifiers_preserve_physical_bytes_and_comment_lookalikes ... ok
test root_and_nested_symlinks_never_escape ... ok
test generated_markers_inside_strings_do_not_exclude_source ... ok
test correction_discovered_configuration_directories_refuse_symlinks ... ok
test correction_maven_mixed_text_cdata_and_whitespace_preserve_literal_hierarchy ... ok
test corrupt_git_and_asserted_configuration_metadata_are_never_trusted ... ok
test workspace_root_members_do_not_invent_self_parent_cycles ... ok
test referenced_manifest_symlinks_are_refused_and_configuration_claims_cannot_panic ... ok
test literal_selection_tests_generated_languages_and_skipped_directories ... ok
test adversary_maven_cdata_hierarchy_is_observed_or_explicitly_incomplete ... ok
test adversary_round2_unclosed_maven_xml_cannot_claim_complete_selection ... FAILED
test quoted_go_paths_and_conditional_gradle_includes_have_honest_selection ... ok
test static_manifests_preserve_hierarchy_and_report_dynamic_selection ... ok
test adversary_selected_source_preserves_ancestor_cargo_configuration ... ok
test manifest_references_cannot_select_repository_metadata ... ok
test confinement_bounds_and_invalid_input_refuse_instead_of_empty_success ... ok
test git_origin_tracks_staged_and_dirty_bytes_without_affecting_identity ... ok
test selected_dirty_untracked_bytes_configuration_and_manifests_define_identity ... ok
test correction_ancestor_configuration_directories_share_discovery_and_excludes ... ok
test aggregate_bytes_and_file_inventory_are_bounded ... ok

failures:

---- adversary_round2_unclosed_maven_xml_cannot_claim_complete_selection stdout ----

thread 'adversary_round2_unclosed_maven_xml_cannot_claim_complete_selection' (3502117) panicked at tests/source_snapshot.rs:633:23:
unclosed Maven XML returned no build-selection gap: []
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace


failures:
    adversary_round2_unclosed_maven_xml_cannot_claim_complete_selection

test result: FAILED. 19 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 3.40s

error: test failed, to rerun pass `-p codegate-cli --test source_snapshot`
```

## 4. Remaining finding and reachability

| Location | Verdict / origin | Finding | What was measured | What reaches it |
| --- | --- | --- | --- | --- |
| src/collection/manifests.rs:209 | CONFIRMED / introduced | An unclosed Maven project element reaches EOF without a build-selection gap, so malformed XML can appear fully collected. | tests/source_snapshot.rs:633, isolated exit101, gaps=[] | Public collect_source on an ordinary on-disk POM truncated after its modules element; dirty/in-progress source inputs are expressly in scope |

The XML reader emits EOF while the parser's open-element stack is nonempty. The branch breaks without checking that stack. Check the completed XML document structure, or retain a MissingBuildSelection gap instead of silent completeness. No fix was applied by this adversary. The collector/parser is introduced by this unit; the base has no corresponding implementation, so this is introduced relative to the unit, not a claim that the CDATA correction itself added the defect.

## 5. What this pass could not break

Both unchanged round-one regression cases passed in the observed 20-case suite. Ancestor .cargo/.mvn discovery, exclusions and symlink refusal class tests passed; mixed ordinary text/CDATA/whitespace hierarchy passed. The additional attack was limited to malformed XML completion and did not broaden into a third campaign. No full integration gate or complete semantic conformance is claimed.

## 6. Output paths

Only /tmp/codegate-source-worker.AAF0ck was reused outside the worktree for compiler artifacts and existing source-fixture machinery. All new fixtures, logs, pre-round copies, hashes and this report are under .scratch/unit. No implementation, AEP or commit was written. Coordinator owns the remaining repair, integration and cleanup.

```findings
- file: src/collection/manifests.rs
  line: 209
  category: boundary
  severity: blocker
  verdict: CONFIRMED
  origin: introduced
  message: An unclosed Maven project element reaches EOF without a build-selection gap, so malformed XML can appear fully collected.
```
