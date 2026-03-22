use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated tokens (matches scanf-style behavior).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = it.next().unwrap().parse().unwrap();

    // Invariant per spec: even_factor == 2^E_p after processing any prefix.
    let mut even_factor: i32 = 1;

    for _ in 0..n {
        let a: i32 = it.next().unwrap().parse().unwrap();
        if a % 2 == 0 {
            even_factor *= 2;
        }
    }

    // Invariant per spec: pow3 == 3^N after this loop.
    let mut pow3: i32 = 1;
    for _ in 0..n {
        pow3 *= 3;
    }

    // Output exactly one integer, no trailing newline (matches C printf("%d", ...)).
    print!("{}", pow3 - even_factor);
}
