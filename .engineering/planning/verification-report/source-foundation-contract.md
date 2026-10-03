---
format: aep.planning-md/3
id: verification-report:source-foundation-contract
kind: verification-report
status: draft
title: Scoped internal source foundation contract verification
relations:
- reviews: executable-system-specification:semantic-analysis
revision: 1
---
# Foundation contract seam

Worker tree: wt-441dc9e84c18. Base: 91dc33be. No commits or AEP mutations.
All paths below are repository-relative. The coordinator owns integration.

## Delivered

- Added internal `CollectSource` and `ValidateSnapshot` commands in
  `ess-semantic/domains/semantic.yaml`, reusing existing value types.
- Added `ess-semantic/components.yaml`; `source-foundation` accepts only those
  commands. It claims no persistent domain ownership or network surface.
- Updated the specification manifest and regenerated semantic behavior/wire
  outputs using pinned ESS 0.50.0 and pinned rustfmt, without hand edits.
- Clarified the internal commands' real result semantics and limited conformance
  claim in `specifications/semantic-contract.md` and `source-baseline.md`.
- Preserved six public command shapes and all existing value fields. Wire schema
  definitions are unchanged; projection provenance changes with the model.

## Verification observed

`ess specify validate --path ess-semantic`:

```text
codegate_semantic v1 — 3 file(s), valid
```

Both generated crates passed `cargo fmt --manifest-path <manifest> --check`.
Freshly projected, formatted files equal their destination trees byte-for-byte,
excluding local `.ess-output` ownership metadata. `git diff --check` passed.

Behavior synthesis: 89 capabilities, 81 generated, 8 obligations, 0 refused.
Types projection: 72 model types. The eight obligations are six public commands
plus the two internal library commands; none are claimed implemented here.

Component suite synthesis succeeded with 2 generated scenarios, 0 authored
sources, 0 refusal occurrences and 6 public outcomes explicitly outside as
OtherComponent. The suite's declared selection is component source-foundation;
this is structural synthesis only, not an executed conformance report.

No runtime tests or full task check were run; the coordinator owns integration.

Model digest: 6429b77034506255e96ca6abd75079983e5e0ca68ef4a30f19a0baf8875612dd.
Behavior contract digest: 53b2681048db3937988b0fb64c9955287e4a7585230662d8b6db2d63142af9a9.

## Commands for coordinator

Use the pinned ESS 0.50.0 executable for these gate-equivalent commands:

```console
ess specify validate --path ess-semantic
ess generate synthesize --path ess-semantic --target rust --layout crate --out <fresh-behavior-dir>
ess generate types --path ess-semantic --target rust --all-types --package codegate-semantic-contract --out <fresh-wire-dir>
cargo fmt --manifest-path <fresh-behavior-dir>/Cargo.toml
cargo fmt --manifest-path <fresh-wire-dir>/Cargo.toml
ess verify conform synthesize --path ess-semantic --component source-foundation --suite-format 5 --out <fresh-generated-suite.json>
```

For real authored foundation execution, once the authored files are explicitly
listed in `ess-semantic/ess-inputs.yaml`, add `--scenarios ess-semantic` to the
component suite synthesis command. Do not run that option while scenarios is
empty: empty authored selections are refused.

Update the gate's stale "six runtime obligations" message to distinguish six
public obligations from the two internal ones. Include the three new generated
source files (`src/ports.rs`, `src/ports/source_foundation.rs`, `src/system.rs`)
when transporting this worker's changes. Existing generator drift traversal
already includes them automatically.

## Executable witness shape for coordinator

No authored scenarios were added: runtime helpers and their filesystem fixture
provider are not present in this worker tree, so expected runtime responses
cannot be justified here without duplicating the coordinator's fixture setup.
Do not substitute dummy expected responses or outcome-only cases.

Implement generated `CollectSourceBehavior` and `ValidateSnapshotBehavior` over
the actual helpers. The generated `ports::source_foundation::SourceFoundation`
port requires exactly those two traits. Its outcomes are `SourceObserved` and
`ValidationObserved`, carrying generated typed response fields.

The native Rust ConformanceTarget forwards command inputs to these handlers and
serializes their actual responses. `CollectSource` returns `source` (optional)
and `gaps`; `ValidateSnapshot` returns `accepted` and `gaps`. Successful valid
partial admission returns accepted=true and no admission diagnostics, without
changing the snapshot's existing partial evidence. Refusals return nonempty
actual diagnostics, not unsupported runner sentinels.

Author these six ESS cases against real fixtures:

- parity-selected-content-identity: collected metadata and identities for exact
  same bytes and changed selected dirty/untracked bytes, with expected identities
  established independently of the implementation under test.
- parity-manifest-identity: actual static manifest/configuration inputs and
  changed configuration identities, preserving deterministic ordering.
- parity-path-confinement: refused escaping/symlink selection, no source, actual
  nonempty refusal gap codes.
- parity-stale-evidence: a valid admitted snapshot and a deliberately mismatched
  evidence identity; assert accepted=false and its precise stable gap code.
- parity-dangling-observation: real admission of an invalid endpoint/span/owner;
  assert actual refusal diagnostics rather than outcome alone.
- parity-coverage-not-zero: valid complete and partial snapshots plus missing or
  contradictory family coverage, preserving admission/completeness distinction.

Keep parity-core-boundary as a Rust architecture gate with actual negative
dependency/IO/language-dispatch probes. It is not an ESS command case. With six
authored cases plus two generated cases the combined selected count is expected
to be eight, but obtain the authoritative count from the synthesized suite and
native report. Retain the outside inventory of six public outcomes unchanged.

Derive the report from native `Runner::run_admitted` and `CountReport::from_run`,
as in `tests/conformance.rs`, and keep exact suite bytes plus scenario records.
An empty target must fail the output assertions; stale-evidence acceptance and
constant-identity mutations must turn named cases red. No helper story closure or
whole semantic ESS conformance is claimed by this specification-only seam.
