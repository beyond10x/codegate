---
format: aep.planning-md/3
id: review-result:gate-scratch-adversary
kind: review-result
status: active
title: Independent gate scratch allocation attack
relations:
- reviews: task:clean-external-target-gate
revision: 1
---
unit: task:clean-external-target-gate — working-tree change over 91dc33be, codegate-check SHA256 d18359d886a4887ca1534b7f23fc811c7aa8f8d0be42ee2614fc68bf0ce0b237
verdict: nothing found
cases: executed 1→3, red 0
origin: introduced 0 / pre-existing 0 / undecided 0
wrote-outside-worktree: $BUILD compiler output; standard Cargo/sccache caches
needs-coordinator: run the full clean-checkout integration gate; retain or discard scratch probes during managed cleanup

## 1. Diff and ownership

`git --no-pager diff --stat`:

```text
 src/bin/codegate-check.rs | 49 +++++++++++++++++++++++++++++++++++++++++++----
 1 file changed, 45 insertions(+), 4 deletions(-)
```

This implementation diff was already present on arrival and remained unchanged; the adversary edited no implementation or tracked file. The two added cases exist only in `.scratch/unit/adversary_probe.rs`, an unchanged copy of the production source with appended Rust tests. No AEP operation or commit was made. Paths in this report use $WORKTREE and $BUILD as required by the private brief.

## 2. Two attacks, written before execution

The scratch probe was compiled using rustc --edition=2024 --test with existing clap/serde_json/sha2 rlibs from $BUILD/debug/deps, CARGO_MANIFEST_DIR=$WORKTREE and CARGO_PKG_VERSION=0.1.0. An initial compilation omitted CARGO_PKG_VERSION and failed to compile; supplying that existing Cargo value corrected the probe setup. This was not counted as a red behavioral result.

Attack one calls the production create_scratch with a genuinely absent target parent, checks its exact invocation path, retains report bytes through a repeated call, and checks refusal/preservation for a file occupying target and a file occupying the invocation path.

Command: `ADVERSARY_ROOT=$WORKTREE/.scratch/unit .scratch/unit/adversary_probe adversary_missing_parent_and_occupied_paths --exact --nocapture`

Exit: 0. Raw runner output:

```text
running 1 test
test adversary_missing_parent_and_occupied_paths ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s
```

Attack two releases eight threads together against one missing target parent. Exactly one receives an invocation directory and seven are refused; a further allocation attempt preserves winner evidence bytes. This directly probes exclusive helper allocation; it does not claim eight real gate processes share a PID.

Command: `ADVERSARY_ROOT=$WORKTREE/.scratch/unit .scratch/unit/adversary_probe adversary_concurrent_allocation_preserves_exclusivity --exact --nocapture`

Exit: 0. Raw runner output:

```text
running 1 test
test adversary_concurrent_allocation_preserves_exclusivity ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 2 filtered out; finished in 0.00s
```

## 3. Suite runs after the added cases

The before count of one is the implementor's reported codegate-check unit lane. The scratch suite includes that exact existing case plus the two appended cases; this is not a package-wide count.

Command: `ADVERSARY_ROOT=$WORKTREE/.scratch/unit .scratch/unit/adversary_probe --nocapture`

Exit: 0. Raw runner output:

```text
running 3 tests
test adversary_missing_parent_and_occupied_paths ... ok
test tests::gate_clean_external_target ... ok
test adversary_concurrent_allocation_preserves_exclusivity ... ok

test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

Command: `PATH=<pinned ESS 0.50.0>:$PATH CARGO_TARGET_DIR=$BUILD RUSTC_WRAPPER=/usr/bin/sccache CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 cargo test --locked -p codegate-cli --test freshness -- --nocapture`

Exit: 0. Raw output, build path normalized:

```text
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.22s
     Running tests/freshness.rs ($BUILD/debug/deps/freshness-66c44dada4ac6cb0)

running 1 test
test gate_requires_this_invocations_conformance_producer ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1.07s
```

This unchanged test executes a real fresh producer, rejects missing/ignored/renamed producers with retained stale evidence and rejects mutated report-binding fields. No full task check was run in this adversary pass.

## 4. Findings

None.

## 5. Attacked without a failure

- Missing target parent creates the exact exclusive invocation directory.
- Repeated allocation cannot reuse or overwrite prior invocation evidence.
- Files occupying the parent or invocation path remain byte-identical after refusal.
- Concurrent helper callers produce exactly one owner.
- Existing fresh-producer and stale-report rejection regression remains green.

## 6. Retained paths

Probe source, executable, fixture directories and logs are inside $WORKTREE/.scratch/unit. The unchanged freshness test retains its probes under $WORKTREE/target/freshness-regression-PID. Compiler/cache writes use the assigned $BUILD and standard Cargo/sccache caches. The private unit brief retains the exact external build path. No build directory or worktree was removed. The coordinator owns final cleanup.

```findings
[]
```
