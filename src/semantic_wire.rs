//! Pure structural bridge between generated semantic contracts.
//!
//! Decoding is bounded and rejects ambiguous JSON. Semantic admission, identity
//! checks and completeness decisions remain the responsibility of admission.
use crate::semantic_model as b;
use codegate_semantic_contract as w;
use serde::{
    Deserialize,
    de::{self, MapAccess, SeqAccess, Visitor},
};
use serde_json::Value;
use std::{collections::BTreeSet, fmt, sync::LazyLock};

pub const MAX_DOCUMENT_BYTES: usize = 4 * 1024 * 1024;

trait Bridge<B>: Sized {
    fn into_behavior(self) -> Result<B, String>;
    fn from_behavior(value: B) -> Self;
}
macro_rules! primitive {
    ($($t:ty),+ $(,)?) => {$(impl Bridge<$t> for $t {
        fn into_behavior(self) -> Result<$t, String> { Ok(self) }
        fn from_behavior(value: $t) -> Self { value }
    })+};
}
primitive!(String, bool);
impl Bridge<i64> for serde_json::Number {
    fn into_behavior(self) -> Result<i64, String> {
        self.as_i64()
            .ok_or_else(|| "integer must fit signed 64-bit range".into())
    }
    fn from_behavior(value: i64) -> Self {
        value.into()
    }
}
impl<T: Bridge<B>, B> Bridge<B> for Box<T> {
    fn into_behavior(self) -> Result<B, String> {
        (*self).into_behavior()
    }
    fn from_behavior(value: B) -> Self {
        Box::new(T::from_behavior(value))
    }
}
impl<T: Bridge<B>, B> Bridge<Vec<B>> for Vec<T> {
    fn into_behavior(self) -> Result<Vec<B>, String> {
        self.into_iter().map(T::into_behavior).collect()
    }
    fn from_behavior(value: Vec<B>) -> Self {
        value.into_iter().map(T::from_behavior).collect()
    }
}
impl<T: Bridge<B>, B> Bridge<Option<B>> for w::EssPresence<T> {
    fn into_behavior(self) -> Result<Option<B>, String> {
        match self {
            Self::Absent => Ok(None),
            Self::Present(value) => value.into_behavior().map(Some),
        }
    }
    fn from_behavior(value: Option<B>) -> Self {
        match value {
            None => Self::Absent,
            Some(value) => Self::Present(T::from_behavior(value)),
        }
    }
}
macro_rules! record {
    ($wire:ident, $behavior:ident, $($field:ident),+ $(,)?) => {
        impl Bridge<b::$behavior> for w::$wire {
            fn into_behavior(self) -> Result<b::$behavior, String> { Ok(b::$behavior { $($field: self.$field.into_behavior()?),+ }) }
            fn from_behavior(value: b::$behavior) -> Self { Self { $($field: Bridge::from_behavior(value.$field)),+ } }
        }
    };
}
macro_rules! enumeration {
    ($wire:ident, $behavior:ident, $($w:ident => $b:ident),+ $(,)?) => {
        impl Bridge<b::$behavior> for w::$wire {
            fn into_behavior(self) -> Result<b::$behavior, String> { Ok(match self { $(Self::$w => b::$behavior::$b),+ }) }
            fn from_behavior(value: b::$behavior) -> Self { match value { $(b::$behavior::$b => Self::$w),+ } }
        }
    };
}
macro_rules! identity {
    ($wire:ident, $behavior:ident) => {
        impl Bridge<b::$behavior> for w::$wire {
            fn into_behavior(self) -> Result<b::$behavior, String> {
                Ok(b::$behavior(self.0))
            }
            fn from_behavior(value: b::$behavior) -> Self {
                Self(value.0)
            }
        }
    };
}
enumeration!(CodegateSemanticSemanticApiEligibility, ApiEligibility, V0 => Eligible, V1 => Ineligible, V2 => Unknown);
record!(
    CodegateSemanticSemanticBuildSelection,
    BuildSelection,
    id,
    modules,
    target,
    go_build_tags,
    rust_features,
    rust_default_features,
    rust_cfg,
    java_target_version,
    java_classpath_sha256,
    build_profiles,
    configuration_files
);
enumeration!(CodegateSemanticSemanticBuildSystem, BuildSystem, V0 => Cargo, V1 => GoModules, V2 => Gradle, V3 => Maven);
record!(
    CodegateSemanticSemanticCall,
    Call,
    caller,
    callee,
    candidates,
    dynamic_dispatch,
    evidence
);
enumeration!(CodegateSemanticSemanticCollectionMode, CollectionMode, V0 => Semantic, V1 => SourceOnly);
record!(
    CodegateSemanticSemanticCollectionRequest,
    CollectionRequest,
    root,
    selection,
    configuration,
    mode,
    timeout_milliseconds
);
enumeration!(CodegateSemanticSemanticCompleteness, Completeness, V0 => Complete, V1 => Failed, V2 => NotApplicable, V3 => Partial, V4 => Unsupported);
identity!(CodegateSemanticSemanticConfigurationId, ConfigurationId);
record!(
    CodegateSemanticSemanticCoverage,
    Coverage,
    family,
    configuration_id,
    unit_ids,
    status,
    gaps,
    applicability_reason
);
enumeration!(CodegateSemanticSemanticDecisionKind, DecisionKind, V0 => CaseBranch, V1 => CatchBranch, V2 => ConditionalBranch, V3 => GoSelectCase, V4 => Loop, V5 => RustTryPropagation, V6 => ShortCircuitAnd, V7 => ShortCircuitOr, V8 => Ternary);
record!(
    CodegateSemanticSemanticDecisionPoint,
    DecisionPoint,
    owner,
    kind,
    evidence
);
record!(
    CodegateSemanticSemanticDeclaration,
    Declaration,
    id,
    unit_id,
    parent,
    name,
    qualified_name,
    signature,
    kind,
    visibility,
    api_eligibility,
    evidence
);
identity!(CodegateSemanticSemanticDeclarationId, DeclarationId);
enumeration!(CodegateSemanticSemanticDeclarationKind, DeclarationKind, V0 => Constant, V1 => Constructor, V2 => Field, V3 => Function, V4 => Interface, V5 => Method, V6 => Module, V7 => Namespace, V8 => Package, V9 => Parameter, V10 => Trait, V11 => Type, V12 => Variable);
record!(
    CodegateSemanticSemanticDependency,
    Dependency,
    source,
    target,
    target_name,
    kind,
    evidence
);
enumeration!(CodegateSemanticSemanticDependencyKind, DependencyKind, V0 => Build, V1 => Runtime, V2 => Test);
record!(
    CodegateSemanticSemanticEffect,
    Effect,
    owner,
    kind,
    related_declaration,
    evidence
);
enumeration!(CodegateSemanticSemanticEffectKind, EffectKind, V0 => AbruptExit, V1 => AllocationInLoop, V2 => DiscardedResult, V3 => GoDefer, V4 => GoUncheckedTypeAssertion, V5 => LargeValueCopy, V6 => MissingCapacity, V7 => Nondeterminism, V8 => PathConstruction, V9 => ProcessExecution, V10 => Reflection, V11 => SqlConstruction, V12 => StringConcatenationInLoop, V13 => UnsafeOperation, V14 => WeakCryptography);
record!(
    CodegateSemanticSemanticEvidence,
    Evidence,
    id,
    snapshot_id,
    configuration_id,
    producer,
    producer_version,
    location,
    resolution,
    basis,
    gaps
);
enumeration!(CodegateSemanticSemanticFactFamily, FactFamily, V0 => Calls, V1 => Decisions, V2 => Declarations, V3 => Dependencies, V4 => Documentation, V5 => Effects, V6 => ExecutionCoverage, V7 => FrameworkDeclarations, V8 => FrameworkWiring, V9 => Implementations, V10 => Occurrences, V11 => References, V12 => Sources, V13 => Structure, V14 => Tests);
identity!(CodegateSemanticSemanticFactId, FactId);
record!(
    CodegateSemanticSemanticFactSnapshot,
    FactSnapshot,
    format,
    source,
    mode,
    tools,
    coverage,
    units,
    declarations,
    occurrences,
    dependencies,
    references,
    calls,
    implementations,
    decisions,
    structure,
    effects,
    framework
);
record!(
    CodegateSemanticSemanticFrameworkFact,
    FrameworkFact,
    kind,
    declaration,
    target,
    annotation_name,
    qualifiers,
    scope,
    produced_type,
    http_method,
    resource_path,
    method_path,
    transaction_mode,
    configuration_key,
    configuration_profile,
    evidence
);
enumeration!(CodegateSemanticSemanticFrameworkKind, FrameworkKind, V0 => Bean, V1 => ConfigurationReference, V2 => InjectionBinding, V3 => InjectionCandidate, V4 => InjectionPoint, V5 => Producer, V6 => Qualifier, V7 => RestResource, V8 => RestRoute, V9 => TransactionDeclaration);
record!(CodegateSemanticSemanticGap, Gap, code, reason, location);
enumeration!(CodegateSemanticSemanticGapCode, GapCode, V0 => AmbiguousInjection, V1 => AmbiguousSymbol, V2 => AugmentationRequired, V3 => ConditionalCompilation, V4 => DynamicDispatch, V5 => GeneratedSources, V6 => IdentityMismatch, V7 => InvalidFact, V8 => InvalidFormat, V9 => InvalidPolicy, V10 => MacroExpansion, V11 => MissingBuildSelection, V12 => MissingClasspath, V13 => MissingTool, V14 => PartialCollection, V15 => ProgrammaticLookup, V16 => SourceChangedDuringCollection, V17 => StaleEvidence, V18 => SyntaxError, V19 => ToolFailure, V20 => ToolTimeout, V21 => UnknownUnit, V22 => UnresolvedSymbol, V23 => UnsupportedCapability);
record!(
    CodegateSemanticSemanticImplementation,
    Implementation,
    declaration,
    implementation,
    candidates,
    evidence
);
enumeration!(CodegateSemanticSemanticLanguage, Language, V0 => Go, V1 => Java, V2 => Rust);
enumeration!(CodegateSemanticSemanticLineClass, LineClass, V0 => Blank, V1 => Code, V2 => Comment, V3 => Mixed);
record!(
    CodegateSemanticSemanticManifest,
    Manifest,
    path,
    content_sha256,
    system,
    module,
    parent_module
);
record!(
    CodegateSemanticSemanticOccurrence,
    Occurrence,
    owner,
    spelling,
    role,
    evidence
);
enumeration!(CodegateSemanticSemanticOccurrenceRole, OccurrenceRole, V0 => Annotation, V1 => Definition, V2 => DocumentationReference, V3 => Import, V4 => Invocation, V5 => Read, V6 => TypeUse, V7 => Write);
record!(
    CodegateSemanticSemanticQualifier,
    Qualifier,
    annotation_name,
    member_name,
    member_value,
    nonbinding
);
record!(
    CodegateSemanticSemanticReference,
    Reference,
    occurrence_id,
    target,
    candidates,
    evidence
);
enumeration!(CodegateSemanticSemanticResolution, Resolution, V0 => Resolved, V1 => SyntacticCandidate, V2 => Unresolved);
enumeration!(CodegateSemanticSemanticResolutionBasis, ResolutionBasis, V0 => DeclaredFramework, V1 => LanguageServer, V2 => MatchedBuildEvidence, V3 => SourceStructure);
identity!(CodegateSemanticSemanticSnapshotId, SnapshotId);
enumeration!(CodegateSemanticSemanticSourceClass, SourceClass, V0 => BuildConfiguration, V1 => Generated, V2 => GeneratedTest, V3 => Production, V4 => Test);
record!(
    CodegateSemanticSemanticSourceFile,
    SourceFile,
    path,
    content_sha256,
    byte_length,
    language,
    classification,
    origin,
    lines
);
record!(
    CodegateSemanticSemanticSourceLine,
    SourceLine,
    number,
    start_byte,
    end_byte,
    classification
);
enumeration!(CodegateSemanticSemanticSourceOrigin, SourceOrigin, V0 => ExternalBuildEvidence, V1 => TrackedClean, V2 => TrackedDirty, V3 => Untracked);
record!(
    CodegateSemanticSemanticSourceRange,
    SourceRange,
    path,
    start_byte,
    end_byte,
    start_line,
    start_column,
    end_line,
    end_column
);
record!(
    CodegateSemanticSemanticSourceSelection,
    SourceSelection,
    include_paths,
    exclude_paths,
    include_tests,
    include_generated,
    languages
);
record!(
    CodegateSemanticSemanticSourceSnapshot,
    SourceSnapshot,
    id,
    selection,
    configuration,
    files,
    manifests
);
enumeration!(CodegateSemanticSemanticStructureKind, StructureKind, V0 => Allocation, V1 => Body, V2 => DebtMarker, V3 => Documentation, V4 => Field, V5 => InterfaceMember, V6 => Loop, V7 => NamingSignal, V8 => NestingRegion, V9 => Parameter, V10 => PublicApiMember, V11 => ReturnStatement, V12 => TestCase);
record!(
    CodegateSemanticSemanticStructureObservation,
    StructureObservation,
    owner,
    kind,
    parent_observation,
    test_kind,
    text,
    evidence
);
enumeration!(CodegateSemanticSemanticTestKind, TestKind, V0 => Benchmark, V1 => Fuzz, V2 => Integration, V3 => Parameterized, V4 => TableDriven, V5 => Unit);
record!(
    CodegateSemanticSemanticToolObservation,
    ToolObservation,
    tool,
    version,
    host_runtime_version,
    configuration_id,
    snapshot_id,
    elapsed_milliseconds,
    exit_code,
    timed_out,
    diagnostics
);
record!(
    CodegateSemanticSemanticUnit,
    Unit,
    id,
    name,
    language,
    paths,
    evidence
);
identity!(CodegateSemanticSemanticUnitId, UnitId);
enumeration!(CodegateSemanticSemanticVisibility, Visibility, V0 => Internal, V1 => Package, V2 => Private, V3 => Protected, V4 => Public, V5 => Unknown);

// This visitor runs before Value can collapse duplicate keys. Serde's default
// recursion limit also bounds nested JSON, including values later refused by schema.
struct UniqueJson;
impl<'de> Deserialize<'de> for UniqueJson {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct UniqueVisitor;
        impl<'de> Visitor<'de> for UniqueVisitor {
            type Value = UniqueJson;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("JSON without duplicate keys")
            }
            fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> Result<UniqueJson, A::Error> {
                let mut keys = BTreeSet::new();
                while let Some(key) = map.next_key::<String>()? {
                    // arbitrary_precision uses this private map representation.
                    // The bridge accepts only native i64 integers; never let an
                    // input object masquerade as a Number during Value decoding.
                    if key == "$serde_json::private::Number" {
                        return Err(de::Error::custom("invalid integer representation"));
                    }
                    if !keys.insert(key.clone()) {
                        return Err(de::Error::custom(format!("duplicate key: {key}")));
                    }
                    map.next_value::<UniqueJson>()?;
                }
                Ok(UniqueJson)
            }
            fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> Result<UniqueJson, A::Error> {
                while seq.next_element::<UniqueJson>()?.is_some() {}
                Ok(UniqueJson)
            }
            fn visit_str<E: de::Error>(self, _: &str) -> Result<UniqueJson, E> {
                Ok(UniqueJson)
            }
            fn visit_bool<E: de::Error>(self, _: bool) -> Result<UniqueJson, E> {
                Ok(UniqueJson)
            }
            fn visit_unit<E: de::Error>(self) -> Result<UniqueJson, E> {
                Ok(UniqueJson)
            }
            fn visit_i64<E: de::Error>(self, _: i64) -> Result<UniqueJson, E> {
                Ok(UniqueJson)
            }
            fn visit_u64<E: de::Error>(self, _: u64) -> Result<UniqueJson, E> {
                Ok(UniqueJson)
            }
            fn visit_f64<E: de::Error>(self, _: f64) -> Result<UniqueJson, E> {
                Ok(UniqueJson)
            }
        }
        deserializer.deserialize_any(UniqueVisitor)
    }
}
static SCHEMA: LazyLock<Value> = LazyLock::new(|| {
    serde_json::from_str(include_str!(
        "../generated/semantic-wire/source.schema.json"
    ))
    .expect("checked-in generated semantic schema is JSON")
});

// Match the legacy boundary's explicit-null-as-absence behavior only where the
// generated schema declares an optional field. Unknown fields remain visible to
// generated deny_unknown_fields; required nulls remain invalid.
fn normalize_optional_nulls(value: &mut Value, schema: &Value, depth: usize) -> Result<(), String> {
    if depth > 128 {
        return Err("document nesting exceeds limit".into());
    }
    if let Some(reference) = schema.get("$ref").and_then(Value::as_str) {
        let target = reference
            .strip_prefix('#')
            .and_then(|pointer| SCHEMA.pointer(pointer))
            .ok_or("generated schema contains an unsupported reference")?;
        return normalize_optional_nulls(value, target, depth + 1);
    }
    if let (Some(object), Some(properties)) = (
        value.as_object_mut(),
        schema.get("properties").and_then(Value::as_object),
    ) {
        for (name, property) in properties {
            let required = schema
                .get("required")
                .and_then(Value::as_array)
                .is_some_and(|names| names.iter().any(|item| item.as_str() == Some(name)));
            if !required && object.get(name).is_some_and(Value::is_null) {
                object.remove(name);
            }
            if let Some(child) = object.get_mut(name) {
                normalize_optional_nulls(child, property, depth + 1)?;
            }
        }
    } else if let (Some(items), Some(item_schema)) = (value.as_array_mut(), schema.get("items")) {
        for item in items {
            normalize_optional_nulls(item, item_schema, depth + 1)?;
        }
    }
    Ok(())
}
fn document(bytes: &[u8], type_name: &str) -> Result<Value, String> {
    if bytes.len() > MAX_DOCUMENT_BYTES {
        return Err("document exceeds 4 MiB limit".into());
    }
    serde_json::from_slice::<UniqueJson>(bytes).map_err(|e| e.to_string())?;
    let mut value: Value = serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
    let schema = SCHEMA["$defs"]
        .get(format!("codegate_semantic.semantic.{type_name}"))
        .ok_or("unknown generated semantic type")?;
    normalize_optional_nulls(&mut value, schema, 0)?;
    Ok(value)
}

/// Decode structural fact values; callers must separately admit their semantics.
pub fn decode_snapshot(bytes: &[u8]) -> Result<b::FactSnapshot, String> {
    serde_json::from_value::<w::CodegateSemanticSemanticFactSnapshot>(document(
        bytes,
        "FactSnapshot",
    )?)
    .map_err(|e| e.to_string())?
    .into_behavior()
}
/// Decode the explicit collection request; no filesystem or tool access occurs.
pub fn decode_request(bytes: &[u8]) -> Result<b::CollectionRequest, String> {
    serde_json::from_value::<w::CodegateSemanticSemanticCollectionRequest>(document(
        bytes,
        "CollectionRequest",
    )?)
    .map_err(|e| e.to_string())?
    .into_behavior()
}
pub fn decode_configuration(bytes: &[u8]) -> Result<b::BuildSelection, String> {
    serde_json::from_value::<w::CodegateSemanticSemanticBuildSelection>(document(
        bytes,
        "BuildSelection",
    )?)
    .map_err(|e| e.to_string())?
    .into_behavior()
}
pub fn snapshot_value(snapshot: &b::FactSnapshot) -> Result<Value, String> {
    serde_json::to_value(w::CodegateSemanticSemanticFactSnapshot::from_behavior(
        snapshot.clone(),
    ))
    .map_err(|e| e.to_string())
}
pub fn request_value(request: &b::CollectionRequest) -> Result<Value, String> {
    serde_json::to_value(w::CodegateSemanticSemanticCollectionRequest::from_behavior(
        request.clone(),
    ))
    .map_err(|e| e.to_string())
}
pub fn source_value(source: &b::SourceSnapshot) -> Result<Value, String> {
    serde_json::to_value(w::CodegateSemanticSemanticSourceSnapshot::from_behavior(
        source.clone(),
    ))
    .map_err(|e| e.to_string())
}
pub fn configuration_value(configuration: &b::BuildSelection) -> Result<Value, String> {
    serde_json::to_value(w::CodegateSemanticSemanticBuildSelection::from_behavior(
        configuration.clone(),
    ))
    .map_err(|e| e.to_string())
}
pub fn gaps_value(gaps: &[b::Gap]) -> Result<Value, String> {
    serde_json::to_value(<Vec<w::CodegateSemanticSemanticGap>>::from_behavior(
        gaps.to_vec(),
    ))
    .map_err(|e| e.to_string())
}
