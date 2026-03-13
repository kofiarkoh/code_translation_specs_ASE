// main.rs
// Translated from the provided C program.
// Counts number of 4-tuples (a,b,c,d) with 0..=9 such that a+b+c+d == n,
// for each integer n read from stdin until EOF.
// Output: one decimal count per input, each followed by a newline.
//
// Implementation notes:
// - Precompute counts for sums 0..36 (10^4 combinations total).
// - For input values outside [0,36], print 0 (matching the C program behavior).
// - We read whitespace-separated tokens from stdin and attempt to parse them as integers.
//   The original C program used scanf("%d", &n) in a loop; that typically reads
//   whitespace-separated integers until EOF. We emulate that behavior.
use std::io::{self, Read};

fn main() {
    // Precompute counts for all possible sums 0..=36.
    let mut counts = [0i32; 37];
    for a in 0..=9 {
        for b in 0..=9 {
            for c in 0..=9 {
                for d in 0..=9 {
                    let s = a + b + c + d;
                    counts[s as usize] += 1;
                }
            }
        }
    }

    // Read all stdin and split on whitespace, parse integers and print results.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading stdin errors, behave as if EOF with no output.
        return;
    }

    for token in input.split_whitespace() {
        // Parse as i64 to accept larger integer literals if present;
        // values outside [0,36] will map to 0 as in the C program.
        match token.parse::<i64>() {
            Ok(n) => {
                if (0..=36).contains(&n) {
                    // counts stored as i32, print as decimal
                    println!("{}", counts[n as usize]);
                } else {
                    println!("0");
                }
            }
            // If a token is not a valid integer, skip it.
            // The original C program used scanf("%d", &n) which would fail on such input;
            // typical judge inputs are integers only.
            Err(_) => {}
        }
    }
}
