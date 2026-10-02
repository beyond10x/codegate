use crate::{admit::Admitted, model::*};
use std::collections::{BTreeMap, BTreeSet};
pub(crate) fn fan_out(graph: &Admitted<'_>) -> Vec<FanOut> {
    let snapshot = graph.snapshot();
    if snapshot.coverage.status == CoverageStatus::Failed {
        return vec![];
    }
    let mut destinations: BTreeMap<&str, BTreeSet<&str>> = snapshot
        .units
        .iter()
        .map(|u| (u.id.0.as_str(), BTreeSet::new()))
        .collect();
    for edge in &snapshot.edges {
        if edge.kind == graph.policy().dependency_kind
            && let Some(target) = &edge.target
        {
            destinations
                .get_mut(edge.source.0.as_str())
                .expect("admitted source")
                .insert(target.0.as_str());
        }
    }
    destinations
        .into_iter()
        .map(|(unit, targets)| FanOut {
            unit_id: UnitId(unit.into()),
            value: if snapshot.coverage.status == CoverageStatus::Complete {
                Some(targets.len() as i64)
            } else {
                None
            },
        })
        .collect()
}
