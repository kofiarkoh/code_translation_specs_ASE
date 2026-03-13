// weekday_2004.rs
//
// Translates the provided Specification into Rust.
// - Reads pairs of integers (month, day) from stdin.
// - Stops when either value is 0 (that pair is not processed).
// - For each non-terminating pair, computes the weekday for that date in year 2004
//   and prints one of "Sunday".."Saturday" followed by a newline.
// - The day_of_week function implements Sakamoto's algorithm and returns 0=Sunday..6=Saturday.
//
// Notes on fidelity to the Specification:
// - The Spec assumes inputs are valid integer tokens supplied in pairs. This program
//   parses all whitespace-separated tokens as i32 and processes them in pairs; if an
//   incomplete final token is present (or a token fails to parse), the program stops
//   (narrow interpretation of the Spec's unchecked scanf).
// - The Spec leaves behavior on invalid month/day unspecified. Here we mirror that
//   assumption by performing direct array indexing with (month-1) as usize inside
//   day_of_week; if month is out of 1..12 this will panic at runtime (Rust's logical
//   equivalent of C's undefined/out-of-bounds behavior). This is deliberate to stay
//   close to the Spec rather than silently correcting invalid inputs.

use std::io::{self, Read};
use std::process::exit;

fn day_of_week(year: i32, month: i32, day: i32) -> i32 {
    // Sakamoto's algorithm for Gregorian calendar.
    // Returns 0 = Sunday, 1 = Monday, ..., 6 = Saturday.
    //
    // The array t is indexed by month-1 (0-based). The Spec's undefined behavior on
    // invalid months is preserved by using direct indexing; invalid month values
    // may panic here.
    let mut y = year;
    let m = month;
    let t = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    if m < 3 {
        y -= 1;
    }
    // All arithmetic in i32. The result of modulo may be negative if the LHS is negative;
    // normalize to 0..6 by adding 7 if necessary.
    let raw = (y + y / 4 - y / 100 + y / 400 + t[(m - 1) as usize] + day) % 7;
    if raw < 0 {
        raw + 7
    } else {
        raw
    }
}

fn main() {
    // Read entire stdin into a string and split into whitespace tokens.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading fails, mirror narrow interpretation: exit with 0.
        exit(0);
    }

    let mut tokens = input.split_whitespace();
    let weekday_names = [
        "Sunday", "Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday",
    ];

    loop {
        // Attempt to read two tokens; if either is missing or fails to parse as i32,
        // stop processing (consistent with the Spec's assumption that tokens are available).
        let m_token = match tokens.next() {
            Some(s) => s,
            None => break,
        };
        let d_token = match tokens.next() {
            Some(s) => s,
            None => break,
        };

        // Parse tokens as integers. If parse fails, stop processing.
        let m: i32 = match m_token.parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        let d: i32 = match d_token.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        // Termination condition: if either is zero, stop without printing.
        if m == 0 || d == 0 {
            break;
        }

        // Compute weekday for year = 2004 as specified.
        let w = day_of_week(2004, m, d);

        // Map 0..6 to names and print. If w is out of 0..6, this will panic (mirrors
        // the Spec's note that invalid inputs can lead to out-of-bounds access).
        println!("{}", weekday_names[w as usize]);
    }

    // On normal termination, return 0.
    exit(0);
}
