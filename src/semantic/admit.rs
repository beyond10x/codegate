//! The sole constructor of admitted rich facts; no collection or IO dependencies.
use crate::{semantic_model::*, source_identity};
use std::collections::{BTreeMap, BTreeSet};

const MAX_RECORDS: usize = 100_000;
const MAX_TEXT: usize = 4096;
const MAX_FILE_BYTES: i64 = 4 * 1024 * 1024;
const MAX_SOURCE_BYTES: i64 = 64 * 1024 * 1024;
const FAMILIES: [FactFamily; 15] = [
    FactFamily::Sources,
    FactFamily::Declarations,
    FactFamily::Occurrences,
    FactFamily::Dependencies,
    FactFamily::References,
    FactFamily::Implementations,
    FactFamily::Calls,
    FactFamily::Decisions,
    FactFamily::Structure,
    FactFamily::Documentation,
    FactFamily::Effects,
    FactFamily::Tests,
    FactFamily::FrameworkDeclarations,
    FactFamily::FrameworkWiring,
    FactFamily::ExecutionCoverage,
];

pub(super) struct Admitted<'a> {
    snapshot: &'a FactSnapshot,
}
impl Admitted<'_> {
    pub(super) fn snapshot(&self) -> &FactSnapshot {
        self.snapshot
    }
}
fn refusal(code: GapCode, reason: impl Into<String>) -> Gap {
    Gap {
        code,
        reason: reason.into(),
        location: None,
    }
}
fn invalid(reason: impl Into<String>) -> Gap {
    refusal(GapCode::InvalidFact, reason)
}
fn require(ok: bool, reason: impl Into<String>) -> Result<(), Gap> {
    if ok { Ok(()) } else { Err(invalid(reason)) }
}
fn text(value: &str, required: bool, subject: &str) -> Result<(), Gap> {
    require(
        value.len() <= MAX_TEXT && (!required || !value.trim().is_empty()),
        format!("invalid text: {subject}"),
    )
}
fn digest(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
fn strings(values: &[String], required: bool, subject: &str) -> Result<(), Gap> {
    require(
        values.len() <= MAX_RECORDS,
        format!("too many values: {subject}"),
    )?;
    for v in values {
        text(v, required, subject)?;
    }
    Ok(())
}
fn optional(value: &Option<String>, subject: &str) -> Result<(), Gap> {
    if let Some(v) = value {
        text(v, false, subject)?;
    }
    Ok(())
}
fn path(value: &str) -> Result<(), Gap> {
    text(value, true, "path")?;
    require(
        !value.contains(['\\', '\0'])
            && value.as_bytes().get(1) != Some(&b':')
            && value
                .split('/')
                .all(|s| !s.is_empty() && s != "." && s != ".."),
        "invalid relative path",
    )
}
fn count(total: &mut usize, n: usize) -> Result<(), Gap> {
    *total = total
        .checked_add(n)
        .ok_or_else(|| invalid("record count overflow"))?;
    require(*total <= MAX_RECORDS, "record limit exceeded")
}
fn ids<'a>(
    values: impl IntoIterator<Item = &'a str>,
    subject: &str,
) -> Result<BTreeSet<&'a str>, Gap> {
    let mut set = BTreeSet::new();
    for id in values {
        text(id, true, subject)?;
        require(set.insert(id), format!("duplicate {subject}: {id}"))?;
    }
    Ok(set)
}
fn cycles(parents: &BTreeMap<&str, Option<&str>>, subject: &str) -> Result<(), Gap> {
    let mut complete = BTreeSet::new();
    for start in parents.keys() {
        let mut path = BTreeSet::new();
        let mut current = Some(*start);
        while let Some(id) = current {
            if complete.contains(id) {
                break;
            }
            require(path.insert(id), format!("containment cycle: {subject}"))?;
            current = parents.get(id).copied().flatten();
        }
        complete.extend(path);
    }
    Ok(())
}
fn selected_path(path: &str, roots: &BTreeSet<&str>) -> bool {
    roots.contains(path)
        || path
            .match_indices('/')
            .any(|(at, _)| roots.contains(&path[..at]))
}

fn resources(s: &FactSnapshot) -> Result<(), Gap> {
    let mut n = 0;
    for len in [
        s.units.len(),
        s.declarations.len(),
        s.occurrences.len(),
        s.dependencies.len(),
        s.references.len(),
        s.calls.len(),
        s.implementations.len(),
        s.decisions.len(),
        s.structure.len(),
        s.effects.len(),
        s.framework.len(),
        s.tools.len(),
        s.coverage.len(),
    ] {
        count(&mut n, len)?;
    }
    let mut members = 0;
    for c in &s.coverage {
        count(&mut members, c.unit_ids.len())?;
        count(&mut n, c.gaps.len())?;
    }
    for r in &s.references {
        count(&mut n, r.candidates.len())?;
    }
    for c in &s.calls {
        count(&mut n, c.candidates.len())?;
    }
    for i in &s.implementations {
        count(&mut n, i.candidates.len())?;
    }
    for f in &s.framework {
        count(&mut n, f.qualifiers.len())?;
    }
    let evidence = evidences(s);
    for e in evidence {
        count(&mut n, e.gaps.len())?;
    }
    require(
        s.source.files.len() <= 10_000
            && s.source.configuration.configuration_files.len() <= 10_000
            && s.source.manifests.len() <= 10_000,
        "source file limit exceeded",
    )?;
    for f in s
        .source
        .files
        .iter()
        .chain(&s.source.configuration.configuration_files)
    {
        count(&mut n, f.lines.len())?;
    }
    Ok(())
}
fn evidences(s: &FactSnapshot) -> Vec<&Evidence> {
    s.units
        .iter()
        .map(|x| &x.evidence)
        .chain(s.declarations.iter().map(|x| &x.evidence))
        .chain(s.occurrences.iter().map(|x| &x.evidence))
        .chain(s.dependencies.iter().map(|x| &x.evidence))
        .chain(s.references.iter().map(|x| &x.evidence))
        .chain(s.calls.iter().map(|x| &x.evidence))
        .chain(s.implementations.iter().map(|x| &x.evidence))
        .chain(s.decisions.iter().map(|x| &x.evidence))
        .chain(s.structure.iter().map(|x| &x.evidence))
        .chain(s.effects.iter().map(|x| &x.evidence))
        .chain(s.framework.iter().map(|x| &x.evidence))
        .collect()
}
fn sources(s: &FactSnapshot) -> Result<BTreeMap<&str, &SourceFile>, Gap> {
    let mut files = BTreeMap::new();
    let mut bytes = 0_i64;
    for list in [&s.source.files, &s.source.configuration.configuration_files] {
        let mut seen = BTreeSet::new();
        for f in list {
            path(&f.path)?;
            require(seen.insert(f.path.as_str()), "duplicate source file path")?;
            require(digest(&f.content_sha256), "invalid file content digest")?;
            require(
                (0..=MAX_FILE_BYTES).contains(&f.byte_length),
                "source file byte limit exceeded",
            )?;
            let mut offset = 0;
            for (number, line) in f.lines.iter().enumerate() {
                require(
                    line.number == number as i64
                        && line.start_byte == offset
                        && line.end_byte > offset
                        && line.end_byte <= f.byte_length,
                    "invalid source line partition",
                )?;
                offset = line.end_byte;
            }
            require(offset == f.byte_length, "invalid source line partition")?;
            if let Some(prior) = files.insert(f.path.as_str(), f) {
                require(prior == f, "conflicting configuration/source file metadata")?;
            } else {
                bytes = bytes
                    .checked_add(f.byte_length)
                    .ok_or_else(|| invalid("source byte overflow"))?;
            }
        }
    }
    require(
        files.len() <= 10_000 && bytes <= MAX_SOURCE_BYTES,
        "source set limit exceeded",
    )?;
    let selection = &s.source.selection;
    strings(&selection.include_paths, true, "include paths")?;
    strings(&selection.exclude_paths, true, "exclude paths")?;
    let includes: BTreeSet<_> = selection.include_paths.iter().map(String::as_str).collect();
    let excludes: BTreeSet<_> = selection.exclude_paths.iter().map(String::as_str).collect();
    for file in &s.source.files {
        if file.classification != SourceClass::BuildConfiguration {
            require(
                (includes.is_empty() || selected_path(&file.path, &includes))
                    && !selected_path(&file.path, &excludes),
                "source file outside selected paths",
            )?;
            require(
                file.language.is_some()
                    && (selection.languages.is_empty()
                        || file
                            .language
                            .is_some_and(|language| selection.languages.contains(&language))),
                "source file outside selected languages",
            )?;
            require(
                selection.include_tests
                    || !matches!(
                        file.classification,
                        SourceClass::Test | SourceClass::GeneratedTest
                    ),
                "test source excluded by selection",
            )?;
            require(
                selection.include_generated
                    || !matches!(
                        file.classification,
                        SourceClass::Generated | SourceClass::GeneratedTest
                    ),
                "generated source excluded by selection",
            )?;
        }
    }
    let config = &s.source.configuration;
    for (v, label) in [
        (&config.modules, "modules"),
        (&config.go_build_tags, "build tags"),
        (&config.rust_features, "features"),
        (&config.rust_cfg, "cfg"),
        (&config.build_profiles, "profiles"),
    ] {
        strings(v, true, label)?;
    }
    for (v, label) in [
        (&config.target, "target"),
        (&config.java_target_version, "Java target"),
        (&config.java_classpath_sha256, "classpath"),
    ] {
        optional(v, label)?;
    }
    if let Some(hash) = &config.java_classpath_sha256 {
        require(digest(hash), "invalid classpath digest")?;
    }
    let mut manifests = BTreeSet::new();
    let mut module_parents = BTreeMap::new();
    for m in &s.source.manifests {
        path(&m.path)?;
        text(&m.module, true, "manifest module")?;
        optional(&m.parent_module, "manifest parent")?;
        require(
            manifests.insert(m.path.as_str()) && digest(&m.content_sha256),
            "invalid manifest identity",
        )?;
        if let Some(f) = files.get(m.path.as_str()) {
            require(
                f.content_sha256 == m.content_sha256,
                "manifest content mismatch",
            )?;
        }
        let parent = m.parent_module.as_deref();
        if let Some(previous) = module_parents.insert(m.module.as_str(), parent) {
            require(previous == parent, "conflicting manifest parent")?;
        }
    }
    cycles(&module_parents, "manifest")?;
    let expected = source_identity::configuration_id(config)?;
    if config.id != expected {
        return Err(refusal(
            GapCode::IdentityMismatch,
            "configuration identity mismatch",
        ));
    }
    if s.source.id != source_identity::snapshot_id(&s.source)? {
        return Err(refusal(
            GapCode::IdentityMismatch,
            "snapshot identity mismatch",
        ));
    }
    Ok(files)
}
struct Context<'a> {
    s: &'a FactSnapshot,
    files: BTreeMap<&'a str, &'a SourceFile>,
    units: BTreeSet<&'a str>,
    declarations: BTreeMap<&'a str, &'a Declaration>,
}
impl Context<'_> {
    fn range(&self, r: &SourceRange) -> Result<(), Gap> {
        let f = self
            .files
            .get(r.path.as_str())
            .ok_or_else(|| invalid("range outside selected files"))?;
        require(
            r.start_byte >= 0 && r.end_byte >= r.start_byte && r.end_byte <= f.byte_length,
            "invalid source range bytes",
        )?;
        for (byte, line, column) in [
            (r.start_byte, r.start_line, r.start_column),
            (r.end_byte, r.end_line, r.end_column),
        ] {
            require(line >= 0 && column >= 0, "invalid source range coordinate")?;
            if byte == f.byte_length && line == f.lines.len() as i64 && column == 0 {
                continue;
            }
            let row = usize::try_from(line)
                .ok()
                .and_then(|i| f.lines.get(i))
                .ok_or_else(|| invalid("invalid source range line"))?;
            require(
                row.start_byte.checked_add(column) == Some(byte)
                    && byte >= row.start_byte
                    && byte <= row.end_byte,
                "invalid source range column",
            )?;
            require(
                byte < row.end_byte || byte == f.byte_length,
                "noncanonical interior line endpoint",
            )?;
        }
        Ok(())
    }
    fn gaps(&self, gaps: &[Gap]) -> Result<(), Gap> {
        for g in gaps {
            text(&g.reason, true, "gap reason")?;
            if let Some(r) = &g.location {
                self.range(r)?;
            }
        }
        Ok(())
    }
    fn evidence(&self, e: &Evidence) -> Result<(), Gap> {
        text(&e.id.0, true, "evidence id")?;
        text(&e.producer, true, "producer")?;
        text(&e.producer_version, true, "producer version")?;
        if e.snapshot_id != self.s.source.id || e.configuration_id != self.s.source.configuration.id
        {
            return Err(refusal(
                GapCode::StaleEvidence,
                format!("stale evidence: {}", e.id.0),
            ));
        }
        if let Some(r) = &e.location {
            self.range(r)?;
        }
        self.gaps(&e.gaps)
    }
    fn declaration(&self, id: &DeclarationId, subject: &str) -> Result<&Declaration, Gap> {
        text(&id.0, true, subject)?;
        self.declarations
            .get(id.0.as_str())
            .copied()
            .ok_or_else(|| invalid(format!("unknown {subject}: {}", id.0)))
    }
    fn contained(&self, parent: &Evidence, child: &Evidence) -> Result<(), Gap> {
        if let (Some(p), Some(c)) = (&parent.location, &child.location)
            && p.path == c.path
        {
            require(
                p.start_byte <= c.start_byte && c.end_byte <= p.end_byte,
                "inconsistent nested source ranges",
            )?;
        }
        Ok(())
    }
    fn relationship(
        &self,
        target: Option<&DeclarationId>,
        candidates: &[DeclarationId],
        e: &Evidence,
    ) -> Result<(), Gap> {
        ids(candidates.iter().map(|x| x.0.as_str()), "candidate")?;
        for c in candidates {
            self.declaration(c, "candidate")?;
        }
        if let Some(t) = target {
            self.declaration(t, "target")?;
        }
        let valid = match e.resolution {
            Resolution::Resolved => target.is_some() && candidates.is_empty(),
            Resolution::SyntacticCandidate | Resolution::Unresolved => {
                target.is_none() && !e.gaps.is_empty()
            }
        };
        require(valid, "invalid relationship resolution")
    }
}
fn structure_and_relationships(c: &Context<'_>) -> Result<(), Gap> {
    let s = c.s;
    for u in &s.units {
        text(&u.name, true, "unit name")?;
        strings(&u.paths, true, "unit paths")?;
        ids(u.paths.iter().map(String::as_str), "unit path")?;
        for p in &u.paths {
            require(
                c.files.contains_key(p.as_str()),
                "unit path outside selected files",
            )?;
        }
    }
    let mut parents = BTreeMap::new();
    for d in &s.declarations {
        text(&d.unit_id.0, true, "declaration unit")?;
        require(
            c.units.contains(d.unit_id.0.as_str()),
            format!("unknown declaration unit: {}", d.unit_id.0),
        )?;
        text(&d.name, true, "declaration name")?;
        text(&d.qualified_name, true, "qualified name")?;
        optional(&d.signature, "signature")?;
        require(
            d.evidence.basis == ResolutionBasis::SourceStructure,
            "declaration requires source structure",
        )?;
        if let Some(p) = &d.parent {
            let parent = c.declaration(p, "declaration parent")?;
            c.contained(&parent.evidence, &d.evidence)?;
            if let (Some(p), Some(child)) = (&parent.evidence.location, &d.evidence.location) {
                require(
                    p.path == child.path
                        || matches!(
                            parent.kind,
                            DeclarationKind::Package
                                | DeclarationKind::Module
                                | DeclarationKind::Namespace
                        ),
                    "cross-file lexical parent",
                )?;
            }
        }
        parents.insert(d.id.0.as_str(), d.parent.as_ref().map(|id| id.0.as_str()));
    }
    cycles(&parents, "declaration")?;
    let occurrences: BTreeMap<_, _> = s
        .occurrences
        .iter()
        .map(|o| (o.evidence.id.0.as_str(), o))
        .collect();
    for o in &s.occurrences {
        text(&o.spelling, true, "occurrence spelling")?;
        if let Some(owner) = &o.owner {
            let d = c.declaration(owner, "occurrence owner")?;
            c.contained(&d.evidence, &o.evidence)?;
        }
    }
    for r in &s.references {
        text(&r.occurrence_id.0, true, "reference occurrence")?;
        require(
            occurrences.contains_key(r.occurrence_id.0.as_str()),
            format!("unknown reference occurrence: {}", r.occurrence_id.0),
        )?;
        c.relationship(r.target.as_ref(), &r.candidates, &r.evidence)?;
    }
    for call in &s.calls {
        c.declaration(&call.caller, "caller")?;
        c.relationship(call.callee.as_ref(), &call.candidates, &call.evidence)?;
    }
    for i in &s.implementations {
        c.declaration(&i.declaration, "implemented declaration")?;
        c.relationship(i.implementation.as_ref(), &i.candidates, &i.evidence)?;
    }
    for d in &s.dependencies {
        text(&d.source.0, true, "dependency source")?;
        require(
            c.units.contains(d.source.0.as_str()),
            format!("unknown dependency source: {}", d.source.0),
        )?;
        text(&d.target_name, true, "dependency target name")?;
        if let Some(t) = &d.target {
            text(&t.0, true, "dependency target")?;
            require(
                c.units.contains(t.0.as_str()),
                format!("unknown dependency target: {}", t.0),
            )?;
        }
        require(
            match d.evidence.resolution {
                Resolution::Resolved => d.target.is_some(),
                Resolution::Unresolved | Resolution::SyntacticCandidate => {
                    d.target.is_none() && !d.evidence.gaps.is_empty()
                }
            },
            "invalid dependency resolution",
        )?;
    }
    for d in &s.decisions {
        let owner = c.declaration(&d.owner, "decision owner")?;
        c.contained(&owner.evidence, &d.evidence)?;
    }
    let structure: BTreeMap<_, _> = s
        .structure
        .iter()
        .map(|o| (o.evidence.id.0.as_str(), o))
        .collect();
    let mut parents = BTreeMap::new();
    for o in &s.structure {
        let owner = c.declaration(&o.owner, "structure owner")?;
        c.contained(&owner.evidence, &o.evidence)?;
        require(
            o.evidence.basis == ResolutionBasis::SourceStructure,
            "structure requires source structure",
        )?;
        require(
            (o.kind == StructureKind::TestCase) == o.test_kind.is_some(),
            "invalid structure test kind",
        )?;
        optional(&o.text, "structure text")?;
        if let Some(p) = &o.parent_observation {
            let p = structure
                .get(p.0.as_str())
                .ok_or_else(|| invalid("unknown structure parent"))?;
            require(p.owner == o.owner, "structure parent owner mismatch")?;
            c.contained(&p.evidence, &o.evidence)?;
        }
        parents.insert(
            o.evidence.id.0.as_str(),
            o.parent_observation.as_ref().map(|x| x.0.as_str()),
        );
    }
    cycles(&parents, "structure")?;
    for e in &s.effects {
        c.declaration(&e.owner, "effect owner")?;
        if let Some(d) = &e.related_declaration {
            c.declaration(d, "related declaration")?;
        }
    }
    for f in &s.framework {
        c.declaration(&f.declaration, "framework declaration")?;
        if let Some(t) = &f.target {
            c.declaration(t, "framework target")?;
        }
        text(&f.annotation_name, true, "annotation name")?;
        for q in &f.qualifiers {
            text(&q.annotation_name, true, "qualifier annotation")?;
            text(&q.member_name, false, "qualifier member")?;
            text(&q.member_value, false, "qualifier value")?;
            require(
                !q.member_name.is_empty() || q.member_value.is_empty(),
                "invalid marker qualifier",
            )?;
        }
        for (v, label) in [
            (&f.scope, "scope"),
            (&f.produced_type, "produced type"),
            (&f.http_method, "HTTP method"),
            (&f.resource_path, "resource path"),
            (&f.method_path, "method path"),
            (&f.transaction_mode, "transaction mode"),
            (&f.configuration_key, "configuration key"),
            (&f.configuration_profile, "configuration profile"),
        ] {
            optional(v, label)?;
        }
        let present = |v: &Option<String>| v.as_ref().is_some_and(|v| !v.trim().is_empty());
        let valid = match f.kind {
            FrameworkKind::Bean => present(&f.scope),
            FrameworkKind::Producer => present(&f.produced_type),
            FrameworkKind::RestResource => f.resource_path.is_some(),
            FrameworkKind::RestRoute => {
                f.resource_path.is_some() && f.method_path.is_some() && present(&f.http_method)
            }
            FrameworkKind::TransactionDeclaration => present(&f.transaction_mode),
            FrameworkKind::ConfigurationReference => present(&f.configuration_key),
            FrameworkKind::InjectionBinding => {
                f.target.is_some()
                    && f.evidence.resolution == Resolution::Resolved
                    && f.evidence.basis == ResolutionBasis::MatchedBuildEvidence
            }
            FrameworkKind::InjectionCandidate => f.evidence.resolution != Resolution::Resolved,
            FrameworkKind::Qualifier | FrameworkKind::InjectionPoint => true,
        };
        require(valid, "invalid framework payload")?;
    }
    Ok(())
}

fn family_evidence<'a>(
    c: &Context<'a>,
    family: FactFamily,
) -> Vec<(Option<&'a str>, &'a Evidence, bool)> {
    let unit = |id: &DeclarationId| {
        c.declarations
            .get(id.0.as_str())
            .map(|d| d.unit_id.0.as_str())
    };
    let s = c.s;
    match family {
        FactFamily::Sources => s
            .units
            .iter()
            .map(|v| (Some(v.id.0.as_str()), &v.evidence, false))
            .collect(),
        FactFamily::Declarations => s
            .declarations
            .iter()
            .map(|v| (Some(v.unit_id.0.as_str()), &v.evidence, false))
            .collect(),
        FactFamily::Occurrences => s
            .occurrences
            .iter()
            .map(|v| (v.owner.as_ref().and_then(unit), &v.evidence, false))
            .collect(),
        FactFamily::Dependencies => s
            .dependencies
            .iter()
            .map(|v| (Some(v.source.0.as_str()), &v.evidence, true))
            .collect(),
        FactFamily::References => {
            let occurrences: BTreeMap<_, _> = s
                .occurrences
                .iter()
                .map(|o| (o.evidence.id.0.as_str(), o))
                .collect();
            s.references
                .iter()
                .map(|v| {
                    (
                        occurrences
                            .get(v.occurrence_id.0.as_str())
                            .and_then(|o| o.owner.as_ref())
                            .and_then(unit),
                        &v.evidence,
                        true,
                    )
                })
                .collect()
        }
        FactFamily::Calls => s
            .calls
            .iter()
            .map(|v| (unit(&v.caller), &v.evidence, true))
            .collect(),
        FactFamily::Implementations => s
            .implementations
            .iter()
            .map(|v| (unit(&v.declaration), &v.evidence, true))
            .collect(),
        FactFamily::Decisions => s
            .decisions
            .iter()
            .map(|v| (unit(&v.owner), &v.evidence, false))
            .collect(),
        FactFamily::Structure => s
            .structure
            .iter()
            .map(|v| (unit(&v.owner), &v.evidence, false))
            .collect(),
        FactFamily::Documentation => s
            .structure
            .iter()
            .filter(|v| {
                matches!(
                    v.kind,
                    StructureKind::Documentation | StructureKind::DebtMarker
                )
            })
            .map(|v| (unit(&v.owner), &v.evidence, false))
            .collect(),
        FactFamily::Tests => s
            .structure
            .iter()
            .filter(|v| v.kind == StructureKind::TestCase)
            .map(|v| (unit(&v.owner), &v.evidence, false))
            .collect(),
        FactFamily::Effects => s
            .effects
            .iter()
            .map(|v| (unit(&v.owner), &v.evidence, false))
            .collect(),
        FactFamily::FrameworkDeclarations => s
            .framework
            .iter()
            .filter(|v| v.kind != FrameworkKind::InjectionBinding)
            .map(|v| (unit(&v.declaration), &v.evidence, false))
            .collect(),
        FactFamily::FrameworkWiring => s
            .framework
            .iter()
            .filter(|v| v.kind == FrameworkKind::InjectionBinding)
            .map(|v| (unit(&v.declaration), &v.evidence, true))
            .collect(),
        FactFamily::ExecutionCoverage => vec![],
    }
}
fn coverage(c: &Context<'_>) -> Result<(), Gap> {
    let s = c.s;
    for family in FAMILIES {
        let mut visited = BTreeSet::new();
        let mut empty = false;
        let facts = family_evidence(c, family);
        let mut present = BTreeSet::new();
        let mut incomplete = BTreeSet::new();
        let mut global_present = false;
        let mut global_incomplete = false;
        for (owner, e, resolution) in facts {
            let bad = !e.gaps.is_empty() || resolution && e.resolution != Resolution::Resolved;
            if let Some(owner) = owner {
                present.insert(owner);
                if bad {
                    incomplete.insert(owner);
                }
            } else {
                global_present = true;
                global_incomplete |= bad;
            }
        }
        for coverage in s.coverage.iter().filter(|v| v.family == family) {
            if coverage.configuration_id != s.source.configuration.id {
                return Err(refusal(
                    GapCode::StaleEvidence,
                    format!("stale coverage: {family:?}"),
                ));
            }
            let scope = ids(
                coverage.unit_ids.iter().map(|x| x.0.as_str()),
                "coverage unit",
            )?;
            if c.units.is_empty() {
                require(scope.is_empty() && !empty, "overlapping empty coverage")?;
                empty = true;
            } else {
                require(
                    !scope.is_empty(),
                    "empty coverage for nonempty unit population",
                )?;
            }
            for id in &scope {
                require(c.units.contains(id), format!("unknown coverage unit: {id}"))?;
                require(
                    visited.insert(*id),
                    format!("overlapping coverage: {family:?}:{id}"),
                )?;
            }
            c.gaps(&coverage.gaps)?;
            optional(&coverage.applicability_reason, "applicability reason")?;
            match coverage.status {
                Completeness::Complete => {
                    require(coverage.gaps.is_empty(), "complete coverage has gaps")?;
                    require(
                        !(s.mode == CollectionMode::SourceOnly
                            && matches!(
                                family,
                                FactFamily::Calls
                                    | FactFamily::References
                                    | FactFamily::FrameworkWiring
                            )),
                        "source-only semantic coverage cannot be complete",
                    )?;
                    require(
                        family != FactFamily::ExecutionCoverage,
                        "execution coverage has no admitted measurement contract",
                    )?;
                    if matches!(
                        family,
                        FactFamily::References
                            | FactFamily::Calls
                            | FactFamily::Implementations
                            | FactFamily::FrameworkWiring
                    ) {
                        require(
                            !s.tools.iter().any(|tool| {
                                tool.timed_out || tool.exit_code.is_some_and(|code| code != 0)
                            }),
                            "complete semantic coverage contradicts tool failure",
                        )?;
                    }
                    require(
                        !global_incomplete && !scope.iter().any(|id| incomplete.contains(id)),
                        format!("complete coverage contradicts evidence: {family:?}"),
                    )?;
                }
                Completeness::Partial | Completeness::Failed | Completeness::Unsupported => {
                    require(!coverage.gaps.is_empty(), "incomplete coverage needs gaps")?
                }
                Completeness::NotApplicable => {
                    require(
                        coverage
                            .applicability_reason
                            .as_ref()
                            .is_some_and(|r| !r.trim().is_empty()),
                        "not-applicable coverage needs semantic reason",
                    )?;
                    require(
                        !coverage.gaps.iter().any(|g| {
                            matches!(
                                g.code,
                                GapCode::MissingTool
                                    | GapCode::ToolFailure
                                    | GapCode::ToolTimeout
                                    | GapCode::UnsupportedCapability
                            )
                        }),
                        "missing capability is not non-applicability",
                    )?;
                    require(
                        !global_present && !scope.iter().any(|id| present.contains(id)),
                        "not-applicable coverage contradicts facts",
                    )?;
                }
            }
        }
        if c.units.is_empty() {
            require(empty, format!("missing coverage: {family:?}:<empty>"))?;
        } else {
            for id in &c.units {
                require(
                    visited.contains(id),
                    format!("missing coverage: {family:?}:{id}"),
                )?;
            }
        }
    }
    Ok(())
}
fn validate(s: &FactSnapshot) -> Result<(), Gap> {
    if s.format != "codegate.semantic-facts/1" {
        return Err(refusal(
            GapCode::InvalidFormat,
            "unsupported semantic snapshot format",
        ));
    }
    resources(s)?;
    let files = sources(s)?;
    let units = ids(s.units.iter().map(|u| u.id.0.as_str()), "unit")?;
    ids(
        s.declarations.iter().map(|d| d.id.0.as_str()),
        "declaration",
    )?;
    let declarations = s
        .declarations
        .iter()
        .map(|d| (d.id.0.as_str(), d))
        .collect();
    let context = Context {
        s,
        files,
        units,
        declarations,
    };
    let evidence = evidences(s);
    ids(evidence.iter().map(|e| e.id.0.as_str()), "fact")?;
    for tool in &s.tools {
        text(&tool.tool, true, "tool name")?;
        text(&tool.version, true, "tool version")?;
        optional(&tool.host_runtime_version, "host runtime")?;
        strings(&tool.diagnostics, false, "tool diagnostics")?;
        require(tool.elapsed_milliseconds >= 0, "negative tool duration")?;
        if tool.snapshot_id != s.source.id || tool.configuration_id != s.source.configuration.id {
            return Err(refusal(
                GapCode::StaleEvidence,
                format!("stale tool evidence: {}", tool.tool),
            ));
        }
    }
    for e in evidence {
        context.evidence(e)?;
    }
    structure_and_relationships(&context)?;
    coverage(&context)
}
pub(super) fn admit(snapshot: &FactSnapshot) -> Result<Admitted<'_>, Vec<Gap>> {
    validate(snapshot).map_err(|gap| vec![gap])?;
    Ok(Admitted { snapshot })
}
