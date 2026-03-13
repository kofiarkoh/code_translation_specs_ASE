// orthogonality.rs
//
// Translates the provided specification into Rust.
// Behavior notes (to preserve semantics described in the specification):
// - The program reads whitespace-separated tokens from stdin and interprets them as f64.
// - For each logical record of eight doubles (x1 y1 x2 y2 x3 y3 x4 y4) it computes the
//   dot product of (x1-x2, y1-y2) and (x3-x4, y3-y4).
// - If the dot product is exactly 0.0 (IEEE-754 exact equality), it prints "YES\n", else "NO\n".
// - In cases of malformed input or partial records, the original specification describes
//   undefined behavior due to uninitialized reads. To emulate that observable effect
//   (i.e., not performing protected error handling and making such cases yield non-"YES"),
//   this implementation substitutes missing or unparsable tokens with NaN. Any arithmetic
//   involving NaN yields NaN and NaN == 0.0 is false, so the program prints "NO" for such cases.
// - If EOF is reached before attempting to read a new record (no tokens available), the program exits
//   without producing further output and returns status 0.
//
// This approach keeps the main, observable behaviors faithful to the specification while
// avoiding undefined behavior in Rust.

use std::io::{self, Read};

fn parse_or_nan(token: &str) -> f64 {
    // If parsing fails, return NaN to emulate "indeterminate" / error-influenced behavior
    token.parse::<f64>().unwrap_or(f64::NAN)
}

fn main() {
    // Read entire stdin as text and iterate over whitespace-separated tokens.
    let mut s = String::new();
    if io::stdin().read_to_string(&mut s).is_err() {
        // If reading stdin fails for some reason, exit normally with no output (status 0).
        return;
    }

    let mut iter = s.split_whitespace();

    loop {
        // Attempt to get the first token for this record.
        let first_tok = iter.next();
        if first_tok.is_none() {
            // EOF encountered before starting a new record: exit normally.
            break;
        }

        // Collect up to 8 values for the record. For any missing or unparsable token,
        // use NaN (so dot == 0 comparison will be false, producing "NO").
        let mut v = [f64::NAN; 8];
        v[0] = parse_or_nan(first_tok.unwrap());
        for i in 1..8 {
            match iter.next() {
                Some(tok) => v[i] = parse_or_nan(tok),
                None => v[i] = f64::NAN,
            }
        }

        // Interpret as (x1 y1 x2 y2 x3 y3 x4 y4)
        let x1 = v[0];
        let y1 = v[1];
        let x2 = v[2];
        let y2 = v[3];
        let x3 = v[4];
        let y3 = v[5];
        let x4 = v[6];
        let y4 = v[7];

        let ux = x1 - x2;
        let uy = y1 - y2;
        let vx = x3 - x4;
        let vy = y3 - y4;

        let dot = ux * vx + uy * vy;

        if dot == 0.0 {
            // Exact equality check as specified.
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
