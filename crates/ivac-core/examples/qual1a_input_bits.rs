//! Diagnostic only: expose the actual `serde_json` decimal-to-binary boundary.
use serde_json::{json, Value};

fn visit(value: &Value, path: &str, out: &mut Vec<Value>) {
    match value {
        Value::Number(n) => {
            if let Some(x) = n.as_f64() {
                out.push(json!({"path": path, "bits": format!("{:016x}", x.to_bits())}));
            }
        }
        Value::Array(values) => {
            for (i, item) in values.iter().enumerate() {
                visit(item, &format!("{path}/{i}"), out);
            }
        }
        Value::Object(values) => {
            for (key, item) in values {
                visit(item, &format!("{path}/{key}"), out);
            }
        }
        _ => {}
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1).ok_or("expected request path")?;
    let value: Value = serde_json::from_slice(&std::fs::read(path)?)?;
    let mut rows = Vec::new();
    visit(&value, "", &mut rows);
    println!("{}", serde_json::to_string(&rows)?);
    Ok(())
}
