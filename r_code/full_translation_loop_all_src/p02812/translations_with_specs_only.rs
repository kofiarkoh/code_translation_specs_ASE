use std::io::{self, Read};

fn main() {
    // Read entire stdin (matches typical competitive-programming style I/O).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let n: i64 = match it.next() {
        Some(tok) => tok.parse().unwrap_or(0),
        None => return,
    };

    let s = match it.next() {
        Some(tok) => tok,
        None => "",
    };

    // Error handling per spec: if n < 3, output 0.
    if n < 3 {
        println!("0");
        return;
    }

    // Spec says behavior is undefined if the string is shorter than n.
    // We choose the narrowest reasonable interpretation: only process up to min(n, s.len()).
    // This avoids panics and keeps behavior well-defined in Rust.
    let n_usize = if n < 0 { 0 } else { n as usize };
    let bytes = s.as_bytes();
    let limit = n_usize.min(bytes.len());

    let mut count: i64 = 0;
    let mut i: usize = 0;

    // Count non-overlapping occurrences of "ABC".
    while i + 2 < limit {
        if bytes[i] == b'A' && bytes[i + 1] == b'B' && bytes[i + 2] == b'C' {
            count += 1;
            i += 3; // non-overlapping: skip past this match
        } else {
            i += 1;
        }
    }

    println!("{}", count);
}
