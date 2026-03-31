use std::io::{self, Read, Write};

fn main() {
    // Fixed-size internal buffer: reads at most 114514 bytes in one call.
    let mut buf = vec![0u8; 114_514];

    // Perform a single read from stdin. Rust's Read::read maps to one OS read call.
    // No explicit error handling is performed; on error we emulate "undefined behavior"
    // by exiting without a defined output.
    let l: usize = match io::stdin().read(&mut buf) {
        Ok(n) => n,
        Err(_) => return,
    };

    // Specification: behavior is undefined if fewer than 2 bytes are read.
    // We avoid out-of-bounds in Rust by exiting early; this keeps behavior unspecified.
    if l < 2 {
        return;
    }

    // Let n = L - 1 (after decrement).
    let n = l - 1;

    // eq is 0 or 1 depending on whether buf[0] == buf[L-2] (i.e., buf[n-1]).
    let eq: usize = (buf[0] == buf[n - 1]) as usize;

    // Condition: ((n + eq) & 1)
    let cond = (n + eq) & 1;

    let out = if cond != 0 { b"First\n" } else { b"Second\n" };

    // Write exactly one line to stdout.
    let mut stdout = io::stdout().lock();
    let _ = stdout.write_all(out);
}
