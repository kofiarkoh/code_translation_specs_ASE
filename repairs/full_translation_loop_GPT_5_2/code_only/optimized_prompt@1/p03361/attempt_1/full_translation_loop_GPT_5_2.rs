use std::io::{self, Read};

fn main() {
    // Read entire stdin, split by whitespace like scanf("%d %d") and scanf("%s")
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let h: i32 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let w: i32 = it.next().unwrap().parse().unwrap();

    // Emulate: char s[51][60] = {""};
    // In C this zero-initializes the whole array.
    // We'll keep a fixed-size grid of bytes initialized to 0.
    let mut s = vec![[0u8; 60]; 51];

    // Read H strings into s[i] like scanf("%s", s[i]);
    // Note: C would overflow if a token > 59 chars; we assume inputs fit.
    for i in 0..(h as usize) {
        if let Some(tok) = it.next() {
            let bytes = tok.as_bytes();
            let len = bytes.len().min(60);
            s[i][..len].copy_from_slice(&bytes[..len]);
            // Remaining bytes stay 0 due to initialization.
        }
    }

    for i in 0..(h as usize) {
        for j in 0..(w as usize) {
            let u: u8 = if i as i32 - 1 < 0 { 0 } else { s[i - 1][j] };
            let l: u8 = if j as i32 - 1 < 0 { 0 } else { s[i][j - 1] };

            // In the C code, these are read unconditionally:
            // d = s[i + 1][j]; r = s[i][j + 1];
            // This is out-of-bounds when i==H-1 or j==W-1 (UB in C).
            // We approximate typical behavior by reading from the zero-initialized
            // backing array when within its declared bounds; otherwise use 0.
            let d: u8 = if i + 1 < 51 { s[i + 1][j] } else { 0 };
            let r: u8 = if j + 1 < 60 { s[i][j + 1] } else { 0 };

            if s[i][j] == b'#' {
                if u == b'#' || d == b'#' || r == b'#' || l == b'#' {
                    continue;
                } else {
                    print!("No");
                    return;
                }
            }
        }
    }

    print!("Yes");
}
