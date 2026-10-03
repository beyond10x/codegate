//! Pure canonical identities for collected semantic source values.
//!
//! These projections use the generated model's declared field and enum names.
//! They omit absent optional fields and tracking provenance, normalize sets, and
//! retain ordered observations such as physical source lines.
use crate::semantic_model::*;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;

fn invalid(subject: &str) -> Gap {
    Gap {
        code: GapCode::InvalidFact,
        reason: format!("invalid identity input: {subject}"),
        location: None,
    }
}

fn path(value: &str, subject: &str, allow_root_module: bool) -> Result<(), Gap> {
    if allow_root_module && value == "." {
        return Ok(());
    }
    if value.is_empty()
        || value.contains('\\')
        || value.contains('\0')
        || value.as_bytes().get(1) == Some(&b':')
        || value
            .split('/')
            .any(|part| part.is_empty() || part == "." || part == "..")
    {
        return Err(invalid(subject));
    }
    Ok(())
}

fn object(fields: Vec<(&str, Value)>) -> Value {
    // Sorting explicitly also preserves canonical ordering if serde_json's
    // preserve_order feature is enabled by another dependency in the future.
    Value::Object(
        fields
            .into_iter()
            .map(|(key, value)| (key.to_owned(), value))
            .collect::<BTreeMap<_, _>>()
            .into_iter()
            .collect(),
    )
}

fn optional<'a>(fields: &mut Vec<(&'a str, Value)>, key: &'a str, value: &Option<String>) {
    if let Some(value) = value {
        fields.push((key, json!(value)));
    }
}

fn strings(
    values: &[String],
    subject: &str,
    paths: bool,
    allow_root_module: bool,
) -> Result<Value, Gap> {
    let mut values: Vec<_> = values.iter().collect();
    values.sort();
    if values.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err(invalid(subject));
    }
    for value in &values {
        if paths {
            path(value, subject, allow_root_module)?;
        } else if value.is_empty() {
            return Err(invalid(subject));
        }
    }
    Ok(json!(values))
}

fn language(value: Language) -> &'static str {
    match value {
        Language::Go => "Go",
        Language::Rust => "Rust",
        Language::Java => "Java",
    }
}

fn source_file(value: &SourceFile) -> Result<Value, Gap> {
    path(&value.path, "source file path", false)?;
    let classification = match value.classification {
        SourceClass::Production => "Production",
        SourceClass::Test => "Test",
        SourceClass::Generated => "Generated",
        SourceClass::GeneratedTest => "GeneratedTest",
        SourceClass::BuildConfiguration => "BuildConfiguration",
    };
    let lines: Vec<_> = value
        .lines
        .iter()
        .map(|line| {
            let classification = match line.classification {
                LineClass::Code => "Code",
                LineClass::Comment => "Comment",
                LineClass::Blank => "Blank",
                LineClass::Mixed => "Mixed",
            };
            object(vec![
                ("number", json!(line.number)),
                ("start_byte", json!(line.start_byte)),
                ("end_byte", json!(line.end_byte)),
                ("classification", json!(classification)),
            ])
        })
        .collect();
    let mut fields = vec![
        ("path", json!(value.path)),
        ("content_sha256", json!(value.content_sha256)),
        ("byte_length", json!(value.byte_length)),
        ("classification", json!(classification)),
        ("lines", Value::Array(lines)),
    ];
    if let Some(value) = value.language {
        fields.push(("language", json!(language(value))));
    }
    Ok(object(fields))
}

fn files(values: &[SourceFile]) -> Result<Value, Gap> {
    let mut values: Vec<_> = values.iter().collect();
    values.sort_by(|a, b| a.path.cmp(&b.path));
    if values.windows(2).any(|pair| pair[0].path == pair[1].path) {
        return Err(invalid("duplicate source file path"));
    }
    values
        .into_iter()
        .map(source_file)
        .collect::<Result<Vec<_>, _>>()
        .map(Value::Array)
}

fn configuration(value: &BuildSelection, include_id: bool) -> Result<Value, Gap> {
    let mut fields = vec![
        (
            "modules",
            strings(&value.modules, "configuration modules", true, true)?,
        ),
        (
            "go_build_tags",
            strings(&value.go_build_tags, "Go build tags", false, false)?,
        ),
        (
            "rust_features",
            strings(&value.rust_features, "Rust features", false, false)?,
        ),
        ("rust_default_features", json!(value.rust_default_features)),
        (
            "rust_cfg",
            strings(&value.rust_cfg, "Rust cfg", false, false)?,
        ),
        (
            "build_profiles",
            strings(&value.build_profiles, "build profiles", false, false)?,
        ),
        ("configuration_files", files(&value.configuration_files)?),
    ];
    if include_id {
        fields.push(("id", json!(value.id.0)));
    }
    optional(&mut fields, "target", &value.target);
    optional(
        &mut fields,
        "java_target_version",
        &value.java_target_version,
    );
    optional(
        &mut fields,
        "java_classpath_sha256",
        &value.java_classpath_sha256,
    );
    Ok(object(fields))
}

fn selection(value: &SourceSelection) -> Result<Value, Gap> {
    let mut languages: Vec<_> = value
        .languages
        .iter()
        .map(|value| language(*value))
        .collect();
    languages.sort();
    if languages.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err(invalid("duplicate selection language"));
    }
    Ok(object(vec![
        (
            "include_paths",
            strings(&value.include_paths, "include paths", true, false)?,
        ),
        (
            "exclude_paths",
            strings(&value.exclude_paths, "exclude paths", true, false)?,
        ),
        ("include_tests", json!(value.include_tests)),
        ("include_generated", json!(value.include_generated)),
        ("languages", json!(languages)),
    ]))
}

fn manifests(values: &[Manifest]) -> Result<Value, Gap> {
    let mut values: Vec<_> = values.iter().collect();
    values.sort_by(|a, b| a.path.cmp(&b.path));
    if values.windows(2).any(|pair| pair[0].path == pair[1].path) {
        return Err(invalid("duplicate manifest path"));
    }
    let values = values
        .into_iter()
        .map(|value| {
            path(&value.path, "manifest path", false)?;
            path(&value.module, "manifest module", true)?;
            if let Some(parent) = &value.parent_module {
                path(parent, "manifest parent module", true)?;
            }
            let system = match value.system {
                BuildSystem::GoModules => "GoModules",
                BuildSystem::Cargo => "Cargo",
                BuildSystem::Maven => "Maven",
                BuildSystem::Gradle => "Gradle",
            };
            let mut fields = vec![
                ("path", json!(value.path)),
                ("content_sha256", json!(value.content_sha256)),
                ("system", json!(system)),
                ("module", json!(value.module)),
            ];
            optional(&mut fields, "parent_module", &value.parent_module);
            Ok(object(fields))
        })
        .collect::<Result<Vec<_>, Gap>>()?;
    Ok(Value::Array(values))
}

fn digest(value: Value) -> Result<String, Gap> {
    let bytes = serde_json::to_vec(&value).map_err(|_| invalid("canonical JSON"))?;
    Ok(Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect())
}

/// Derive a configuration identity, excluding its asserted id and file origins.
/// Assertion comparison belongs to collection/admission; this function does not
/// trust or validate the existing id while calculating its replacement.
pub fn configuration_id(value: &BuildSelection) -> Result<ConfigurationId, Gap> {
    digest(configuration(value, false)?).map(ConfigurationId)
}

/// Derive a source identity, excluding its asserted id and all file origins.
/// The nested configuration id remains an input, as required by the contract;
/// callers finalize or validate it with [`configuration_id`] independently.
pub fn snapshot_id(value: &SourceSnapshot) -> Result<SnapshotId, Gap> {
    digest(object(vec![
        ("selection", selection(&value.selection)?),
        ("configuration", configuration(&value.configuration, true)?),
        ("files", files(&value.files)?),
        ("manifests", manifests(&value.manifests)?),
    ]))
    .map(SnapshotId)
}
