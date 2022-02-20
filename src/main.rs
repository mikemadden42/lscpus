use std::{io, thread};

fn main() -> io::Result<()> {
    // https://twitter.com/yoshuawuyts/status/1479776297115136000
    let count = thread::available_parallelism()?.get();
    assert!(count >= 1_usize);
    // https://blog.rust-lang.org/2022/01/13/Rust-1.58.0.html#captured-identifiers-in-format-strings
    println!("cores: {count}");
    Ok(())
}
