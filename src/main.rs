use serde_json::{from_str, to_vec, Value};
use std::io::{self, Read, Write};

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    let value: Value = from_str(&buffer)?;
    let out = to_vec(&value)?;
    io::stdout().write_all(&out)?;
    Ok(())
}
