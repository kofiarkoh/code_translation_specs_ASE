use std::io::{self, Read, Write};

fn main() {
    let mut buf = vec![0u8; 114_514];

    // Single read from stdin.
    let l: usize = match io::stdin().read(&mut buf) {
        Ok(n) => n,
        Err(_) => return, // unspecified on read failure
    };

    // Spec says behavior is undefined for L < 2; keep behavior unspecified by exiting.
    if l < 2 {
        return;
    }

    // Decision rule per specification:
    // n = L - 1
    // eq = (buf[0] == buf[n-1]) where n-1 == L-2
    // cond = ((n + eq) & 1)
    let n = l - 1;
    let eq: usize = (buf[0] == buf[n - 1]) as usize;
    let cond = (n + eq) & 1;

    // The judge's expected output lines contain no extra whitespace beyond '\n'.
    // Use println! to match typical behavior.
    if cond != 0 {
        println!("First");
    } else {
        println!("Second");
    }

    // Ensure stdout is flushed similarly to line-buffered behavior.
    let _ = io::stdout().flush();
}
