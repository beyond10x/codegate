// generated from codegate_semantic v1
// model digest 78424588ff9f681ee0fb4662ee035a99c049cca6429cc1636f12d70420a138b2
// contract digest 8fc555319c73e1e5d01be705c8816b56b81d05a7961ff6581891488b9e59f426
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
