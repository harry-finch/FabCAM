from pathlib import Path
import json,subprocess
root=Path('/Users/mugen/Documents/01_Projets/FabCAM/qual-1-checkout'); backup={}
def edit(rel,a,b):
 p=root/rel;s=p.read_text();backup.setdefault(rel,s);assert a in s,(rel,a);p.write_text(s.replace(a,b,1))
def append(rel,s):
 p=root/rel;old=p.read_text();backup.setdefault(rel,old);p.write_text(old+s)
append('crates/ivac-core/src/lib.rs','\n#[doc(hidden)]\npub mod qual1_trace;\n')
(root/'crates/ivac-core/src/qual1_trace.rs').write_text('''use std::cell::RefCell;
use serde_json::{Value,json};
thread_local! { static TRACE: RefCell<Vec<Value>> = const { RefCell::new(Vec::new()) }; }
pub fn record(stage: &str, value: Value) {
 fn bits(v: &Value) -> Value { match v {
  Value::Number(n) => json!({"value":n,"f64_bits":format!("{:016x}",n.as_f64().unwrap().to_bits())}),
  Value::Array(a) => Value::Array(a.iter().map(bits).collect()),
  Value::Object(o) => Value::Object(o.iter().map(|(k,v)|(k.clone(),bits(v))).collect()),
  _ => v.clone()
 }}
 TRACE.with(|t|t.borrow_mut().push(json!({"stage":stage,"data":value,"bits":bits(&value)})));
}
pub fn take() -> Vec<Value> { TRACE.with(|t| std::mem::take(&mut *t.borrow_mut())) }
''')
f='crates/ivac-core/src/pipeline/op_drivers/vcarve.rs'
edit(f,'    let diameter = tool.effective_diameter();','    crate::qual1_trace::record("parsed-input", serde_json::json!({"segments":project.segments,"tool":tool,"op":op,"machine":project.machine}));\n    let diameter = tool.effective_diameter();')
edit(f,'    let r_cap = params.carve_max_width_mm.unwrap_or(reach).min(reach);','    crate::qual1_trace::record("tool-tan", serde_json::json!([tool.tip_angle_deg.to_radians(),tan]));\n    let r_cap = params.carve_max_width_mm.unwrap_or(reach).min(reach);')
edit(f,'        let holes = &region.holes;','        let holes = &region.holes;\n        crate::qual1_trace::record("prepared-region", serde_json::json!({"boundary":boundary,"holes":holes}));')
edit(f,'            for axis in &axes {','            crate::qual1_trace::record("medial-axes", serde_json::json!(axes.iter().map(|a|a.iter().map(|p|[p.x,p.y,p.r]).collect::<Vec<_>>()).collect::<Vec<_>>()));\n            for axis in &axes {')
edit(f,'            let path = vcarve_plan::plan(&target, dpp, angle).map_err(|e| invalid(&e))?;','            crate::qual1_trace::record("target",serde_json::json!(target));\n            let path = vcarve_plan::plan(&target, dpp, angle).map_err(|e| invalid(&e))?;\n            crate::qual1_trace::record("planned-path",serde_json::json!(path));')
f='crates/ivac-core/src/cam/vcarve.rs'
edit(f,'    crate::profile_count!("medial.samples", samples.len());','    crate::qual1_trace::record("samples",serde_json::json!(samples));\n    crate::profile_count!("medial.samples", samples.len());')
edit(f,'    let n_tri = tri.len();','    crate::qual1_trace::record("triangulation",serde_json::json!({"triangles":tri.triangles,"halfedges":tri.halfedges}));\n    let n_tri = tri.len();')
edit(f,'    let boundary_segments = collect_boundary_segments(region);','    crate::qual1_trace::record("centers",serde_json::json!(centers));\n    let boundary_segments = collect_boundary_segments(region);')
f='crates/ivac-core/src/cam/vcarve_plan.rs'
edit(f,'    let slope = angle.to_radians().tan();','    let slope = angle.to_radians().tan();\n    crate::qual1_trace::record("plan-start",serde_json::json!({"axis":axis,"step":step,"angle":angle,"slope":slope,"lengths":axis.windows(2).map(|p|(p[1].0-p[0].0).hypot(p[1].1-p[0].1)).collect::<Vec<_>>()}));')
edit(f,'        let complete = next.iter().all','        crate::qual1_trace::record("sweep-before-simplify",serde_json::json!(next.iter().map(|p|[p.x,p.y,p.target,p.cut]).collect::<Vec<_>>()));\n        let complete = next.iter().all')
edit(f,'        simplify(&mut next);','        simplify(&mut next);\n        crate::qual1_trace::record("sweep-after-simplify",serde_json::json!(next.iter().map(|p|[p.x,p.y,p.target,p.cut]).collect::<Vec<_>>()));')
append('crates/ivac-wasm/src/lib.rs','\n#[wasm_bindgen]\npub fn qual1_take_trace() -> String { serde_json::to_string(&ivac_core::qual1_trace::take()).unwrap() }\n')
f='crates/ivac-core/examples/perf1_probe.rs'
edit(f,'    let report = json!({"request":','    fs::write(format!("{}.trace.json",args[2]),serde_json::to_vec(&ivac_core::qual1_trace::take())?)?;\n    let report = json!({"request":')
Path('dist-test/qual1/diagnostic-originals.json').write_text(json.dumps(backup))
Path('dist-test/qual1/diagnostic.patch').write_bytes(subprocess.check_output(['git','-C',str(root),'diff']))
print('Diagnostic-only instrumentation in isolated checkout; original production tree untouched')
