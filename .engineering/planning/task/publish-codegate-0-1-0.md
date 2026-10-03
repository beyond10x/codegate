---
format: aep.planning-md/3
id: task:publish-codegate-0-1-0
kind: task
status: implemented
title: Publish Codegate documentation, Gates enrollment and first release
relations:
- serves: vision:language-neutral-code-quality
- decomposes: story:public-delivery
revision: 8
transitions:
- {from: "draft", to: "proposed", at: "2026-10-02T13:12:49Z", actor: "human:timo", revision: 2, executor: "agent:codegate-delivery"}
- {from: "proposed", to: "active", at: "2026-10-02T13:12:49Z", actor: "human:timo", revision: 3, executor: "agent:codegate-delivery"}
- {from: "active", to: "implemented", at: "2026-10-03T07:45:49Z", actor: "human:timo", revision: 8, decided_on: {"recorded":{"test_result":1}}, executor: "agent:codegate-collection"}
---
## Context

Operator explicitly requested public documentation like Mantle, common Gates enrollment, bot-authored history, publication notification and the first version release. No remote or tag exists yet. All five existing commits already have the exact bot author and committer. Preserve commit identities unless a measured admission defect requires rewriting; no false claim of an unnecessary authorship rewrite.

## Done When

Public beyond10x/codegate exists with bot-only direct history; common Gates policy/signing grants, hooks, selected CI policy and required security check are enrolled; repository correctness, documentation validation and release builds pass for the exact published tag0.1.0; a bot-owned GitHub Release has Linux x86_64 executable archive plus SHA256SUMS; the public /codegate/ site serves matching source and publisher provenance. Record actual URLs, commit/tag, checks and any limitation before claiming publication.

## Scope

Codegate: public website source and Rust clap docs builder, README/AGENTS, Cargo metadata/version CLI, Taskfile and checks, pinned CI/site/release build workflows, license/release notes, governed delivery evidence. Coordinator owns Gates private registry/signer/selected-secret enrollment and GitHub repository/Pages/rules setup through bot API. No credential or private policy is committed to public source. Existing evaluator ESS semantics remain unchanged.

## Authority

This user request explicitly authorizes GitHub repository creation/publication, bot attribution correction as needed, Gates registration, Pages configuration, tagged0.1.0 source release and release artifacts. Standing wave approval remains applicable. All runnable implementation is Rust, CLI clap derive; managed trees only. Integrations use Connectors first; configured adapters have no GitHub capability, so bot-authenticated Gates API is the declared fallback. Baseline historical privacy is audited before public delivery; no refusal is bypassed.

## Delivery progress

Created public GitHub repository `beyond10x/codegate`, numeric id1401762311, through the bot App. Copied Mantle's active App-only branch/tag authority and identity rulesets through the bot API. Required common/repository checks will be enabled after the first successful main run, per Gates adoption procedure.

Private Gates policy change committed/published as18642a4: exact repository id and original empty root56edf02005fb019957b16ecc2f092e0b2e67790e as baseline; existing authorized Mantle signers granted Codegate. This baseline retains all nonempty source commits inside admission. Coordinated hooks installed. No blanket historical exception or baseline advancement excludes the private-path findings.

Bot API GET of the repository Actions secret public key returned403 Forbidden. CI policy-secret provisioning is pending bot Secrets permission or an authorized owner's direct repository-secret setup. The operator was asked for that external action; no personal-account write or alternative credential bypass was attempted.

Public history projection will replace private filesystem prefixes in historical log text with `$HOME/`, preserve product behavior and retained evidence counts, keep all commit authors/committers as the bot, and record the old/new commit map. Original history and unsanitized evidence remain in local recovery archives. This fulfills the explicit history-rewrite/publication request while removing the measured privacy defect rather than exempting it.

## Documentation implementation evidence

The bounded docs worker returned three files: website/index.html, website/styles.css and src/bin/codegate-docs.rs. Its targeted cargo test executed5passed/0failed; examples are decoded and evaluated through the real core, first Pass then forbidden-edge Fail. Targeted Clippy with-Dwarnings, formatter and diff checks exited0. Static validation only; no browser render claimed. Source was copied into the coordinator checkout unchanged. Full integration check remains pending.

Only the worker's stopped528MiB target was removed. Its source and raw logs were archived under the managed id codegate-docs-20261002. Primary implementation evidence and unsanitized original Git history are already retained in the earlier codegate-wave1-plan-20261002 recovery archive. No new source-language binding is introduced by this delivery.

## Verified public delivery

Source is public at https://github.com/beyond10x/codegate, main f8fe6c13fe246f9708d3270fec690d763097838b. All six public commits have b10x-bot[bot] as author and committer. Historical private path prefixes were redacted in the authorized public-history projection; original commits and logs remain in local recovery archives. No force push to public history was necessary because the repository was empty.

The complete Gate passed: https://github.com/beyond10x/codegate/actions/runs/37013274829. The native ESS report executed 27, passed 27, failed/error/unsupported/skipped 0. Root tests, generated crate checks, formatting, Clippy, planning and documentation checks passed. Local signed common Gates admission scanned all five nonempty commits successfully; receipt check https://github.com/beyond10x/codegate/runs/110857864626.

Documentation validation run37013274937 and site publication run37013437007 both succeeded. https://beyond10x.github.io/codegate/ is live. Its HTML and CSS were fetched and matched source bytes. The live .well-known/b10x-site.json identifies source f8fe6c13fe246f9708d3270fec690d763097838b; .well-known/b10x-docs.json identifies the same control commit, runtime fb4024ef7846729e5456591b9070db3d48c87e64 and artifact SHA256 1ab1dc28a4e9ae705b13b034625d8a759f54ad3d1f3bb81b646d77c08543d99b. This is verified publication, not a queued deployment.

Release 0.1.0 has NOT been tagged or published. The Actions shared-gates run37013275938 failed with "repository is not enrolled": CI still reads the old policy secret. Repository Secrets access returned HTTP403 for the bot. Required-check ruleset activation and tagged release remain pending the credential blocker; no alternate credentials or policy exemptions were used.

Resume after an authorized owner updates Codegate's B10X_GATES_POLICY repository secret from the updated private policy, or grants the App Secrets read/write so the bot can provision it. Re-run the failed common check via the bot API, verify green, activate prepared required-check rules, then tag 0.1.0, build, publish via bot and verify archive plus SHA256SUMS. No source behavior change is needed to resolve the credential blocker.

## Release completion reconciled 2026-10-03

PR https://github.com/beyond10x/codegate/pull/1 merged to remote main 8ebd4a200c8181b0d48221cab5c1fd25f8d30f45. Annotated tag 0.1.0 object 775d9e4393597209535c3aa9027ca4a18789886f points to that exact commit. The bot-owned release is public at https://github.com/beyond10x/codegate/releases/tag/0.1.0, published 2026-10-03T01:43:01Z. Linux archive and SHA256SUMS were uploaded and downloaded checksums verified; archive SHA256 1fa12b20dfc0cc57f0d6bce18c68c0b8ccd8d609f43dfeb8f48aa6d9dbe31e1b.

Required tag checks succeeded: common security run37086993620, Gate run37086993142, release artifacts run37086993155. Main docs/site runs37086809592 and37086884075 succeeded. Required-check ruleset24402354 is active and has no bypass actors. The policy secret enrollment was completed using the operator's single explicit gh exception; this grants no ongoing personal-write authority. General bot Secrets access remains unavailable but no longer blocks this completed release.

Live https://beyond10x.github.io/codegate/.well-known/b10x-site.json and .well-known/b10x-docs.json were re-read during collection planning and both identify 8ebd4a200c8181b0d48221cab5c1fd25f8d30f45. Publisher runtime fb4024ef7846729e5456591b9070db3d48c87e64; artifact e01d174ec402f464fef61cd8c07825e66d4b0dbbf577e26f5007ec0f01cabf0f. Primary main is clean at that remote commit; previous release worktrees were retired with recovery proof. Current collection trees belong to subsequent work.

This resolves the earlier historical pending-release text. No new release is part of the collection wave.
