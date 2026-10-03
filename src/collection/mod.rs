//! Bounded, source-only collection. No build or language-server process is invoked.
use crate::{bindings::lines::parse_source, semantic_model::*, source_identity};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
};
pub mod compose;
mod manifests;
mod secure_fs;

pub const MAX_FILE_BYTES: usize = 4 * 1024 * 1024;
pub const MAX_FILES: usize = 10_000;
pub const MAX_TOTAL_BYTES: usize = 64 * 1024 * 1024;
#[derive(Debug)]
pub struct CollectedSource {
    pub snapshot: SourceSnapshot,
    pub contents: BTreeMap<String, String>,
    pub gaps: Vec<Gap>,
}
pub(super) fn failure(code: GapCode, reason: impl Into<String>) -> Gap {
    Gap {
        code,
        reason: reason.into(),
        location: None,
    }
}
fn valid_path(path: &str) -> bool {
    !path.is_empty()
        && !path.contains(['\\', '\0'])
        && path.as_bytes().get(1) != Some(&b':')
        && !path
            .split('/')
            .any(|p| p.is_empty() || p == "." || p == "..")
}
fn metadata_path(path: &str) -> bool {
    path.split('/')
        .any(|part| matches!(part, ".git" | ".hg" | ".svn"))
}
fn beneath(path: &str, parent: &str) -> bool {
    parent == "."
        || parent.is_empty()
        || path == parent
        || path
            .strip_prefix(parent)
            .is_some_and(|suffix| suffix.starts_with('/'))
}
fn parent(path: &str) -> String {
    path.rsplit_once('/')
        .map_or(".", |(parent, _)| parent)
        .into()
}
// Build configuration in these conventional containers applies to its owning
// project directory, even when selection names a sibling source directory.
fn configuration_directory_owner(path: &str) -> Option<String> {
    let name = path.rsplit('/').next()?;
    matches!(name, ".cargo" | ".mvn").then(|| parent(path))
}
fn lang(path: &str) -> Option<Language> {
    match path.rsplit('.').next() {
        Some("rs") => Some(Language::Rust),
        Some("go") => Some(Language::Go),
        Some("java") => Some(Language::Java),
        _ => None,
    }
}
fn located(mut gap: Gap, path: &str, _text: &str) -> Gap {
    gap.location = Some(SourceRange {
        path: path.into(),
        start_byte: 0,
        end_byte: 0,
        start_line: 0,
        start_column: 0,
        end_line: 0,
        end_column: 0,
    });
    gap
}
fn generated_header(tree: &tree_sitter::Tree, text: &str) -> bool {
    let mut pending = vec![tree.root_node()];
    while let Some(node) = pending.pop() {
        if node.start_position().row >= 10 {
            continue;
        }
        if matches!(node.kind(), "comment" | "line_comment" | "block_comment") {
            let comment = &text[node.byte_range()];
            if (comment.contains("Code generated") && comment.contains("DO NOT EDIT"))
                || comment.contains("@generated")
                || comment.to_ascii_lowercase().contains("auto-generated")
            {
                return true;
            }
            continue;
        }
        let mut cursor = node.walk();
        pending.extend(node.children(&mut cursor));
    }
    false
}
fn opaque_lines(text: &str) -> Vec<SourceLine> {
    let mut offset = 0;
    text.split_inclusive('\n')
        .enumerate()
        .map(|(number, line)| {
            let start = offset;
            offset += line.len();
            SourceLine {
                number: number as i64,
                start_byte: start as i64,
                end_byte: offset as i64,
                classification: if line.trim().is_empty() {
                    LineClass::Blank
                } else {
                    LineClass::Code
                },
            }
        })
        .collect()
}
fn normalize_reference(base: &str, relative: &str) -> Result<String, Gap> {
    if relative.starts_with('/')
        || relative.contains(['\\', '\0'])
        || relative.as_bytes().get(1) == Some(&b':')
    {
        return Err(failure(GapCode::InvalidFact, "manifest path escapes root"));
    }
    let mut parts: Vec<&str> = if base == "." {
        vec![]
    } else {
        base.split('/').collect()
    };
    for part in relative.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                if parts.pop().is_none() {
                    return Err(failure(GapCode::InvalidFact, "manifest path escapes root"));
                }
            }
            p => parts.push(p),
        }
    }
    if parts.is_empty() {
        return Err(failure(
            GapCode::InvalidFact,
            "manifest reference has no file",
        ));
    }
    let path = parts.join("/");
    if metadata_path(&path) {
        return Err(failure(
            GapCode::InvalidFact,
            "repository metadata cannot be selected",
        ));
    }
    Ok(path)
}
struct Provenance {
    repository: Option<git2::Repository>,
    prefix: PathBuf,
}
impl Provenance {
    fn open(root: &Path) -> Result<Self, Gap> {
        match git2::Repository::discover(root) {
            Ok(repository) => {
                let workdir = repository.workdir().ok_or_else(|| {
                    failure(
                        GapCode::PartialCollection,
                        "Git worktree provenance unavailable",
                    )
                })?;
                let absolute = std::fs::canonicalize(root).map_err(|_| {
                    failure(
                        GapCode::PartialCollection,
                        "Git root provenance unavailable",
                    )
                })?;
                let prefix = absolute
                    .strip_prefix(workdir)
                    .map_err(|_| {
                        failure(
                            GapCode::PartialCollection,
                            "Git root is outside its worktree",
                        )
                    })?
                    .to_owned();
                Ok(Self {
                    repository: Some(repository),
                    prefix,
                })
            }
            Err(error) if error.code() == git2::ErrorCode::NotFound => {
                if root
                    .ancestors()
                    .any(|p| p.join(".git").symlink_metadata().is_ok())
                {
                    return Err(failure(
                        GapCode::PartialCollection,
                        "Git repository provenance cannot be inspected",
                    ));
                }
                Ok(Self {
                    repository: None,
                    prefix: PathBuf::new(),
                })
            }
            Err(_) => Err(failure(
                GapCode::PartialCollection,
                "Git repository provenance cannot be inspected",
            )),
        }
    }
    fn origin(&self, path: &str, text: &str) -> Result<SourceOrigin, Gap> {
        let Some(repo) = &self.repository else {
            return Ok(SourceOrigin::Untracked);
        };
        let path = self.prefix.join(path);
        let index = repo
            .index()
            .map_err(|_| failure(GapCode::PartialCollection, "Git index cannot be inspected"))?;
        let entry = index.get_path(&path, 0);
        let head = match repo.head() {
            Ok(head) => Some(head.peel_to_tree().map_err(|_| {
                failure(
                    GapCode::PartialCollection,
                    "Git HEAD tree cannot be inspected",
                )
            })?),
            Err(e)
                if matches!(
                    e.code(),
                    git2::ErrorCode::UnbornBranch | git2::ErrorCode::NotFound
                ) =>
            {
                None
            }
            Err(_) => {
                return Err(failure(
                    GapCode::PartialCollection,
                    "Git HEAD cannot be inspected",
                ));
            }
        };
        let committed = head
            .as_ref()
            .and_then(|tree| tree.get_path(&path).ok())
            .map(|e| e.id());
        if committed.is_none() && entry.is_none() {
            return Ok(SourceOrigin::Untracked);
        }
        let digest =
            git2::Oid::hash_object(git2::ObjectType::Blob, text.as_bytes()).map_err(|_| {
                failure(
                    GapCode::PartialCollection,
                    "Git content identity cannot be inspected",
                )
            })?;
        Ok(
            if committed == Some(digest) && entry.as_ref().is_some_and(|e| Some(e.id) == committed)
            {
                SourceOrigin::TrackedClean
            } else {
                SourceOrigin::TrackedDirty
            },
        )
    }
}
struct Collector<'a> {
    request: &'a CollectionRequest,
    root: secure_fs::Root,
    provenance: Provenance,
    contents: BTreeMap<String, String>,
    stamps: BTreeMap<String, secure_fs::Stamp>,
    files: BTreeMap<String, SourceFile>,
    config: BTreeMap<String, SourceFile>,
    gaps: Vec<Gap>,
    bytes: usize,
    visited: usize,
}
impl Collector<'_> {
    fn excluded(&self, path: &str) -> bool {
        self.request
            .selection
            .exclude_paths
            .iter()
            .any(|e| beneath(path, e))
    }
    fn selected(&self, path: &str) -> bool {
        (self.request.selection.include_paths.is_empty()
            || self
                .request
                .selection
                .include_paths
                .iter()
                .any(|p| beneath(path, p)))
            && (self.request.configuration.modules.is_empty()
                || self
                    .request
                    .configuration
                    .modules
                    .iter()
                    .any(|m| beneath(path, m)))
    }
    fn intersects(&self, path: &str) -> bool {
        self.request.selection.include_paths.is_empty()
            || self
                .request
                .selection
                .include_paths
                .iter()
                .any(|p| beneath(path, p) || beneath(p, path))
    }
    fn config_relevant(&self, path: &str) -> bool {
        let dir = parent(path);
        let owner = configuration_directory_owner(&dir).unwrap_or(dir);
        self.config_scope_relevant(path, &owner)
    }
    fn config_directory_relevant(&self, path: &str) -> bool {
        configuration_directory_owner(path)
            .is_some_and(|owner| self.config_scope_relevant(path, &owner))
    }
    fn config_scope_relevant(&self, path: &str, dir: &str) -> bool {
        let include = self.request.selection.include_paths.is_empty()
            || self
                .request
                .selection
                .include_paths
                .iter()
                .any(|p| beneath(path, p) || beneath(p, dir));
        let modules = self.request.configuration.modules.is_empty()
            || self
                .request
                .configuration
                .modules
                .iter()
                .any(|m| beneath(path, m) || beneath(m, dir));
        include && modules
    }
    fn walk(&mut self, directory: &str) -> Result<(), Gap> {
        for name in self.root.entries(directory)? {
            let path = if directory.is_empty() {
                name.clone()
            } else {
                format!("{directory}/{name}")
            };
            if matches!(name.as_str(), ".git" | ".hg" | ".svn") || self.excluded(&path) {
                continue;
            }
            if matches!(
                name.as_str(),
                "target" | "node_modules" | "build" | ".gradle"
            ) && !self
                .request
                .selection
                .include_paths
                .iter()
                .any(|p| beneath(p, &path))
            {
                continue;
            }
            if !self.intersects(&path)
                && !manifests::recognized(&path)
                && !self.config_directory_relevant(&path)
            {
                continue;
            }
            self.visited += 1;
            if self.visited > 100_000 {
                return Err(failure(
                    GapCode::InvalidFact,
                    "source traversal exceeds 100000 entry limit",
                ));
            }
            let metadata = self.root.metadata(&path)?;
            if metadata.is_dir() {
                if self.intersects(&path) || self.config_directory_relevant(&path) {
                    self.walk(&path)?
                }
                continue;
            }
            if !metadata.is_file() {
                return Err(failure(
                    GapCode::InvalidFact,
                    format!("selected input is not a regular file: {path}"),
                ));
            }
            if (manifests::recognized(&path)
                || self
                    .request
                    .configuration
                    .configuration_files
                    .iter()
                    .any(|file| file.path == path))
                && self.config_relevant(&path)
            {
                self.retain(&path, None, true)?;
                continue;
            }
            if let Some(language) = lang(&path)
                && self.selected(&path)
                && (self.request.selection.languages.is_empty()
                    || self.request.selection.languages.contains(&language))
            {
                self.retain(&path, Some(language), false)?;
            }
        }
        Ok(())
    }
    fn retain(
        &mut self,
        path: &str,
        language: Option<Language>,
        configuration: bool,
    ) -> Result<(), Gap> {
        if self.contents.contains_key(path) {
            return Ok(());
        }
        let (text, stamp) = self.root.read(path, MAX_FILE_BYTES)?;
        let test = path.ends_with("_test.go")
            || path.split('/').any(|p| p == "tests")
            || path.contains("/src/test/")
            || path.starts_with("src/test/")
            || path.ends_with("Test.java")
            || path.ends_with("Tests.java");
        let parsed = language
            .map(|language| parse_source(language, path, &text))
            .transpose()?;
        let generated = parsed
            .as_ref()
            .is_some_and(|parsed| generated_header(&parsed.tree, &text));
        if !configuration
            && ((test && !self.request.selection.include_tests)
                || (generated && !self.request.selection.include_generated))
        {
            return Ok(());
        }
        if self.contents.len() >= MAX_FILES {
            return Err(failure(
                GapCode::InvalidFact,
                "selected set exceeds 10000 file limit",
            ));
        }
        if self.bytes + text.len() > MAX_TOTAL_BYTES {
            return Err(failure(
                GapCode::InvalidFact,
                "selected set exceeds 64 MiB limit",
            ));
        }
        let lines = if let Some(classified) = parsed {
            self.gaps.extend(classified.gaps);
            classified.lines
        } else {
            opaque_lines(&text)
        };
        let origin = self.provenance.origin(path, &text)?;
        let classification = if configuration {
            SourceClass::BuildConfiguration
        } else {
            match (test, generated) {
                (true, true) => SourceClass::GeneratedTest,
                (true, false) => SourceClass::Test,
                (false, true) => SourceClass::Generated,
                (false, false) => SourceClass::Production,
            }
        };
        let file = SourceFile {
            path: path.into(),
            content_sha256: Sha256::digest(text.as_bytes())
                .iter()
                .map(|b| format!("{b:02x}"))
                .collect(),
            byte_length: text.len() as i64,
            language,
            classification,
            origin,
            lines,
        };
        self.bytes += text.len();
        self.stamps.insert(path.into(), stamp);
        self.contents.insert(path.into(), text);
        if configuration {
            self.config.insert(path.into(), file);
        } else {
            self.files.insert(path.into(), file);
        }
        Ok(())
    }
    fn manifest_records(&mut self) -> Result<Vec<Manifest>, Gap> {
        let mut parsed = BTreeMap::new();
        let mut pending: Vec<String> = self.config.keys().cloned().collect();
        let mut seen = BTreeSet::new();
        let mut parents = BTreeMap::new();
        while let Some(path) = pending.pop() {
            if !seen.insert(path.clone()) {
                continue;
            }
            let Some(info) = manifests::parse(&path, &self.contents[&path]) else {
                continue;
            };
            for gap in &info.gaps {
                self.gaps
                    .push(located(gap.clone(), &path, &self.contents[&path]));
            }
            let dir = parent(&path);
            for child in &info.children {
                let mut child = normalize_reference(&dir, child)?;
                if child.ends_with("/build.gradle") && self.root.metadata(&child).is_err() {
                    let kotlin = format!("{child}.kts");
                    if self.root.metadata(&kotlin).is_ok() {
                        child = kotlin
                    }
                }
                if self.excluded(&child) {
                    self.gaps.push(located(
                        failure(
                            GapCode::MissingBuildSelection,
                            "declared module is excluded",
                        ),
                        &path,
                        &self.contents[&path],
                    ));
                    continue;
                }
                match self.root.metadata(&child) {
                    Ok(_) => {
                        self.retain(&child, None, true)?;
                        pending.push(child.clone());
                        if parent(&child) != dir
                            && let Some(existing) = parents.insert(child.clone(), dir.clone())
                            && existing != dir
                        {
                            self.gaps.push(located(
                                failure(
                                    GapCode::MissingBuildSelection,
                                    "module has conflicting declared parents",
                                ),
                                &path,
                                &self.contents[&path],
                            ));
                        }
                    }
                    Err(gap) if gap.code == GapCode::MissingBuildSelection => {
                        self.gaps.push(located(
                            failure(
                                GapCode::MissingBuildSelection,
                                format!("declared module is unavailable: {child}"),
                            ),
                            &path,
                            &self.contents[&path],
                        ))
                    }
                    Err(gap) => return Err(gap),
                }
            }
            if let Some(relative) = &info.parent {
                match normalize_reference(&dir, relative) {
                    Ok(p) => {
                        if self.excluded(&p) {
                            self.gaps.push(located(
                                failure(
                                    GapCode::MissingBuildSelection,
                                    "declared parent is excluded",
                                ),
                                &path,
                                &self.contents[&path],
                            ));
                        } else {
                            match self.root.metadata(&p) {
                                Ok(_) => {
                                    self.retain(&p, None, true)?;
                                    pending.push(p.clone());
                                    parents.insert(path.clone(), parent(&p));
                                }
                                Err(gap) if gap.code == GapCode::MissingBuildSelection => {
                                    self.gaps.push(located(
                                        failure(
                                            GapCode::MissingBuildSelection,
                                            "declared parent is unavailable",
                                        ),
                                        &path,
                                        &self.contents[&path],
                                    ))
                                }
                                Err(gap) => return Err(gap),
                            }
                        }
                    }
                    Err(_) => self.gaps.push(located(
                        failure(
                            GapCode::MissingBuildSelection,
                            "declared parent is outside selected root",
                        ),
                        &path,
                        &self.contents[&path],
                    )),
                }
            }
            // Declared names stay separate from path-based module identities; retain the
            // exact manifest in contents for later binding interpretation.
            let _declared_name = &info.declared_name;
            parsed.insert(path, info);
        }
        for module in &self.request.configuration.modules {
            if !parsed.keys().any(|path| parent(path) == *module) {
                self.gaps.push(failure(
                    GapCode::MissingBuildSelection,
                    format!("selected module is unavailable: {module}"),
                ));
            }
        }
        // A graph of declared module parents must not silently become cyclic.
        // Multiple manifest systems in one directory still share module identity.
        let module_parents: BTreeMap<_, _> = parents
            .iter()
            .map(|(path, p)| (parent(path), p.clone()))
            .collect();
        for (path, module_parent) in &parents {
            let module = parent(path);
            let mut seen = BTreeSet::from([module]);
            let mut current = Some(module_parent);
            while let Some(next) = current {
                if !seen.insert(next.clone()) {
                    self.gaps.push(located(
                        failure(
                            GapCode::MissingBuildSelection,
                            "cyclic declared module hierarchy",
                        ),
                        path,
                        &self.contents[path],
                    ));
                    break;
                }
                current = module_parents.get(next);
            }
        }
        Ok(parsed
            .into_iter()
            .map(|(path, info)| Manifest {
                content_sha256: self.config[&path].content_sha256.clone(),
                module: parent(&path),
                parent_module: parents.remove(&path),
                path,
                system: info.system,
            })
            .collect())
    }
}
/// Collect exact selected bytes and static build declarations without executing tools.
pub fn collect_source(request: &CollectionRequest) -> Result<CollectedSource, Vec<Gap>> {
    collect(request, || {}).map_err(|gap| vec![gap])
}
fn collect(
    request: &CollectionRequest,
    after_collection: impl FnOnce(),
) -> Result<CollectedSource, Gap> {
    for (paths, including) in [
        (&request.selection.include_paths, true),
        (&request.selection.exclude_paths, false),
    ] {
        let mut seen = BTreeSet::new();
        for path in paths {
            if !valid_path(path) {
                return Err(failure(
                    GapCode::InvalidFact,
                    format!("invalid selection path: {path}"),
                ));
            }
            if !seen.insert(path) {
                return Err(failure(
                    GapCode::InvalidFact,
                    format!("duplicate selection path: {path}"),
                ));
            }
            if including && metadata_path(path) {
                return Err(failure(
                    GapCode::InvalidFact,
                    "repository metadata cannot be selected",
                ));
            }
        }
    }
    if request
        .configuration
        .modules
        .iter()
        .any(|path| metadata_path(path))
        || request
            .configuration
            .configuration_files
            .iter()
            .any(|file| metadata_path(&file.path))
    {
        return Err(failure(
            GapCode::InvalidFact,
            "repository metadata cannot be selected",
        ));
    }
    let mut languages = BTreeSet::new();
    for language in &request.selection.languages {
        if !languages.insert(match language {
            Language::Go => 0,
            Language::Java => 1,
            Language::Rust => 2,
        }) {
            return Err(failure(GapCode::InvalidFact, "duplicate selected language"));
        }
    }
    if request.mode != CollectionMode::SourceOnly {
        return Err(failure(
            GapCode::UnsupportedCapability,
            "semantic source collection requires an implemented adapter",
        ));
    }
    if request.root.is_empty() {
        return Err(failure(GapCode::InvalidFact, "empty source root"));
    }
    if request.timeout_milliseconds <= 0 {
        return Err(failure(
            GapCode::InvalidFact,
            "collection timeout must be positive",
        ));
    }
    // Validate selected configuration before any IO; the identity is asserted after
    // discovered configuration descriptors have been collected from actual bytes.
    let _ = source_identity::configuration_id(&request.configuration)?;
    let root_path = Path::new(&request.root);
    let root = secure_fs::Root::open(root_path)?;
    let provenance = Provenance::open(root_path)?;
    let mut collector = Collector {
        request,
        root,
        provenance,
        contents: BTreeMap::new(),
        stamps: BTreeMap::new(),
        files: BTreeMap::new(),
        config: BTreeMap::new(),
        gaps: vec![],
        bytes: 0,
        visited: 0,
    };
    for include in &request.selection.include_paths {
        if !collector.excluded(include) {
            collector.root.metadata(include)?;
        }
    }
    collector.walk("")?;
    for expected in &request.configuration.configuration_files {
        if collector.excluded(&expected.path) {
            return Err(failure(
                GapCode::InvalidFact,
                "asserted configuration file is excluded",
            ));
        }
        collector.retain(&expected.path, None, true)?;
        let actual = collector.config.get(&expected.path).ok_or_else(|| {
            failure(
                GapCode::InvalidFact,
                "configuration path conflicts with selected source",
            )
        })?;
        if expected.content_sha256 != actual.content_sha256
            || expected.byte_length != actual.byte_length
        {
            return Err(failure(
                GapCode::IdentityMismatch,
                format!(
                    "configuration input does not match selected bytes: {}",
                    expected.path
                ),
            ));
        }
    }
    let manifests = collector.manifest_records()?;
    after_collection();
    for (path, stamp) in &collector.stamps {
        if collector.root.stamp(path).as_ref() != Ok(stamp) {
            return Err(failure(
                GapCode::SourceChangedDuringCollection,
                format!("selected input changed: {path}"),
            ));
        }
    }
    let mut configuration = request.configuration.clone();
    configuration.configuration_files = collector.config.into_values().collect();
    configuration.modules.sort();
    configuration.go_build_tags.sort();
    configuration.rust_features.sort();
    configuration.rust_cfg.sort();
    configuration.build_profiles.sort();
    let id = source_identity::configuration_id(&configuration)?;
    if !request.configuration.id.0.is_empty() && request.configuration.id != id {
        return Err(failure(
            GapCode::IdentityMismatch,
            "configuration identity does not match collected inputs",
        ));
    }
    configuration.id = id;
    let mut selection = request.selection.clone();
    selection.include_paths.sort();
    selection.exclude_paths.sort();
    selection.languages.sort_by_key(|l| match l {
        Language::Go => 0,
        Language::Java => 1,
        Language::Rust => 2,
    });
    let mut snapshot = SourceSnapshot {
        id: SnapshotId(String::new()),
        selection,
        configuration,
        files: collector.files.into_values().collect(),
        manifests,
    };
    snapshot.id = source_identity::snapshot_id(&snapshot)?;
    Ok(CollectedSource {
        snapshot,
        contents: collector.contents,
        gaps: collector.gaps,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mutation_after_read_refuses_the_actual_collection_pipeline() {
        let root = std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .join(format!("mutation-fixture-{}", std::process::id()));
        std::fs::create_dir(&root).unwrap();
        std::fs::write(root.join("a.rs"), "fn a(){}\n").unwrap();
        let configuration=crate::semantic_wire::decode_configuration(br#"{"id":"","modules":[],"go_build_tags":[],"rust_features":[],"rust_default_features":true,"rust_cfg":[],"build_profiles":[],"configuration_files":[]}"#).unwrap();
        let request = CollectionRequest {
            root: root.to_str().unwrap().into(),
            selection: SourceSelection {
                include_paths: vec![],
                exclude_paths: vec![],
                include_tests: true,
                include_generated: true,
                languages: vec![],
            },
            configuration,
            mode: CollectionMode::SourceOnly,
            timeout_milliseconds: 1000,
        };
        let changed = collect(&request, || {
            std::fs::write(root.join("a.rs"), "fn changed(){}\n").unwrap()
        })
        .unwrap_err();
        assert_eq!(changed.code, GapCode::SourceChangedDuringCollection);
        let removed = collect(&request, || {
            std::fs::remove_file(root.join("a.rs")).unwrap()
        })
        .unwrap_err();
        assert_eq!(removed.code, GapCode::SourceChangedDuringCollection);
    }
}
