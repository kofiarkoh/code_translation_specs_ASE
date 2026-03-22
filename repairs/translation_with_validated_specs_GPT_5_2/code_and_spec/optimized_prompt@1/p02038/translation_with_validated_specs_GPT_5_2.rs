use std::io::{self, Read};
use std::process;

fn souti(a: char, b: char) -> char {
    if a == 'T' && b == 'F' { 'F' } else { 'T' }
}

fn main() {
    // Fixed-size internal storage, matching the C program's capacity.
    const CAP: usize = 114_514;
    let mut p: [char; CAP] = ['\0'; CAP];

    // Read entire stdin, then parse whitespace-separated tokens (scanf-like).
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        process::exit(1);
    }
    let mut it = input.split_whitespace();

    let n: usize = match it.next().and_then(|s| s.parse::<usize>().ok()) {
        Some(v) => v,
        None => process::exit(1), // count read failure => non-zero exit, no guaranteed output
    };

    // Per spec: if n < 2 or n > CAP, behavior is undefined; we do not add explicit handling.
    for i in 0..n {
        let tok = match it.next() {
            Some(t) => t,
            None => process::exit(1), // token read failure => non-zero exit, no guaranteed output
        };
        // Only the first character of each token is used.
        let ch = tok.chars().next().unwrap(); // tokens provide at least one character (precondition)
        p[i] = ch;
    }

    // Left-associative reduction.
    let mut acc = souti(p[0], p[1]);
    for i in 2..n {
        acc = souti(acc, p[i]);
    }

    print!("{acc}\n");
}
