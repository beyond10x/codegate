// generated from codegate_semantic v1
// model digest c64e8e2f5e1a2e791ed83c172875fb04a67c095ec27f188220b1d5644654c126
// contract digest bc8bafef42abf16c63fe7d2bf7a90b4f4e1e3d2b25bff37f64532fd9ad9d5f1d
// do not edit: regenerate with `ess synthesize --layout crate`

//! The typed refusal of an unmet obligation, and the conversion seams owed between contexts.
//!
//! An obligation is a capability the synthesis plan owes the implementor — the contract is declared,
//! the behaviour is not. Until an implementation satisfies one, its stub returns [`UnmetObligation`]:
//! a value naming the plan entry, never a panic and never a guess, so a workspace built on stubs
//! compiles and reports its own gaps.

/// A capability the synthesis plan owes and nothing has satisfied yet.
///
/// The two fields spell the plan entry: look the pair up in `PLAN.md` for the contract being
/// refused. A satisfying implementation never constructs one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnmetObligation {
    /// The capability kind, as the plan spells it.
    pub capability: &'static str,
    /// The construct that requires it, in the specification's own spelling.
    pub source: &'static str,
}

impl core::fmt::Display for UnmetObligation {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "unmet obligation: {} `{}` — see PLAN.md",
            self.capability, self.source
        )
    }
}
