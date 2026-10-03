---
format: aep.planning-md/3
id: review-result:composition-design-round-1
kind: review-result
status: active
title: Collection composition design review round 1
relations:
- reviews: story:collection-composition-seam
- reviews: story:source-structure
revision: 1
---
needs-revision

story:collection-composition-seam — Specify and validate the Collect refusal contract before implementation because the proposed gaps/refusal response has no representation in the existing required-snapshot, collected-only ESS operation — .engineering/planning/story/collection-composition-seam.md:33; ess-semantic/domains/semantic.yaml:438

story:source-structure — Remove shared production registration from this story’s remaining responsibility because collection-composition-seam now delivers that same abstraction before language acceptance, leaving source-structure responsible for cross-language equivalence and aggregate integration — .engineering/planning/story/source-structure.md:84; .engineering/planning/story/collection-composition-seam.md:47

Read 10 artifacts through `aep plan artifact show`, the referenced ESS/generated contract and profile; inspected all 46 `depends_on` edges, including outside the reviewed set, through `relations` and `graph`; no ordering cycle found. `artifact validate` passed, with two existing prose-only review-record warnings.

Could not establish runtime behavior; this was a read-only decomposition review without builds.

```findings
- file: .engineering/planning/story/collection-composition-seam.md
  line: 33
  category: design
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: Specify and validate the Collect refusal contract before implementation because the proposed gaps/refusal response has no representation in the existing required-snapshot, collected-only ESS operation
- file: .engineering/planning/story/source-structure.md
  line: 84
  category: design
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: Remove shared production registration from this story’s remaining responsibility because collection-composition-seam now delivers that same abstraction before language acceptance, leaving source-structure responsible for cross-language equivalence and aggregate integration
```
