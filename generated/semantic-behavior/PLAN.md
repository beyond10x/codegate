<!--
  generated from codegate_semantic v1
  model digest c64e8e2f5e1a2e791ed83c172875fb04a67c095ec27f188220b1d5644654c126
  contract digest bc8bafef42abf16c63fe7d2bf7a90b4f4e1e3d2b25bff37f64532fd9ad9d5f1d
  do not edit: regenerate with `ess synthesize --layout crate`
-->
# Synthesis plan — codegate_semantic v1

Scope: `component-skeletons`, laid out as `crate`, planned by `ess-synth`. Regenerate with `ess synthesize --layout crate`.

90 capabilities: **82 generated**, **8 obligations**, **0 refused**. An obligation is yours to implement against its contract; a refusal is a fact about this synthesis scope, not about the specification.

## Generated

| capability | source |
| --- | --- |
| domain type | `codegate_semantic.semantic.ApiEligibility` |
| domain type | `codegate_semantic.semantic.Assessment` |
| domain type | `codegate_semantic.semantic.BuildSelection` |
| domain type | `codegate_semantic.semantic.BuildSystem` |
| domain type | `codegate_semantic.semantic.Call` |
| domain type | `codegate_semantic.semantic.Capability` |
| domain type | `codegate_semantic.semantic.CheckKind` |
| domain type | `codegate_semantic.semantic.CheckResult` |
| domain type | `codegate_semantic.semantic.CollectionMode` |
| domain type | `codegate_semantic.semantic.CollectionRequest` |
| domain type | `codegate_semantic.semantic.Completeness` |
| domain type | `codegate_semantic.semantic.ConfigurationId` |
| domain type | `codegate_semantic.semantic.Coverage` |
| domain type | `codegate_semantic.semantic.DecisionKind` |
| domain type | `codegate_semantic.semantic.DecisionPoint` |
| domain type | `codegate_semantic.semantic.Declaration` |
| domain type | `codegate_semantic.semantic.DeclarationId` |
| domain type | `codegate_semantic.semantic.DeclarationKind` |
| domain type | `codegate_semantic.semantic.Dependency` |
| domain type | `codegate_semantic.semantic.DependencyKind` |
| domain type | `codegate_semantic.semantic.Effect` |
| domain type | `codegate_semantic.semantic.EffectKind` |
| domain type | `codegate_semantic.semantic.Evidence` |
| domain type | `codegate_semantic.semantic.FactFamily` |
| domain type | `codegate_semantic.semantic.FactId` |
| domain type | `codegate_semantic.semantic.FactSnapshot` |
| domain type | `codegate_semantic.semantic.Finding` |
| domain type | `codegate_semantic.semantic.FrameworkFact` |
| domain type | `codegate_semantic.semantic.FrameworkKind` |
| domain type | `codegate_semantic.semantic.Gap` |
| domain type | `codegate_semantic.semantic.GapCode` |
| domain type | `codegate_semantic.semantic.Implementation` |
| domain type | `codegate_semantic.semantic.Language` |
| domain type | `codegate_semantic.semantic.LineClass` |
| domain type | `codegate_semantic.semantic.LookupSelector` |
| domain type | `codegate_semantic.semantic.Manifest` |
| domain type | `codegate_semantic.semantic.Measurement` |
| domain type | `codegate_semantic.semantic.MetricAggregation` |
| domain type | `codegate_semantic.semantic.MetricKind` |
| domain type | `codegate_semantic.semantic.MetricSelection` |
| domain type | `codegate_semantic.semantic.NavigationResult` |
| domain type | `codegate_semantic.semantic.Occurrence` |
| domain type | `codegate_semantic.semantic.OccurrenceRole` |
| domain type | `codegate_semantic.semantic.Policy` |
| domain type | `codegate_semantic.semantic.PolicyException` |
| domain type | `codegate_semantic.semantic.PolicyRule` |
| domain type | `codegate_semantic.semantic.Qualifier` |
| domain type | `codegate_semantic.semantic.QueryKind` |
| domain type | `codegate_semantic.semantic.Reference` |
| domain type | `codegate_semantic.semantic.Resolution` |
| domain type | `codegate_semantic.semantic.ResolutionBasis` |
| domain type | `codegate_semantic.semantic.Score` |
| domain type | `codegate_semantic.semantic.ScoreComponent` |
| domain type | `codegate_semantic.semantic.SelectorKind` |
| domain type | `codegate_semantic.semantic.Severity` |
| domain type | `codegate_semantic.semantic.SnapshotId` |
| domain type | `codegate_semantic.semantic.SourceClass` |
| domain type | `codegate_semantic.semantic.SourceFile` |
| domain type | `codegate_semantic.semantic.SourceLine` |
| domain type | `codegate_semantic.semantic.SourceOrigin` |
| domain type | `codegate_semantic.semantic.SourceRange` |
| domain type | `codegate_semantic.semantic.SourceSelection` |
| domain type | `codegate_semantic.semantic.SourceSnapshot` |
| domain type | `codegate_semantic.semantic.StructureKind` |
| domain type | `codegate_semantic.semantic.StructureObservation` |
| domain type | `codegate_semantic.semantic.Suggestion` |
| domain type | `codegate_semantic.semantic.TestKind` |
| domain type | `codegate_semantic.semantic.ToolObservation` |
| domain type | `codegate_semantic.semantic.Unit` |
| domain type | `codegate_semantic.semantic.UnitId` |
| domain type | `codegate_semantic.semantic.Verdict` |
| domain type | `codegate_semantic.semantic.Visibility` |
| command contract | `codegate_semantic.semantic.Assess` |
| command contract | `codegate_semantic.semantic.Capabilities` |
| command contract | `codegate_semantic.semantic.Collect` |
| command contract | `codegate_semantic.semantic.CollectSource` |
| command contract | `codegate_semantic.semantic.Evaluate` |
| command contract | `codegate_semantic.semantic.Lookup` |
| command contract | `codegate_semantic.semantic.Suggest` |
| command contract | `codegate_semantic.semantic.ValidateSnapshot` |
| component port | `collection` |
| component port | `source-foundation` |

## Obligations — yours to implement

| capability | source | why not generated | contract |
| --- | --- | --- | --- |
| command behaviour | `codegate_semantic.semantic.Assess` | kept an obligation by a typed response (`response:`) | given `codegate_semantic.semantic.Assess` input, decide and enact exactly one outcome. Declared outcomes (declaration order, not selection precedence): `assessed` otherwise |
| command behaviour | `codegate_semantic.semantic.Capabilities` | kept an obligation by a typed response (`response:`) | given `codegate_semantic.semantic.Capabilities` input, decide and enact exactly one outcome. Declared outcomes (declaration order, not selection precedence): `listed` otherwise |
| command behaviour | `codegate_semantic.semantic.Collect` | kept an obligation by a typed response (`response:`) | given `codegate_semantic.semantic.Collect` input, decide and enact exactly one outcome. Declared outcomes (declaration order, not selection precedence): `collected` otherwise |
| command behaviour | `codegate_semantic.semantic.CollectSource` | kept an obligation by a typed response (`response:`) | given `codegate_semantic.semantic.CollectSource` input, decide and enact exactly one outcome. Declared outcomes (declaration order, not selection precedence): `source-observed` otherwise |
| command behaviour | `codegate_semantic.semantic.Evaluate` | kept an obligation by a typed response (`response:`) | given `codegate_semantic.semantic.Evaluate` input, decide and enact exactly one outcome. Declared outcomes (declaration order, not selection precedence): `evaluated` otherwise |
| command behaviour | `codegate_semantic.semantic.Lookup` | kept an obligation by a typed response (`response:`) | given `codegate_semantic.semantic.Lookup` input, decide and enact exactly one outcome. Declared outcomes (declaration order, not selection precedence): `looked-up` otherwise |
| command behaviour | `codegate_semantic.semantic.Suggest` | kept an obligation by a typed response (`response:`) | given `codegate_semantic.semantic.Suggest` input, decide and enact exactly one outcome. Declared outcomes (declaration order, not selection precedence): `suggested` otherwise |
| command behaviour | `codegate_semantic.semantic.ValidateSnapshot` | kept an obligation by a typed response (`response:`) | given `codegate_semantic.semantic.ValidateSnapshot` input, decide and enact exactly one outcome. Declared outcomes (declaration order, not selection precedence): `validation-observed` otherwise |

## Refused — not represented by this synthesis

| capability | source | stage | why |
| --- | --- | --- | --- |
