# Planned semantic conformance manifest

Status: **planned, not executable, not run**. These IDs are acceptance obligations
for AEP implementation stories after the typed foundation. They are not entries in
`ess-semantic/ess-inputs.yaml` until real authored inputs and expected observations
exist. No pass count or conformance status is claimed by this manifest. The existing
27 dependency evaluator scenarios remain separate and must continue to pass.

| Planned ID | Required witness / expected observation |
|---|---|
| semantic-source-selection | Include/exclude, test and generated selections deterministically choose exact files. |
| semantic-dirty-untracked-identity | Dirty and untracked selected bytes change snapshot ID; unrelated files do not. |
| semantic-configuration-identity | Tags/features/cfg/classpath/profiles/manifests change the bound configuration/snapshot. |
| semantic-stale-import-refused | Imported semantic evidence for changed source/configuration is refused. |
| semantic-source-mutates-during-collection | Changing selected bytes during tool execution prevents complete results. |
| semantic-admission-invalid-reference | Duplicate/dangling IDs, invalid ranges and cycles in ownership are refused. |
| semantic-resolution-distinct | Same spelling as candidate/resolved/unresolved stays observably distinct. |
| semantic-coverage-family-configuration | One complete family/configuration cannot complete another family/configuration. |
| semantic-missing-is-not-zero | Missing/partial required facts yield absent measurement and incomplete check/score. |
| semantic-witnessed-violation-with-gaps | A proved violation remains visible despite unrelated incomplete evidence. |
| semantic-complete-empty | Complete empty selected graph yields zero fan-out while empty ratios are not applicable. |
| semantic-go-first-slice | Go fixture emits exact declarations, locations, dependency and fan-out/forbidden-edge result. |
| semantic-rust-first-slice | Equivalent Rust fixture has same shared graph/check result. |
| semantic-java-first-slice | Equivalent Java fixture has same shared graph/check result. |
| semantic-syntax-error | Syntax error is located and required affected coverage is partial/failed. |
| semantic-source-only-default | Default CLI collection invokes no semantic process and marks unresolved semantic families. |
| semantic-semantic-mode-no-downgrade | Explicit semantic mode with missing tool refuses completeness instead of source-only success. |
| semantic-tool-timeout | Timeout retains tool identity/configuration/diagnostics and cannot yield complete coverage. |
| semantic-tool-failure | Nonzero language server outcome remains visible, preventing complete dependent results. |
| semantic-name-qualified-position | Name, qualified name and byte-position lookups find exact source declarations. |
| semantic-overload-ambiguity | Java overload and same-name candidates stay multiple unless evidence resolves them. |
| semantic-go-navigation | gopls definitions/references/implementations/static callers/callees match selected Go snapshot. |
| semantic-go-build-tags | Selected tags change admitted declarations/calls; excluded configuration cannot complete selection. |
| semantic-dynamic-dispatch-gap | Dynamic calls omitted by a tool remain an explicit completeness limitation. |
| semantic-rust-navigation | rust-analyzer relationships normalize to shared navigation with evidence basis. |
| semantic-rust-cfg-macro | cfg/features/macro expansion gaps remain visible; matching evidence can close only its scope. |
| semantic-java-navigation | JDT LS definitions/references/implementations/calls honor overload signatures. |
| semantic-java-build-matrix | Maven/Gradle multi-module fixtures with Java 17/21 bind exact classpath and target. |
| semantic-java-host-jdk-independent | Host JDK version is recorded separately and does not rewrite project target. |
| semantic-java-generated-sources | Unavailable generated declarations cannot become complete empty relations. |
| semantic-framework-declarations | Quarkus 3 bean/qualifier/producer/injection/REST/transaction/config facts retain locations. |
| semantic-cdi-qualifier-ambiguity | Multiple matching beans yield ambiguous candidates and no effective injection binding. |
| semantic-cdi-producer-injection | Producer declaration, produced type and qualifier members link declared candidates. |
| semantic-cdi-build-evidence | Matching augmentation evidence establishes wiring; stale evidence is refused. |
| semantic-cdi-programmatic-generated-gap | Programmatic lookup/generated beans without build evidence remain explicit gaps. |
| semantic-rest-path-composition | Resource/method paths compose predictably while declared/deployed distinction survives. |
| semantic-transactions-declared | Transaction modes are observations; interceptor activation is not inferred. |
| semantic-config-profiles | Configuration reference key/profile is preserved, profile changes identity; secret values absent. |
| semantic-portable-counts | The three contract examples agree at parameters=1, returns=2, complexity=2, nesting=1. |
| semantic-counting-boundaries | Newlines, empty files, default arms, short circuits, nested callables and receivers follow contract. |
| semantic-go-defer-specific | GoDefer remains explicitly language-specific without invented Rust/Java facts. |
| semantic-shared-architecture | Direction/layers/cycles/test boundaries/unknown units/forbidden calls/effects use admitted shared facts. |
| semantic-maintainability-breadth | Sizes/API/docs/debt/naming each bind their required normalized evidence. |
| semantic-effects-breadth | Discarded results/exits/exec/SQL/path/unsafe/crypto/reflection/loop-allocation facts preserve evidence. |
| semantic-test-inventory-not-coverage | Test/source/generated ratios and nondeterminism never imply execution coverage. |
| semantic-policy-exception | Reasoned scoped exceptions retain witnessed findings; explicit evaluation_time controls expiry. |
| semantic-score-requires-all | Removing any selected required evidence removes overall_score and complete gate status. |
| semantic-score-version | Unknown scoring contract refuses; selected version fixes ranking and weighting. |
| semantic-suggestions-advisory | Suggestions refer to ranked findings and contain no executable edits. |
| semantic-mixed-language | One snapshot holds all three language facts without guessing cross-language edges. |
| semantic-offline-replay | Imported evaluation/lookup/suggestion bytes repeat without tools, IO or wall-clock dependence. |
| semantic-core-boundary | Architectural test refuses any binding/IO dependency or language dispatch in shared core. |
| semantic-json-html-evidence | JSON and standalone HTML retain same coverage, gaps, absent scores and findings. |
| semantic-reference-comparison | Pinned Go reference fixture outputs mapped capability-by-capability, differences documented. |
| semantic-advisory-pilots | Repository pilots record gaps and advisory findings before any enforcement enrollment. |
| semantic-generated-drift | Regeneration under pinned ESS/Rust matches both generated semantic crates and reports. |
| semantic-dependency-compatibility | Original /0.1 JSON, API and 27 executable scenarios remain unchanged and pass. |

Each implementation story selects concrete IDs from this manifest, turns those
obligations into executable authored scenarios, and records real conformance
report/suite identity. Expanding a story's semantic scope requires corresponding
contract and scenario changes first.
