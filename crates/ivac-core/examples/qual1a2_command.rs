//! Feed original JSON bytes through the current preset command contract.
use std::io::{self, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let request = serde_json::from_str(&input)?;
    let response = ivac_core::presets::command(request)?;
    println!("{}", serde_json::to_string(&response)?);
    Ok(())
}
