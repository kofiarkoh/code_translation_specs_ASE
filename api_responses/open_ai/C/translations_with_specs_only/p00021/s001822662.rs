// slopes.rs
// Translation of the given specification into Rust.
// Behavior notes (per specification):
// - Reads an integer n followed by n * 8 floating-point tokens from stdin.
// - For each test case computes slope1 = (y2 - y1) / (x2 - x1) and
//   slope2 = (y4 - y3) / (x4 - x3).
// - Compares the slopes using exact floating-point equality (==).
// - Prints "YES\n" if equal, otherwise "NO\n".
// - If input is missing or malformed, the program stops producing outputs
//   (matching the specification's unspecified behavior for such cases).
// - Division by zero is not specially handled; f64 semantics (Inf/NaN) apply.

use std::io::{self, Read};

fn main() {
    // Read entire stdin into a string (mimics token-based scanf input).
    let mut input = String::new();
    // If read_to_string fails, treat as no input (exit with code 0).
    let _ = io::stdin().read_to_string(&mut input);

    let toks: Vec<&str> = input.split_whitespace().collect();
    let mut idx: usize = 0;

    // Need at least one token for n.
    if idx >= toks.len() {
        return;
    }

    // Parse n as a signed integer. If parse fails, behave as unspecified: exit quietly.
    let n_i64: i64 = match toks[idx].parse() {
        Ok(v) => v,
        Err(_) => return,
    };
    idx += 1;

    // Per specification, negative n results in zero iterations.
    if n_i64 <= 0 {
        return;
    }
    let n = n_i64 as usize;

    let mut out = String::new();

    // For each test case try to read 8 floating-point tokens; if tokens are missing
    // or parsing fails we stop producing further outputs (unspecified behavior).
    for _case in 0..n {
        // Ensure there are at least 8 tokens remaining.
        if idx + 8 > toks.len() {
            break;
        }

        // Parse eight f64 values; on any parse error, stop processing further cases.
        let mut vals = [0f64; 8];
        let mut ok = true;
        for j in 0..8 {
            match toks[idx + j].parse::<f64>() {
                Ok(v) => vals[j] = v,
                Err(_) => {
                    ok = false;
                    break;
                }
            }
        }
        idx += 8;
        if !ok {
            break;
        }

        let x1 = vals[0];
        let y1 = vals[1];
        let x2 = vals[2];
        let y2 = vals[3];
        let x3 = vals[4];
        let y3 = vals[5];
        let x4 = vals[6];
        let y4 = vals[7];

        // Compute slopes using f64 arithmetic. Division by zero is intentionally
        // not guarded against; f64 semantics apply (Inf/NaN).
        let slope1 = (y2 - y1) / (x2 - x1);
        let slope2 = (y4 - y3) / (x4 - x3);

        if slope1 == slope2 {
            out.push_str("YES\n");
        } else {
            out.push_str("NO\n");
        }
    }

    // Write accumulated output to stdout.
    // This preserves one line per processed test case.
    print!("{}", out);
}
