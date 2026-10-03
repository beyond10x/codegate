---
format: aep.planning-md/3
id: verification-report:collection-composition
kind: verification-report
status: draft
title: Public collection composition integration evidence
relations:
- verifies: story:collection-composition-seam
revision: 2
---
## Verified integration

At dc0f2a084eebce244cb6ad2d720f75b8a87539e2, task check exited 0 using pinned ESS 0.50.0, Rust 1.98.1, two Cargo jobs and sccache. Native evaluator 27/27, foundation 8/8 and public collection 4/4 all passed with zero failed, error, unsupported or skipped results. Root Rust test summaries total 93 passed. Every generated-drift, formatting, Clippy and documentation step exited 0; exact step outputs are verification/collection-composition/gate-steps.txt. Native suites, reports and runs are retained in that directory's evaluator, foundation and collection subdirectories.

The collection report's model is c64e8e2f5e1a2e791ed83c172875fb04a67c095ec27f188220b1d5644654c126; suite digest is sha256:0d4eb161d607ab3062d0a2607b84d3362856c5570af800fc7eea32701d274de2. Three authored scenarios plus one structural scenario invoke the actual generated CollectBehavior and admit the actual returned snapshot. This is component-scoped evidence with thirteen scenarios outside its selection, not semantic parity.

## Red-capable witnesses and corrections

The API was absent at baseline 2132b5b, producing a compiler failure before implementation. A separate executed runtime test observed a foreign slot's coverage being admitted before the ownership guard was added; its corrected class covers every emitted observation array, with foreign targets permitted only for dependency relationships. Runtime package summaries increased from 81 to 92; adding native collection conformance increases integrated root tests to 93.

Seven seeded mutation checks make named authored cases fail for empty responses, false complete coverage, constant identities and semantic-mode downgrade. Their native run records are retained alongside collection reports. The fixture source and empty-failure hashes were derived from literal bytes and independently canonicalized JSON before the implementation was available.

The root boundary initially rejected the public collector reexports. Exact public wiring is now distinguished from algorithm calls: all original assertions remain, and regression tests refuse direct or renamed calls from shared code, non-root imports and wildcard exports.

ESS initially refused tracked generation roots: `error: unowned output destination: Cargo.toml; adopt exact generated reference bytes explicitly`. The established gate pattern generated into fresh scratch, formatted generated output and copied exact bytes. No generated source was hand-edited. Structural Collect input is supplied through declared fixture_inputs because automatic placeholder identities are invalid production requests; authored literals and production validation remain unchanged.

## Limits and pending closure

Default registration has no language extractors. Valid source capture alone never certifies declarations, calls, measurements, scores or framework wiring. Semantic requests retain Semantic and report absent adapters; no installed-tool claim is fabricated. Invalid requests fail the invocation, while valid failed capture returns an admitted empty observed snapshot with Failed Sources coverage.

Independent adversarial review, any required corrections, PR publication and managed cleanup remain pending at this record. Five other public handlers, the semantic CLI, Go/Rust/Java extraction, assessments and polished HTML reporting remain delivery work under the original goal. No version/tag is selected by this wave.

## Final integration and wave closure

Final task check at e53a4d4 exited 0 after integrating all four adversarial tests. Native evaluator 27/27, foundation 8/8, collection 4/4 and root Rust 97/97 passed. Final suites, reports, runs and per-step exit outputs are retained under verification/collection-composition/final/. Review-result:collection-composition-adversary records the one bounded independent pass verbatim, including its author-normalized public log paths and empty findings block. There are no open adversary findings and no second attack was needed. Original private logs remain retained for archive cleanup.

Confirmed implementation scope includes composition/runtime helpers, public exports, the narrow boundary regression, exact ESS fixture/component/scenario registration, generated semantic projections, native harness and gate, CI evidence upload paths, and current runtime-limit documentation. The scoper's inferred absence of documentation changes was corrected from actual stale text; collection module registration and regression tests were also confirmed. No shared analysis or admission policy was relaxed.

This completes story:collection-composition-seam, not epic:source-collection-baseline or the repository-report goal. Replan next from the three language-source stories against this now-frozen API. PR integration is authorized; publish the reviewed candidate, merge required green checks, archive private evidence and clean exact managed trees. No new release is included. Cost counters remain unavailable in this host.
