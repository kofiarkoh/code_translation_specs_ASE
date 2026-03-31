use std::io::{self, Read, Write};

fn main() {
    const CAP: usize = 114_514;
    let mut buf = vec![0u8; CAP];

    // Single read from stdin. No explicit error handling per spec; on error we
    // use a sentinel that will likely lead to a panic below (undefined behavior).
    let l: isize = match io::stdin().read(&mut buf) {
        Ok(n) => n as isize,
        Err(_) => -1,
    };

    // Spec: decrement L, then access buf[0] and buf[L-2] (i.e., buf[n-1]).
    // If l < 2 or l < 0, behavior is undefined; we intentionally allow panic.
    let n: isize = l - 1;

    let eq: isize = if buf[0] == buf[(n - 1) as usize] { 1 } else { 0 };
    let cond: isize = (n + eq) & 1;

    let mut stdout = io::stdout();
    if cond != 0 {
        let _ = stdout.write_all(b"First\n");
    } else {
        let _ = stdout.write_all(b"Second\n");
    }
}
