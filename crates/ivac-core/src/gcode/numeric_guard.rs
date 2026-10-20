//! Fail-closed record of values that must never reach a machine program
//! (NUMSAFE): non-finite numbers and zero or missing cutting feeds.
//!
//! The post emit API is infallible (dozens of call sites per driver), so the
//! leaf formatters and feed handlers record the FIRST fault here instead of
//! substituting a value. The pipeline resets the record before emitting,
//! checks it at every op boundary (before the op body can be cached), and
//! refuses the whole program; a streaming sink stops writing as soon as a
//! fault is recorded. Thread-local because a post runs on the pipeline
//! thread, like the offset diagnostics channel.

use std::cell::RefCell;

/// Why a program was refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFaultKind {
    /// NaN or ±infinity reached a coordinate, arc, dwell or axis word.
    NonFinite,
    /// A zero feed was commanded for cutting or plunging motion.
    ZeroFeed,
    /// A cutting move was emitted before any feed was established.
    MissingFeed,
}

/// First fault seen since the last [`reset`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OutputFault {
    pub kind: OutputFaultKind,
    /// What was being emitted, e.g. `X word` or `G1 move`.
    pub detail: String,
}

thread_local! {
    static FAULT: RefCell<Option<OutputFault>> = const { RefCell::new(None) };
}

/// Record a fault; the first one wins so the diagnostic names the origin.
pub(crate) fn record(kind: OutputFaultKind, detail: impl Into<String>) {
    FAULT.with(|f| {
        let mut f = f.borrow_mut();
        if f.is_none() {
            *f = Some(OutputFault {
                kind,
                detail: detail.into(),
            });
        }
    });
}

/// Record a non-finite value and return `false`, or return `true`.
pub(crate) fn finite(v: f64, what: &str) -> bool {
    if v.is_finite() {
        true
    } else {
        record(OutputFaultKind::NonFinite, format!("{what} = {v}"));
        false
    }
}

/// Clear any record left by an earlier (possibly aborted) run.
pub(crate) fn reset() {
    FAULT.with(|f| f.borrow_mut().take());
}

/// Take the recorded fault, clearing it.
pub(crate) fn take() -> Option<OutputFault> {
    FAULT.with(|f| f.borrow_mut().take())
}

/// Whether a fault is recorded (a streaming sink stops writing).
pub(crate) fn is_set() -> bool {
    FAULT.with(|f| f.borrow().is_some())
}

impl std::fmt::Display for OutputFault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let what = match self.kind {
            OutputFaultKind::NonFinite => "non-finite value",
            OutputFaultKind::ZeroFeed => "zero feed rate",
            OutputFaultKind::MissingFeed => "cutting move without a feed rate",
        };
        write!(f, "{what} ({})", self.detail)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_fault_wins_and_take_clears() {
        reset();
        assert!(finite(1.5, "X word"));
        assert!(!is_set());
        assert!(!finite(f64::NAN, "Y word"));
        record(OutputFaultKind::ZeroFeed, "later");
        let f = take().expect("recorded");
        assert_eq!(f.kind, OutputFaultKind::NonFinite);
        assert_eq!(f.to_string(), "non-finite value (Y word = NaN)");
        assert!(take().is_none());
    }
}
