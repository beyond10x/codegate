---
format: aep.planning-md/3
id: review-result:composition-acceptance-round-1
kind: review-result
status: active
title: Collection composition acceptance review round 1
relations:
- reviews: story:collection-composition-seam
- reviews: story:source-structure
revision: 1
---
needs-revision

story:collection-composition-seam — the Semantic-refusal acceptance has no declared observable response because Collect exposes only collected with a mandatory snapshot and no top-level gaps, so specify an existing typed representation or amend ESS before requiring this native scenario — .engineering/planning/story/collection-composition-seam.md:37; ess-semantic/domains/semantic.yaml:438

story:collection-composition-seam — the empty-registration acceptance names unsupported Units although FactFamily has no Units variant and leaves Sources status and empty coverage scope unspecified, so state the exact existing-family records and gaps that distinguish unavailable unit discovery from an observed empty population — .engineering/planning/story/collection-composition-seam.md:39; ess-semantic/domains/semantic.yaml:20; specifications/source-baseline.md:29

story:collection-composition-seam — the acceptance lists independent source-only success and Semantic-refusal outcomes without one aggregate completion witness, so require a named report with both exact scenarios passing and the preserved suites verified — .engineering/planning/story/collection-composition-seam.md:37

Read seven complete artifacts through `aep plan artifact show`: collection-composition-seam, source-go-baseline, source-rust-baseline, source-java-baseline, source-structure, first-slice and source-collection-baseline; also read the critic procedures, ESS contract, generated Collect behavior, admission coverage logic and source-baseline specification; ran kinds, story lifecycle and validation.

The revised prerequisite edges permit language closure before downstream first-slice work. Runtime conformance was not established; this was read-only with no builds. Validation reports 80 valid artifacts and two existing adversary reviews without findings blocks; those validator notices are not critic findings. All report paths are repository-relative.

```findings
- file: .engineering/planning/story/collection-composition-seam.md
  line: 37
  category: acceptance
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: the Semantic-refusal acceptance has no declared observable response because Collect exposes only collected with a mandatory snapshot and no top-level gaps, so specify an existing typed representation or amend ESS before requiring this native scenario
- file: .engineering/planning/story/collection-composition-seam.md
  line: 39
  category: acceptance
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: the empty-registration acceptance names unsupported Units although FactFamily has no Units variant and leaves Sources status and empty coverage scope unspecified, so state the exact existing-family records and gaps that distinguish unavailable unit discovery from an observed empty population
- file: .engineering/planning/story/collection-composition-seam.md
  line: 37
  category: acceptance
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: the acceptance lists independent source-only success and Semantic-refusal outcomes without one aggregate completion witness, so require a named report with both exact scenarios passing and the preserved suites verified
```
