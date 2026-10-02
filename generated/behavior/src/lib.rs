// generated from codegate v1
// model digest e27ccc45957cf4fe2ef83d9362e81d867eb46bff9a72f1fd28e43b69bf53ec93
// contract digest df71e33e5abc7d133716034004ba68a3a08d1c620351660f3940aa973a3c93b8
// do not edit: regenerate with `ess synthesize --layout crate`

//! Semantic types synthesised from the `codegate` specification, v1.
//!
//! Generated, not written: the specification is the source of truth, and the door to changing
//! anything here is `ess synthesize`. What is deliberately absent — behaviour, queries,
//! escalations — is listed with reasons in the `PLAN.md` beside this workspace, and every entry
//! there is owed through a typed seam in an `obligations` module here.

// `deny`, not the source workspace's lint set: this crate must hold on its own, and an undocumented
// public item here is an emitter defect worth failing the gate over.
#![forbid(unsafe_code)]
#![deny(missing_docs)]

pub mod dependency;
pub mod obligation;
pub mod primitives;
