---
format: aep.planning-md/3
id: credential-blocker:codegate-ci-policy-secret
kind: credential-blocker
status: open
title: Codegate release needs the updated CI policy secret
relations:
- blocks: story:public-delivery
- blocks: task:publish-codegate-0-1-0
- blocks: task:integrate-semantic-foundation-release
withholds: deployment_result
revision: 1
---
The common source gate cannot produce successful release evidence until GitHub Actions receives the updated private Gates policy for beyond10x/codegate. Private registry enrollment and local signed admission already succeeded. GitHub Actions run37013275938 fails "repository is not enrolled" with the old secret. The bot's GET of the repository Actions secret public key returned HTTP403.

An authorized owner must set repository secret B10X_GATES_POLICY from the current private gates-policy/policy.json, or grant the bot App repository Secrets read/write access for provisioning. Do not place policy values in chat or public source. The operator has been asked for this external action; no answer or external change has been observed.

Clear only after the correct secret is installed and a bot-triggered rerun of the common check succeeds. Then enable required checks and complete the authorized 0.1.0 tagged artifact release. Website and source publication are already verified; this blocker specifically withholds release completion.
