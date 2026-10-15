//! Exact, bounded memo of the planar subtraction only; eligibility stays live.
use crate::{geometry::Point2, pipeline::RegionPreview};
use std::{collections::VecDeque, mem::size_of, sync::Mutex};
const MAX_BYTES: usize = 8 * 1024 * 1024;
const MAX_ENTRIES: usize = 4;
#[derive(Debug)]
struct Entry {
    key: Vec<u8>,
    regions: Vec<RegionPreview>,
    bytes: usize,
}
#[derive(Debug, Default)]
pub(crate) struct RemainingMemo {
    entries: Mutex<VecDeque<Entry>>,
}
impl RemainingMemo {
    pub(crate) fn get(&self, key: &[u8]) -> Option<Vec<RegionPreview>> {
        let mut entries = self.entries.lock().ok()?;
        let index = entries.iter().position(|e| e.key == key)?;
        let entry = entries.remove(index)?;
        let result = entry.regions.clone();
        entries.push_front(entry);
        Some(result)
    }
    pub(crate) fn put(&self, key: Vec<u8>, regions: &[RegionPreview]) {
        let estimated = key.capacity()
            + std::mem::size_of_val(regions)
            + regions
                .iter()
                .map(|r| {
                    r.outer.len() * size_of::<Point2>()
                        + r.holes.len() * size_of::<Vec<Point2>>()
                        + r.holes
                            .iter()
                            .map(|h| h.len() * size_of::<Point2>())
                            .sum::<usize>()
                })
                .sum::<usize>();
        if estimated > MAX_BYTES {
            return;
        }
        let regions = regions.to_vec();
        let bytes = size_of::<Entry>()
            + key.capacity()
            + regions.capacity() * size_of::<RegionPreview>()
            + regions
                .iter()
                .map(|r| {
                    r.outer.capacity() * size_of::<Point2>()
                        + r.holes.capacity() * size_of::<Vec<Point2>>()
                        + r.holes
                            .iter()
                            .map(|h| h.capacity() * size_of::<Point2>())
                            .sum::<usize>()
                })
                .sum::<usize>();
        if bytes > MAX_BYTES {
            return;
        }
        if let Ok(mut entries) = self.entries.lock() {
            entries.retain(|e| e.key != key);
            while entries.len() >= MAX_ENTRIES
                || entries.iter().map(|e| e.bytes).sum::<usize>() + bytes > MAX_BYTES
            {
                entries.pop_back();
            }
            entries.push_front(Entry {
                key,
                regions,
                bytes,
            });
        }
    }
    pub(crate) fn clear(&self) {
        if let Ok(mut entries) = self.entries.lock() {
            entries.clear();
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn bounded_exact_lru_clear_and_oversized_bypass() {
        let memo = RemainingMemo::default();
        for i in 0..4 {
            memo.put(vec![i], &[]);
        }
        assert!(memo.get(&[0]).is_some());
        memo.put(vec![4], &[]);
        assert!(memo.get(&[1]).is_none());
        memo.put(vec![8; MAX_BYTES], &[]);
        assert_eq!(memo.entries.lock().unwrap().len(), 4);
        memo.clear();
        assert!(memo.get(&[0]).is_none());
        memo.put(vec![1; MAX_BYTES / 2], &[]);
        memo.put(vec![2; MAX_BYTES / 2], &[]);
        assert_eq!(memo.entries.lock().unwrap().len(), 1);
    }
    #[test]
    fn concurrent_entries_keep_exact_native_coordinates() {
        let memo = std::sync::Arc::new(RemainingMemo::default());
        let workers: Vec<_> = (0..4)
            .map(|i| {
                let memo = memo.clone();
                std::thread::spawn(move || {
                    let x = f64::from_bits(0x3ff0000000000001 + u64::from(i));
                    let regions = vec![RegionPreview {
                        op_id: u32::from(i),
                        outer: vec![Point2::new(x, -0.)],
                        holes: vec![vec![Point2::new(1., 2.)]],
                    }];
                    for _ in 0..20 {
                        memo.put(vec![i], &regions);
                        let found = memo.get(&[i]).unwrap();
                        assert_eq!(found[0].outer[0].x.to_bits(), x.to_bits());
                        assert_eq!(found[0].outer[0].y.to_bits(), (-0.0f64).to_bits());
                        assert_eq!(found[0].op_id, u32::from(i));
                    }
                })
            })
            .collect();
        for worker in workers {
            worker.join().unwrap();
        }
        assert!(
            memo.entries
                .lock()
                .unwrap()
                .iter()
                .map(|e| e.bytes)
                .sum::<usize>()
                <= MAX_BYTES
        );
    }
}
