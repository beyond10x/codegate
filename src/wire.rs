//! Explicit bridge between the two generated contracts; no domain model is copied here.
use crate::model as b;
use codegate_contract as w;
use serde::{
    Deserialize,
    de::{self, MapAccess, SeqAccess, Visitor},
};
use serde_json::Value;
use std::{collections::BTreeSet, fmt};

pub const MAX_DOCUMENT_BYTES: usize = 4 * 1024 * 1024;
pub const MAX_UNITS: usize = 10_000;
pub const MAX_EDGES: usize = 50_000;

pub trait Bridge<B>: Sized {
    fn into_behavior(self) -> Result<B, String>;
    fn from_behavior(value: B) -> Self;
}
impl Bridge<String> for String {
    fn into_behavior(self) -> Result<String, String> {
        Ok(self)
    }
    fn from_behavior(value: String) -> Self {
        value
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
impl Bridge<i64> for serde_json::Number {
    fn into_behavior(self) -> Result<i64, String> {
        self.as_i64()
            .filter(|v| *v >= 0 && *v <= MAX_UNITS as i64)
            .ok_or_else(|| "fan-out must be a nonnegative bounded integer".into())
    }
    fn from_behavior(value: i64) -> Self {
        value.into()
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
identity!(CodegateDependencyUnitId, UnitId);
identity!(CodegateDependencyEdgeId, EdgeId);
enumeration!(CodegateDependencyCoverageStatus, CoverageStatus, V0=>Complete, V1=>Failed, V2=>Partial, V3=>Unsupported);
enumeration!(CodegateDependencyDependencyKind, DependencyKind, V0=>Build, V1=>Runtime, V2=>Test);
enumeration!(CodegateDependencyVerdict, Verdict, V0=>Error, V1=>Fail, V2=>Incomplete, V3=>Pass, V4=>Unsupported);
enumeration!(CodegateDependencyDiagnosticCode, DiagnosticCode, V0=>ConfigurationMismatch,V1=>DanglingSource,V2=>DanglingTarget,V3=>DuplicateEdge,V4=>DuplicateUnit,V5=>EmptyIdentity,V6=>FailedCoverage,V7=>InvalidCoverage,V8=>InvalidFormat,V9=>InvalidPolicy,V10=>InvalidTarget,V11=>PartialCoverage,V12=>SourceMismatch,V13=>UnsupportedCoverage);
record!(CodegateDependencyCoverage, Coverage, status, gaps);
record!(CodegateDependencyUnit, Unit, id, language);
record!(
    CodegateDependencyEdge,
    Edge,
    id,
    source,
    target,
    unresolved_target,
    kind
);
record!(
    CodegateDependencyFactSnapshot,
    FactSnapshot,
    format,
    source_id,
    configuration_id,
    producer,
    producer_version,
    units,
    edges,
    coverage
);
record!(
    CodegateDependencyPolicy,
    Policy,
    format,
    expected_source_id,
    expected_configuration_id,
    dependency_kind,
    forbidden
);
record!(
    CodegateDependencyForbiddenDependency,
    ForbiddenDependency,
    source,
    target,
    kind
);
record!(CodegateDependencyFanOut, FanOut, unit_id, value);
record!(
    CodegateDependencyFinding,
    Finding,
    edge_id,
    source,
    target,
    kind
);
record!(CodegateDependencyDiagnostic, Diagnostic, code, subject);
record!(
    CodegateDependencyEvaluation,
    Evaluation,
    format,
    source_id,
    configuration_id,
    policy,
    metric_id,
    checker_id,
    coverage,
    verdict,
    fan_out,
    findings,
    diagnostics
);

// Check keys before Value decoding can collapse them. No numbers are needed here:
// all admitted fact/policy primitive values are strings, nulls, arrays or objects.
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
fn document(bytes: &[u8]) -> Result<Value, String> {
    if bytes.len() > MAX_DOCUMENT_BYTES {
        return Err("document exceeds 4 MiB limit".into());
    }
    serde_json::from_slice::<UniqueJson>(bytes).map_err(|e| e.to_string())?;
    serde_json::from_slice(bytes).map_err(|e| e.to_string())
}
pub fn decode_snapshot(bytes: &[u8]) -> Result<b::FactSnapshot, String> {
    let mut value = document(bytes)?;
    if let Some(edges) = value.get_mut("edges").and_then(Value::as_array_mut) {
        if edges.len() > MAX_EDGES {
            return Err("snapshot exceeds 50000 edge limit".into());
        }
        for edge in edges {
            for field in ["target", "unresolved_target"] {
                if edge.get(field).is_some_and(Value::is_null) {
                    edge.as_object_mut()
                        .expect("object field exists")
                        .remove(field);
                }
            }
        }
    }
    if value
        .get("units")
        .and_then(Value::as_array)
        .is_some_and(|v| v.len() > MAX_UNITS)
    {
        return Err("snapshot exceeds 10000 unit limit".into());
    }
    serde_json::from_value::<w::CodegateDependencyFactSnapshot>(value)
        .map_err(|e| e.to_string())?
        .into_behavior()
}
pub fn decode_policy(bytes: &[u8]) -> Result<b::Policy, String> {
    serde_json::from_value::<w::CodegateDependencyPolicy>(document(bytes)?)
        .map_err(|e| e.to_string())?
        .into_behavior()
}
pub fn report_value(report: b::Evaluation) -> Result<Value, String> {
    // Check the Number projection obligation in both directions before serialization.
    let projected = w::CodegateDependencyEvaluation::from_behavior(report);
    projected.clone().into_behavior()?;
    let mut value = serde_json::to_value(projected).map_err(|e| e.to_string())?;
    for metric in value["fan_out"].as_array_mut().expect("generated list") {
        metric
            .as_object_mut()
            .expect("generated record")
            .entry("value")
            .or_insert(Value::Null);
    }
    Ok(value)
}
