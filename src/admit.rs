//! Admission is the only constructor of the private validated graph.
use crate::{
    model::*,
    wire::{MAX_EDGES, MAX_UNITS},
};
use std::collections::BTreeSet;
pub(crate) struct Admitted<'a> {
    snapshot: &'a FactSnapshot,
    policy: &'a Policy,
}
impl Admitted<'_> {
    pub(crate) fn snapshot(&self) -> &FactSnapshot {
        self.snapshot
    }
    pub(crate) fn policy(&self) -> &Policy {
        self.policy
    }
}
fn diag(code: DiagnosticCode, subject: impl Into<String>) -> Diagnostic {
    Diagnostic {
        code,
        subject: subject.into(),
    }
}
fn finish(mut diagnostics: Vec<Diagnostic>) -> Result<(), Vec<Diagnostic>> {
    diagnostics.sort_by_key(|d| (format!("{:?}", d.code), d.subject.clone()));
    diagnostics.dedup();
    if diagnostics.is_empty() {
        Ok(())
    } else {
        Err(diagnostics)
    }
}
pub(crate) fn admit(input: &Evaluate) -> Result<Admitted<'_>, Vec<Diagnostic>> {
    let s = &input.snapshot;
    let p = &input.policy;
    let mut ds = vec![];
    if s.format != "codegate-dependency-facts/0.1" {
        ds.push(diag(DiagnosticCode::InvalidFormat, "snapshot.format"));
    }
    if p.format != "codegate-dependency-policy/0.1" {
        ds.push(diag(DiagnosticCode::InvalidFormat, "policy.format"));
    }
    finish(ds)?;
    let mut ds = vec![];
    for (v, path) in [
        (&s.source_id, "snapshot.source_id"),
        (&s.configuration_id, "snapshot.configuration_id"),
        (&s.producer, "snapshot.producer"),
        (&s.producer_version, "snapshot.producer_version"),
    ] {
        if v.is_empty() {
            ds.push(diag(DiagnosticCode::EmptyIdentity, path));
        }
    }
    for u in &s.units {
        if u.id.0.is_empty() {
            ds.push(diag(
                DiagnosticCode::EmptyIdentity,
                format!("unit:{}", u.id.0),
            ));
        }
    }
    for e in &s.edges {
        if e.id.0.is_empty() {
            ds.push(diag(
                DiagnosticCode::EmptyIdentity,
                format!("edge:{}", e.id.0),
            ));
        }
    }
    finish(ds)?;
    let mut ds = vec![];
    let mut units = BTreeSet::new();
    let mut edges = BTreeSet::new();
    for u in &s.units {
        if !units.insert(u.id.0.as_str()) {
            ds.push(diag(
                DiagnosticCode::DuplicateUnit,
                format!("unit:{}", u.id.0),
            ));
        }
    }
    for e in &s.edges {
        if !edges.insert(e.id.0.as_str()) {
            ds.push(diag(
                DiagnosticCode::DuplicateEdge,
                format!("edge:{}", e.id.0),
            ));
        }
    }
    finish(ds)?;
    let mut ds = vec![];
    for e in &s.edges {
        let subject = format!("edge:{}", e.id.0);
        if !units.contains(e.source.0.as_str()) {
            ds.push(diag(DiagnosticCode::DanglingSource, &subject));
        }
        let valid_target = matches!((&e.target,&e.unresolved_target),(Some(t),None) if !t.0.is_empty())
            || matches!((&e.target,&e.unresolved_target),(None,Some(t)) if !t.is_empty());
        if !valid_target {
            ds.push(diag(DiagnosticCode::InvalidTarget, &subject));
        }
        if let Some(t) = &e.target
            && !t.0.is_empty()
            && !units.contains(t.0.as_str())
        {
            ds.push(diag(DiagnosticCode::DanglingTarget, &subject));
        }
    }
    finish(ds)?;
    let c = &s.coverage;
    let valid_coverage = !c.gaps.iter().any(String::is_empty)
        && match c.status {
            CoverageStatus::Complete => {
                c.gaps.is_empty() && s.edges.iter().all(|e| e.unresolved_target.is_none())
            }
            CoverageStatus::Partial => !c.gaps.is_empty(),
            CoverageStatus::Unsupported | CoverageStatus::Failed => {
                !c.gaps.is_empty() && s.edges.is_empty()
            }
        };
    // The decoder enforces counts before conversion; typed callers are bounded here too.
    let mut ds = vec![];
    if !valid_coverage {
        ds.push(diag(DiagnosticCode::InvalidCoverage, "coverage"));
    }
    if s.units.len() > MAX_UNITS {
        ds.push(diag(
            DiagnosticCode::InvalidCoverage,
            "snapshot.units limit 10000",
        ));
    }
    if s.edges.len() > MAX_EDGES {
        ds.push(diag(
            DiagnosticCode::InvalidCoverage,
            "snapshot.edges limit 50000",
        ));
    }
    finish(ds)?;
    let mut rules = BTreeSet::new();
    let mut ds = vec![];
    if p.expected_source_id.is_empty()
        || p.expected_configuration_id.is_empty()
        || p.forbidden.iter().any(|r| {
            !units.contains(r.source.0.as_str())
                || !units.contains(r.target.0.as_str())
                || r.kind != p.dependency_kind
                || !rules.insert((
                    r.source.0.as_str(),
                    r.target.0.as_str(),
                    format!("{:?}", r.kind),
                ))
        })
    {
        ds.push(diag(DiagnosticCode::InvalidPolicy, "policy.forbidden"));
    }
    finish(ds)?;
    let mut ds = vec![];
    if p.expected_source_id != s.source_id {
        ds.push(diag(DiagnosticCode::SourceMismatch, "snapshot.source_id"));
    }
    if p.expected_configuration_id != s.configuration_id {
        ds.push(diag(
            DiagnosticCode::ConfigurationMismatch,
            "snapshot.configuration_id",
        ));
    }
    finish(ds)?;
    Ok(Admitted {
        snapshot: s,
        policy: p,
    })
}
