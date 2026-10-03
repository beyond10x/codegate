# First source collection round

This document selects the first implementation profile of the unimplemented semantic v1 contract. It adds two internal library command contracts using existing value types, without changing the public wire contracts. Its typed home is `ess-semantic/domains/semantic.yaml`; `semantic-contract.md` remains normative. Capabilities outside this profile remain delivery gaps.

## Selection and collection

Include and exclude paths are normalized relative literal file/directory paths, not glob expressions. Empty include paths select the root; a directory includes descendants. Excludes win. Empty languages select Rust, Go and Java. Duplicate paths/languages are invalid. Do not traverse `.git`, `.hg`, `.svn`, `target`, `node_modules`, `build` or `.gradle` by default; explicitly selecting a skipped directory is permitted except repository metadata. Symlink inputs are refused, including symlink directories, rather than followed. Refuse non-UTF-8 selected source and any path that cannot be represented faithfully.

Bound one file to 4 MiB, the selected set to 10,000 files and total selected bytes to 64 MiB. Exceeding a bound refuses collection; never silently truncate a complete snapshot. A changed file during collection prevents completeness. Read exact bytes once into an internal collection buffer; downstream syntax extraction consumes that buffer, not a second filesystem read.

Read Git origin through a Rust library without spawning Git. Nonrepository files are Untracked. A Git repository that cannot be inspected produces a gap rather than falsely labelling everything Untracked. A file's staged or unstaged bytes differing from HEAD are TrackedDirty; origin does not enter content identity.

Default file classification recognizes Go `_test.go`, Java test source directories and `*Test.java`/`*Tests.java`, and Rust `tests/` files. Recognize conventional generated headers or declared generated source roots. Inline Rust `#[cfg(test)]` does not make a whole production file Test; its test observations belong to later structure extraction. Unknown effective test/generated classification is a gap when it affects an explicitly requested inventory.

Discover and hash Go modules/workspaces, Cargo manifests/lockfiles, Maven POM hierarchy and Gradle settings/build files in selected module roots. Parse only static module declarations. Preserve unresolved properties, computed Gradle includes, missing parents and missing selected modules as gaps; do not execute builds to fill them. Relevant configuration files and manifest bytes affect identity even when they are not source-language files. Caller-supplied configuration IDs are assertions when nonempty; derive the canonical ID and reject mismatch. Empty IDs request collection to derive them.

## Pure identity seam

Configuration files use opaque physical line classification: Blank for whitespace-only lines, Code otherwise. This does not claim syntax-aware comment or documentation coverage for build languages. BuildConfiguration files are excluded from source-language line metrics. Manifest module identities are normalized root-relative parent directory paths, with `.` for the root; parent_module uses the same convention. Declared package/module names remain binding metadata, distinct from module selection identity.

Canonical JSON omits absent optional fields, matching generated wire presence. A configuration file also appearing in source.files must have identical metadata; duplicates within either list are invalid. Offline EOF positions may use the final line's end column or `(lines.len(), 0)` at byte_length, because line metadata alone cannot distinguish a final newline. Collected positions use the actual bytes. Logical package/module parents may contain declarations in other files; same-file lexical containment is checked where ranges exist.

The coordinator provides `source_identity::configuration_id(&BuildSelection)` and `source_identity::snapshot_id(&SourceSnapshot)` over generated behavior types. They implement semantic-contract canonical JSON, with explicit generated-type projection and normalized set ordering. Collection and offline admission use this same pure module. It reads no source bytes, filesystem, Git state, environment or clock. Collection establishes content digests; imported offline evidence only verifies internal identity consistency, not authenticity of unavailable bytes.

## Coverage and admission

Coverage unit IDs denote exactly the listed units; an empty list denotes the empty population and never means all units. For a nonempty snapshot every selected unit must have one unambiguous coverage record for each FactFamily. A snapshot with no units retains one empty-scope record per family. Per-family records may combine disjoint units; duplicates and overlaps are refused. NotApplicable requires a reason. Explicit Unsupported/Partial/Failed with gaps is required wherever this baseline cannot supply a family; missing records never imply zero.

The frozen families are Sources, Declarations, Occurrences, Dependencies, References, Implementations, Calls, Decisions, Structure, Documentation, Effects, Tests, FrameworkDeclarations, FrameworkWiring and ExecutionCoverage. Source-only mode cannot claim semantic call/reference completeness or effective framework wiring. A supported capability describes what an operation can collect; it is distinct from observed coverage for a particular snapshot.

Offline admission verifies nonnegative bounded ranges, file/line partitions and line/column arithmetic from supplied metadata. It cannot establish actual newline placement or UTF-8 character boundaries without bytes. Collection establishes those against the retained exact bytes. Producer overlap is judged through fact identities and conflicting unit/family coverage, not an invented producer field on Coverage.

## Executable acceptance

Use the story's named `parity-*` IDs for foundation cases and the new `semantic-<language>-source-*` IDs for language cases. The older semantic-* manifest contains related detailed obligations, not duplicate executed cases. The coordinator maps these explicitly in the authored suite; no prose ID contributes to executed counts. Foundation admission exercises valid complete and partial snapshots plus stable refusals. Collection exercises actual filesystem fixtures, all three line classifiers, dirty/untracked identity, manifests and path confinement. Core-boundary verification includes `src/lib.rs`, all semantic algorithms and pure identity/bridge code, and contains negative dependency/IO/language-dispatch probes.

Public full Collect/Assess behavior is integrated by story:first-slice after the language bindings; foundation helpers are intermediate Rust APIs and are not reported as completed public generated command handlers. Missing semantic tools never silently downgrade a requested Semantic collection to SourceOnly.

The intermediate helpers have their own generated CollectSource and
ValidateSnapshot behavior traits, selected through the source-foundation
component in ess-semantic/components.yaml. Implementing these traits does not
satisfy any of the six public behavior obligations. CollectSource forwards the
actual collection result; ValidateSnapshot forwards the actual admission result.
The conformance target must never derive a response from expected fixture values
or scenario names.

The foundation ESS selection names parity-selected-content-identity,
parity-manifest-identity, parity-path-confinement, parity-stale-evidence,
parity-dangling-observation and parity-coverage-not-zero. Its authored cases assert
actual source identities/content metadata and actual admission decisions/gap codes.
Keep parity-core-boundary as a separate Rust architecture gate with negative
dependency/IO/language-dispatch probes; do not count it as an ESS command scenario.
The foundation completion witness combines that gate, the native foundation ESS
report and all 27 legacy evaluator cases. An empty or generated-only foundation
suite is not completion evidence. The semantic specification stays validated
until evidence supports the separately declared scope of conformance.
