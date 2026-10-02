use crate::{admit::Admitted, model::*};
use std::collections::BTreeSet;
pub(crate) fn forbidden(graph: &Admitted<'_>) -> Vec<Finding> {
    let policy = graph.policy();
    let rules: BTreeSet<_> = policy
        .forbidden
        .iter()
        .map(|r| (r.source.0.as_str(), r.target.0.as_str()))
        .collect();
    let mut findings: Vec<_> = graph
        .snapshot()
        .edges
        .iter()
        .filter(|e| e.kind == policy.dependency_kind)
        .filter_map(|e| {
            e.target
                .as_ref()
                .filter(|t| rules.contains(&(e.source.0.as_str(), t.0.as_str())))
                .map(|t| Finding {
                    edge_id: e.id.clone(),
                    source: e.source.clone(),
                    target: t.clone(),
                    kind: e.kind,
                })
        })
        .collect();
    findings.sort_by(|a, b| a.edge_id.0.cmp(&b.edge_id.0));
    findings
}
