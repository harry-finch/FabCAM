//! Structured opt-in errors; legacy String APIs remain compatible at adapter boundaries.
use crate::{DatasetLoadError, SourcePack, ValidationIssue, MAX_PACK_BYTES};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SourcePackError {
    SizeLimit { maximum_bytes: usize },
    InvalidJson { message: String },
    InvalidDataset { issues: Vec<ValidationIssue> },
    InvalidPack { message: String },
    TooManyPacks { maximum: usize },
    DuplicatePack { id: String },
    Recommendation { message: String },
}
impl std::fmt::Display for SourcePackError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SizeLimit { .. } => write!(f, "Source pack exceeds 2 MiB"),
            Self::InvalidJson { message } => write!(f, "Invalid source pack: {message}"),
            Self::InvalidDataset { issues } => {
                write!(f, "{}", DatasetLoadError::Invalid(issues.clone()))
            }
            Self::InvalidPack { message } | Self::Recommendation { message } => {
                f.write_str(message)
            }
            Self::TooManyPacks { maximum } => write!(f, "At most {maximum} packs may be enabled"),
            Self::DuplicatePack { id } => {
                write!(f, "Duplicate source pack ID; enable one version: {id}")
            }
        }
    }
}
impl std::error::Error for SourcePackError {}

// Enforce the serialized size boundary without allocating a full JSON buffer.
pub(crate) fn check_pack_size(pack: &SourcePack) -> Result<(), SourcePackError> {
    struct Counter(usize);
    impl std::io::Write for Counter {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            self.0 = self.0.saturating_add(bytes.len());
            if self.0 > MAX_PACK_BYTES {
                return Err(std::io::Error::other("size limit"));
            }
            Ok(bytes.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let mut count = Counter(0);
    serde_json::to_writer(&mut count, pack).map_err(|e| {
        if count.0 > MAX_PACK_BYTES {
            SourcePackError::SizeLimit {
                maximum_bytes: MAX_PACK_BYTES,
            }
        } else {
            SourcePackError::InvalidJson {
                message: e.to_string(),
            }
        }
    })
}
