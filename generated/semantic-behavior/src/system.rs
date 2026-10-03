// generated from codegate_semantic v1
// model digest 6429b77034506255e96ca6abd75079983e5e0ca68ef4a30f19a0baf8875612dd
// contract digest 53b2681048db3937988b0fb64c9955287e4a7585230662d8b6db2d63142af9a9
// do not edit: regenerate with `ess synthesize --layout crate`

//! The `codegate_semantic` system, v1: its components assembled, its bindings wired, and its one transport.
//!
//! The transport is derived from the specification, not chosen: `at_least_once` is the only
//! delivery guarantee the model declares, so published events land on an append-only log and a
//! pump delivers each to every binding that reacts to it. The log is the system's observable
//! record, and so is the record of what each binding invoked. What no specification determines
//! — how an escalation event is filled, behaviour behind the ports — stays an obligation; see
//! the `PLAN.md` beside this workspace.

#![forbid(unsafe_code)]
#![deny(missing_docs)]

/// An event on the system's log: everything any component publishes, and everything a binding
/// escalates into.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SystemEvent {}

impl SystemEvent {
    /// The qualified name the specification declares this event under.
    pub fn name(&self) -> &'static str {
        match *self {}
    }
}

impl From<crate::ports::source_foundation::PublishedEvent> for SystemEvent {
    fn from(event: crate::ports::source_foundation::PublishedEvent) -> Self {
        match event {}
    }
}

/// The `codegate_semantic` system: every component behind its port, and the transport between them.
///
/// The component fields are public because commands enter the system through a component's own
/// port; the log and its delivery cursor are not, because publishing happens by pumping, not by
/// writing history directly.
pub struct System<SourceFoundationBehaviors> {
    /// The `source-foundation` component.
    pub source_foundation:
        crate::ports::source_foundation::SourceFoundation<SourceFoundationBehaviors>,
    published: Vec<SystemEvent>,
    cursor: usize,
}

impl<SourceFoundationBehaviors> System<SourceFoundationBehaviors> {
    /// Assembles the system from its components.
    pub fn new(
        source_foundation: crate::ports::source_foundation::SourceFoundation<
            SourceFoundationBehaviors,
        >,
    ) -> Self {
        Self {
            source_foundation,
            published: Vec::new(),
            cursor: 0,
        }
    }

    /// Everything published so far, in publication order — the system's observable record.
    pub fn published(&self) -> &[SystemEvent] {
        &self.published
    }

    /// Takes every event the pump has already delivered off the log, in publication order.
    ///
    /// A long-running shell calls this after each `pump`, or the log holds every event the
    /// process ever published. A `pump` returns with every logged event delivered: each
    /// reacting binding has had its attempt, and a binding whose attempt stopped holds the event in
    /// its own held-back list, not on the log. Events published since the last `pump` stay on the
    /// log, so the next `pump` still delivers them; taking never skips a binding.
    pub fn take_published(&mut self) -> Vec<SystemEvent> {
        let delivered: Vec<SystemEvent> = self.published.drain(..self.cursor).collect();
        self.cursor = 0;
        delivered
    }
}

impl<SourceFoundationBehaviors> System<SourceFoundationBehaviors>
where
    SourceFoundationBehaviors: crate::semantic::obligations::CollectSourceBehavior
        + crate::semantic::obligations::ValidateSnapshotBehavior,
{
    /// Delivers until quiescent: collects every component's outbox onto the log. No binding
    /// reacts to anything this specification publishes, so collecting is the whole delivery.
    pub fn pump(&mut self) -> Result<(), crate::obligation::UnmetObligation> {
        loop {
            self.collect();
            if self.cursor == self.published.len() {
                return Ok(());
            }
            self.cursor += 1;
        }
    }

    /// Moves every component's outbox onto the log, in component order.
    fn collect(&mut self) {
        for event in self.source_foundation.drain_outbox() {
            self.published.push(SystemEvent::from(event));
        }
    }
}
