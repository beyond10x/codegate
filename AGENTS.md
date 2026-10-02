# Codegate

Codegate is a language-neutral fact IR and quality-analysis system. Language bindings
produce facts; shared analyses and checkers consume validated IR. Rust is the first
binding, not a dependency of the semantic core. This repository currently contains a
design and a draft ESS dependency contract; no implementation or executed conformance
result is claimed.

## Engineering rules

- Use managed worktrees. Keep the primary checkout clean.
- All committed executable implementation, CLIs, importers, checkers and gates are
  Rust. Use clap derive for CLI definitions. Do not add executable Python or Go code.
- Use `aep plan artifact` for all planning-store mutations. The governed design is
  `.engineering/planning/architecture-design/language-neutral-fact-ir.md`.
- Specify the domain in ESS before writing implementation epics or stories. Generate
  supported contracts from ESS; acceptance uses named conformance scenarios.
- Shared analyses and checkers must not import language bindings, parse language
  syntax, inspect source files, or switch behavior on a language identifier.
- Binding capability claims never replace per-snapshot coverage evidence. Missing or
  incomplete required facts must not become a passing check or a measured zero.
- Beyond10x commits and publication use the organization bot and the applicable
  workspace delivery route. This initial local design has no GitHub publication.

## Validation at this stage

Run `aep plan artifact validate` and `ess specify validate --path ess`. Compile the
authored and combined suites with `ess verify conform author` and `synthesize`, using
`--path ess --scenarios ess --out <scratch-file>`. This is contract validation, not
executed conformance. Generated-contract drift checking, actual conformance,
formatting, linting and Rust tests join `task check` in the first implementation.
Do not create placeholder green checks for absent implementation.

The first wave uses ESS 0.50.0 and Rust 1.98.1, isolated build directories, two Cargo
jobs per build and the installed compiler cache. Recheck at least 20 GiB free disk
before implementation. Unit workers read `ess/` as the contract and return semantic
disagreements to the coordinator instead of changing expected scenario responses.
