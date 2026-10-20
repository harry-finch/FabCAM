#!/usr/bin/env python3
"""Bounded NUMSAFE3 baseline probes; never run an unbounded planner workload.

Uses the actual Thread module and Waterline helper at baseline 274f543.
Other probes check the loop's arithmetic only, without allocating its output.
Run: python3 scripts/numsafe3-audit-probes.py
Requires rustc. These expectations intentionally preserve the pre-fix baseline;
current-source regressions are in crates/ivac-core/tests/numsafe3.rs.
"""
from pathlib import Path
import subprocess
import tempfile

root = Path(__file__).resolve().parents[1]
def baseline(path):
    return subprocess.check_output(["git", "show", f"274f543:{path}"], cwd=root, text=True)

thread_source = baseline("crates/ivac-core/src/cam/thread.rs")
waterline = baseline("crates/ivac-core/src/cam/waterline.rs")
start = waterline.index("pub fn z_levels(")
end = waterline.index("\n}\n", start) + 2
source = r'''
mod geometry { pub struct Point2 { pub x: f64, pub y: f64 } }
mod project { pub mod tool {
    #[allow(dead_code)] pub enum SpindleDirection { Cw, Ccw }
} }
#[path = "THREAD_PATH"] mod thread;
Z_LEVELS
fn main() {
    assert_eq!(z_levels(0.0, -1001.0, 1.0).len(), 1001);
    println!("Waterline: actual helper accepts 1001 levels");
    let top = -1e20_f64;
    let bottom = -2e20_f64;
    let step = 1.0_f64;
    let z = top - step;
    assert!(z > bottom + step * 1e-6 && z - step == z);
    println!("Waterline: finite subtraction stalls (one arithmetic step only)");
    // 64 / 1e-300 saturates to usize::MAX before +2; debug overflow
    // occurs BEFORE any large allocation.
    let failure = std::panic::catch_unwind(|| thread::helix_waypoints(
        geometry::Point2 { x: 0.0, y: 0.0 },
        2.0, 0.0, -1.0, 1e-300, true, true, 1.0, 0.0,
        project::tool::SpindleDirection::Cw,
    ));
    assert!(failure.is_err());
    println!("Thread: actual helper panics on finite tiny pitch, before allocation");
    let kr = 30_000_i32; // radius 3 mm / cell 0.0001 mm
    let side = 2 * kr + 1;
    assert!(side.checked_mul(side).is_none());
    println!("Relief: kernel capacity i32 multiplication overflows at R/cell=30000");
    let lo = 1e16_f64;
    let hi = lo + 10.0;
    assert!(lo < hi - 1e-9 && lo + 0.5 == lo);
    println!("Relief: finite axis increment stalls (one arithmetic step only)");
}
'''
source = source.replace("Z_LEVELS", waterline[start:end])
with tempfile.TemporaryDirectory(prefix="fabcam-numsafe3-") as tmp:
    path = Path(tmp)
    (path / "thread.rs").write_text(thread_source)
    (path / "probe.rs").write_text(source.replace("THREAD_PATH", str(path / "thread.rs")))
    subprocess.run(
        ["rustc", "--edition=2021", "-C", "overflow-checks=yes",
         str(path / "probe.rs"), "-o", str(path / "probe")],
        check=True,
    )
    subprocess.run([str(path / "probe")], check=True, timeout=5)
