use std::io::{self, Read};

fn main() {
    let mut buf = [0u8; 114_514];

    // Single read from stdin (as per spec/C). On error, treat as 0 bytes.
    let l: usize = match io::stdin().read(&mut buf) {
        Ok(n) => n,
        Err(_) => 0,
    };

    // Spec says behavior is undefined for l < 2; avoid panic in Rust.
    if l < 2 {
        print!("Second\n");
        return;
    }

    // C: n = read(...); n--;
    let n = l - 1;

    // C: (buf[0] == buf[n-1]) where n-1 == l-2
    let eq: usize = if buf[0] == buf[n - 1] { 1 } else { 0 };

    // IMPORTANT: match C operator precedence exactly:
    // n + ( (buf[0]==buf[n-1]) & 1 )
    // because '&' has lower precedence than '+', so the C expression
    // `n+(buf[0]==buf[n-1])&1` parses as `(n + (buf[0]==buf[n-1])) & 1`.
    // Wait: In C, '+' has higher precedence than '&', so it is:
    // (n + (buf[0]==buf[n-1])) & 1
    // But the conditional uses the whole expression as condition.
    let cond = (n + eq) & 1;

    if cond != 0 {
        print!("First\n");
    } else {
        print!("Second\n");
    }
}
