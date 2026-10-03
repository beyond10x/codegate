---
format: aep.planning-md/3
id: verification-report:first-collection-foundation
kind: verification-report
status: draft
title: Integrated source collection foundation evidence
relations:
- verifies: epic:source-collection-foundation
- verifies: story:source-snapshot
- verifies: story:capability-admission
revision: 2
---
## Result

Foundation implementation at 5d5192d passed actual `task check` (exit0). All 27 legacy evaluator ESS scenarios passed. The separate source-foundation component executed8 passed8: six authored behavioral cases plus two structural cases, failed0/error0/unsupported0/skipped0. Six public semantic commands remain outside this component and unimplemented. Root package tests executed81 passed81, including source21, admission11 and architecture8; these Rust test counts are separate from native ESS scenario counts.

Exact admitted suites, native reports and source-foundation scenario run/mutation runs are retained in verification/source-foundation/{foundation,evaluator}. The gate log is verification/source-foundation/task-check.log; only personal worktree/build path prefixes are normalized. Original raw evidence remains in the managed-tree recovery archive under .scratch/review and target/codegate-check-3713312. Gate reports were generated in an exclusive fresh directory, not copied in to satisfy the gate.

## Conformance and faults

Internal generated CollectSource/ValidateSnapshot handlers forward actual collection/admission responses. The first runtime harness run observed3pass5error because ESS Number serialized exact integral inputs as float tokens; input transport now explicitly converts exact i64 numbers before invoking the unchanged strict product decoder. Authored scenario expectations were not changed. Final component run passed all8. Test-only empty-response, admit-everything and constant-identity faults each made the named authored case Failed; their raw scenario runs are retained beside the passing report. This demonstrates observable response assertions, not exhaustive correctness.

## Review and correction

Collection: two bounded adversary passes found2 then1; both prior findings resolved, no carried finding, one new malformed-Maven-EOF gap. All three retained regressions pass after corrections. Admission architectural guard: two passes found1 then1, prior dependency bypass resolved, new platform namespace bypass corrected; coordinator verified the same eight boundary tests and eleven admission tests. The thread-current bypass inference was withdrawn because its assertion had not run; structured replacement records preserve this correction. No third attack campaign ran. Source and admission production changes were merged from bot commits4c99cdc and a68963c respectively.

## Remaining delivery and sequencing

The baseline epic stays draft. New reviewed story:collection-composition-seam supplies real public Collect composition/registration/conformance before the three language stories close, resolving a practical acceptance cycle in the earlier decomposition. All four critics approved its second round; first-round findings and outcomes remain recorded. Next come the disjoint Rust/Go/Java extractors, then equivalence integration and JSON CLI/shared assessment/declaration lookup. Broader observations and full parity stay explicit backlog gaps. No new release/tag is part of this wave.

## Cost and storage

At most three workers ran alongside coordination. Host tooling exposes no per-agent total tokens, tool-use count or duration; those are unavailable rather than estimated. Unit targets were isolated with two Cargo jobs and sccache. Exact retained tree/target triples are in design:source-collection-round-one and the manager registry. Closing publication and managed cleanup are coordinator-owned; archive private evidence before retiring trees.

## Gate step observations

CHECK AEP: exit 0
CHECK ESS: exit 0
CHECK generate behavior: exit 0
CHECK generate wire: exit 0
CHECK semantic ESS: exit 0
CHECK generate semantic behavior: exit 0
CHECK generate semantic wire: exit 0
CHECK normalize generated formatting: exit 0
CHECK normalize generated formatting: exit 0
CHECK normalize generated formatting: exit 0
CHECK normalize generated formatting: exit 0
CHECK generated drift: exit 0
CHECK combined suite: exit 0
CHECK real ESS conformance: exit Some(0)
CHECK native ESS report: executed27 passed27 failed0 error0 unsupported0 skipped0, exit0; evidence $WORKTREE/target/codegate-check-3713312/conformance
CHECK foundation suite: exit 0
CHECK real foundation conformance: exit Some(0)
CHECK native foundation ESS report: executed8 passed8 failed0 error0 unsupported0 skipped0, exit0; six public commands outside component
CHECK Rust tests: exit 0
CHECK format root: exit 0
CHECK Clippy root: exit 0
CHECK test generated: exit 0
CHECK format generated: exit 0
CHECK Clippy generated: exit 0
CHECK test generated: exit 0
CHECK format generated: exit 0
CHECK Clippy generated: exit 0
CHECK test generated: exit 0
CHECK format generated: exit 0
CHECK Clippy generated: exit 0
CHECK test generated: exit 0
CHECK format generated: exit 0
CHECK Clippy generated: exit 0
CHECK public documentation: exit 0

CI retention now includes both fresh evaluator and foundation JSON suites/reports/runs, including seeded-mutation evidence. The workflow artifact path is additive; product code and locally verified tests are unchanged. Remote required checks validate the final candidate.
