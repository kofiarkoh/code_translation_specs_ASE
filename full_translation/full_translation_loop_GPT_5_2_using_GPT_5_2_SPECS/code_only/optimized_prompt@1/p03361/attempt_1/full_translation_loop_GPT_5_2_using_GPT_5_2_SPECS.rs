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

    // C: char s[51][60] = {""};
    // We'll model as fixed-size 2D array of bytes, zero-initialized.
    // Important: the C code reads strings into s[i] (null-terminated), but later
    // it also reads s[i][j+1] and s[i+1][j] without bounds checks, which in C is UB.
    // Here we emulate the *likely intended* behavior by keeping the array zeroed
    // and only writing the provided bytes; out-of-range accesses within the fixed
    // array yield 0, similar to reading zero-initialized padding.
    let mut s = vec![[0u8; 60]; 51];

    for i in 0..(h as usize) {
        if let Some(tok) = it.next() {
            let bytes = tok.as_bytes();
            let max = bytes.len().min(59); // leave room for implicit '\0' like C
            s[i][..max].copy_from_slice(&bytes[..max]);
            // s[i][max] is already 0 due to initialization
        } else {
            // If input is missing lines, leave row as zeros (matches zero-init behavior)
        }
    }

    for i in 0..(h as usize) {
        for j in 0..(w as usize) {
            let u: u8 = if i == 0 { 0 } else { s[i - 1][j] };
            let l: u8 = if j == 0 { 0 } else { s[i][j - 1] };

            // In the C code, these are read unconditionally; we keep the same indexing
            // within the fixed 51x60 buffer.
            let d: u8 = s[i + 1][j];
            let r: u8 = s[i][j + 1];

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
