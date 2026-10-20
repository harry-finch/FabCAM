//! Autodesk-`.cps`-compatible post-processor runtime.
//!
//! Executes Fusion-style `.cps` posts (ES5 JavaScript) on the pure-Rust
//! [`boa_engine`] so every ivacam transport — CLI, server, Tauri and
//! wasm — can run vendor posts without a C toolchain. The crate is
//! deliberately independent of `ivac-core`: the pipeline talks to it
//! through the serialized program IR (`ir` module, from cps.1 on) and
//! gets NC text + diagnostics back in one envelope.

pub mod diag;
mod engine;
pub mod ir;
pub mod library;
pub mod meta;
pub mod run;

pub use diag::{Diagnostic, PostError, Severity};
pub use run::{
    inspect_post, inspect_post_with_limits, run_post, run_post_cancellable, run_post_with_limits,
    PostOutput, RunLimits,
};

#[cfg(test)]
extern crate self as ivac_cps;
#[cfg(test)]
#[path = "../tests/cancellation.rs"]
mod cancellation;
#[cfg(test)]
#[path = "../tests/constants_consistency.rs"]
mod constants_consistency;
#[cfg(test)]
#[path = "../tests/differential.rs"]
mod differential;
#[cfg(test)]
#[path = "../tests/golden_fanuc.rs"]
mod golden_fanuc;
#[cfg(test)]
#[path = "../tests/shopbot_guards.rs"]
mod shopbot_guards;
#[cfg(test)]
#[path = "../tests/spike.rs"]
mod spike;
#[cfg(test)]
#[path = "../tests/symbol_audit.rs"]
mod symbol_audit;

#[cfg(test)]
#[path = "../tests/probe3.rs"]
mod probe3;
