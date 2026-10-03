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
  path: src/bindings/java.rs
- confidence: inferred
  path: tests/fixtures/structure/java/**
- confidence: inferred
  path: tests/source_java.rs
revision: 5
---
## Intent and contract

Implement the Java source-only binding over exact collected bytes using Tree-sitter, including declared Quarkus annotations. Existing typed home: ess-semantic/domains/semantic.yaml (Declaration, Occurrence, Dependency, DecisionPoint, StructureObservation, FrameworkFact, Evidence and Coverage); normative rules: specifications/semantic-contract.md. No Maven, Gradle, javac, JDT LS or augmentation process.

## Acceptance

The semantic-java-source-baseline authored ESS fixture returns expected packages, classes, interfaces, fields, constructors and overloaded method declarations plus import and annotation occurrences with exact spans, while syntax errors, wildcard ambiguity, classpath and generated-source gaps cannot appear complete.

## Named scenarios

semantic-java-source-baseline, semantic-java-source-syntax-error, semantic-java-source-quarkus-declarations. Include static/wildcard imports, nested classes, overload signatures, Unicode, Java 17/21 syntax fixtures and declared CDI/REST/transaction/configuration annotations. Effective injection and deployed route claims are forbidden without build evidence; annotation values must not leak configuration secrets.

## Scope

Inferred: src/bindings/java.rs, tests/source_java.rs, tests/fixtures/structure/java/**. Coordinator owns shared module registration, manifests, public API, ESS scenario registration and conformance runner. This story supplies inputs/expected observations and Rust tests; Java files are fixture data only; no AEP edits.

## Dependencies and completion

Depends on story:source-snapshot and story:capability-admission. Unit tests and real authored ESS execution, followed by task check preserving 27 legacy cases. Static Maven/Gradle selection evidence does not establish an effective classpath.

## Production conformance prerequisite


Depends additionally on story:collection-composition-seam. Its real public Collect Rust handler, shared registration and native ESS runner exist before this language unit forks. Coordinator registers this story's authored scenarios through that production path; the language story closes on those actual results, without waiting for downstream first-slice CLI work. Existing Foundation CollectSource returns only SourceSnapshot and cannot substitute for this FactSnapshot witness.
