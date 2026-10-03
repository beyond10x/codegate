// generated from codegate_semantic v1
// model digest 6429b77034506255e96ca6abd75079983e5e0ca68ef4a30f19a0baf8875612dd
// contract digest 53b2681048db3937988b0fb64c9955287e4a7585230662d8b6db2d63142af9a9
// do not edit: regenerate with `ess synthesize --layout crate`

//! source-foundation — the `source-foundation` component of `codegate_semantic` v1.
//!
//! Collects exact source inputs and validates imported fact snapshots.
//!
//! The component's outer surface exactly as the specification declares it: accepted commands as
//! handlers, declared views as queries, published events as a typed outbox. The behaviour behind
//! every handler is an implementation obligation — see the `PLAN.md` beside this workspace — and
//! until one is satisfied, its stub answers with a typed refusal naming what is owed.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

/// An event this component declares it publishes, on its way to the system's transport.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PublishedEvent {}

/// source-foundation — the port over the component's obligations.
///
/// `B` bundles every behaviour and query this component owes; constructing it over the domain's
/// `obligations::Unimplemented` yields a component that compiles and refuses, in the type system,
/// everything not yet implemented.
pub struct SourceFoundation<B> {
    behaviors: B,
    outbox: Vec<PublishedEvent>,
}

impl<B> SourceFoundation<B> {
    /// A new port over the given obligation implementations.
    pub fn new(behaviors: B) -> Self {
        Self {
            behaviors,
            outbox: Vec::new(),
        }
    }

    /// Hands over everything published since the last drain, in publication order.
    ///
    /// The system's transport calls this; anything else reading it is taking events the transport
    /// will then never deliver.
    pub fn drain_outbox(&mut self) -> Vec<PublishedEvent> {
        core::mem::take(&mut self.outbox)
    }
}

impl<B> SourceFoundation<B>
where
    B: crate::semantic::obligations::CollectSourceBehavior
        + crate::semantic::obligations::ValidateSnapshotBehavior,
{
    /// Accepts `codegate_semantic.semantic.CollectSource`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn collect_source(
        &mut self,
        input: crate::semantic::CollectSource,
    ) -> Result<crate::semantic::CollectSourceOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.collect_source(input)?;
        match &outcome {
            crate::semantic::CollectSourceOutcome::SourceObserved { .. } => {}
        }
        Ok(outcome)
    }

    /// Accepts `codegate_semantic.semantic.ValidateSnapshot`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn validate_snapshot(
        &mut self,
        input: crate::semantic::ValidateSnapshot,
    ) -> Result<crate::semantic::ValidateSnapshotOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.validate_snapshot(input)?;
        match &outcome {
            crate::semantic::ValidateSnapshotOutcome::ValidationObserved { .. } => {}
        }
        Ok(outcome)
    }
}
