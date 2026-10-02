# Changelog

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
