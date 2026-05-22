use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse as whitespace-delimited tokens (like scanf).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = it.next().unwrap().parse().unwrap();
    let s = it.next().unwrap();

    // Per spec: consider only the first n characters; precondition says s has at least n chars.
    // The C code indexes bytes; here we assume ASCII input (as implied by "ABC").
    let bytes = s.as_bytes();
    let n_usize = if n < 0 { 0 } else { n as usize };

    let mut count: i32 = 0;
    let mut i: usize = 0;
    while i + 2 < n_usize {
        if bytes[i] == b'A' && bytes[i + 1] == b'B' && bytes[i + 2] == b'C' {
            count += 1;
            i += 3; // non-overlapping: skip past this occurrence
        } else {
            i += 1;
        }
    }

    print!("{}", count);
}
