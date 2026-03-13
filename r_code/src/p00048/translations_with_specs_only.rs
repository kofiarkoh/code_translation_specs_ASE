use std::io::{self, Read};

/// This program implements the behavior described in the Specification.
/// Notes on faithful emulation of the original C semantics:
/// - The original (buggy) C loop used `while (scanf("%lf", &v) != EOF) { ... }`.
///   That means:
///     * On EOF, scanf returns EOF and the loop ends.
///     * If a token is not a valid double, scanf returns 0 and does NOT consume
///       the token from stdin. The original program did not check for this case
///       and proceeded into classification using the (possibly uninitialized)
///       `v`, and continued the loop because 0 != EOF. This can cause an
///       infinite loop on a malformed token. We emulate that behavior precisely:
///         - We split the entire input on whitespace into tokens (the
///           Specification assumes whitespace-separated tokens).
///         - We keep a token index `pos`. On successful parse we increment pos.
///         - On parse failure we DO NOT increment pos, so subsequent iterations
///           see the same token again, causing the same failure repeatedly
///           (infinite loop), matching the C scanf behavior.
/// - To emulate an uninitialized `v` before any successful parse, we initialize
///   `v` to NaN. This choice is a narrow, deterministic way to reflect the
///   "indeterminate" initial state; importantly, comparisons with NaN are
///   always false, which makes the classification fall through to the final
///   `else -> "heavy"`, consistent with the Specification notes about NaN.
/// - For successfully parsed numeric tokens (including "inf", "-inf", "nan"
///   when parse recognizes them), we update `v` and perform the classification.
/// - For every loop iteration that is not EOF (whether the token was parsed or
///   not), we print exactly one label followed by a newline.
///
/// This program prints labels exactly as specified and exits with code 0 on
/// normal EOF termination. It may infinite-loop on malformed (non-numeric)
/// tokens, as the Specification describes for the original C program.
fn main() {
    // Read entire stdin, then split on whitespace into tokens.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading stdin fails, exit (this mirrors the C program's lack of
        // sophisticated error handling; returning here yields exit code 0 as
        // requested by the Specification for normal termination isn't strictly
        // applicable, but this path is unlikely in normal use).
        return;
    }

    let tokens: Vec<&str> = input.split_whitespace().collect();
    let mut pos: usize = 0;

    // Emulate uninitialized `v` by starting as NaN.
    let mut v: f64 = f64::NAN;

    // Loop until EOF: i.e., until there are no more tokens.
    loop {
        // If no more tokens, scanf would return EOF and the loop would end.
        if pos >= tokens.len() {
            break;
        }

        let token = tokens[pos];

        // Attempt to parse the token as f64, akin to scanf("%lf", &v).
        // If parsing succeeds, update v and consume the token (pos += 1).
        // If parsing fails, scanf would return 0 and NOT consume the token;
        // the original program did not check for this and proceeded anyway.
        match token.parse::<f64>() {
            Ok(parsed) => {
                v = parsed;
                pos += 1; // consume token only on successful parse
            }
            Err(_) => {
                // Do not advance pos: emulate scanf leaving the non-numeric
                // token in the input stream. The loop will therefore try again
                // on the same token, leading to an infinite loop (as specified).
            }
        }

        // Classification according to the exact mapping in the Specification.
        // Comparisons with NaN are false, so NaN will reach the final else -> "heavy".
        let label = if v <= 48.0 {
            "light fly"
        } else if v <= 51.0 {
            "fly"
        } else if v <= 54.0 {
            "bantam"
        } else if v <= 57.0 {
            "feather"
        } else if v <= 60.0 {
            "light"
        } else if v <= 64.0 {
            "light welter"
        } else if v <= 69.0 {
            "welter"
        } else if v <= 75.0 {
            "light middle"
        } else if v <= 81.0 {
            "middle"
        } else if v <= 91.0 {
            "light heavy"
        } else {
            "heavy"
        };

        // Print the label followed by a newline for this iteration.
        println!("{}", label);
        // Loop continues until EOF (pos >= tokens.len()).
    }
}
