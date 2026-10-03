---
format: aep.planning-md/3
id: epic:semantic-parity
kind: epic
status: active
title: Semantic parity across Go, Rust and Java Quarkus
relations:
- serves: vision:language-neutral-code-quality
- informed_by: architecture-design:language-neutral-fact-ir
- informed_by: executable-system-specification:semantic-analysis
revision: 3
transitions:
- {from: "draft", to: "proposed", at: "2026-10-03T00:06:53Z", actor: "human:timo", revision: 2}
- {from: "proposed", to: "active", at: "2026-10-03T00:06:53Z", actor: "human:timo", revision: 3}
---
## Intent and source

Implement the operator's 2026-10-03 semantic parity plan under the existing
`vision:language-neutral-code-quality`. The accepted design is
`architecture-design:language-neutral-fact-ir`. The pinned reference and exhaustive
capability inventory are `docs/parity-baseline.md` (38 metrics, 38 findings,
9 violations, four gates plus noncatalogue navigation/analysis/reporting surfaces).

## Contract first

`ess-semantic/domains/semantic.yaml` gives the richer values and six operations typed
homes before this decomposition. `specifications/semantic-contract.md` defines
admission, counting, evidence and completeness semantics. This separately versioned
contract preserves `ess/domains/dependency.yaml` and the existing /0.1 evaluator.
Validation and generated obligations do not establish runtime conformance.

## Acceptance

The completed parity audit accounts for every in-scope baseline and requested
extension across Go, Rust and Java/Quarkus with executable evidence or a justified
language-specific non-applicability, leaving no unsupported required capability.

## Delivery sequence

1. Pin reference and generate validated ESS foundation contracts.
2. Collect source/configuration snapshots, admit facts, bind all three languages,
   expose the first dependency assessment slice and offline declaration navigation.
3. Add bounded semantic sessions and gopls, rust-analyzer, JDT LS adapters.
4. Deliver portable measurements, architecture policies, safety/performance/security,
   testability, Quarkus interpretation, scoring and advisory suggestions.
5. Deliver complete JSON/HTML reports, real semantic conformance and parity comparison
   fixtures, then advisory repository pilots before making any new check required.

Eighteen scoped stories decompose this epic; typed depends_on edges sequence them.
Implementation may start only once its contracts validate and prerequisites are
implemented. Four decomposition critics judge acceptance/design/scope/parallel safety;
keep every verdict and record one disposition for every finding, with at most two rounds.

## Required behavior

Bindings emit observations; shared code consumes a private admitted wrapper and never
imports bindings, performs IO or branches on producer/language. Source-only is default.
Explicit semantic mode invokes installed tools, records versions/configuration/
diagnostics, and never silently downgrades. Dirty/untracked selected bytes and relevant
build configuration contribute to source identity; imported evidence must match.
Syntactic candidates remain distinguishable from resolved and unresolved relationships.
Coverage is per family/scope/configuration. Missing evidence never becomes measured zero,
a passing absence check, or a complete required score. Query/evaluate imported facts offline.

Quarkus facts include CDI qualifiers/producers/injection, REST path composition,
transaction declarations and profile-aware configuration. Annotation scanning alone
cannot establish augmentation-dependent wiring. Java17/21 project fixtures use Maven
and Gradle multi-module builds; JDT's host JDK is independent and recorded.

## Verification and non-goals

Keep all 27 legacy evaluator scenarios. Add equivalent-language fixture metrics/policies,
pinned-reference Go comparisons, macro/cfg/build-tag/overload/generated-source cases,
CDI ambiguity/producers, REST/transaction/config profile cases, missing tool/timeout/
stale/partial evidence refusals, deterministic offline replay and enforced core boundaries.
`task check` owns generated drift and actual ESS conformance. Test inventory is distinct
from execution coverage. All authored running code/harnesses are Rust with clap CLI.

Exclude source editing, executable refactoring, MCP, hosted service and Markdown
analysis. No tag/release/GitHub publication or changes to required branch checks are
part of this implementation authorization. Existing 0.1.0 delivery and its Secrets
blocker remain separate and do not block specification/planning.

## Current evidence

Reference archive pin and SHA256 verified. ESS foundation validation and generation
are recorded against the specification artifact. Runtime parity remains unimplemented;
no catalogue gap is marked complete on the strength of generated stubs.
