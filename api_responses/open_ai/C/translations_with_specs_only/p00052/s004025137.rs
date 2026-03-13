// Translated from the provided specification.
// Behavior:
//  - Read whitespace-separated integers from stdin (interpreted as C long).
//  - For each non-zero integer n, compute and print the total multiplicity
//    of the prime 5 in 1*2*...*n (i.e., number of trailing zeros of n!).
//  - Stop processing when a zero (0) is read. Each printed result is on its own line.
// Notes on interpretation:
//  - Inputs are treated as signed integers. We use i64 to represent C long here.
//  - If a token fails to parse as an integer, processing stops (the spec leaves such behavior unspecified).
//  - Negative n are treated as < 5 and thus result in printing 0 (unless n == 0 which terminates).
use std::io::{self, Read};

fn main() {
    // Read entire stdin. The spec expects scanf("%ld")-style token parsing;
    // reading all and splitting on whitespace is the closest high-level match.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading stdin fails, exit; specification treats I/O errors as unspecified.
        return;
    }

    for token in input.split_whitespace() {
        // Try to parse each token as a signed long (i64).
        // The spec assumes tokens to be valid integers; on parse failure we stop processing.
        let n: i64 = match token.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        if n == 0 {
            // Termination sentinel: stop processing and exit with status 0.
            break;
        }

        // For n < 5, the count is 0. This also covers negative n per spec.
        if n < 5 {
            println!("0");
            continue;
        }

        // Compute sum_{k>=1} floor(n / 5^k)
        // Use i64 for accumulation consistent with interpreting outputs as C long.
        let mut count: i64 = 0;
        let mut m: i64 = n;
        while m >= 5 {
            m /= 5;
            count = count.wrapping_add(m);
            // Note: wrapping_add mirrors possible wrap-around behavior on overflow.
            // The spec indicates overflow in C long is undefined; using wrapping_add
            // is a reasonable conservative choice to avoid panic while keeping
            // low-level semantics similar to integer wraparound.
        }

        println!("{}", count);
    }
}
