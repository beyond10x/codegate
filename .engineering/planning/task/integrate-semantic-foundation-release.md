---
format: aep.planning-md/3
id: task:integrate-semantic-foundation-release
kind: task
status: active
title: Integrate semantic foundations by PR and release remote main
relations:
- serves: vision:language-neutral-code-quality
- delivers: story:parity-foundations
revision: 3
transitions:
- {from: "draft", to: "proposed", at: "2026-10-03T00:17:17Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-03T00:17:17Z", actor: "human:timo", revision: 3}
---
## Request and authority

The operator requested on 2026-10-03: integrate all retained changes via a pull
request, leave the local repository clean, and cut the new version from remote main.
This authorizes bot commits/push, PR creation/merge and the release/tag/artifacts.
No personal-account GitHub writes or failing-gate bypass is authorized.

## Scope

Integrate the retained semantic foundation from story:parity-foundations, including
its contracts, generated crates, gate changes, baseline and reviewed backlog. Preserve
unfinished runtime parity as explicit work. Reconcile retained public-delivery evidence
and the CI-policy credential blocker. Local managed-worktree cleanup follows verified
remote recovery or exact archives. All authored executable changes remain Rust.

## Version and completion

Remote main is f8fe6c13fe246f9708d3270fec690d763097838b and no remote release/tag is
published as observed in this session. Cargo metadata remains 0.1.0, the repository's
configured initial version. Release that initial version only from verified merged
remote main after required checks pass. Verify bot author, annotated tag, release
build, GitHub Release and downloaded artifact checksums. Documentation is asynchronous.

## Build environment

Use this task's private temporary Cargo target on the /tmp tmpfs, whose free space
is separately checked against the required 20 GiB before the large build. Keep two
jobs and sccache; retain small logs/reports in the managed tree and remove only this
task's reproducible output after verification. This avoids deleting other owners' data.

## Acceptance

The bot-authored PR is merged with required checks passing, primary main is clean
and matches remote main, task worktrees are safely retired, and the exact remote-main
0.1.0 tag has a verified GitHub Release with Linux archive and matching SHA256SUMS.
