use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let s = fs::read_to_string("input/input.txt")?;

    Ok(())
}
