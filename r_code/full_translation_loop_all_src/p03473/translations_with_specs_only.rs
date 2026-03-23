use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Narrow interpretation: read one integer token from stdin.
    // If missing/malformed/out of range, behavior is unspecified; we allow panic via unwrap().
    let x: i128 = input.split_whitespace().next().unwrap().parse().unwrap();

    // Compute exactly (48 - input_value).
    let result: i128 = 48 - x;

    // Output one integer followed by a line terminator.
    println!("{}", result);
}
