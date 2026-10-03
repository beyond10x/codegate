// generated from codegate_semantic v1
// model digest 6429b77034506255e96ca6abd75079983e5e0ca68ef4a30f19a0baf8875612dd
// contract digest 53b2681048db3937988b0fb64c9955287e4a7585230662d8b6db2d63142af9a9
// do not edit: regenerate with `ess synthesize --layout crate`

//! Semantic types synthesised from the `codegate_semantic` specification, v1.
//!
//! Generated, not written: the specification is the source of truth, and the door to changing
//! anything here is `ess synthesize`. What is deliberately absent — behaviour, queries,
//! escalations — is listed with reasons in the `PLAN.md` beside this workspace, and every entry
//! there is owed through a typed seam in an `obligations` module here.

// `deny`, not the source workspace's lint set: this crate must hold on its own, and an undocumented
// public item here is an emitter defect worth failing the gate over.
#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod obligation;
pub mod primitives;
pub mod semantic;

pub mod ports;
pub mod system;
