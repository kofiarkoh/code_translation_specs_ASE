use std::io::{self, Read};

fn main() {
    // Read entire stdin (matches typical competitive-programming style; no extra prompts).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let a: i32 = it.next().unwrap().parse().unwrap();
    let b: i32 = it.next().unwrap().parse().unwrap();

    // Spec assumes non-negative inputs and non-zero divisor; no explicit error handling.
    // Match C semantics: integer division truncates toward zero; remainder non-zero => increment.
    let mut c = a / b;
    if a % b != 0 {
        c += 1;
    }

    print!("{c}\n");
}
