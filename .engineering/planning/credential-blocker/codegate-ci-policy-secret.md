---
format: aep.planning-md/3
id: credential-blocker:codegate-ci-policy-secret
kind: credential-blocker
status: cleared
title: Codegate release needs the updated CI policy secret
relations:
- blocks: story:public-delivery
- blocks: task:publish-codegate-0-1-0
- blocks: task:integrate-semantic-foundation-release
withholds: deployment_result
revision: 3
transitions:
- {from: "open", to: "cleared", at: "2026-10-03T07:07:07Z", actor: "human:timo", revision: 3, decided_on: {"recorded":{"deployment_result":1}}, executor: "agent:codegate-review"}
---
The common source gate cannot produce successful release evidence until GitHub Actions receives the updated private Gates policy for beyond10x/codegate. Private registry enrollment and local signed admission already succeeded. GitHub Actions run37013275938 fails "repository is not enrolled" with the old secret. The bot's GET of the repository Actions secret public key returned HTTP403.

An authorized owner must set repository secret B10X_GATES_POLICY from the current private gates-policy/policy.json, or grant the bot App repository Secrets read/write access for provisioning. Do not place policy values in chat or public source. The operator has been asked for this external action; no answer or external change has been observed.

Clear only after the correct secret is installed and a bot-triggered rerun of the common check succeeds. Then enable required checks and complete the authorized 0.1.0 tagged artifact release. Website and source publication are already verified; this blocker specifically withholds release completion.

## Resolution observed on 2026-10-03

The operator explicitly authorized a one-time personal gh use for enrollment. The full private policy exceeded the GitHub secret limit; a Codegate-scoped copy retained all global rules, its exact baseline and all Codegate exceptions. Repository secret B10X_GATES_POLICY was installed. No private policy values enter public source.

The bot-triggered security rerun and tag check succeeded. Evidence: https://github.com/beyond10x/codegate/actions/runs/37086993620 at commit 8ebd4a200c8181b0d48221cab5c1fd25f8d30f45. Required-check ruleset 24402354 enforces common security, Gate and documentation checks on main without bypass actors. Bot release https://github.com/beyond10x/codegate/releases/tag/0.1.0 contains the verified executable archive and SHA256SUMS. The prior paragraphs describe the historical blocker, which is now cleared. The bot's general Secrets permission was not changed.
