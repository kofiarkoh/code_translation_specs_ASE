use std::io::{self, Read, Write};

fn main() {
    const CAP: usize = 114_514;
    let mut buf = vec![0u8; CAP];

    // Single read from stdin; no retries/looping.
    let l = io::stdin().read(&mut buf).unwrap() as usize;

    // Undefined behavior for l < 2 per spec; we intentionally do not guard it.
    let n = l.wrapping_sub(1);
    let eq: usize = if buf[0] == buf[n.wrapping_sub(1)] { 1 } else { 0 };
    let cond = (n.wrapping_add(eq)) & 1;

    let mut stdout = io::stdout();
    if cond != 0 {
        let _ = stdout.write_all(b"First\n");
    } else {
        let _ = stdout.write_all(b"Second\n");
    }
}
