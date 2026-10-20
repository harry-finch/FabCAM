//! NUMSAFE3 initial product budgets. These bound individual planning resources,
//! not total process memory, CPU time or geometric correctness.

pub const MAX_MESH_TRIANGLES: usize = 2_000_000;
pub const MAX_RELIEF_FOOTPRINT: usize = 1_000_000;
pub const MAX_RELIEF_EVALUATIONS: usize = 100_000_000;
pub const MAX_SPECIALIZED_POINTS: usize = 1_000_000;

/// Check a resource count before allocating or iterating.
///
/// # Errors
///
/// Refuses arithmetic overflow and counts above the named product budget.
pub fn check_count(count: Option<usize>, limit: usize, resource: &str) -> Result<usize, String> {
    match count {
        Some(n) if n <= limit => Ok(n),
        _ => Err(format!("{resource} exceeds the {limit} resource budget")),
    }
}

/// Check the two-triangles-per-quad height-grid expansion without allocating it.
///
/// # Errors
///
/// Refuses an oversized mesh or arithmetic overflow.
pub fn mesh_triangles(cols: u32, rows: u32) -> Result<usize, String> {
    let count = usize::try_from(cols.saturating_sub(1))
        .ok()
        .zip(usize::try_from(rows.saturating_sub(1)).ok())
        .and_then(|(x, y)| x.checked_mul(y))
        .and_then(|n| n.checked_mul(2));
    check_count(count, MAX_MESH_TRIANGLES, "generated mesh triangles")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mesh_boundary_and_extreme_are_checked_without_allocation() {
        assert_eq!(mesh_triangles(1001, 1001), Ok(MAX_MESH_TRIANGLES));
        assert!(mesh_triangles(1002, 1001).is_err());
        assert!(mesh_triangles(u32::MAX, u32::MAX).is_err());
    }

    #[test]
    fn counters_accept_boundary_and_refuse_overflow() {
        for limit in [
            MAX_MESH_TRIANGLES,
            MAX_RELIEF_FOOTPRINT,
            MAX_RELIEF_EVALUATIONS,
            MAX_SPECIALIZED_POINTS,
        ] {
            assert_eq!(check_count(Some(limit), limit, "test"), Ok(limit));
            assert!(check_count(limit.checked_add(1), limit, "test").is_err());
            assert!(check_count(None, limit, "test").is_err());
        }
    }
}
