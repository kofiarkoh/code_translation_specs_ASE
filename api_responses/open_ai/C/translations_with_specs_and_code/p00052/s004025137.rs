use std::io::{self, Read};

fn main() {
    // This program follows the given specification:
    // - Read integers (treated as C `long`, here i64) from stdin.
    // - For each non-zero integer n, print the total multiplicity of prime 5
    //   in 1*2*...*n (the number of trailing zeros of n!) on its own line.
    // - Stop when a 0 token is encountered (do not print for the 0).
    //
    // Behavior on malformed input or EOF without a terminating 0 is unspecified
    // by the original spec; here we stop processing on parse failure or EOF.

    let mut input = String::new();
    // Read entire stdin (safe, idiomatic). If reading fails, exit.
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    for token in input.split_whitespace() {
        // Parse each whitespace-separated token as a signed 64-bit integer.
        // i64 is used as a practical stand-in for C's `long` on common platforms.
        let n: i64 = match token.parse() {
            Ok(v) => v,
            // On parse error, stop processing (spec leaves this unspecified).
            Err(_) => break,
        };

        if n == 0 {
            // Termination sentinel: stop without printing.
            break;
        }

        // For n <= 0, the result should be 0 (no factors of 5 in factorial of <= 0).
        // The standard efficient method: sum floor(n/5^k) for k>=1.
        let mut m = n;
        let mut count: i64 = 0;
        while m > 0 {
            m /= 5;
            count += m;
        }

        println!("{}", count);
    }
}
