# Planned parity acceptance index

This index links the AEP decomposition to named future executable verification cases.
It is not an ESS scenario file or a conformance report. Runtime cases must be authored
in ESS and executed against real handlers when their stories are implemented. Tooling
cases in parity-foundations instead verify compilation, generation/drift and retention
of the existing 27-case real evaluator suite. The detailed semantic-* cases in
[semantic-scenarios.md](semantic-scenarios.md) supplement these obligations.

| Owning story | Planned case IDs | Required observable result |
|---|---|---|
| `story:parity-foundations` | `parity-catalog-complete`, `parity-generated-drift`, `parity-old-contract-preserved` | When task check runs on the new foundation, it verifies exact generated semantic contracts beside the unchanged 27 evaluator cases with zero drift or omitted cases. |
| `story:source-snapshot` | `parity-selected-content-identity`, `parity-manifest-identity`, `parity-path-confinement` | Collecting equivalent selected bytes and configuration produces the same snapshot identity, while any selected dirty/untracked input or relevant build configuration change changes that identity. |
| `story:capability-admission` | `parity-stale-evidence`, `parity-dangling-observation`, `parity-coverage-not-zero`, `parity-core-boundary` | Admitting malformed or mismatched evidence returns a stable refusal before any shared checker can observe an admitted graph. |
| `story:source-structure` | `parity-three-language-structure`, `parity-syntax-error`, `parity-candidate-not-resolved`, `parity-quarkus-annotations` | Collecting equivalent Go/Rust/Java structure fixtures returns the specified declarations, source spans and syntactic observations with syntax errors reflected in family coverage. |
| `story:first-slice` | `parity-first-slice-three-languages`, `parity-forbidden-with-gaps`, `parity-assess-equals-offline` | Assessing equivalent Go/Rust/Java first-slice fixtures with one shared forbidden-dependency policy yields identical normalized unique fan-out and witnessed policy outcomes. |
| `story:offline-navigation` | `parity-offline-lookup`, `parity-ambiguous-name`, `parity-unicode-position`, `parity-query-truncation` | Lookup over imported admitted facts returns expected definitions/references/implementations/callers/callees for name, qualified name and position selectors without invoking a source collector or tool. |
| `story:semantic-navigation` | `parity-missing-tool`, `parity-tool-timeout`, `parity-partial-tool`, `parity-stale-tool-result` | Given the healthy deterministic Rust LSP fixture server, semantic collection returns exact resolved relationships and provenance; separate failure fixtures produce their specified outcomes. |
| `story:go-semantics` | `parity-gopls-navigation`, `parity-go-build-tags`, `parity-go-dynamic-calls` | A real installed gopls session resolves the named Go fixture references/implementations/calls under selected build tags while documenting dynamic-call omissions as gaps. |
| `story:rust-semantics` | `parity-rust-analyzer-navigation`, `parity-rust-cfg-features`, `parity-rust-macro-gap` | A real installed rust-analyzer session resolves the selected Cargo fixture navigation while cfg/features and unexpanded macros retain their specified completeness outcomes. |
| `story:java-semantics` | `parity-jdtls-maven17`, `parity-jdtls-gradle21`, `parity-java-overload`, `parity-java-generated-gap` | A real installed JDT LS session resolves Java overload/reference/call fixtures for Maven and Gradle multi-module projects targeting Java 17 and 21 with the supported host JDK recorded independently. |
| `story:portable-metrics` | `parity-metric-equivalence`, `parity-ratio-aggregation`, `parity-missing-metric`, `parity-debt-comment-only` | Shared metric evaluation of equivalent Go/Rust/Java fixtures matches the exact expected counts and rational ratios under the named counting contract. |
| `story:architecture-policies` | `parity-architecture-matrix`, `parity-cycle`, `parity-exception-expiry`, `parity-partial-architecture` | Evaluating the architecture scenario matrix produces the expected direction/layer/cycle/fan-in-out/import/call/effect/test-boundary/unknown-unit and exception results from admitted facts. |
| `story:safety-observations` | `parity-safety-matrix`, `parity-discarded-result-not-error`, `parity-language-specific-effects` | Shared checks over the safety/performance/security fixture matrix produce evidence-qualified findings for every applicable baseline row without labeling syntactic suspicion as a proven runtime vulnerability. |
| `story:testability-signals` | `parity-test-inventory`, `parity-generated-ratio`, `parity-nondeterminism`, `parity-inventory-not-coverage` | Testability fixture evaluation reports expected tests, benchmarks/fuzz/parameterized shapes, source/generated ratios and nondeterminism observations while leaving execution coverage unavailable without execution evidence. |
| `story:quarkus-relationships` | `parity-cdi-qualifier-ambiguity`, `parity-cdi-producer`, `parity-rest-path`, `parity-transactions`, `parity-config-profiles`, `parity-augmentation-gap` | Quarkus 3 fixtures yield expected declared CDI, REST, transaction and configuration relationships while ambiguous or augmentation-dependent wiring remains incomplete without matching build evidence. |
| `story:scoring-suggestions` | `parity-score-required-gap`, `parity-score-policy-version`, `parity-suggestion-ranking`, `parity-advice-never-edits` | A selected versioned scoring policy produces deterministic ranked findings, pressure and advisory suggestions, with no complete overall score when any required component lacks evidence. |
| `story:reporting-parity` | `parity-report-json-html`, `parity-html-escaping`, `parity-offline-replay`, `parity-exit-codes` | The report conformance matrix produces deterministic versioned JSON and escaped standalone HTML containing exact evidence, capability gaps, limits, gate statuses, ranked findings and scores for admitted inputs. |
| `story:parity-adoption` | `parity-pinned-reference-comparison`, `parity-mixed-language`, `parity-advisory-pilots` | One parity-adoption audit artifact contains all reconciled catalogue evidence and the required advisory pilot results, with no unsupported required capability. |

The AEP story body is authoritative if revised after this index. In particular the
semantic-navigation story requires successful evidence from the healthy Rust fixture
server, and parity-adoption requires a single reconciled audit artifact. Each real
language adapter additionally requires installed-tool fixture execution, never only
mock-server success. New names are acceptance obligations, not counted passing tests.
