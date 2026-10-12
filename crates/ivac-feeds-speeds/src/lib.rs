//! Milling recommendations from explicit, scoped data; no embedded cutting data.
//!
//! This crate proposes values only. Applying/persisting them, CAM context adapters,
//! geometry, warning presentation and post-processing belong to the caller.
//! Construct an [`Engine`] from validated supplied profiles, then call
//! [`Engine::recommend`]. There is no production dataset bundled with the crate.
//!
//! ```
//! use ivac_feeds_speeds::{Engine, FeedsSpeedsRecommendation, FeedsSpeedsRequest,
//!                       ProfileSet, ValidationIssue};
//! fn propose(data: ProfileSet, request: &FeedsSpeedsRequest)
//!     -> Result<FeedsSpeedsRecommendation, Vec<ValidationIssue>>
//! {
//!     Ok(Engine::new(data)?.recommend(request))
//! }
//! ```

mod dataset;
mod engine;
mod matching;
mod model;
mod source_pack;
mod validation;
pub use source_pack::*;

pub use dataset::{
    load_dataset_json, CatalogEntry, DatasetDocument, DatasetLoadError, LoadedDataset,
    ToolDefinition, DATASET_FORMAT_VERSION,
};
pub use engine::Engine;
pub use model::*;

/// Domain contract revision, separate from dataset and application versions.
pub const DOMAIN_SCHEMA_VERSION: u32 = 1;
/// Explicit policy version so algorithm changes remain visible in provenance.
pub const ENGINE_VERSION: &str = concat!(env!("CARGO_PKG_VERSION"), "/fs1-policy-1");
