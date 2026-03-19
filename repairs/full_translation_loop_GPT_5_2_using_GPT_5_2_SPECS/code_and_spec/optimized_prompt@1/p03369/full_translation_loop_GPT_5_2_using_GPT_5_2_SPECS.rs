use std::io::{self, Read};

fn main() {
    // Read one whitespace-delimited token from stdin.
    // Per spec, missing input or token shorter than 3 characters is undefined behavior;
    // we therefore do not add explicit error handling beyond basic parsing.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let token = input.split_whitespace().next().unwrap();

    // Count occurrences of 'o' among the first three character positions.
    // The spec defines "characters"; we interpret this as Rust `char` (Unicode scalar values).
    // For typical ASCII inputs (as in the original C), this matches byte indexing behavior.
    let m = token.chars().take(3).filter(|&c| c == 'o').count() as i32;

    let score = 700 + 100 * m;
    print!("{score}\n");
}
