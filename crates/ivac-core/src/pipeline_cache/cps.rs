//! Process-local memo for the audited bundled ShopBot post only.
//! Runtime, prelude and compiler identity cannot change within this process;
//! nothing is persisted. Exact serialized inputs avoid hash collisions.
use std::{collections::VecDeque, mem::size_of, sync::Mutex};

use ivac_cps::{Diagnostic, PostOutput};

const MAX_ENTRIES: usize = 4;
const MAX_BYTES: usize = 16 * 1024 * 1024;

#[derive(Debug)]
struct Entry {
    key: Vec<u8>,
    output: PostOutput,
    bytes: usize,
}

#[derive(Debug, Default)]
pub(super) struct CpsMemo {
    entries: Mutex<VecDeque<Entry>>,
}

impl CpsMemo {
    pub(super) fn get(&self, key: &[u8]) -> Option<PostOutput> {
        let mut entries = self.entries.lock().ok()?;
        let index = entries.iter().position(|entry| entry.key == key)?;
        let entry = entries.remove(index)?;
        let output = entry.output.clone();
        entries.push_front(entry);
        Some(output)
    }

    pub(super) fn put(&self, key: Vec<u8>, output: &PostOutput) {
        // Avoid copying an oversized result merely to discover it cannot fit.
        if key.len()
            + output.text.len()
            + output.extension.len()
            + output.unit.len()
            + output.program_name.len()
            + output.description.len()
            + output
                .diagnostics
                .iter()
                .map(|d| d.message.len())
                .sum::<usize>()
            > MAX_BYTES
        {
            return;
        }
        let output = output.clone();
        let bytes = size_of::<Entry>()
            + key.capacity()
            + output.text.capacity()
            + output.extension.capacity()
            + output.unit.capacity()
            + output.program_name.capacity()
            + output.description.capacity()
            + output.diagnostics.capacity() * size_of::<Diagnostic>()
            + output
                .diagnostics
                .iter()
                .map(|d| d.message.capacity())
                .sum::<usize>();
        if bytes > MAX_BYTES {
            return;
        }
        if let Ok(mut entries) = self.entries.lock() {
            entries.retain(|entry| entry.key != key);
            while entries.len() >= MAX_ENTRIES
                || entries.iter().map(|entry| entry.bytes).sum::<usize>() + bytes > MAX_BYTES
            {
                entries.pop_back();
            }
            entries.push_front(Entry { key, output, bytes });
        }
    }

    pub(super) fn clear(&self) {
        if let Ok(mut entries) = self.entries.lock() {
            entries.clear();
        }
    }
}

/// All variable run_post inputs, plus fixed execution budgets/version. The
/// binary owns the remaining runtime identity; a restart always starts empty.
pub(crate) fn key(
    source: &str,
    name: &str,
    program: &ivac_cps::ir::Program,
    properties: &serde_json::Value,
) -> Option<Vec<u8>> {
    let limits = ivac_cps::RunLimits::default();
    serde_json::to_vec(&(
        super::PIPELINE_VERSION,
        ivac_cps::ir::IR_VERSION,
        limits.loop_iterations,
        limits.recursion,
        source,
        name,
        program,
        properties,
    ))
    .ok()
    .filter(|bytes| bytes.len() <= MAX_BYTES)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn output(text: &str) -> PostOutput {
        PostOutput {
            text: text.into(),
            extension: "sbp".into(),
            unit: "mm".into(),
            program_name: "test".into(),
            description: "ShopBot".into(),
            diagnostics: vec![Diagnostic {
                severity: ivac_cps::Severity::Warning,
                message: "manual spindle".into(),
            }],
        }
    }
    #[test]
    fn exact_lru_bounded_and_clear() {
        let memo = CpsMemo::default();
        for i in 0..4 {
            memo.put(vec![i], &output("program"));
        }
        assert_eq!(memo.get(&[0]), Some(output("program")));
        memo.put(vec![4], &output("next"));
        assert!(memo.get(&[1]).is_none());
        assert!(memo.get(&[0]).is_some());
        memo.put(vec![4], &output("replacement"));
        assert_eq!(memo.get(&[4]), Some(output("replacement")));
        assert_eq!(memo.entries.lock().unwrap().len(), 4);
        memo.put(vec![9; MAX_BYTES], &output("too large"));
        assert!(memo.get(&[9; 8]).is_none());
        assert_eq!(memo.entries.lock().unwrap().len(), 4);
        memo.clear();
        assert!(memo.get(&[4]).is_none());
    }
    #[test]
    fn bytes_evict_before_entry_limit_and_concurrent_access_is_exact() {
        let memo = std::sync::Arc::new(CpsMemo::default());
        let big = output(&"x".repeat(MAX_BYTES / 2));
        memo.put(vec![1], &big);
        memo.put(vec![2], &big);
        assert!(memo.get(&[1]).is_none());
        assert!(
            memo.entries
                .lock()
                .unwrap()
                .iter()
                .map(|e| e.bytes)
                .sum::<usize>()
                <= MAX_BYTES
        );
        memo.clear();
        let workers: Vec<_> = (0..4)
            .map(|i| {
                let memo = memo.clone();
                std::thread::spawn(move || {
                    let out = output(&format!("program {i}"));
                    for _ in 0..20 {
                        memo.put(vec![i], &out);
                        assert_eq!(memo.get(&[i]), Some(out.clone()));
                    }
                })
            })
            .collect();
        for worker in workers {
            worker.join().unwrap();
        }
    }
    #[test]
    fn key_covers_source_name_properties_and_entire_ir() {
        let mut program = ivac_cps::ir::Program {
            version: ivac_cps::ir::IR_VERSION,
            header: ivac_cps::ir::Header {
                program_name: "a".into(),
                program_comment: String::new(),
                tolerance_mm: 0.01,
                parameters: vec![],
            },
            sections: vec![],
            machine: None,
        };
        let properties = serde_json::json!({"labSpindle1Control":false});
        let original = key("source", "shopbot.cps", &program, &properties).unwrap();
        assert_ne!(
            Some(&original),
            key("changed", "shopbot.cps", &program, &properties).as_ref()
        );
        assert_ne!(
            Some(&original),
            key("source", "renamed.cps", &program, &properties).as_ref()
        );
        assert_ne!(
            Some(&original),
            key(
                "source",
                "shopbot.cps",
                &program,
                &serde_json::json!({"labSpindle1Control":true})
            )
            .as_ref()
        );
        program.header.program_name = "b".into();
        assert_ne!(
            Some(&original),
            key("source", "shopbot.cps", &program, &properties).as_ref()
        );
    }
}
