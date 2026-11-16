use std::{io, thread};

fn main() -> io::Result<()> {
    // https://doc.rust-lang.org/std/thread/fn.available_parallelism.html
    // Reports usable parallelism (respects affinity and cgroup quotas), not physical cores.
    let count = thread::available_parallelism()?.get();
    // https://blog.rust-lang.org/2022/01/13/Rust-1.58.0.html#captured-identifiers-in-format-strings
    println!("available parallelism: {count}");
    Ok(())
}
