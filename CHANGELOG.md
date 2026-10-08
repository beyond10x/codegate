# Changelog

## 0.3.1

- Generate behavior and wire contracts with ESS 0.56.0 (was 0.50.0); the gate
  and CI now require ESS 0.56.0. Exact JSON numbers stay enabled through the
  generated `exact-numbers` default feature. The test-only `ess-conformance`
  and `ess-primitives` runtime crates move to the ESS 0.56.0 tag.
- No change to CLI or library behavior; the 27 evaluator, eight
  source-foundation and four collection conformance cases still execute.

## 0.3.0

- Expose the public Rust `Collect` handler, composing source capture, binding
  registration and rich-fact admission.
- Isolate language binding slots and report missing bindings and semantic tools
  as explicit capability gaps.
- Add four native collection conformance cases and adversarial boundary checks;
  retain all 27 evaluator and eight source-foundation cases.

The CLI still evaluates supplied dependency JSON. Language declaration extraction,
semantic navigation, collection CLI commands and repository HTML reports remain
planned work. The default collector currently has no registered language bindings.

## 0.1.0

First public Codegate release.

- Evaluate normalized, language-neutral dependency facts with a Rust CLI.
- Compute unique destination fan-out and trace forbidden dependency edges.
- Distinguish complete, partial, unsupported and failed fact coverage.
- Validate JSON structure, graph references and source/configuration assertions.
- Generate behavior and wire contracts from ESS and execute all 27 selected scenarios.
- Publish a standalone documentation site, signed common Gates evidence and a
  Linux x86_64 executable archive with checksums.

The dependency JSON formats are experimental `/0.1`. This release consumes supplied
facts; Rust and Go source collectors are not implemented yet. It does not verify
source bytes or compute source-content hashes.
