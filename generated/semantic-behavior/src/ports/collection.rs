// generated from codegate_semantic v1
// model digest c64e8e2f5e1a2e791ed83c172875fb04a67c095ec27f188220b1d5644654c126
// contract digest bc8bafef42abf16c63fe7d2bf7a90b4f4e1e3d2b25bff37f64532fd9ad9d5f1d
// do not edit: regenerate with `ess synthesize --layout crate`

//! collection — the `collection` component of `codegate_semantic` v1.
//!
//! Composes exact captured sources and registered language observations.
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

/// collection — the port over the component's obligations.
///
/// `B` bundles every behaviour and query this component owes; constructing it over the domain's
/// `obligations::Unimplemented` yields a component that compiles and refuses, in the type system,
/// everything not yet implemented.
pub struct Collection<B> {
    behaviors: B,
    outbox: Vec<PublishedEvent>,
}

impl<B> Collection<B> {
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

impl<B> Collection<B>
where
    B: crate::semantic::obligations::CollectBehavior,
{
    /// Accepts `codegate_semantic.semantic.Collect`: runs the behaviour obligation, then publishes the declared events
    /// the outcome carries.
    ///
    /// `Err` is the typed refusal of an unmet obligation — never a domain outcome, which always
    /// arrives as a variant of the outcome type, refusals included.
    pub fn collect(
        &mut self,
        input: crate::semantic::Collect,
    ) -> Result<crate::semantic::CollectOutcome, crate::obligation::UnmetObligation> {
        let outcome = self.behaviors.collect(input)?;
        match &outcome {
            crate::semantic::CollectOutcome::Collected { .. } => {}
        }
        Ok(outcome)
    }
}
