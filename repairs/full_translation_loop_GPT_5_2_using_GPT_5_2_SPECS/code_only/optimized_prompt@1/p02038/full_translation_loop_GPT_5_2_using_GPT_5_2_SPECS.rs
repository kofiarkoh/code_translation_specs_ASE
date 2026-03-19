use std::io::{self, Read};

fn souti(a: u8, b: u8) -> u8 {
    if a == b'T' && b == b'F' { b'F' } else { b'T' }
}

fn main() {
    // Read entire stdin, then parse tokens like scanf("%d") and scanf("%3s").
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        std::process::exit(1);
    }
    let mut it = input.split_whitespace();

    let n: usize = match it.next().and_then(|t| t.parse::<i32>().ok()) {
        Some(v) => v as usize, // C stores into int; negative would be UB-ish for loop bounds; keep cast behavior.
        None => std::process::exit(1),
    };

    // C has a fixed-size global array P[114514]; we emulate with a fixed buffer.
    let mut p = vec![0u8; 114_514];

    for i in 0..n {
        let tok = match it.next() {
            Some(t) => t,
            None => std::process::exit(1),
        };
        // scanf("%3s", hoge) reads up to 3 non-whitespace chars; we only use hoge[0].
        // If token is empty (can't happen with split_whitespace), we'd be in trouble; mimic failure by exiting.
        let first = match tok.as_bytes().first() {
            Some(&c) => c,
            None => std::process::exit(1),
        };
        if i < p.len() {
            p[i] = first;
        } else {
            // In C this would overflow P; behavior undefined. We choose to exit to avoid panic.
            std::process::exit(1);
        }
    }

    // The C code assumes N >= 2; otherwise it reads uninitialized P[1] etc. (UB).
    // We emulate the same indexing behavior by exiting on too-small N to avoid Rust panics.
    if n < 2 {
        std::process::exit(1);
    }

    let mut notomamiko = souti(p[0], p[1]);
    for i in 2..n {
        notomamiko = souti(notomamiko, p[i]);
    }

    print!("{}\n", notomamiko as char);
}
