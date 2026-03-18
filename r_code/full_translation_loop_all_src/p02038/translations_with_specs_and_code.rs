use std::io::{self, Read};
use std::process;

fn souti(a: char, b: char) -> char {
    if a == 'T' && b == 'F' { 'F' } else { 'T' }
}

fn main() {
    // Read entire stdin, then parse as whitespace-separated tokens (matches scanf("%d") and scanf("%3s")).
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        process::exit(1);
    }
    let mut it = input.split_whitespace();

    let n: usize = match it.next().and_then(|t| t.parse::<usize>().ok()) {
        Some(v) => v,
        None => process::exit(1),
    };

    // SPEC precondition: n >= 2. The C code does not enforce it, but the spec requires it.
    if n < 2 {
        process::exit(1);
    }

    // Read N tokens; each token corresponds to scanf("%3s", hoge) and we take hoge[0].
    let mut p: Vec<char> = Vec::with_capacity(n);
    for _ in 0..n {
        let tok = match it.next() {
            Some(t) => t,
            None => process::exit(1), // "not read successfully" => non-zero exit
        };
        let ch = match tok.chars().next() {
            Some(c) => c,
            None => process::exit(1),
        };
        // SPEC invariant: only 'T' or 'F' are valid.
        if ch != 'T' && ch != 'F' {
            process::exit(1);
        }
        p.push(ch);
    }

    let mut acc = souti(p[0], p[1]);
    for i in 2..n {
        acc = souti(acc, p[i]);
    }

    print!("{acc}\n");
}
