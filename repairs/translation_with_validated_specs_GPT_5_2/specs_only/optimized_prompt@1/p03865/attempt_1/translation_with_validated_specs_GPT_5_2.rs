use std::io::{self, Read, Write};

fn main() {
    const CAP: usize = 114_514;
    let mut buf = vec![0u8; CAP];

    // Specification requires a single unbuffered read-like operation.
    // Rust's stdin().read performs one read attempt and returns the number of bytes read.
    let l = io::stdin().read(&mut buf).unwrap() as isize;

    // If read fails or returns fewer than 2 bytes, behavior is undefined per spec.
    // We intentionally do not add explicit error handling or bounds checks beyond what Rust enforces.
    let l_usize = l as usize;

    // n = L - 1 (after decrement in the spec)
    let n = l_usize.wrapping_sub(1);

    // eq = (buf[0] == buf[n-1]) ? 1 : 0, where n-1 == L-2
    let eq: usize = if buf[0] == buf[n.wrapping_sub(1)] { 1 } else { 0 };

    // condition = ((n + eq) & 1)
    let cond = (n.wrapping_add(eq)) & 1;

    let out = if cond != 0 { b"First\n" } else { b"Second\n" };
    let _ = io::stdout().write_all(out);
}
