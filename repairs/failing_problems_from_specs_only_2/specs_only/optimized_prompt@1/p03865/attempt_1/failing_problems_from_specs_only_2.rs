use std::io::{self, Read, Write};

fn main() {
    const CAP: usize = 114_514;
    let mut buf = vec![0u8; CAP];

    // Single unbuffered read attempt. We intentionally do not add any explicit
    // error handling beyond mapping errors to a sentinel that will likely
    // trigger a panic below, reflecting the spec's "undefined behavior".
    let l: isize = match io::stdin().read(&mut buf) {
        Ok(n) => n as isize,
        Err(_) => -1,
    };

    // The specification states the original program decrements L and then
    // accesses buf[0] and buf[L-2]. If read fails or returns < 2 bytes,
    // behavior is undefined; here we allow a panic via out-of-bounds indexing.
    let n: isize = l - 1;

    let eq: isize = if buf[0] == buf[(n - 1) as usize] { 1 } else { 0 };

    let cond: isize = (n + eq) & 1;

    let out = if cond != 0 { b"First\n" } else { b"Second\n" };
    let _ = io::stdout().write_all(out);
}
