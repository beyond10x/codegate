---
format: aep.planning-md/3
id: review-result:parity-acceptance-round-1
kind: review-result
status: active
title: Parity acceptance critic round 1
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

story:semantic-navigation — the acceptance permits a failed/partial result even for a healthy installed tool, so it must specify the successful-session trigger and expected evidence to distinguish working collection from an implementation that always fails — .engineering/planning/story/semantic-navigation.md:38
story:parity-adoption — the acceptance combines independently passing catalogue accounting and pilot publication, so it must name one observable completion artifact containing both the reconciled audit and the required pilot evidence — .engineering/planning/story/parity-adoption.md:36

Read: all 19 requested artifacts, comprising epic:semantic-parity and its 18 decomposing stories, through `aep plan artifact show`; also read the acceptance critic, rubric, repository AGENTS.md, semantic contract and scenario manifest, artifact kinds and story/epic lifecycles; `aep plan artifact validate` reports valid.

Limits: runtime implementation and execution were not assessed; pending fixtures and conformance handlers are appropriate for draft stories. Contract coverage, decomposition coupling and shared ownership remain outside this acceptance review.

```findings
- file: .engineering/planning/story/semantic-navigation.md
  line: 38
  category: acceptance
  severity: blocker
  verdict: needs-revision
  origin: introduced
  message: the acceptance permits a failed/partial result even for a healthy installed tool, so it must specify the successful-session trigger and expected evidence to distinguish working collection from an implementation that always fails
- file: .engineering/planning/story/parity-adoption.md
  line: 36
  category: acceptance
  severity: warning
  verdict: needs-revision
  origin: introduced
  message: the acceptance combines independently passing catalogue accounting and pilot publication, so it must name one observable completion artifact containing both the reconciled audit and the required pilot evidence
```

