---
format: aep.planning-md/3
id: resource-blocker:source-baseline-build-space
kind: resource-blocker
status: open
title: Restore build capacity for the source baseline wave
relations:
- blocks: story:source-go-baseline
- blocks: story:source-rust-baseline
- blocks: story:source-java-baseline
withholds: test_result
revision: 1
---
## Observed resource condition

On 2026-10-03, after release 0.3.0 cleanup, df -BG for the home filesystem reported 13 GiB available; the immediately preceding read was 14 GiB. AGENTS.md requires at least 20 GiB free before any new large build, and the aep:implementing wave preflight refuses dispatch below that floor. The release build measured approximately 2.3 GiB including standalone generated targets before its own reproducible outputs were removed with cargo clean. All prior Codegate managed trees are retired; no owned disposable target remains to reclaim.

## What clears it

Reobserve at least 20 GiB available on the approved build filesystem before each new large build. Three simultaneous 2.3 GiB targets require approximately 27 GiB at initial dispatch to preserve the floor for all launches. With less capacity, keep the three-agent cap but serialize build admissions and clean only completed owned targets after retaining evidence. Never remove another repository's output or reduce the floor silently. The operator or the owner of other storage can free capacity; subsequent preflight must measure actual free space and process state rather than infer cleanup from a message.

## Work still possible

Read-only parallel scoping, ownership corrections, deterministic language-mapping decisions and a reviewable dispatch/conformance plan proceed. No language implementation has been dispatched and no language test is reported green. This resource blocker does not block planning publication or remote validation of planning-only changes. The repository-report objective remains active and incomplete.
