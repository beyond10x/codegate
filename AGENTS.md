# Codegate

Codegate is a language-neutral fact IR and quality-analysis system. Language bindings
produce facts; shared analyses and checkers consume validated IR. The first offline
dependency evaluator, source collection and rich-fact admission are implemented.
Declaration/dependency language bindings remain implementation work.

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
under `target/codegate-check-<pid>/conformance/`. All 27 evaluator scenarios must
execute and pass. Eight source-foundation cases run separately with fresh evidence
under the same invocation's `foundation-conformance/` directory.

`ess-semantic/` is the separately versioned semantic-parity foundation. Its generated
`generated/semantic-behavior` and `generated/semantic-wire` crates also participate in
drift, compilation, formatting and Clippy checks. The six public semantic behavior
traits remain runtime obligations. The two internal source-foundation traits have
six authored behavioral cases plus two structural cases; the latter alone cannot
establish real semantic conformance. Never report semantic parity from either
foundation or legacy results. The program
and precise remaining work are in `epic:semantic-parity` and `docs/parity-baseline.md`.

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

The internal source-foundation component publishes no events. ESS 0.50.0 emits an
unreachable push converting its uninhabited event enum; generated/semantic-behavior
alone allows `unreachable_code` and `clippy::unneeded_struct_pattern` during Clippy
(the generated component also matches unit outcomes with `{ .. }`). Root warnings remain denied and
byte-for-byte regeneration remains required. Do not invent events to silence it.

## Public delivery

Codegate is enrolled in common Gates. Use coordinated hooks, `b10x-gates bot`,
signed `check` receipts and `publish`; private policy and signing keys remain outside
this repository. Inspect all new history, not only the final tree. Published history
redacts personal filesystem paths in historical diagnostic logs; original local
evidence is retained in recovery archives. Product behavior and evidence counts are
unchanged by that publication redaction.

`website/` is the public documentation source. `codegate-docs` is its Rust builder;
`task site-build` emits a static site plus exact source provenance. `pages.yml` builds
without credentials and `b10x-docs-site.yml` calls the pinned shared project-site
publisher for `/codegate/`. Never add App credentials to this repository. Verify
both live provenance documents before reporting documentation published.

Versions use bare annotated tags matching Cargo metadata, starting at `0.1.0`.
Run `task check`, publish signed common Gates evidence for the exact commit/tag,
and verify the required GitHub checks. `release-build.yml` builds the Linux x86_64
archive and SHA256SUMS without publication credentials. The operator's authorized
release uses the bot-authenticated Gates delivery route to create the GitHub Release
and upload those verified artifacts. Verify release author, exact tag and downloaded
checksums before reporting released. Do not publish a release merely because a tag
was pushed.

Do not commit ESS `.ess-output/` directory ownership state. It is local generator
management metadata, excluded only at each generated crate root. All generated
product/plan/schema/obligation bytes remain checked. The integration gate must run
the exact conformance test into a fresh private output directory and bind report
identity, complete counts, suite hash and observed completion interval. A preexisting
standalone report is never proof that the gate's producer actually ran.
