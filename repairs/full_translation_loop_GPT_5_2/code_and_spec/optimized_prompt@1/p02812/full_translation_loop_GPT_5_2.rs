use std::io::{self, Read};

fn main() {
    // Read entire stdin (matches scanf behavior for whitespace-separated tokens).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = it.next().unwrap_or("0").parse().unwrap_or(0);
    let s = it.next().unwrap_or("");

    // Spec: if n < 3, output 0.
    if n < 3 {
        print!("0");
        return;
    }

    // C code indexes bytes; the intended substring is ASCII "ABC", so byte-wise is correct.
    let bytes = s.as_bytes();
    let n_usize = n as usize;

    let mut count = 0i32;
    let mut i = 0usize;

    // Spec precondition: string contains at least n characters; otherwise undefined.
    // We follow the spec and do not add extra error handling beyond the n < 3 case.
    while i + 2 < n_usize {
        if bytes[i] == b'A' && bytes[i + 1] == b'B' && bytes[i + 2] == b'C' {
            count += 1;
            i += 3; // non-overlapping: skip past this "ABC"
        } else {
            i += 1;
        }
    }

    print!("{}", count);
}
