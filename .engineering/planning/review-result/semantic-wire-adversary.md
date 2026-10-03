---
format: aep.planning-md/3
id: review-result:semantic-wire-adversary
kind: review-result
status: active
title: Independent semantic wire bridge probes
relations:
- reviews: story:source-snapshot
revision: 1
---
unit: coordinator-owned semantic wire bridge prerequisite, working-tree bytes in assigned checkout
verdict: nothing found
cases: executed 2 targeted scratch probes, red 0
origin: introduced 0 / pre-existing 0 / undecided 0
wrote-outside-worktree: none
needs-coordinator: decide whether to retain the scratch regression source; this is not story closure

## Scope

Read src/semantic_wire.rs, tests/semantic_wire.rs and generated/semantic-wire/types.rs. No tracked files or production files changed. Added only .scratch/unit/wire_adversary.rs and its compiled executable plus this report. Waited for implementor build completion; linked existing rlibs read-only using rustc into assigned scratch. No Cargo process or shared target writes. Own worktree lease released after probes. Full package was not run by this reviewer.

## Attack one

Command: `ADVERSARY_ROOT=$WORKTREE .scratch/unit/wire_adversary adversary_enumeration_arms_equal_generated_renames --exact --nocapture`

Exit: 0

```text
running 1 test
checked 159 enum variants against generated serde names
test adversary_enumeration_arms_equal_generated_renames ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.00s
```

Every enumeration macro mapping compared to actual generated serde rename and V-index arms; 159 variants match. This detects symmetric wrong bridge mappings that roundtrips alone would miss.

## Attack two

Command: `ADVERSARY_ROOT=$WORKTREE .scratch/unit/wire_adversary adversary_signed_boundaries_nested_nulls_and_escaped_duplicates --exact --nocapture`

Exit: 0

```text
running 1 test
test adversary_signed_boundaries_nested_nulls_and_escaped_duplicates ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 5 filtered out; finished in 0.01s
```

Reached public decode_snapshot with native i64::MIN optional exit_code, i64::MAX required integer, null optional framework target/profile and nested gap location. Confirmed typed values and missing projections, then decoded projected output. Refused both signed overflows, fractional/exponent numeric representations, an escaped duplicate key (`exit_code` versus `exit_\u0063ode`) and missing required timed_out. Existing fixture is intentionally structural, not semantically admissible; semantic checks remain admission's responsibility.

No finding. This review makes no full-package, conformance or story-completion claim.

```findings
[]
```
