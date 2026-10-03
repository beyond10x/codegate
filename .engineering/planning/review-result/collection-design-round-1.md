---
format: aep.planning-md/3
id: review-result:collection-design-round-1
kind: review-result
status: active
title: First collection design critic round 1
relations:
- reviews: epic:source-collection-foundation
- reviews: epic:source-collection-baseline
revision: 1
---
needs-revision

epic:source-collection-foundation — The declared prerequisite task:clean-external-target-gate has no ordering relation, so add a depends_on edge recording that harness repair before the foundation round — .engineering/planning/epic/source-collection-foundation.md:22

Read nine requested artifacts, two prerequisite artifacts and specifications/source-baseline.md; ran artifact show, relations, graph and validate, walked all 37 depends_on edges including those outside the reviewed set, and found no dependency cycle. Validation returned 51 artifacts, valid.

Could not establish: acceptance completeness and ownership collisions are outside this design review’s lane.

```findings
- file: .engineering/planning/epic/source-collection-foundation.md
  line: 22
  category: design
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: The declared prerequisite task:clean-external-target-gate has no ordering relation, so add a depends_on edge recording that harness repair before the foundation round
```
