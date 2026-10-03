---
format: aep.planning-md/3
id: review-result:admission-boundary-adversary-round-2
kind: review-result
status: archived
title: Second admission boundary platform IO attack
relations:
- reviews: story:capability-admission
revision: 2
transitions:
- {from: "active", to: "archived", at: "2026-10-03T08:27:20Z", actor: "human:timo", revision: 2}
---
needs-revision

The corrected external dependency policy rejects clap and aliases, and the real core remains admitted. In the second independent attack, nested standard-library effects still escaped: std::os::unix::net::UnixStream::connect is a normal supported Unix API but the scanner only checks std's immediate child. std::thread::current().id() likewise exposes host thread context to a supposedly pure calculation.

Added tests-only nested_platform_io_and_thread_context_are_rejected. The exact targeted cargo test ran1 failed1, exit101 at the UnixStream assertion. This is a runtime guard failure, not a compile-only result; raw evidence is .scratch/unit/adversary-round-2-red.log. No source/admission implementation changed. Preserve the positive generated-model/serde/hash projections while denying platform and thread effects. This exhausts the two independent attack passes; subsequent execution verifies these same retained regressions and the integration gate, not a new attack campaign.
