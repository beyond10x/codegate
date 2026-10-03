---
format: aep.planning-md/3
id: review-result:parity-design-round-1
kind: review-result
status: active
title: Parity design critic round 1
relations:
- reviews: epic:semantic-parity
- reviews: story:parity-foundations
- reviews: story:source-snapshot
- reviews: story:capability-admission
- reviews: story:source-structure
- reviews: story:first-slice
- reviews: story:offline-navigation
- reviews: story:semantic-navigation
- reviews: story:go-semantics
- reviews: story:rust-semantics
- reviews: story:java-semantics
- reviews: story:portable-metrics
- reviews: story:architecture-policies
- reviews: story:safety-observations
- reviews: story:testability-signals
- reviews: story:quarkus-relationships
- reviews: story:scoring-suggestions
- reviews: story:reporting-parity
- reviews: story:parity-adoption
revision: 1
---
needs-revision

story:source-snapshot — Snapshot identity includes syntax-classified SourceLine observations supplied by the later source-structure story, so define an explicit raw-collection-to-final-snapshot seam and assign finalization to source-structure, or move the required line classifier into source-snapshot — .engineering/planning/story/source-snapshot.md:32; specifications/semantic-contract.md:47; specifications/semantic-contract.md:63; .engineering/planning/story/source-structure.md:10

Read: All 19 requested artifacts through `aep plan artifact show`, architecture/specification artifacts, the semantic domain and normative contract; ran `relations`, `graph` and `validate` (valid); examined all 87 graph edges, including outside the requested set, and walked its 27 `depends_on` edges without finding a declared cycle.

Limits: This is a decomposition review, not runtime verification; acceptance completeness and concurrent file ownership remain outside this lane. The source-snapshot finding concerns ownership of the finalized identity, not a requirement that draft stories already contain implemented code.

```findings
- file: .engineering/planning/story/source-snapshot.md
  line: 32
  category: design
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: Snapshot identity includes syntax-classified SourceLine observations supplied by the later source-structure story, so define an explicit raw-collection-to-final-snapshot seam and assign finalization to source-structure, or move the required line classifier into source-snapshot
```

