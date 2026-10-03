---
format: aep.planning-md/3
id: story:source-java-baseline
kind: story
status: draft
title: Extract the Java and declared Quarkus source-only baseline
relations:
- decomposes: epic:source-collection-baseline
- depends_on: story:source-snapshot
- depends_on: story:capability-admission
- informed_by: executable-system-specification:semantic-analysis
- depends_on: story:collection-composition-seam
scope:
- confidence: inferred
  path: .github/workflows/ci.yml
- confidence: inferred
  path: AGENTS.md
- confidence: inferred
  path: README.md
- confidence: inferred
  path: ess-semantic/ess-inputs.yaml
- confidence: inferred
  path: ess-semantic/scenarios/collection/languages/semantic-java-source-*.yaml
- confidence: inferred
  path: generated/semantic-behavior
- confidence: inferred
  path: generated/semantic-wire
- confidence: inferred
  path: specifications/semantic-contract.md
- confidence: inferred
  path: specifications/semantic-scenarios.md
- confidence: inferred
  path: src/bin/codegate-check.rs
- confidence: cited
  path: src/bindings/java.rs
- confidence: inferred
  path: src/bindings/mod.rs
- confidence: inferred
  path: src/collection/compose.rs
- confidence: inferred
  path: src/lib.rs
- confidence: inferred
  path: tests/collection_adversary.rs
- confidence: inferred
  path: tests/collection_composition.rs
- confidence: inferred
  path: tests/collection_conformance.rs
- confidence: cited
  path: tests/fixtures/structure/java/**
- confidence: inferred
  path: tests/freshness.rs
- confidence: inferred
  path: tests/gate_adversary.rs
- confidence: cited
  path: tests/source_java.rs
- confidence: inferred
  path: tests/source_language_conformance.rs
- confidence: inferred
  path: tests/support/collection_conformance.rs
revision: 9
---
## Intent and contract

Implement the Java source-only binding over exact collected bytes using Tree-sitter, including declared Quarkus annotations. Existing typed home: ess-semantic/domains/semantic.yaml (Declaration, Occurrence, Dependency, DecisionPoint, StructureObservation, FrameworkFact, Evidence and Coverage); normative rules: specifications/semantic-contract.md. No Maven, Gradle, javac, JDT LS or augmentation process.

## Acceptance

The semantic-java-source-baseline authored ESS fixture returns expected packages, classes, interfaces, fields, constructors and overloaded method declarations plus import and annotation occurrences with exact spans, while syntax errors, wildcard ambiguity, classpath and generated-source gaps cannot appear complete.

## Named scenarios

semantic-java-source-baseline, semantic-java-source-syntax-error, semantic-java-source-quarkus-declarations. Include static/wildcard imports, nested classes, overload signatures, Unicode, Java 17/21 syntax fixtures and declared CDI/REST/transaction/configuration annotations. Effective injection and deployed route claims are forbidden without build evidence; annotation values must not leak configuration secrets.

## Scope

Derived 2026-10-03 by read-only aep:story-scoper against 821de3626e019c5c8203ea7f8f026b981c7a0d09; corrected by coordinator after the typed design-scope refusal.

- cited: src/bindings/java.rs, tests/source_java.rs, tests/fixtures/structure/java/** are the explicitly assigned unit-owned implementation, test and fixture surfaces. Their existence as future files is not an implementation claim.
- cited: SourceBinding, BindingFacts, retained CollectedSource and shared parser/ID/range/evidence helpers already supply the language extraction seam. The worker consumes them without source IO or external tools.
- inferred: this story also owns the shared coordinator integration paths recorded in its typed scope and design:source-baseline-wave: registration, collection defaults, native target/gate, ESS scenario registration, generated projections when changed and semantic/user documentation. Coordinator ownership does not erase these paths from scheduling.
- inferred: would collide with other language stories at these shared surfaces. The final computed wave output serializes the complete language stories; only separately assigned files within one story may be worked in parallel.
- cited: confidence high for the language unit paths and interface from the story and code; confidence inferred for the full shared edit set until the implementation diff establishes it. Earlier narrow scope was corrected visibly rather than treated as safe concurrency.
- cited: safety fact, level 2 and unproven by execution: src/collection/compose.rs:182 enforces slot ownership and src/semantic/admit.rs validates composed identities, lexical containment and completeness. Read-only scope review ran no builds or tests. All authored running code is Rust; Go and Java are fixture data.

## Dependencies and completion

Depends on story:source-snapshot and story:capability-admission. Unit tests and real authored ESS execution, followed by task check preserving 27 legacy cases. Static Maven/Gradle selection evidence does not establish an effective classpath.

## Production conformance prerequisite


Depends additionally on story:collection-composition-seam. Its real public Collect Rust handler, shared registration and native ESS runner exist before this language unit forks. Coordinator registers this story's authored scenarios through that production path; the language story closes on those actual results, without waiting for downstream first-slice CLI work. Existing Foundation CollectSource returns only SourceSnapshot and cannot substitute for this FactSnapshot witness.
