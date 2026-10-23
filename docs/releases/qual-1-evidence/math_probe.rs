#[no_mangle]
pub extern "C" fn probe_tan(x: f64) -> f64 { x.tan() }
#[no_mangle]
pub extern "C" fn probe_hypot(x: f64,y: f64) -> f64 { x.hypot(y) }
#[cfg(not(target_arch="wasm32"))]
fn main() {
 let a: Vec<f64> = std::env::args().skip(1).map(|s|s.parse().unwrap()).collect();
 println!("{:016x} {:016x}",probe_tan(a[0]).to_bits(),probe_hypot(a[1],a[2]).to_bits());
}
