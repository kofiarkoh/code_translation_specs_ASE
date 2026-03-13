// main.rs
// Translated from the given C program according to the provided specification.
// Behavior:
// - Reads whitespace-separated tokens from stdin until EOF.
// - For each token that parses as an `i32`, prints the number of ordered 4-tuples
//   (a,b,c,d) with each in 0..=9 whose sum equals the parsed integer, followed by a newline.
// - If a token does not parse as an integer, to reflect the original program's lack of
//   robustness described in the spec (where scanf("%d") would repeatedly fail on the same
//   token and the program would make no progress), this program will enter an infinite
//   loop (sleeping) when encountering the first non-integer token. This reproduces the
//   observable effect (the program hangs) while keeping the implementation safe and idiomatic.
//
// The counting is performed by enumerating all 10^4 combinations (bounded constant work).

use std::io::{self, Read};
use std::time::Duration;

fn main() {
    // Read entire stdin to a string. This matches the behavior of reading until EOF.
    let mut input = String::new();
    if let Err(e) = io::stdin().read_to_string(&mut input) {
        // If there is an I/O error reading stdin, exit (matching normal program termination).
        eprintln!("Error reading stdin: {}", e);
        return;
    }

    // Iterate over whitespace-separated tokens.
    for token in input.split_whitespace() {
        match token.parse::<i32>() {
            Ok(n) => {
                // Count number of 4-tuples (a,b,c,d) in 0..=9 with sum == n.
                // Use explicit i32 ranges to avoid type issues.
                let mut count: usize = 0;
                for a in 0i32..=9 {
                    for b in 0i32..=9 {
                        for c in 0i32..=9 {
                            for d in 0i32..=9 {
                                if a + b + c + d == n {
                                    count += 1;
                                }
                            }
                        }
                    }
                }
                // Print the count followed by a newline (println ensures newline).
                println!("{}", count);
            }
            Err(_) => {
                // As documented in the specification, the original C program using scanf("%d")
                // is not robust to non-integer tokens: scanf would return 0 and not consume the token,
                // causing the program to repeatedly attempt to read the same invalid token and make
                // no progress. To preserve that observable behavior (hang), we enter an infinite loop.
                //
                // Note: we do not attempt to replicate the exact low-level token non-consumption;
                // producing an infinite hang at this point reflects the same high-level effect.
                loop {
                    std::thread::sleep(Duration::from_secs(1));
                }
            }
        }
    }

    // Normal termination (implicit exit code 0).
}
