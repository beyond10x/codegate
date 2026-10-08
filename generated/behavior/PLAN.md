<!--
  generated from codegate v1
  model digest e27ccc45957cf4fe2ef83d9362e81d867eb46bff9a72f1fd28e43b69bf53ec93
  contract digest df71e33e5abc7d133716034004ba68a3a08d1c620351660f3940aa973a3c93b8
  do not edit: regenerate with `ess synthesize --layout crate`
-->
# Synthesis plan — codegate v1

Scope: `component-skeletons`, laid out as `crate`, planned by `ess-synth`. Regenerate with `ess synthesize --layout crate`.

18 capabilities: **17 generated**, **1 obligations**, **0 refused**. An obligation is yours to implement against its contract; a refusal is a fact about this synthesis scope, not about the specification.

## Generated

| capability | source |
| --- | --- |
| domain type | `codegate.dependency.Coverage` |
| domain type | `codegate.dependency.CoverageStatus` |
| domain type | `codegate.dependency.DependencyKind` |
| domain type | `codegate.dependency.Diagnostic` |
| domain type | `codegate.dependency.DiagnosticCode` |
| domain type | `codegate.dependency.Edge` |
| domain type | `codegate.dependency.EdgeId` |
| domain type | `codegate.dependency.Evaluation` |
| domain type | `codegate.dependency.FactSnapshot` |
| domain type | `codegate.dependency.FanOut` |
| domain type | `codegate.dependency.Finding` |
| domain type | `codegate.dependency.ForbiddenDependency` |
| domain type | `codegate.dependency.Policy` |
| domain type | `codegate.dependency.Unit` |
| domain type | `codegate.dependency.UnitId` |
| domain type | `codegate.dependency.Verdict` |
| command contract | `codegate.dependency.Evaluate` |

## Obligations — yours to implement

| capability | source | why not generated | contract |
| --- | --- | --- | --- |
| command behaviour | `codegate.dependency.Evaluate` | kept an obligation by a typed response (`response:`) | given `codegate.dependency.Evaluate` input, decide and enact exactly one outcome. Declared outcomes (declaration order, not selection precedence): `evaluated` otherwise |

## Refused — not represented by this synthesis

| capability | source | stage | why |
| --- | --- | --- | --- |
