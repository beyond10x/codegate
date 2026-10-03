# Semantic parity baseline

Reference: [fluxplane/codegate at 4d1515e](https://github.com/fluxplane/codegate/tree/4d1515ea925a0d4018ca59da2de3c31a91187f4e/).
Verified 2026-10-03 against the pinned GitHub archive and its PAX commit header.
Archive SHA256: `81add809411fbaa27671bec42111879972c505f61464a64b916c019c011c2ae4`.
This is an inventory of obligations, not a claim that bindings or parity have shipped.

## Status and counting

The catalogue contains 38 metrics (including two report metadata fields), 38 findings,
9 violations and four gates. Every catalogue row below is **gap** for all applicable
languages until an implementation and executable evidence replace that status.
N/A is restricted to the exact language-specific identity, with the reason shown;
it does not excuse a missing shared safety analysis. Mapping means proposed semantics,
not implemented support. Existing dependency /0.1 fan-out and forbidden edges cover
only imported dependency facts, not source collection or complete reference parity.

Each row names its implementation story slug under `epic:semantic-parity`. The
extension/noncatalogue tables are additional obligations; the 85-row catalogue alone
is not parity. No reference capability is excluded solely because it is unsupported.

## Exhaustive advertised catalogue

| Kind | Reference ID and citation | Go | Rust | Java | Owner story |
|---|---|---|---|---|---|
| metric | [`score_model`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L7) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | scoring-suggestions |
| metric | [`gates`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L8) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | scoring-suggestions |
| metric | [`debt_marker_count`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L9) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | portable-metrics |
| metric | [`debt_marker_counts`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L10) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | portable-metrics |
| metric | [`max_cyclomatic_complexity`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L11) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | portable-metrics |
| metric | [`max_nesting_depth`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L12) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | portable-metrics |
| metric | [`max_function_loc`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L13) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | portable-metrics |
| metric | [`function_count`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L14) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | portable-metrics |
| metric | [`large_function_count`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L15) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | portable-metrics |
| metric | [`high_complexity_function_count`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L16) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | portable-metrics |
| metric | [`package_loc`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L17) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | portable-metrics |
| metric | [`package_file_count`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L18) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | portable-metrics |
| metric | [`exported_symbol_count`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L19) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | portable-metrics |
| metric | [`exported_ratio`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L20) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | portable-metrics |
| metric | [`branch_density_per_kloc`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L21) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | portable-metrics |
| metric | [`generated_loc_percent`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L22) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | portable-metrics |
| metric | [`doc_coverage_percent`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L23) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | portable-metrics |
| metric | [`undocumented_export_count`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L24) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | portable-metrics |
| metric | [`test_file_count`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L25) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | testability-signals |
| metric | [`test_function_count`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L26) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | testability-signals |
| metric | [`test_to_code_ratio`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L27) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | testability-signals |
| metric | [`table_test_count`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L28) | Go.table-test-shape | Rust.parameterized-test-shape | Java.parameterized-test-shape | testability-signals |
| metric | [`flaky_test_smell_count`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L29) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | testability-signals |
| metric | [`weak_package_name_count`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L30) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | portable-metrics |
| metric | [`weak_identifier_count`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L31) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | portable-metrics |
| metric | [`ignored_error_count`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L32) | discarded call result, not proven error | discarded call result | discarded call result | safety-observations |
| metric | [`unchecked_type_assertion_count`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L33) | Go.unchecked-type-assertion | N/A: no Go assertion construct; downcast separate | N/A: no Go assertion construct; casts separate | safety-observations |
| metric | [`defer_in_loop_count`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L34) | Go.defer-in-loop | N/A: no Go defer construct | N/A: no Go defer construct | safety-observations |
| metric | [`process_exit_count`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L35) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | safety-observations |
| metric | [`string_concat_in_loop_count`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L36) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | safety-observations |
| metric | [`unsafe_usage_count`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L37) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | safety-observations |
| metric | [`weak_crypto_count`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L38) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | safety-observations |
| metric | [`dynamic_exec_count`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L39) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | safety-observations |
| metric | [`sql_concat_count`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L40) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | safety-observations |
| metric | [`path_risk_count`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L41) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | safety-observations |
| metric | [`reflect_usage_count`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L42) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | safety-observations |
| metric | [`missing_capacity_count`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L43) | Go.bounded-append-capacity | Rust.collection-capacity | Java.collection-capacity | safety-observations |
| metric | [`large_range_copy_count`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L44) | Go.range-value-copy | Rust.by-value-loop-copy (own identity) | N/A: enhanced-for copies references for objects | safety-observations |
| finding | [`coverage_no_go_files`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L47) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | testability-signals |
| finding | [`architecture_high_fan_out`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L48) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | architecture-policies |
| finding | [`architecture_internal_import`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L49) | Go.internal-boundary | Rust.visibility-boundary (own identity) | Java.package-module-boundary (own identity) | architecture-policies |
| finding | [`architecture_fan_out`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L50) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | architecture-policies |
| finding | [`architecture_<effect_name>`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L51) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | architecture-policies |
| finding | [`maintainability_high_pressure_unit`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L52) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | portable-metrics |
| finding | [`maintainability_debt_marker`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L53) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | portable-metrics |
| finding | [`quality_high_complexity_function`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L54) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | portable-metrics |
| finding | [`quality_deeply_nested_function`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L55) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | portable-metrics |
| finding | [`quality_large_function`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L56) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | portable-metrics |
| finding | [`quality_many_parameters`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L57) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | portable-metrics |
| finding | [`quality_many_returns`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L58) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | portable-metrics |
| finding | [`quality_large_file`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L59) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | portable-metrics |
| finding | [`quality_large_struct`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L60) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | portable-metrics |
| finding | [`quality_broad_interface`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L61) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | portable-metrics |
| finding | [`quality_low_doc_coverage`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L62) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | portable-metrics |
| finding | [`quality_undocumented_export`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L63) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | portable-metrics |
| finding | [`quality_high_branch_density`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L64) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | portable-metrics |
| finding | [`quality_high_generated_ratio`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L65) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | portable-metrics |
| finding | [`quality_weak_package_name`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L66) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | portable-metrics |
| finding | [`quality_weak_identifier_name`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L67) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | portable-metrics |
| finding | [`coverage_flaky_test_smell`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L68) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | testability-signals |
| finding | [`coverage_no_go_tests`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L69) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | testability-signals |
| finding | [`coverage_low_test_to_code_ratio`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L70) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | testability-signals |
| finding | [`safety_incomplete_validation`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L71) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | safety-observations |
| finding | [`safety_ignored_error`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L72) | discarded call result, not proven error | discarded call result | discarded call result | safety-observations |
| finding | [`safety_unchecked_type_assertion`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L73) | Go.unchecked-type-assertion | N/A: no Go assertion construct; downcast separate | N/A: no Go assertion construct; casts separate | safety-observations |
| finding | [`safety_defer_in_loop`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L74) | Go.defer-in-loop | N/A: no Go defer construct | N/A: no Go defer construct | safety-observations |
| finding | [`safety_process_exit`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L75) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | safety-observations |
| finding | [`performance_string_concat_in_loop`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L76) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | safety-observations |
| finding | [`security_unsafe_usage`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L77) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | safety-observations |
| finding | [`security_weak_crypto`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L78) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | safety-observations |
| finding | [`security_dynamic_exec`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L79) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | safety-observations |
| finding | [`security_sql_concat`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L80) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | safety-observations |
| finding | [`security_dynamic_file_path`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L81) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | safety-observations |
| finding | [`performance_reflect_usage`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L82) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | safety-observations |
| finding | [`performance_missing_capacity`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L83) | Go.bounded-append-capacity | Rust.collection-capacity | Java.collection-capacity | safety-observations |
| finding | [`performance_large_range_copy`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L84) | Go.range-value-copy | Rust.by-value-loop-copy (own identity) | N/A: enhanced-for copies references for objects | safety-observations |
| violation | [`architecture_boundary_violation`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L87) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | architecture-policies |
| violation | [`architecture_test_boundary_violation`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L88) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | architecture-policies |
| violation | [`architecture_denied_import`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L89) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | architecture-policies |
| violation | [`architecture_test_boundary_import`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L90) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | architecture-policies |
| violation | [`architecture_unknown_package`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L91) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | architecture-policies |
| violation | [`architecture_effect_import`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L92) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | architecture-policies |
| violation | [`architecture_effect_call`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L93) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | architecture-policies |
| violation | [`architecture_<effect_name>`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L94) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | architecture-policies |
| violation | [`safety_validation_diagnostic`](https://github.com/fluxplane/codegate/blob/4d1515ea925a0d4018ca59da2de3c31a91187f4e/internal/lang/goast/support.go#L95) | shared / Go mapping | shared / Rust mapping | shared / Java mapping | safety-observations |

## Implementations and reference tests

Paths below are relative to the pinned reference; line citations can be resolved with
the same commit permalink used above. Test citations establish reference behavior,
not successful execution by this session.

| Obligation | Source | Tests | Shared mapping / owner (all pending) |
|---|---|---|---|
| File selection; tests/generated/vendor/build constraints | files.go:17; index.go:91 | engine_test.go:828,1361; editor_test.go:2135 | Source selection plus explicit build adapters / source-snapshot |
| Inventory, outlines, signatures, docs, source locations | editor.go:142; index.go:192 | editor_test.go:30,2014 | Declaration facts / source-structure |
| Name, qualified name and source-position definitions | assessment.go:7; navigation.go:11; editor.go:188 | engine_test.go:16; editor_test.go:2204; cmd/codegate/main_test.go:705 | Offline queries, byte/line coordinates / offline-navigation |
| Read/write/call/import/doc references; query scopes and limits | editor.go:234,254; index.go:408,459,543 | editor_test.go:634,2247,2357 | Typed occurrences, bounded result with limits / offline-navigation |
| Callers and callees | editor.go:328,352; index.go:408 | editor_test.go:30,2014 | Typed call relationships and shared query / semantic-navigation |
| Implementations | editor.go:305; index.go:583 | editor_test.go:30 | Candidate vs resolved implementation edges / semantic-navigation |
| Import graph and reverse imports | editor.go:376,384 | editor_test.go:2014 | Dependency observations and query / first-slice |
| Unit symbol/file/LOC/interface and direct/symbol/call fan-in/out | metrics.go:10; internal/core/types.go:629 | editor_test.go:30,1947 | Shared measurements / portable-metrics |
| Symbol reference/call/implementation counts and pressure | metrics.go:65 | editor_test.go:1947,1980 | Shared measurement + versioned pressure policy / scoring-suggestions |
| Import allow/deny precedence and test boundaries | assess.go:241,269 | engine_test.go:1425,1469,1511 | Shared dependency policy / architecture-policies |
| Layers, direction, unknown units | architecture.go:86,115 | engine_test.go:1555,1694 | Explicit classification and policy / architecture-policies |
| Effects, custom rule names/severities/scopes | architecture.go:137,164,258 | engine_test.go:1602; cmd/codegate/main_test.go:538 | Typed effects + rule policy / architecture-policies |
| Fan-out thresholds and reasoned exceptions | architecture.go:221,425,453 | engine_test.go:1644,1694 | Bounded exceptions and visible waivers / architecture-policies |
| Shape/complexity/nesting/parameters/returns/fields/methods | quality.go:280,428,471,510 | engine_test.go:391 | One counting contract / portable-metrics |
| Public API, docs and naming signals | quality.go:343,408,423,1120 | engine_test.go:903,943,987,1016,1264 | Go exported; Rust effective public; Java access/module semantics / portable-metrics |
| Debt marker counts and locations | internal/core/debt.go:11; index.go:163 | engine_test.go:355; cmd/codegate/main_test.go:177 | Comment-only observations / portable-metrics |
| Discarded results, assertions, defer, abrupt exits | quality.go:559,663,715,761 | engine_test.go:488 | Shared normalized or distinct language identities / safety-observations |
| Unsafe/weak crypto/exec/SQL/paths | quality.go:319,794,819,839 | engine_test.go:544,601 | API mappings + evidence-qualified observations / safety-observations |
| Reflection/loop concatenation/capacity/copy | quality.go:319,663,878,1288 | engine_test.go:644,713,750,787 | Language allocation and copy semantics / safety-observations |
| Test inventory/LOC ratios/table tests/nondeterminism/generated ratios | quality.go:122,186,776,1047,1095 | engine_test.go:1131,1192,1226,828 | Inventory signals, never execution coverage / testability-signals |
| Parse/typecheck diagnostics and incompleteness | validate.go:21,103; assess.go:196 | editor_test.go:1718; engine_test.go:1304,1326 | Collection diagnostics and coverage / source-structure, semantic-navigation |
| Four gates: architecture, maintainability, safety, coverage | support.go:5; assess.go:322 | internal/lang/goast/assess_score_test.go:5,23,34,57 | Selected policy requirements / scoring-suggestions |
| Scores, severity/LOC weighting, pressure | assess.go:322,394,441,462,514 | internal/lang/goast/assess_score_test.go:73,86,99,109 | Explicit versioned policy; complete evidence required / scoring-suggestions |
| Advisory unused symbols, extraction, parameter objects, boolean flags, high fan-in/pressure, debt | suggest.go:15,38,65,119,146,173,198,222 | editor_test.go:326,407,439,1980; engine_test.go:1094 | Preserve signals, omit executable edits / scoring-suggestions |
| JSON compact/full/summary, HTML, ranked/top-unit reports, fail-on categories | cmd/codegate/main.go:220,439,721; cmd/codegate/report.go:183,418,476 | cmd/codegate/main_test.go:14,41,132,153,249,289,503,538,576 | Versioned report and escaped standalone HTML / reporting-parity |
| Capability filters, aggregation, unsupported languages | engine.go:327,404; cmd/codegate/main.go:154,624 | engine_test.go:65,98; cmd/codegate/main_test.go:729,751 | Per-family/configuration capability matrix / capability-admission |
| Core cannot parse or invoke host processes | editor_test.go:2456,2474 | Same tests | Dependency-boundary enforcement / capability-admission |

In this table, unqualified `files.go`, `index.go`, `quality.go`, `assess.go`,
`architecture.go`, `validate.go`, `support.go` and `suggest.go` are under
`internal/lang/goast/`; all other paths are relative to reference root.

## Intentional semantic differences to verify

1. Reference navigation and imports report incomplete AST knowledge (`editor.go:188,384`).
   Its implementation matcher compares names without signatures (`index.go:583`).
   Preserve candidates as candidates; tool observations require separate resolution basis.
2. `ignored_error_count` observes blank-identifier discarded call results (`quality.go:663`).
   Do not claim it proves a discarded error type. Typed error classification needs semantic facts.
3. Reference complexity starts at one and includes control/case/communication and Boolean
   decision nodes (`quality.go:471`). Cross-language comparisons must cite one counting
   convention; deviations are separately named, never silently normalized.
4. Suggestions use 40 physical lines (`suggest.go:119`); assessment uses 80 non-comment
   lines (`quality.go:12`). Keep both reference cases and document the chosen policy mapping.
5. Reference thresholds are fixed; scoring is `go-architecture-v1` (`assess.go:90`).
   Configurable scoring and refusing a complete score for missing evidence are requested
   improvements, not numerical identity with every reference grade.
6. The reference `cycle` command is assess/suggest/apply/validate (`cmd/codegate/main.go:331`).
   Dependency graph cycles are a requested addition, not that command's parity mapping.
7. No-data, malformed syntax and stale imported evidence cannot be treated as complete
   zero-valued measurements in the richer contract, even where legacy heuristics did so.

## Requested extensions

| Required extension | Acceptance ownership |
|---|---|
| Rust and Java structural bindings beside Go | source-structure, first-slice |
| gopls / rust-analyzer / JDT LS, explicit build selections | go-semantics, rust-semantics, java-semantics |
| Maven/Gradle multi-module; Java 17/21 target independent from host JDK | java-semantics |
| Dependency cycles, forbidden calls and selected effect policies | architecture-policies |
| Quarkus 3 beans/qualifiers/producers/injections/routes/transactions/config profiles | quarkus-relationships |
| Ambiguous/programmatic/generated/augmentation wiring gaps | quarkus-relationships |
| Snapshot/config identities including dirty/untracked inputs; stale evidence refusal | source-snapshot, capability-admission |
| Deterministic offline rich facts, queries and evaluation | offline-navigation, first-slice, reporting-parity |
| Bounded tools, missing binaries, timeouts, partial collection refusal | semantic-navigation and three semantic adapters |
| Exact generated drift and actual conformance alongside old 27 | parity-foundations, reporting-parity |
| Pinned-reference Go comparisons, mixed-language fixtures and advisory pilots | parity-adoption |

## Explicit exclusions

| Surface | Reference citation | Reason |
|---|---|---|
| Editing, structured operations, formatting/writes/diffs and executable refactoring | operations.go:49; internal/lang/goast/index.go:40; editor_test.go:123,355 | Explicit user exclusion; retain advisory unused-private detection coupled to delete in reference |
| Apply/reassess mutation cycle | cmd/codegate/main.go:331; cmd/codegate/main_test.go:358 | Executes source edits |
| Markdown analysis/navigation/editing | internal/lang/markdown/{index,assess,support}.go; engine_test.go:139,205,297 | Outside the three-language target |
| MCP and hosted service | No implementation identified in pinned reference | Explicit user exclusion, no reference capability claimed |
| Provider-specific validation adapter transport | validation_adapter.go; validation_adapter_test.go | External integration outside local library/CLI boundary; generic diagnostics/completeness retained |

## Updating this matrix

Change a gap to implemented only with a repository source citation, named executable
scenario and retained result. A mapping must state counting/resolution differences
for each language. N/A must name the absent language concept and leave any broader
applicable capability in scope. Final adoption must account for all rows including
noncatalogue capabilities, requested extensions and reference test comparisons.
