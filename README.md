# Codegate

[Documentation](https://beyond10x.github.io/codegate/) ·
[Releases](https://github.com/beyond10x/codegate/releases)

Codegate evaluates language-neutral dependency facts against shared quality rules.
The CLI reads normalized JSON offline. The Rust library also collects selected
Rust, Go and Java source bytes and build configuration, and validates rich imported
facts. Declaration and dependency extraction remain planned work.

The [Go/Rust/Java semantic-parity program](docs/parity-baseline.md) now has a
[separate ESS foundation](specifications/semantic-contract.md), generated Rust
contracts and a reviewed AEP backlog. Source collection is a library foundation;
public collection commands, semantic adapters, navigation and broader assessment
remain implementation work.

The implementation is Rust. Its domain and examples live in [ESS](ess/README.md);
behavior and wire models are generated from that contract. The broader design and
backlog live in `.engineering/planning/` and are managed with `aep plan artifact`.

## Evaluate

```console
cargo run --locked --bin codegate -- evaluate --facts facts.json --policy policy.json
cargo run --locked --bin codegate -- evaluate --facts facts.json --policy policy.json --out report.json
```

The facts document identifies the source/configuration, producer, units, dependency
edges and evidence coverage. The policy binds the expected source/configuration,
selects a dependency kind and names forbidden source/target tuples. See the `input`
of [one-runtime-edge](ess/scenarios/one-runtime-edge.yaml) for both documents, and
[forbidden-runtime-edge](ess/scenarios/forbidden-runtime-edge.yaml) for a policy failure.

Reports contain the applied policy, coverage, unique destination fan-out per unit,
traceable forbidden-edge findings and diagnostics. Complete passing evidence exits
0; a complete policy failure exits 1; incomplete, unsupported, failed or invalid
input exits 2. A witnessed violation with partial coverage stays a Fail but exits 2.
Missing evidence never becomes a measured zero. Structural JSON refusal emits a
stderr diagnostic and no report. Each document is limited to 4 MiB; snapshots to
10,000 units and 50,000 edges.

The `/0.1` JSON formats are experimental. Source/configuration IDs are compared as
producer assertions; Codegate does not claim to verify source bytes or provenance.
Facts marked Go, Rust or arbitrary labels take the same evaluation path. The Go
example demonstrates shared-checker independence, not an implemented Go binding.

## Verify

The source-only library entry point is `collection::collect_source`. It retains
selected content, derives snapshot/configuration identities, classifies physical
lines with Tree-sitter and reads static Go, Cargo, Maven and Gradle declarations.
It executes no build tools or language servers. Dynamic or incomplete build
selection produces explicit gaps. `semantic::validate_snapshot` checks imported
facts offline; accepting a partial snapshot does not make its evidence complete.
See the [collection counting and selection contract](specifications/source-baseline.md).

Install Rust 1.98.1, ESS 0.50.0, AEP and Task, then run:

```console
task check
```

The Rust gate checks the planning store and both ESS roots, regenerates all four contract crates,
normalizes generated Rust with the pinned rustfmt, checks byte drift, runs the real
Rust ESS target across all 27 evaluator scenarios and eight source-foundation
scenarios (six authored behavioral cases and two structural cases), runs boundary/CLI regressions,
and checks formatting and Clippy including generated crates. Every step prints its
own exit status. No built-in ESS reference implementation substitutes for Codegate.

`target/conformance/suite.json` and `report.json` pair the exact admitted suite with
its native count report; `run.json` records scenario results. Synthesis explicitly
uses `--suite-format 5` so the current direct-response contract produces coverage
inventory in `ess-conformance/29`. A green gate requires 27 passed, zero other
outcomes, complete selection and matching regenerated suite bytes. This establishes
the declared suite, not exhaustive correctness of every possible graph.

The ESS generators expose two Rust representations. The explicit bridge in
`src/wire.rs` maps generated names/enums/records and checks fan-out integrality and
bounds. ESS wire projection uses omitted properties for `Optional`; the offline
format also accepts null for edge target absence and emits null for unknown metrics.
Only these declared optional fields are normalized. Generated source remains owned
by the regeneration pipeline; change ESS inputs rather than hand-editing it.

The generated `EvaluateBehavior` seam returns an outcome only. The local adapter
retains that invocation's generated typed response, which the CLI and conformance
target consume. The underlying evaluator is a pure function; admission constructs
an inaccessible validated graph before analysis/checking.

ESS 0.50.0 emits an equivalent manual `Default` implementation for `EssPresence<T>`.
The generated wire crates' Clippy lanes allow `clippy::derivable_impls`.
The semantic behavior crate also needs scoped allowances for generated unreachable
event conversion and unit-variant patterns; see `AGENTS.md`. Root warnings remain
denied, and generated byte drift is strict.

The gate admits evidence only from its dedicated conformance invocation under
`target/codegate-check-<pid>/conformance/`. A missing, ignored or unselected producer
cannot reuse `target/conformance/report.json`. It verifies the exact suite bytes and
SHA-256 digest, model/implementation identities, full counts and selection, and an
observed completion timestamp within that invocation. Standalone tests still retain
their most recent report under `target/conformance/`. Timing lives in the harness;
the evaluator reads no clock. ESS `.ess-output/` ownership state describes its local
output directory and is ignored; generated product files remain strictly compared.

The source-foundation report is retained beside that evaluator report under
`foundation-conformance/`, with the same freshness checks. Its target invokes the
real collection and admission functions. Seeded empty-response, admit-everything
and constant-identity faults must fail the authored expectations. These eight cases
cover the two internal library obligations. A separate `collection-conformance/`
report covers the public Collect Rust handler: three authored cases and one
structural case. It returns exact captured source identities and explicit coverage
gaps; an unavailable semantic adapter never becomes source-only success. The
default binding registry is empty until the language bindings land. The five other
public semantic handlers, semantic CLI commands, source-language extraction and
full three-language parity remain delivery gaps.
