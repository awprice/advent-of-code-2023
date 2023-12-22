use std::env;
use anyhow::{anyhow, Result};

pub fn read_input() -> Result<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(anyhow!("Not enough args"));
    }
    let input = std::fs::read_to_string( &args[1])?;
    Ok(input)
}