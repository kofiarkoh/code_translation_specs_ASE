use std::io::{self, Read};

fn main() {
    // Read entire stdin (matches typical competitive-programming style; no extra prompts).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let a: i64 = it.next().unwrap().parse().unwrap();
    let b: i64 = it.next().unwrap().parse().unwrap();

    let sum = a + b;

    if sum % 2 != 0 {
        print!("IMPOSSIBLE");
        return;
    }

    // Spec: output integer half of the sum when even.
    // Note: C code prints with "%d" despite using long int; spec governs behavior, so we print the value.
    print!("{}", sum / 2);
}
