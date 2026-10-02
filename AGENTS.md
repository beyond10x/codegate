# Codegate

Codegate is a language-neutral fact IR and quality-analysis system. Language bindings
produce facts; shared analyses and checkers consume validated IR. The first offline
dependency evaluator is implemented. No source-language bindings ship yet.

## Engineering rules

- Use managed worktrees. Keep the primary checkout clean.
- All committed executable implementation, CLIs, importers, checkers and gates are
  Rust. Use clap derive for CLI definitions. No executable Python or Go code.
- Use `aep plan artifact` for all planning-store mutations. The governed design is
  `.engineering/planning/architecture-design/language-neutral-fact-ir.md`.
- Specify the domain in ESS before writing implementation epics or stories.
  Generate supported contracts; acceptance uses named conformance scenarios.
- Shared analyses/checkers receive the private admitted wrapper; they must not
  import bindings, parse source syntax, access IO or dispatch on language/producer.
- Missing or incomplete required facts never become passing checks or measured zero.
- Bot commits/publication follow the organization workspace delivery route. No
  GitHub publication is implied by implementing a story.

## Validation

Run `task check`. This calls the Rust `codegate-check` binary, which validates AEP
and ESS, checks generated drift, runs actual conformance and Rust tests, and checks
formatting and Clippy. It never invokes itself recursively. Standalone ESS reports
are under `target/conformance/`; the gate retains its private invocation evidence
under `target/codegate-check-<pid>/conformance/`. All 27 scenarios must execute and pass.

Pin ESS 0.50.0 and Rust 1.98.1. Generated source in `generated/behavior` and
`generated/wire` is produced by ESS and deterministically formatted by pinned
rustfmt. Never hand-edit it. Reproduce the commands in `src/bin/codegate-check.rs`
when deliberately updating projections. Keep synthesis plans and runtime-obligation
reports. Dependency resolution for the generated wire crate is constrained by its
exact versions; root `Cargo.lock` is committed.

Unit workers use their own target directory, two Cargo jobs and installed sccache
(`RUSTC_WRAPPER=/usr/bin/sccache CARGO_BUILD_JOBS=2`). Recheck at least 20 GiB free
disk before new large builds. Unit workers run package tests/format/Clippy; the
coordinator runs the complete integration gate once. Do not change expected ESS
scenario responses to make an implementation pass.

Known upstream style exception: the generated wire crate's Clippy invocation adds
`-A clippy::derivable_impls` after `-D warnings`, because ESS 0.50.0 generates the
manual `EssPresence<T>` Default implementation. No root/behavior exception applies;
never hand-edit generated source to satisfy that style lint.

Do not commit ESS `.ess-output/` directory ownership state. It is local generator
management metadata, excluded only at each generated crate root. All generated
product/plan/schema/obligation bytes remain checked. The integration gate must run
the exact conformance test into a fresh private output directory and bind report
identity, complete counts, suite hash and observed completion interval. A preexisting
standalone report is never proof that the gate's producer actually ran.
