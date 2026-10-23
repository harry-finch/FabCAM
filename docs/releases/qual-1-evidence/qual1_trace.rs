use std::cell::RefCell;
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
