---
format: aep.planning-md/3
id: review-result:admission-boundary-adversary-round-1
kind: review-result
status: archived
title: Independent admission boundary IO attack
relations:
- reviews: story:capability-admission
revision: 2
transitions:
- {from: "active", to: "archived", at: "2026-10-03T08:27:20Z", actor: "human:timo", revision: 2}
---
needs-revision

Coordinator's independent first attack against stable admission unit: the AST architectural guard admits a shared function calling clap::Command::new("x").get_matches(). Clap is already a normal dependency and get_matches calls std::env::args_os; this is a reachable IO path, not a hypothetical future dependency. Direct std::env checks therefore do not establish the claimed pure-core boundary.

Added tests-only indirect_process_argument_io_is_rejected in tests/semantic_boundary.rs. Actual command: CARGO_TARGET_DIR=$BUILD RUSTC_WRAPPER=/usr/bin/sccache CARGO_BUILD_JOBS=2 CARGO_INCREMENTAL=0 cargo test --locked -p codegate-cli --test semantic_boundary indirect_process_argument_io_is_rejected -- --exact --nocapture. Executed1 failed1, exit101; runtime assertion: IO dependency escaped: fn evaluate(){clap::Command::new("x").get_matches();}. Production admission was not changed. Raw log retained at .scratch/unit/adversary-red.log in wt-7d9dea940be5.

The proposed destructured-language probe was already covered by the implementor's latest visit_pat_struct change before this attack, so no second finding is claimed. Correct the admitted external dependency boundary without weakening existing positive projection/admission cases. Root fixture tests and native ESS remain separate witnesses.
