---
format: aep.planning-md/3
id: approval-record:standing-wave-approval
kind: approval-record
status: draft
title: Operator standing approval for upcoming Codegate waves
summary: Explicit operator approval of all upcoming waves, retaining recorded proposals and separate release authority.
relations:
- decides: design:first-wave-offline-dependency
- decides: story:offline-dependency-slice
revision: 1
---
## Source

The operator said in this session on 2026-10-02: "I approve all upcoming waves".
This is direct interactive authorization, not a non-interactive bypass and not an
agent review substituted for a human decision.

## Scope

Accept the recorded Wave 1 proposal and grant standing approval for subsequent
Codegate implementation waves selected through AEP planning and wave procedures.
Each wave still records its scope, computed selection, resource pre-flight, bounded
commit authority, evidence and independent adversary findings before integration.
The current worker cap is up to three implementation workers; Wave 1 selects one.

This covers the proposed opening planning/specification commit, per-unit commits,
integration merges, closing evidence/store commit and local merge to main. It does
not authorize a GitHub push, repository creation, tag, release or bypass of a failing
gate, missing evidence or unresolved blocker. Replan before selecting later waves.

## Current work

The current requested batch is story:offline-dependency-slice. The coordinator now
uses aep:planning to advance its legal lifecycle to active before dispatch. The
store has no literal accepted story status; this record carries operator acceptance.
