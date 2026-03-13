// leap_years.rs
//
// Translates the provided specification into Rust.
// Reads pairs of integers (i32) from stdin until the pair (0, 0) is encountered.
// For each non-terminating pair (a, b), prints all leap years in [a, b],
// one per line in ascending order, or "NA" if none, followed by a blank line.
//
// Notes on fidelity to the specification:
// - Parsing uses unwrap() so that malformed input or premature EOF will cause a panic,
//   reflecting the specification's statement that such cases are not handled (unspecified).
// - Uses i32 to model "standard C int" behavior as closely as is reasonable here.
// - Divisibility is tested by remainder == 0, matching the spec (works for negative years too).

use std::io::{self, Read};

fn is_leap_year(y: i32) -> bool {
    (y % 400 == 0) || ((y % 4 == 0) && (y % 100 != 0))
}

fn main() {
    // Read all stdin into a string. unwrap() will panic on I/O errors;
    // this is acceptable per the specification (no special error handling).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Split by whitespace and parse tokens into i32.
    // unwrap() on parse will panic on malformed input, matching "unspecified" behavior.
    let mut tokens = input.split_whitespace().map(|t| t.parse::<i32>().unwrap());

    loop {
        // Obtain the next two integers. unwrap() will panic if EOF / missing tokens.
        let a = tokens.next().unwrap();
        let b = tokens.next().unwrap();

        // Termination condition: do not print anything for (0, 0), just exit.
        if a == 0 && b == 0 {
            break;
        }

        let mut found = false;

        // If a <= b, iterate inclusive. If a > b, treat as empty range and print "NA".
        if a <= b {
            // RangeInclusive will iterate from a to b by 1; the spec guarantees no overflow in such cases.
            for y in a..=b {
                if is_leap_year(y) {
                    println!("{}", y);
                    found = true;
                }
            }
        }

        if !found {
            println!("NA");
        }

        // Print an additional blank line after each pair's output block.
        println!();
    }

    // Normal termination (reaching here after encountering 0 0) returns exit code 0.
}
