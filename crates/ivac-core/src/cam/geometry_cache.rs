//! Sub-op geometry caches.
//!
//! [`crate::pipeline_cache`] keys the WHOLE-OP output by every field that
//! affects the emitted G-code — depth, feeds, plunge, leads, tabs, the
//! full machine config. That's correct for the output level, but it
//! invalidates aggressively: a depth tweak on a V-Carve op forces
//! re-running `medial_axis` even though the boundary geometry is
//! unchanged. On a 1000-segment contour that's 2–4 s of pure waste
//! every time the user nudges the depth spinner.
//!
//! This module caches the expensive geometry primitives one level below:
//! medial axis, pocket cascade rings. The keys depend only on input
//! SHAPE (and tool radius where applicable), not on depth / feeds /
//! leads / tabs. So depth-only edits are now free, and full regenerates
//! after a deep-cache run only pay the I/O cost of cloning the cached
//! output (~100 µs per medial axis).
//!
//! ## Hashing discipline
//!
//! Full equality keys: f64 → canonical bits (+0 and -0 share a key), sort maps
//! before iterating, hand-written impls because [`Point2`] carries
//! f64. Each cache folds [`crate::version::PIPELINE_VERSION`]
//! into the key so any algorithm change invalidates every entry.
//!
//! ## Capacity
//!
//! Default 64 per cache. A medial axis or cascade-rings result is
//! typically <1 MB; 64 keeps memory bounded while still covering the
//! common authoring loop where the user oscillates between a handful
//! of recently-edited ops.

use std::num::NonZeroUsize;
use std::sync::{Mutex, MutexGuard, OnceLock};

use lru::LruCache;

use crate::cam::vcarve::{medial_axis_cancellable, VPoint, VcRegion};
use crate::cancel::CancelToken;
use crate::geometry::Point2;
use crate::version::PIPELINE_VERSION;

/// Algorithm discriminator byte. Folded into every key so two different
/// algorithms can't collide even if they happened to hash their inputs
/// identically.
const ALG_MEDIAL_AXIS: u8 = 1;
const ALG_POCKET_CASCADE_WITH_ISLANDS: u8 = 2;

/// LRU capacity per algorithm. 64 entries fits the typical authoring
/// loop (user oscillates between ~3–5 ops at a time, each with 1–3
/// distinct boundary shapes); larger would burn memory without paying
/// for itself.
const CAPACITY: usize = 64;

// Full canonical keys: the LRU's hash table resolves collisions by exact equality.
// Retain ring lengths/order, all coordinates, algorithm version and optional delta.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct GeometryKey(Vec<u64>);
fn canonical_bits(v: f64) -> u64 {
    if v == 0.0 {
        0
    } else {
        v.to_bits()
    }
}
fn geometry_key(
    algorithm: u8,
    outer: &[Point2],
    holes: &[Vec<Point2>],
    delta: Option<f64>,
) -> GeometryKey {
    let mut words = vec![
        u64::from(PIPELINE_VERSION),
        u64::from(algorithm),
        holes.len() as u64,
    ];
    for ring in std::iter::once(outer).chain(holes.iter().map(Vec::as_slice)) {
        words.push(ring.len() as u64);
        for p in ring {
            words.extend([canonical_bits(p.x), canonical_bits(p.y)]);
        }
    }
    if let Some(delta) = delta {
        words.push(canonical_bits(delta));
    }
    GeometryKey(words)
}

/// One bounded LRU per algorithm. The process-global instances below back
/// the public entry points; tests build private instances so their entry
/// counts never observe work done by unrelated tests running in parallel.
struct GeometryCache<V> {
    inner: OnceLock<Mutex<LruCache<GeometryKey, V>>>,
    name: &'static str,
}

impl<V> GeometryCache<V> {
    const fn new(name: &'static str) -> Self {
        Self {
            inner: OnceLock::new(),
            name,
        }
    }

    fn lock(&self) -> MutexGuard<'_, LruCache<GeometryKey, V>> {
        self.inner
            .get_or_init(|| {
                Mutex::new(LruCache::new(
                    NonZeroUsize::new(CAPACITY).expect("non-zero capacity"),
                ))
            })
            .lock()
            .unwrap_or_else(|_| panic!("{} cache mutex poisoned", self.name))
    }
}

// ── medial axis ──────────────────────────────────────────────────────

type MedialAxisOutput = Vec<Vec<VPoint>>;

static MEDIAL_AXIS_CACHE: GeometryCache<MedialAxisOutput> = GeometryCache::new("medial-axis");

/// Cached wrapper around [`medial_axis_cancellable`]. Keys by the
/// region's outer ring + every hole; recomputes on a miss, returns the
/// cloned cached value on a hit.
///
/// Cancellation: if `cancel` fires during a miss, the partial result is
/// returned WITHOUT being cached — caching a cancellation would poison
/// the cache and burn the next caller. Hit path doesn't check cancel
/// (a clone is cheap and the caller still gets to react to the flag
/// itself).
#[must_use]
pub fn medial_axis_cached(region: &VcRegion, cancel: Option<&CancelToken>) -> MedialAxisOutput {
    medial_axis_cached_in(&MEDIAL_AXIS_CACHE, region, cancel)
}

fn medial_axis_cached_in(
    cache: &GeometryCache<MedialAxisOutput>,
    region: &VcRegion,
    cancel: Option<&CancelToken>,
) -> MedialAxisOutput {
    crate::profile_scope!("medial.cache_lookup");
    let key = geometry_key(ALG_MEDIAL_AXIS, &region.outer, &region.holes, None);
    if let Some(v) = cache.lock().get(&key).cloned() {
        return v;
    }
    let out = medial_axis_cancellable(region, cancel);
    if !cancel.is_some_and(CancelToken::is_cancelled) {
        cache.lock().put(key, out.clone());
    }
    out
}

// ── pocket cascade with islands ──────────────────────────────────────

type PocketCascadeOutput = Vec<Vec<Point2>>;

static POCKET_CASCADE_CACHE: GeometryCache<PocketCascadeOutput> =
    GeometryCache::new("pocket-cascade");

/// Cached wrapper around [`crate::cam::offsets::pocket_cascade_with_islands`].
/// Keys by the boundary, every island, and the step delta. Re-runs of
/// the same pocket op with different depth / feed / plunge inputs all
/// hit the cache.
#[must_use]
pub fn pocket_cascade_with_islands_cached(
    boundary: &[Point2],
    islands: &[Vec<Point2>],
    delta: f64,
) -> PocketCascadeOutput {
    pocket_cascade_with_islands_cached_in(&POCKET_CASCADE_CACHE, boundary, islands, delta)
}

fn pocket_cascade_with_islands_cached_in(
    cache: &GeometryCache<PocketCascadeOutput>,
    boundary: &[Point2],
    islands: &[Vec<Point2>],
    delta: f64,
) -> PocketCascadeOutput {
    let key = geometry_key(
        ALG_POCKET_CASCADE_WITH_ISLANDS,
        boundary,
        islands,
        Some(delta),
    );
    if let Some(v) = cache.lock().get(&key).cloned() {
        return v;
    }
    let out = crate::cam::offsets::pocket_cascade_with_islands(boundary, islands, delta);
    cache.lock().put(key, out.clone());
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A private cache per test: no shared state, so no serialization guard.
    fn medial() -> GeometryCache<MedialAxisOutput> {
        GeometryCache::new("test medial-axis")
    }

    fn cascade() -> GeometryCache<PocketCascadeOutput> {
        GeometryCache::new("test pocket-cascade")
    }

    fn square(side: f64) -> Vec<Point2> {
        vec![
            Point2::new(0.0, 0.0),
            Point2::new(side, 0.0),
            Point2::new(side, side),
            Point2::new(0.0, side),
        ]
    }

    fn square_region(side: f64) -> VcRegion {
        VcRegion {
            outer: square(side),
            holes: Vec::new(),
        }
    }

    #[test]
    fn cached_medial_axis_matches_uncached_on_cold_and_warm_calls() {
        let region = square_region(13.0);
        let reference = crate::cam::vcarve::medial_axis(&region);
        assert_eq!(medial_axis_cached(&region, None), reference);
        assert_eq!(medial_axis_cached(&region, None), reference);
        let with_island = VcRegion {
            outer: region.outer,
            holes: vec![square(2.0)],
        };
        assert_eq!(
            medial_axis_cached(&with_island, None),
            crate::cam::vcarve::medial_axis(&with_island)
        );
    }
    #[test]
    fn canonical_keys_resolve_forced_collisions_and_signed_zero() {
        use std::hash::{BuildHasherDefault, Hasher};
        #[derive(Default)]
        struct ConstantHasher;
        impl Hasher for ConstantHasher {
            fn finish(&self) -> u64 {
                0
            }
            fn write(&mut self, _: &[u8]) {}
        }
        let positive = geometry_key(ALG_MEDIAL_AXIS, &[Point2::new(0.0, 1.0)], &[], None);
        let negative = geometry_key(ALG_MEDIAL_AXIS, &[Point2::new(-0.0, 1.0)], &[], None);
        assert_eq!(positive, negative);
        let other = geometry_key(ALG_MEDIAL_AXIS, &[Point2::new(0.0, 2.0)], &[], None);
        let mut cache = LruCache::with_hasher(
            NonZeroUsize::new(2).unwrap(),
            BuildHasherDefault::<ConstantHasher>::default(),
        );
        cache.put(positive.clone(), 1);
        cache.put(other.clone(), 2);
        assert_eq!(cache.get(&negative), Some(&1));
        assert_eq!(cache.get(&other), Some(&2));
        assert_ne!(
            geometry_key(ALG_POCKET_CASCADE_WITH_ISLANDS, &[], &[], Some(1.0)),
            geometry_key(ALG_POCKET_CASCADE_WITH_ISLANDS, &[], &[], Some(2.0))
        );
    }
    #[test]
    fn medial_axis_cached_same_region_hits_cache() {
        let cache = medial();
        let r = square_region(20.0);
        let v1 = medial_axis_cached_in(&cache, &r, None);
        let len_after_miss = cache.lock().len();
        let v2 = medial_axis_cached_in(&cache, &r, None);
        let len_after_hit = cache.lock().len();
        assert_eq!(
            len_after_miss, 1,
            "first call should populate the cache (got {len_after_miss})"
        );
        assert_eq!(
            len_after_hit, 1,
            "second call should hit (no new entry; got {len_after_hit})"
        );
        assert_eq!(
            v1.len(),
            v2.len(),
            "same region must return same chain count"
        );
    }

    #[test]
    fn medial_axis_cached_different_region_misses() {
        let cache = medial();
        let _ = medial_axis_cached_in(&cache, &square_region(20.0), None);
        let _ = medial_axis_cached_in(&cache, &square_region(30.0), None);
        assert_eq!(cache.lock().len(), 2);
    }

    #[test]
    fn medial_axis_cached_holes_change_key() {
        let cache = medial();
        let mut r = square_region(20.0);
        let _ = medial_axis_cached_in(&cache, &r, None);
        // Same outer, add a hole — different region, must miss.
        r.holes.push(vec![
            Point2::new(5.0, 5.0),
            Point2::new(10.0, 5.0),
            Point2::new(10.0, 10.0),
            Point2::new(5.0, 10.0),
        ]);
        let _ = medial_axis_cached_in(&cache, &r, None);
        assert_eq!(cache.lock().len(), 2);
    }

    #[test]
    fn pocket_cascade_cached_same_inputs_hits_cache() {
        let cache = cascade();
        let boundary = square(20.0);
        let _ = pocket_cascade_with_islands_cached_in(&cache, &boundary, &[], 2.0);
        let len_after_miss = cache.lock().len();
        let _ = pocket_cascade_with_islands_cached_in(&cache, &boundary, &[], 2.0);
        let len_after_hit = cache.lock().len();
        assert_eq!(len_after_miss, 1);
        assert_eq!(len_after_hit, 1);
    }

    #[test]
    fn pocket_cascade_cached_different_delta_misses() {
        let cache = cascade();
        let boundary = square(20.0);
        let _ = pocket_cascade_with_islands_cached_in(&cache, &boundary, &[], 2.0);
        let _ = pocket_cascade_with_islands_cached_in(&cache, &boundary, &[], 2.5);
        assert_eq!(cache.lock().len(), 2);
    }

    #[test]
    fn pocket_cascade_cached_islands_change_key() {
        let cache = cascade();
        let boundary = square(20.0);
        let island = vec![
            Point2::new(5.0, 5.0),
            Point2::new(10.0, 5.0),
            Point2::new(10.0, 10.0),
            Point2::new(5.0, 10.0),
        ];
        let _ = pocket_cascade_with_islands_cached_in(&cache, &boundary, &[], 2.0);
        let _ = pocket_cascade_with_islands_cached_in(&cache, &boundary, &[island], 2.0);
        assert_eq!(cache.lock().len(), 2);
    }
}
