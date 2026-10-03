---
format: aep.planning-md/3
id: task:clean-external-target-gate
kind: task
status: active
title: Make the gate work from a clean checkout with external Cargo output
relations:
- decomposes: story:parity-foundations
- serves: vision:language-neutral-code-quality
revision: 3
transitions:
- {from: "draft", to: "proposed", at: "2026-10-03T07:19:23Z", actor: "human:timo", revision: 2, executor: "agent:codegate-collection"}
- {from: "proposed", to: "active", at: "2026-10-03T07:19:23Z", actor: "human:timo", revision: 3, executor: "agent:codegate-collection"}
---
## Context

Post-release review of 8ebd4a200c8181b0d48221cab5c1fd25f8d30f45 reproduced a clean-checkout failure. With CARGO_TARGET_DIR pointing outside the checkout and no repository-local target directory, the real codegate-check binary passes AEP and ESS validation, then exits 1: `codegate-check: No such file or directory (os error 2)`.

`src/bin/codegate-check.rs:207-210` calls create_dir on target/codegate-check-PID without creating its parent. `Taskfile.yml:11-16` only invokes cargo run. External Cargo target directories therefore leave the parent absent. The source rule requiring isolated worker build output makes this a supported workflow, not malformed input.

## Acceptance

The named gate-clean-external-target regression runs task check from a checkout with no local target directory and an external Cargo target directory, passes the full required gate, and retains fresh conformance evidence without accepting a preexisting report.

## Scope

Rust harness maintenance only: src/bin/codegate-check.rs and an appropriate regression in tests/gate_adversary.rs or tests/freshness.rs. Preserve unique scratch-directory creation and the existing fresh evidence checks; create the necessary parent without weakening freshness. No product-domain noun or new runtime API is introduced; the existing dependency ESS suite remains the acceptance witness for product behavior.

## Verification

Name the regression gate-clean-external-target. Check both ordinary and external Cargo output layouts; preserve all 27 evaluator scenarios. Keep the existing missing, ignored, renamed and stale conformance-producer regressions green. A test that precreates target before invoking the gate cannot satisfy this acceptance.

## Priority and ownership

P2 tooling defect; fix before the next implementation wave, whose workers use isolated targets. Draft only: this review records the defect and does not implement its correction.
