---
format: aep.planning-md/3
id: review-result:collection-scope-round-1
kind: review-result
status: active
title: First collection scope critic round 1
relations:
- reviews: epic:source-collection-foundation
- reviews: epic:source-collection-baseline
revision: 1
---
needs-revision

epic:source-collection-baseline — “story:first-slice owns the public collect/assess/capabilities API and CLI plus initial name/qualified-name/source-position declaration lookup” promises lookup within this round, but first-slice claims only collection and assessment while lookup is claimed by the later offline-navigation story, so assign the initial lookup outcome explicitly to first-slice and distinguish the later extension — .engineering/planning/epic/source-collection-baseline.md:23

Read 11 artifacts through `aep plan artifact show`, the two epic files, `specifications/source-baseline.md`, graph, kinds and relations; extracted 10 grouped outcome promises and traced 9 to this round’s children, with initial declaration lookup traced only to its later successor. Validation evidence below normalizes the checkout path to repository-relative `.engineering/planning`:
```text
51 file(s) in .engineering/planning: 51 artifact(s)
valid
```

Could not establish runtime completion; this is a scope review of draft artifacts. Implementation ownership consistency and strength of executable acceptance belong to the parallel-safety and acceptance critics.

```findings
- file: .engineering/planning/epic/source-collection-baseline.md
  line: 23
  category: scope
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: “story:first-slice owns the public collect/assess/capabilities API and CLI plus initial name/qualified-name/source-position declaration lookup” promises lookup within this round, but first-slice claims only collection and assessment while lookup is claimed by the later offline-navigation story, so assign the initial lookup outcome explicitly to first-slice and distinguish the later extension
```
