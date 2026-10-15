//! Opt-in, thread-local native diagnostics. No request/response or CAM semantics.
//! The caller must reset/take outside active scopes, on the pipeline thread.
use serde::Serialize;
use std::cell::RefCell;
use std::time::Instant;

#[derive(Debug, Clone, Serialize)]
pub struct Record {
    pub name: &'static str,
    pub depth: usize,
    pub start_ns: u128,
    pub elapsed_ns: u128,
    pub count: Option<usize>,
}

struct Session {
    origin: Instant,
    depth: usize,
    records: Vec<Record>,
}
thread_local! {
    static SESSION: RefCell<Session> = RefCell::new(Session {
        origin: Instant::now(), depth: 0, records: Vec::new(),
    });
}

#[derive(Debug)]
pub struct Span {
    name: &'static str,
    depth: usize,
    start: Instant,
    start_ns: u128,
}
impl Span {
    pub fn enter(name: &'static str) -> Self {
        SESSION.with(|s| {
            let mut s = s.borrow_mut();
            let start = Instant::now();
            let span = Self {
                name,
                depth: s.depth,
                start,
                start_ns: start.duration_since(s.origin).as_nanos(),
            };
            s.depth += 1;
            span
        })
    }
}
impl Drop for Span {
    fn drop(&mut self) {
        let elapsed_ns = self.start.elapsed().as_nanos();
        SESSION.with(|s| {
            let mut s = s.borrow_mut();
            s.depth -= 1;
            s.records.push(Record {
                name: self.name,
                depth: self.depth,
                start_ns: self.start_ns,
                elapsed_ns,
                count: None,
            });
        });
    }
}
pub fn count(name: &'static str, value: usize) {
    SESSION.with(|s| {
        let mut s = s.borrow_mut();
        let record = Record {
            name,
            depth: s.depth,
            start_ns: s.origin.elapsed().as_nanos(),
            elapsed_ns: 0,
            count: Some(value),
        };
        s.records.push(record);
    });
}
pub fn reset() {
    SESSION.with(|s| {
        let mut s = s.borrow_mut();
        assert_eq!(s.depth, 0, "reset outside active profiling scopes");
        s.records.clear();
        s.origin = Instant::now();
    });
}
pub fn take() -> Vec<Record> {
    SESSION.with(|s| {
        let mut s = s.borrow_mut();
        assert_eq!(s.depth, 0, "take outside active profiling scopes");
        std::mem::take(&mut s.records)
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn nesting_thread_isolation_and_early_returns() {
        reset();
        crate::profile_measure!("outer", {
            crate::profile_scope!("inner");
        });
        let records = take();
        assert_eq!(
            records
                .iter()
                .map(|r| (r.name, r.depth))
                .collect::<Vec<_>>(),
            vec![("inner", 1), ("outer", 0)]
        );
        assert!(records[1].elapsed_ns >= records[0].elapsed_ns);
        let value: Result<(), ()> = (|| {
            crate::profile_measure!("refusal", {
                Err(())?;
                Ok(())
            })
        })();
        assert!(value.is_err());
        assert_eq!(take()[0].name, "refusal");
        std::thread::spawn(|| {
            reset();
            crate::profile_measure!("worker", ());
            assert_eq!(take().len(), 1);
        })
        .join()
        .unwrap();
        assert!(take().is_empty());
    }
}
