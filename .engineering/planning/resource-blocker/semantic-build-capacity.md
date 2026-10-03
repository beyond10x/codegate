---
format: aep.planning-md/3
id: resource-blocker:semantic-build-capacity
kind: resource-blocker
status: cleared
title: Insufficient free disk for the semantic parity integration build
relations:
- blocks: story:parity-foundations
withholds: test_result
revision: 2
transitions:
- {from: "open", to: "cleared", at: "2026-10-03T00:22:37Z", actor: "human:timo", revision: 2, decided_on: {"recorded":{"verification":1}}}
---
## Blocked operation

The large integration build and full `task check` required to finish
`story:parity-foundations` and start the remaining implementation waves.

## Observed evidence

Codegate `AGENTS.md` requires at least 20 GiB free before new large builds.
`df -B1 .` during this session reported 11,230,052,352 available bytes, below
21,474,836,480 bytes (20 GiB). Space also changed while this task was running;
no unrelated cache or managed worktree was deleted. The operator was asked to free
space or identify an authorized disposable cache. No answer has been recorded.

ESS validation/generation, deterministic file comparisons and formatting completed.
Only bounded metadata compilation of the root binaries/generated crates is attempted;
this is not the complete test/Clippy/conformance build and cannot discharge that gate.
No legacy or semantic runtime conformance result is claimed for this session.

## Clearing condition

Recheck actual free bytes >= 21,474,836,480 on the worktree filesystem, or obtain an
explicit operator instruction identifying a suitable alternate build filesystem or
revising the resource constraint. Then run the pinned full integration gate, retain
its exact result and resolve any defects before closing the foundation story.

## Separation

This is a local resource blocker, not the separate 0.1.0 release's Secrets permission
blocker. It neither authorizes cleanup of other owners' data nor excuses required
parity capabilities. The specification and reviewed backlog remain usable.
