---
format: aep.planning-md/3
id: verification-report:post-release-source-review
kind: verification-report
status: draft
title: Code review and next collection priorities after 0.1.0
relations:
- reviews: epic:semantic-parity
- informed_by: task:clean-external-target-gate
revision: 1
---
## Review boundary

Interactive review of release 0.1.0 at 8ebd4a200c8181b0d48221cab5c1fd25f8d30f45. Read the offline evaluator, admission, JSON bridge, CLI, gate, relevant tests and semantic foundation/backlog. This is a bounded source review, not a claim that every planned capability or security property was tested.

## Findings

P2: clean external Cargo builds cannot start the gate. src/bin/codegate-check.rs:207-210 creates target/codegate-check-PID without its parent; Taskfile.yml:11-16 creates none. Running the actual compiled gate with external CARGO_TARGET_DIR and no local target passed AEP/ESS then exited 1 with `No such file or directory (os error 2)`. task:clean-external-target-gate owns the regression/fix.

P2: the architectural boundary test is incomplete. tests/semantics.rs:122-136 scans only three literal paths and strings. It omits evaluate in src/lib.rs:10 and cannot establish indirect imports or IO hidden by aliases. This is an enforcement gap, not an observed IO call in the current pure evaluator. story:capability-admission now explicitly requires whole-core negative probes before collectors are admitted.

Planning drift: credential-blocker:codegate-ci-policy-secret still described the pre-release failure even though tag security run37086993620 and the bot 0.1.0 release succeeded. The blocker was cleared on verified evidence during this review. Existing active delivery artifacts retain historical progress; this review makes no bulk lifecycle moves.

## Product recommendation and resulting plan

The current CLI has only Evaluate (src/main.rs:26-35). The six rich generated handlers remain obligations (generated/semantic-behavior/src/semantic.rs:1617-1693), and specifications/semantic-scenarios.md explicitly contains no executable runtime cases. This is a delivery gap already honestly recorded, not an evaluator regression.

The operator expanded this review into a source-collection round. New epic:source-collection-foundation contains existing source-snapshot/capability-admission; epic:source-collection-baseline contains three new disjoint language binding stories and the existing structure/first-slice integration stories. This sequence produces visible collection and shared policy behavior in all three languages before spending effort on language servers, broad metrics or scoring. Those remain later parity obligations.

## Verification

Fresh ESS semantic validation: `codegate_semantic v1 — 2 file(s), valid`. The existing release gate passed 27 legacy conformance scenarios; this review does not count those as semantic conformance. Reproduction of the external-target failure used the real gate binary and pinned ESS 0.50.0. Raw reproduction retained in the managed worktree scratch and recovery evidence. All authored runnable code remains Rust.
