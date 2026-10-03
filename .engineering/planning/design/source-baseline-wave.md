---
format: aep.planning-md/3
id: design:source-baseline-wave
kind: design
status: draft
title: 'Three-language source baseline: ownership, mappings and native witnesses'
relations:
- designs: story:source-go-baseline
- designs: story:source-rust-baseline
- designs: story:source-java-baseline
- informed_by: approval-record:standing-wave-approval
revision: 3
---
## Intent, authorization and current status

Prepare the next source collection wave after Codegate 0.3.0. The repository-report objective remains incomplete: source extraction feeds source assessment/navigation, then repository JSON/HTML reporting. This page records implementation decisions and dispatch conditions; it does not claim that bindings or their cases already execute.

AEP implementing skill version 0.19.1. Standing interactive wave approval is approval-record:standing-wave-approval; the operator separately authorized integration via bot PR and a clean primary. The worker cap is three alongside coordination. Approval covers the opening planning commit, three bounded unit commits and their necessary corrections, integration merges, closing evidence/store commit and reviewed PR merge. It does not authorize another release. The last goal turn made progress by verifying and publishing 0.3.0; this turn advances scoped preparation.

Selected candidates are story:source-go-baseline, story:source-rust-baseline and story:source-java-baseline, serving vision:language-neutral-code-quality. Their dependencies are implemented at baseline 821de3626e019c5c8203ea7f8f026b981c7a0d09. Three read-only aep:story-scoper roles inspected the actual code. This host uses generic collaboration agents loading the plugin role procedure rather than plugin subagent_type selectors; scope_go, compose_scope and compose_conformance were the three independent readers. Their prior implementation trees are retired. No new implementation worker has been dispatched.

## Resource preflight and ownership

Observed clean primary main matches remote main at 821de3626e019c5c8203ea7f8f026b981c7a0d09. Worktree inventory contained only primary before this planning tree. The coordinator planning tree is wt-bc178b2aad87, branch plan/source-baseline-wave, scratch .scratch/source-wave; its exact absolute path/lease are retained by the private worktree registry. No build target is allocated in this planning-only tree.

Home free space fell from 14 GiB to 13 GiB during preflight. AGENTS.md requires 20 GiB before a large build, and aep:implementing refuses implementation dispatch below the floor. resource-blocker:source-baseline-build-space records the condition and clearance evidence. Previous release builds measured roughly 2.3 GiB per tree including standalone generated targets and were cleaned after evidence retention. No earlier owned Codegate target remains available for reclamation. Three simultaneous targets need approximately 27 GiB at launch; otherwise serialize build admissions while keeping at most three workers. Each new launch rechecks actual capacity. Other repositories' storage is outside this task's cleanup authority.

Keep implementation stories draft while the dispatch preflight fails. Read-only scoping and this planning PR are useful independent work. After clearance, re-fetch main, re-run readiness/waves, preserve this decision record and record each exact tree/branch/base/target/scratch triple before dispatch. Unit targets stay inside their own managed trees, two Cargo jobs, RUSTC_WRAPPER=/usr/bin/sccache and CARGO_INCREMENTAL=0. ESS must resolve to pinned 0.50.0 and Rust to 1.98.1. A measured resource refusal is not permission to send workers into unverified implementation.

## Scope and the scheduler

The three scopers confirmed src/bindings/<language>.rs, tests/source_<language>.rs and tests/fixtures/structure/<language>/** as cited unit assignments. Shared registration and native conformance are also real story-owned surfaces even when the coordinator, rather than the language worker, edits them.

An attempted typed scope on this design was refused: scope belongs to story, not design. The initial inference that coordinator ownership could keep all three complete stories disjoint was therefore withdrawn. Shared paths are now recorded on every affected story, and the computed scheduling serializes Go, Java and Rust. Follow that scheduling; do not dispatch three simultaneous language implementation stories based only on their parser files. Within one story, a language implementor and a separately assigned conformance worker may work concurrently on disjoint files under coordinator integration, within the three-worker cap. The immediate candidate after resource clearance is source-go-baseline; replan after it closes.

Coordinator-owned edits inside each selected story: src/bindings/mod.rs; src/collection/compose.rs; src/lib.rs if exports require it; tests/collection_conformance.rs; new tests/source_language_conformance.rs and tests/support/collection_conformance.rs if transport is factored; tests/collection_composition.rs and tests/collection_adversary.rs for explicit empty-registry setup; src/bin/codegate-check.rs; tests/freshness.rs and tests/gate_adversary.rs; ess-semantic/ess-inputs.yaml; the language's authored files under ess-semantic/scenarios/collection/languages/; generated/semantic-behavior and generated/semantic-wire only if deterministic regeneration proves changes; .github/workflows/ci.yml evidence paths; AGENTS.md, README.md, specifications/semantic-contract.md and specifications/semantic-scenarios.md. All are inferred modification surfaces, to be narrowed from actual diffs at closure. Required parser dependencies already exist. Source-foundation and offline evaluator behavior stays governed by its existing acceptance.

The full final computed story scheduling follows verbatim. Scope/dependency placement does not establish resource readiness; the open blocker still prevents dispatch. There are no unassessed stories or cycles. This output supersedes the original three-language wave inference.

```text
wave 1
  story:source-go-baseline (inferred)
wave 2
  story:source-java-baseline (inferred)
wave 3
  story:source-rust-baseline (inferred)
wave 4
  story:source-structure (inferred)
wave 5
  story:first-slice (inferred)
  story:source-structural-observations (inferred)
wave 6
  story:offline-navigation (inferred)
  story:portable-metrics (inferred)
wave 7
  story:architecture-policies (inferred)
  story:safety-observations (inferred)
  story:semantic-navigation (inferred)
  story:testability-signals (inferred)
wave 8
  story:go-semantics (inferred)
  story:java-semantics (inferred)
  story:rust-semantics (inferred)
wave 9
  story:quarkus-relationships (inferred)
wave 10
  story:scoring-suggestions (inferred)
wave 11
  story:reporting-parity (inferred)
wave 12
  story:parity-adoption (inferred)
collision: story:first-slice story:offline-navigation src/lib.rs (inferred)
collision: story:first-slice story:offline-navigation src/main.rs (inferred)
collision: story:first-slice story:reporting-parity src/main.rs (inferred)
collision: story:first-slice story:scoring-suggestions src/lib.rs (inferred)
collision: story:first-slice story:scoring-suggestions src/main.rs (inferred)
collision: story:first-slice story:semantic-navigation src/lib.rs (inferred)
collision: story:first-slice story:semantic-navigation src/main.rs (inferred)
collision: story:first-slice story:source-go-baseline src/lib.rs (inferred)
collision: story:first-slice story:source-java-baseline src/lib.rs (inferred)
collision: story:first-slice story:source-rust-baseline src/lib.rs (inferred)
collision: story:offline-navigation story:reporting-parity src/main.rs (inferred)
collision: story:offline-navigation story:scoring-suggestions src/lib.rs (inferred)
collision: story:offline-navigation story:scoring-suggestions src/main.rs (inferred)
collision: story:offline-navigation story:semantic-navigation src/lib.rs (inferred)
collision: story:offline-navigation story:semantic-navigation src/main.rs (inferred)
collision: story:offline-navigation story:source-go-baseline src/lib.rs (inferred)
collision: story:offline-navigation story:source-java-baseline src/lib.rs (inferred)
collision: story:offline-navigation story:source-rust-baseline src/lib.rs (inferred)
collision: story:reporting-parity story:scoring-suggestions src/main.rs (inferred)
collision: story:reporting-parity story:semantic-navigation src/main.rs (inferred)
collision: story:reporting-parity story:source-go-baseline src/bin/codegate-check.rs (inferred)
collision: story:reporting-parity story:source-java-baseline src/bin/codegate-check.rs (inferred)
collision: story:reporting-parity story:source-rust-baseline src/bin/codegate-check.rs (inferred)
collision: story:scoring-suggestions story:semantic-navigation src/lib.rs (inferred)
collision: story:scoring-suggestions story:semantic-navigation src/main.rs (inferred)
collision: story:scoring-suggestions story:source-go-baseline src/lib.rs (inferred)
collision: story:scoring-suggestions story:source-java-baseline src/lib.rs (inferred)
collision: story:scoring-suggestions story:source-rust-baseline src/lib.rs (inferred)
collision: story:semantic-navigation story:source-go-baseline src/lib.rs (inferred)
collision: story:semantic-navigation story:source-java-baseline src/lib.rs (inferred)
collision: story:semantic-navigation story:source-rust-baseline src/lib.rs (inferred)
collision: story:source-go-baseline story:source-java-baseline .github/workflows/ci.yml (inferred)
collision: story:source-go-baseline story:source-java-baseline AGENTS.md (inferred)
collision: story:source-go-baseline story:source-java-baseline README.md (inferred)
collision: story:source-go-baseline story:source-java-baseline ess-semantic/ess-inputs.yaml (inferred)
collision: story:source-go-baseline story:source-java-baseline generated/semantic-behavior (inferred)
collision: story:source-go-baseline story:source-java-baseline generated/semantic-wire (inferred)
collision: story:source-go-baseline story:source-java-baseline specifications/semantic-contract.md (inferred)
collision: story:source-go-baseline story:source-java-baseline specifications/semantic-scenarios.md (inferred)
collision: story:source-go-baseline story:source-java-baseline src/bin/codegate-check.rs (inferred)
collision: story:source-go-baseline story:source-java-baseline src/bindings/mod.rs (inferred)
collision: story:source-go-baseline story:source-java-baseline src/collection/compose.rs (inferred)
collision: story:source-go-baseline story:source-java-baseline src/lib.rs (inferred)
collision: story:source-go-baseline story:source-java-baseline tests/collection_adversary.rs (inferred)
collision: story:source-go-baseline story:source-java-baseline tests/collection_composition.rs (inferred)
collision: story:source-go-baseline story:source-java-baseline tests/collection_conformance.rs (inferred)
collision: story:source-go-baseline story:source-java-baseline tests/freshness.rs (inferred)
collision: story:source-go-baseline story:source-java-baseline tests/gate_adversary.rs (inferred)
collision: story:source-go-baseline story:source-java-baseline tests/source_language_conformance.rs (inferred)
collision: story:source-go-baseline story:source-java-baseline tests/support/collection_conformance.rs (inferred)
collision: story:source-go-baseline story:source-rust-baseline .github/workflows/ci.yml (inferred)
collision: story:source-go-baseline story:source-rust-baseline AGENTS.md (inferred)
collision: story:source-go-baseline story:source-rust-baseline README.md (inferred)
collision: story:source-go-baseline story:source-rust-baseline ess-semantic/ess-inputs.yaml (inferred)
collision: story:source-go-baseline story:source-rust-baseline generated/semantic-behavior (inferred)
collision: story:source-go-baseline story:source-rust-baseline generated/semantic-wire (inferred)
collision: story:source-go-baseline story:source-rust-baseline specifications/semantic-contract.md (inferred)
collision: story:source-go-baseline story:source-rust-baseline specifications/semantic-scenarios.md (inferred)
collision: story:source-go-baseline story:source-rust-baseline src/bin/codegate-check.rs (inferred)
collision: story:source-go-baseline story:source-rust-baseline src/bindings/mod.rs (inferred)
collision: story:source-go-baseline story:source-rust-baseline src/collection/compose.rs (inferred)
collision: story:source-go-baseline story:source-rust-baseline src/lib.rs (inferred)
collision: story:source-go-baseline story:source-rust-baseline tests/collection_adversary.rs (inferred)
collision: story:source-go-baseline story:source-rust-baseline tests/collection_composition.rs (inferred)
collision: story:source-go-baseline story:source-rust-baseline tests/collection_conformance.rs (inferred)
collision: story:source-go-baseline story:source-rust-baseline tests/freshness.rs (inferred)
collision: story:source-go-baseline story:source-rust-baseline tests/gate_adversary.rs (inferred)
collision: story:source-go-baseline story:source-rust-baseline tests/source_language_conformance.rs (inferred)
collision: story:source-go-baseline story:source-rust-baseline tests/support/collection_conformance.rs (inferred)
collision: story:source-go-baseline story:source-structural-observations src/bindings/go.rs (inferred)
collision: story:source-java-baseline story:source-rust-baseline .github/workflows/ci.yml (inferred)
collision: story:source-java-baseline story:source-rust-baseline AGENTS.md (inferred)
collision: story:source-java-baseline story:source-rust-baseline README.md (inferred)
collision: story:source-java-baseline story:source-rust-baseline ess-semantic/ess-inputs.yaml (inferred)
collision: story:source-java-baseline story:source-rust-baseline generated/semantic-behavior (inferred)
collision: story:source-java-baseline story:source-rust-baseline generated/semantic-wire (inferred)
collision: story:source-java-baseline story:source-rust-baseline specifications/semantic-contract.md (inferred)
collision: story:source-java-baseline story:source-rust-baseline specifications/semantic-scenarios.md (inferred)
collision: story:source-java-baseline story:source-rust-baseline src/bin/codegate-check.rs (inferred)
collision: story:source-java-baseline story:source-rust-baseline src/bindings/mod.rs (inferred)
collision: story:source-java-baseline story:source-rust-baseline src/collection/compose.rs (inferred)
collision: story:source-java-baseline story:source-rust-baseline src/lib.rs (inferred)
collision: story:source-java-baseline story:source-rust-baseline tests/collection_adversary.rs (inferred)
collision: story:source-java-baseline story:source-rust-baseline tests/collection_composition.rs (inferred)
collision: story:source-java-baseline story:source-rust-baseline tests/collection_conformance.rs (inferred)
collision: story:source-java-baseline story:source-rust-baseline tests/freshness.rs (inferred)
collision: story:source-java-baseline story:source-rust-baseline tests/gate_adversary.rs (inferred)
collision: story:source-java-baseline story:source-rust-baseline tests/source_language_conformance.rs (inferred)
collision: story:source-java-baseline story:source-rust-baseline tests/support/collection_conformance.rs (inferred)
collision: story:source-java-baseline story:source-structural-observations src/bindings/java.rs (inferred)
collision: story:source-rust-baseline story:source-structural-observations src/bindings/rust.rs (inferred)
12 wave(s), 91 collision(s), 0 unassessed

```

## Frozen common binding contract

Use the existing SourceBinding::collect(&CollectedSource)->BindingFacts interface in src/bindings/mod.rs, retained source/configuration bytes from src/collection/mod.rs and parse_source from src/bindings/lines.rs. Never reread source files or invoke Cargo, Go, Java, Maven, Gradle or language servers. Use shared stable_id/source_range/source_evidence helpers. All authored implementation and harnesses are Rust; Go/Java source is fixture data.

Identifiers bind source snapshot, configuration, language-specific unit anchor, declaration kind and exact location using length-delimited stable_id. Qualified names are display names, not proof of relationship resolution. Whole-declaration spans belong on declarations; narrow identifier spans belong on Definition occurrences. Declaration.parent is lexical containment in the same file, never a type-membership shortcut. Cross-file/package association is represented by unit_id rather than an invalid parent span. Syntax recovery must not create repaired declarations or inferred signatures.

Bindings emit only observed units, baseline declarations, definition/import occurrences, candidate relationship observations where explicitly supported, dependencies and the initial Java framework catalogue. Body/parameter/return/nesting/decision/documentation/test observations remain the later source-structural-observations story. The scopers' initial Go/Rust fixture recommendations included portable metric observations; the coordinator corrected that expansion against story:source-structure's explicit boundary. Do not silently count closures as ordinary functions: the current FunctionCount contract includes all Function declarations and has no closure exemption. Nested closures/function literals remain fixture gap witnesses until the structural story specifies their counting and ownership. Calls inside unsupported anonymous callables must not be attributed to their enclosing named function.

Do not emit Sources coverage from a binding. Unit evidence participates in source admission, so put extraction/cfg/macro/classpath uncertainty in the affected fact families and facts, not a unit evidence gap that contradicts Complete captured Sources. Sources still reports actual capture/parser gaps. Unassigned selected files, omitted declaration forms and unsupported occurrence roles prevent complete affected populations. Distinguish complete known syntactic occurrences from complete occurrence-family support; emitting only definitions/imports is not full read/write/type-use coverage. Candidate relationships have no resolved target and require a reason. Calls/references/implementations cannot claim complete semantic resolution in source-only mode. Unknown target units remain targetless dependencies, never fabricated empty-path units.

## Language mapping decisions

### Go

Proposed implementation convention, grounded in the story and scoper inspection: group a unit by observed manifest/module root, package directory and declared package name, with sorted selected paths. Split external-test packages and distinct module roots even if the package spelling matches. A package declaration is observed per source file and cannot parent unrelated files. Methods remain under a containing lexical package/file declaration when that range actually contains them; receiver type is part of the signature/qualified name, not an out-of-range lexical parent.

Manifest.module is a directory identity, not the Go import path. Read the retained go.mod module directive and establish a unique selected package target before resolving a local import. Preserve grouped/aliased and repeated import occurrences. Missing/external/ambiguous targets stay unresolved. Go build directives and OS/architecture suffixes carry explicit ConditionalCompilation gaps when effective inclusion is not established; changing requested tags/target changes source/configuration identity and never itself proves inclusion. Directive-shaped strings and non-directive comments must not trigger directive handling.

### Rust

Use observed module units, not one file or one Cargo package as an artificial equivalent. Establish a target anchor from retained manifest/source evidence; distinguish lib and bin targets. Explicit literal [lib].path and [[bin]].path are direct witnesses. Conventional target/module paths require documented supported rules and tests; manifest directory alone is not target membership. Unsupported/custom/ambiguous target selection retains UnknownUnit or build-selection gaps. Unassigned selected files remain represented in captured source and incomplete family coverage.

Inline modules have their own units and exact full-module locations; several units may share one source file. Out-of-line mod inclusion resolves only under a supported unique selected path rule. Two conventional candidate files, unsupported #[path], conditional membership or missing selected content produce ambiguity/selection gaps. A namespace such as cargo:<manifest-path>#lib:<target-name>::crate::util distinguishes targets and nested modules. The namespace is not resolution evidence.

An impl method's parent is the enclosing lexical module declaration, never a separately ranged struct/trait declaration. Its qualified name/signature preserves written impl subject and trait with a location discriminator if needed; this does not establish type resolution. Trait methods may parent to their containing trait. Grouped/aliased imports preserve leaf occurrences; crate/self/super paths resolve only when established module lineage and selected declarations prove a unique target. Globs, reexports and externals otherwise remain unresolved. Module containment does not automatically create an import dependency. Cfg and macros retain their explicit gaps; no invented expanded declarations. Anonymous callable semantics remain the later structural contract as above.

### Java and declared Quarkus

Group declared package units with their observed Maven/Gradle module anchor to avoid merging identical packages from distinct modules. Package-less files retain an explicit default-package namespace under that anchor; ambiguous module membership remains a gap. Parent classes/interfaces/methods/fields by actual lexical containment; overload signatures preserve types/modifiers without copying method bodies, field initializers or annotation values.

Recognize declared framework annotation names only through unambiguous explicit imports, fully qualified spellings or established local declarations. Wildcards, same-name local types and conflicting imports require ambiguity gaps. Classpath digests and Java target version alone establish no classpath resolution. Generated classification and Java 17/21 syntax are independently exercised.

Preserve declared bean scope, qualifier identity/nonbinding declarations, producer type, injection point, resource/method paths, HTTP method, transaction mode and configuration key/profile. Effective CDI injection, generated/augmentation-dependent beans and deployed routes remain FrameworkWiring gaps. Configuration defaults and arbitrary annotation values must not enter occurrence spellings, signatures, diagnostic prose or serialized facts. Qualifier identity values cannot simply be silently redacted: unsupported unsafe-to-represent values produce explicit incomplete evidence. Whole-response sentinel checks cover configuration defaults, field initializers and arbitrary annotations.

## Native ESS conformance topology

Source-verified recommendation from compose_conformance: retain one collection component, two explicit authored directories and two fixed targets. Pinned ESS 0.50.0 coverage input discovery reads immediate YAML files without recursion when that directory has no immediate manifest (ess input_discovery.rs:497); component filtering selects required commands/events/views, not scenario names (ess-conformance synthesize.rs:1894). A second component accepting Collect would not separate the scenarios.

Preserve existing three authored YAML files unchanged in ess-semantic/scenarios/collection/. Put the nine language cases directly in ess-semantic/scenarios/collection/languages/ and register all in root ess-inputs.yaml. Synthesize old cases with --path ess-semantic --scenarios ess-semantic/scenarios/collection --component collection --suite-format 5. Synthesize new cases with the same flags except --scenarios ess-semantic/scenarios/collection/languages.

The existing target uses Collector::with_bindings(BindingRegistry::default()) for every invocation, preserving explicit empty-registry configuration and the codegate-source-collection identity. BindingRegistry::default stays empty. The new target uses production Collector::default for every invocation with identity codegate-source-languages. Production Collector::default and top-level codegate::collect share registration of Go, Rust and Java. Neither test target switches registries based on a scenario ID, fixture name, input language or expected snapshot. Both invoke generated CollectBehavior and admit the exact actual response. Only fixture root URIs are translated; other inputs and expected snapshots are not rewritten.

Extend ComponentConformance with explicit scenarios path and lane identifier. Use the lane for suite filenames to avoid collection-suite.json collisions. Preserve old count four, environment and output directory. Add language lane count ten (nine authored plus generated structural Collect), separate test name, CODEGATE_LANGUAGE_COLLECTION_OUTPUT and private fresh output. Enforce exact IDs, suite bytes/hash, implementation identity, counts, completion interval and language-specific seeded failure checks. Preserve evaluator27 and foundation8: total49 native executions, with the structural Collect witness deliberately executed under both registry configurations; do not call these49 distinct semantic capabilities. This topology is source-verified, not yet execution-verified.

## Required fixture witnesses

Go: semantic-go-source-baseline covers local go.mod, two selected packages, grouped/aliased/repeated imports, external import gaps, package/function/method/type/interface declarations, receiver signatures, Unicode and comment/string decoys. semantic-go-source-syntax-error preserves recoverable declarations and located SyntaxError with affected partial coverage. semantic-go-source-build-constraints exercises real directive positions, legacy directives, filename constraints and changed selected configuration without invented inclusion. Anonymous callables are explicit bounded gaps.

Rust: semantic-rust-source-baseline uses explicit target roots and inline modules with structs/traits/impl methods and grouped aliased imports; add conventional/out-of-line ambiguity and unassigned-source witnesses. Assert deterministic IDs, lexical parents and Unicode byte locations through actual admission. semantic-rust-source-syntax-error retains valid surrounding syntax without fabricated repaired declarations. semantic-rust-source-cfg-macro covers features/configuration identities, unexpanded macros, unresolved external imports and no tool invocation. Portable structure metrics are deferred, not silently asserted complete.

Java: semantic-java-source-baseline covers module-qualified packages, nested classes, interfaces, fields, constructors, overloads, static/wildcard imports and Java17/21 syntax. The independent identifier witness Café in class Café starts at byte6 and ends at byte11. semantic-java-source-syntax-error combines recoverable sources, generated classification/exclusion and absent classpath evidence. semantic-java-source-quarkus-declarations covers declared CDI qualifiers/nonbinding/producers/injection, REST path components, transactions and configuration keys; sentinel configuration defaults and arbitrary annotation/initializer literals must be absent from serialized responses. No effective injection/deployed-route claims.

## Dispatch, verification and completion

Before dispatch, write each complete unit brief to its ignored scratch directory and pass its path. Each brief freezes the exact base, owned paths, build/lease/cleanup boundaries, Rust-only rule, supplied constructor and fixture expectations. Confirm inferred mappings before code changes. Obtain a red-capable baseline, implement the unit, run package-scoped tests/fmt/Clippy and retain raw evidence. Coordinator alone registers module exports/ESS scenarios and performs shared changes. Each independently green unit receives an independent aep:adversary tests-only pass, at most two full attacks, with every finding and disposition retained through AEP. Never weaken expectations to fit produced output.

Close stories only on actual production-path named conformance and full integrated task check. Before merge compare each claim on base and treatment, and record VERIFIED/NOT VERIFIED/INCONCLUSIVE with both outputs. Gate expected family counts and native identities are evidence, not a substitute for inspecting coverage. Serialize large builds when measured storage requires it. Publish wanted commits through bot PR, archive retained logs and clean exact managed worktrees/branches. No new release is included. Agent token/tool counters are not exposed by this host; do not invent them. Three bounded read-only scoping passes completed; no implementation gate, native language result or cost estimate is claimed.

## Final scheduling clarification

The round comprises three language stories, but the final scope analysis serializes those stories because their production acceptance shares registration and conformance files. The next implementation wave is therefore Go plus its coordinator-owned native acceptance, not three concurrently written language stories. Its first lane has three new authored cases plus the structural Collect case; following Java and Rust stories add their own three cases, reaching the recorded target of ten language-lane executions only after all nine cases exist. Never pin ten while only one language has shipped, and never create pending cases that report green. Collector defaults register only implemented bindings at each intermediate delivery; missing-language gaps remain explicit. Replan and update actual scopes/counts after each story.

This is a scope/readiness refinement of existing accepted stories, not a new decomposition or a claim of implementation. No new epic/story or domain entity was introduced, so the earlier four-critic decomposition verdicts remain historical; three independent read-only scopers supplied this bounded review. The planned language mappings and lane topology are recorded before workers fork. Every implementation dispatch still requires a written brief and the ordinary adversary/integration gate.
