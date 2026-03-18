use std::io::{self, Read, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Reads three integers: N, A, B. N is not used for the decision per specification.
    let _n: i128 = it.next().unwrap().parse().unwrap();
    let a: i128 = it.next().unwrap().parse().unwrap();
    let b: i128 = it.next().unwrap().parse().unwrap();

    // Winner depends on the parity of (B - A).
    // Observed behavior indicates: even -> "Alice", odd -> "Borys".
    let diff = b - a;
    let out = if diff % 2 == 0 { "Alice" } else { "Borys" };

    let mut stdout = io::stdout().lock();
    write!(stdout, "{}", out).unwrap();
    stdout.flush().unwrap();
}
