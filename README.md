# Codegate

Codegate is a proposed Rust implementation of a language-neutral code-quality system.
Language bindings produce typed facts about code and execution evidence. Shared
analyses derive metrics and shared checkers evaluate policy against that IR.

```text
source + tool evidence
         |
    language binding
         |
    validated fact IR
         |
 shared analyses + policy checks
         |
 metrics + findings + verdicts
```

Rust is the first binding. Other languages use the same fact contract and checkers;
each binding declares the facts it supports and each snapshot records what was
actually observed. Unknown or missing facts never silently become zero violations.

This repository is at the design stage. The authoritative records are:

- [Product intent](.engineering/planning/vision/language-neutral-code-quality.md)
- [First architecture design](.engineering/planning/architecture-design/language-neutral-fact-ir.md)

The next design step is a validated ESS domain and named conformance scenarios,
followed by AEP implementation stories. No executable implementation exists yet.

Validate the planning store with `aep plan artifact validate`.
