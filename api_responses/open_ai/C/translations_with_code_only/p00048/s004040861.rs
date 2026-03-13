// Translated from the provided C program.
// Behavior: read whitespace-separated floating-point numbers from stdin and,
// for each number, print the corresponding weight class on its own line.
//
// Note: The original C used `while (scanf("%lf", &w) != EOF)`. That code
// would behave oddly on non-numeric input (scanf would return 0 and, because
// the conversion fails without consuming input, could lead to an infinite
// loop). Here we make a pragmatic, safer choice: we split stdin on
// whitespace and attempt to parse each token as f64; tokens that fail to
// parse are skipped. This preserves the intended behavior for normal
// numeric inputs while avoiding pathological infinite loops.
//
// Exact output formatting (one class name per line) is preserved.

use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    // Read entire stdin (like repeated scanf until EOF)
    io::stdin().read_to_string(&mut input).unwrap();

    for token in input.split_whitespace() {
        // Try to parse each whitespace-separated token as f64.
        // If parsing fails, skip the token (see note above).
        let w: f64 = match token.parse() {
            Ok(v) => v,
            Err(_) => continue,
        };

        let s = if w <= 48.0 {
            "light fly"
        } else if w <= 51.0 {
            "fly"
        } else if w <= 54.0 {
            "bantam"
        } else if w <= 57.0 {
            "feather"
        } else if w <= 60.0 {
            "light"
        } else if w <= 64.0 {
            "light welter"
        } else if w <= 69.0 {
            "welter"
        } else if w <= 75.0 {
            "light middle"
        } else if w <= 81.0 {
            "middle"
        } else if w <= 91.0 {
            "light heavy"
        } else {
            "heavy"
        };

        println!("{}", s);
    }
}
