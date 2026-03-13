// translation.rs
// Translated from the provided C program according to the specification.
// Reads whitespace-separated f64 values from stdin in groups of 8:
// x0 y0 x1 y1 x2 y2 x3 y3
// For each full group prints either "YES\n" or "NO\n" according to the spec.

use std::io::{self, Read, Write};

fn in_inclusive_range(value: f64, lo: f64, hi: f64) -> bool {
    lo <= value && value <= hi
}

fn main() -> io::Result<()> {
    // Read entire stdin as UTF-8 text and split on whitespace.
    // This follows the spec: process full records of 8 doubles until EOF.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut tokens = input.split_whitespace();
    let mut out = io::stdout();
    let mut handle = out.lock();

    loop {
        // Attempt to read 8 tokens; if any token is missing (EOF) or can't be parsed,
        // stop processing and exit normally (no diagnostics), per the specification.
        let mut vals = [0f64; 8];
        for i in 0..8 {
            match tokens.next() {
                Some(tok) => match tok.parse::<f64>() {
                    Ok(v) => vals[i] = v,
                    Err(_) => return Ok(()), // malformed token: stop processing (no output)
                },
                None => return Ok(()), // fewer than 8 tokens remain: exit normally
            }
        }

        let x0 = vals[0];
        let y0 = vals[1];
        let x1 = vals[2];
        let y1 = vals[3];
        let x2 = vals[4];
        let y2 = vals[5];
        let x3 = vals[6];
        let y3 = vals[7];

        let y_in = in_inclusive_range(y2, y0, y1) || in_inclusive_range(y3, y0, y1);

        let cond = (in_inclusive_range(x2, x0, x1) && y_in)
            || (in_inclusive_range(x3, x0, x1) && y_in);

        if cond {
            handle.write_all(b"YES\n")?;
        } else {
            handle.write_all(b"NO\n")?;
        }
    }
}
