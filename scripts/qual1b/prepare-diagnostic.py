"""Build a separate, never-shipped diagnostic source copy. No product escape hatch."""
from pathlib import Path
import json, subprocess
root=Path(__file__).resolve().parents[2]
out=root/'dist-test/qual1b'; copy=out/'diagnostic-source';copy.mkdir(exist_ok=False)
archive=subprocess.Popen(['git','archive','HEAD'],cwd=root,stdout=subprocess.PIPE)
subprocess.run(['tar','-x','-C',str(copy)],stdin=archive.stdout,check=True); assert archive.wait()==0
# Copy only the reviewed runtime delta. The independent source copy stays off Git.
for name in ['crates/ivac-cps/posts/shopbot.cps','crates/ivac-cps/src/run.rs']:
 (copy/name).write_bytes((root/name).read_bytes())
original={}
def edit(name,a,b):
 p=copy/name;s=p.read_text();original.setdefault(name,s);assert a in s,(name,a);p.write_text(s.replace(a,b,1))
f='crates/ivac-cps/src/run.rs'
edit(f,'    let check_cancel = || {','    let qual_time = std::time::Instant::now();\n    eprintln!("QUAL1B run_start 0");\n    let check_cancel = || {')
edit(f,'    let mut engine = Engine::with_cancel(cancel.cloned());','    let mut engine = Engine::with_cancel(cancel.cloned());\n    eprintln!("QUAL1B init {}",qual_time.elapsed().as_secs_f64()*1000.);')
edit(f,'    check_cancel()?;\n    let map = evaluated?;','    eprintln!("QUAL1B eval {}",qual_time.elapsed().as_secs_f64()*1000.);\n    check_cancel()?;\n    let map = evaluated?;')
edit(f,'    let program_json =','    if let Ok(path) = std::env::var("QUAL1B_IR") { std::fs::write(path, serde_json::to_vec(program).unwrap()).unwrap(); }\n    let program_json =')
edit(f,'    check_cancel()?;\n    let executed =','    eprintln!("QUAL1B convert {}",qual_time.elapsed().as_secs_f64()*1000.);\n    check_cancel()?;\n    let executed =')
edit(f,'    check_cancel()?;\n    let envelope = executed.map_err','    eprintln!("QUAL1B execute_return {}",qual_time.elapsed().as_secs_f64()*1000.);\n    if cancel.is_some_and(|f|f.load(Ordering::Relaxed)) {\n        drop(engine);\n        eprintln!("QUAL1B cleanup {}",qual_time.elapsed().as_secs_f64()*1000.);\n        return Err(PostError::Cancelled);\n    }\n    check_cancel()?;\n    let envelope = executed.map_err')
f='crates/ivac-cps/src/engine.rs'
edit(f,'        let log = NativeFunction::from_copy_closure(|_this, args, ctx| {','        let log = NativeFunction::from_copy_closure_with_captures(|_this, args, probe, ctx| {')
edit(f,'            tracing::debug!(target: "ivac_cps::post", "{text}");\n            Ok(JsValue::undefined())\n        });','''            if text.starts_with("QUAL1B") {
                eprintln!("{text} {:?}", std::time::Instant::now());
                if std::env::var("QUAL1B_INJECT").ok().as_deref() == Some(text.as_str()) {
                    probe.flag.as_ref().unwrap().store(true, Ordering::Relaxed);
                    eprintln!("QUAL1B signalled {:?}", std::time::Instant::now());
                }
            }
            tracing::debug!(target: "ivac_cps::post", "{text}");
            Ok(JsValue::undefined())
        }, AbortProbe { flag: cancel.clone() });''')
edit(f,'                Ok(JsValue::from(\n                    probe','                if probe.flag.as_ref().is_some_and(|f|f.load(Ordering::Relaxed)) { eprintln!("QUAL1B detected {:?}",std::time::Instant::now()); }\n                Ok(JsValue::from(\n                    probe')
f='crates/ivac-cps/posts/shopbot.cps'
for a,b in [('function onOpen() {','function onOpen() { log("QUAL1B onOpen");'),('function preflight() {','function preflight() { log("QUAL1B preflight");'),('var r = s.records[j];','if (j == 65) log("QUAL1B record65"); var r = s.records[j];'),('preflight(); // whole program checked before any output','preflight(); log("QUAL1B preflight_done"); // whole program checked before any output')]: edit(f,a,b)
# Preserve both exactly identified instrumented posts for the two build steps.
(out/'diagnostic-candidate-post.cps').write_bytes((copy/f).read_bytes())
control=subprocess.check_output(['git','show','8e2c9ce427007f9c5a3787b34dd28647b8cdcf95:'+f],cwd=root).decode()
for a,b in [('function onOpen() {','function onOpen() { log("QUAL1B onOpen");'),('function preflight() {','function preflight() { log("QUAL1B preflight");'),('var r = s.records[j];','if (j == 65) log("QUAL1B record65"); var r = s.records[j];'),('preflight(); // whole program checked before any output','preflight(); log("QUAL1B preflight_done"); // whole program checked before any output')]:
 assert a in control;control=control.replace(a,b,1)
(out/'diagnostic-control-post.cps').write_text(control)
# This diagnostic binary injects through the same actual CancelToken, with no timer
# thread. Post-only deterministic injection is labelled separately from production.
f='crates/ivac-core/examples/perf1_probe.rs'
edit(f,'        let worker = std::thread::spawn(move || {','        let worker = std::thread::spawn(move || {\n            if std::env::var("QUAL1B_INJECT").is_ok() { return 0.; }')
edit(f,'        let clone = cancel.clone();','        if std::env::var("QUAL1B_PRE_CANCEL").is_ok() { cancel.cancel(); }\n        let clone = cancel.clone();')
edit(f,'        let returned_ms = start.elapsed().as_secs_f64() * 1000.;','        eprintln!("QUAL1B pipeline_return {:?}", Instant::now());\n        let returned_ms = start.elapsed().as_secs_f64() * 1000.;')
# Captured IR is for a staged Boa probe; diagnostic I/O is outside timed execute.
(out/'diagnostic-originals.json').write_text(json.dumps(original,indent=2)+'\n')
(out/'diagnostic-source-files.json').write_text(json.dumps({n:(copy/n).read_text() for n in original},indent=2)+'\n')
print(copy)
