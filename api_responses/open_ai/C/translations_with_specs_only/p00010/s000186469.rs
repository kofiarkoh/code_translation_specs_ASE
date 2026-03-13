// circumcenter.rs
//
// Translates the provided Specification into Rust.
// Reads an integer N followed by 6*N doubles from stdin.
// For each triple of points, computes the circumcenter and radius and
// prints "center_x center_y radius" with three digits after the decimal point.
//
// Behavior notes (kept faithful to Spec):
// - If the initial integer N cannot be read, the program does nothing and exits 0.
// - If fewer than 6*N numeric tokens are available, the program processes only
//   the fully-read test cases (i.e., cases for which all 6 values were parsed).
// - Degenerate geometric configurations (collinear or coincident points) will
//   produce IEEE-754 infinities or NaNs as produced by f64 arithmetic; those
//   values are printed using Rust's floating-point formatting with 3 decimals.
// - No panics on malformed numeric tokens; parsing failure stops further processing.
//
// Implementation choices:
// - Uses the standard algebraic formula for circumcenter based on determinants:
//     d = 2*(x1*(y2-y3) + x2*(y3-y1) + x3*(y1-y2))
//     ux = ((x1^2+y1^2)*(y2-y3) + (x2^2+y2^2)*(y3-y1) + (x3^2+y3^2)*(y1-y2)) / d
//     uy = ((x1^2+y1^2)*(x3-x2) + (x2^2+y2^2)*(x1-x3) + (x3^2+y3^2)*(x2-x1)) / d
//   This mirrors common C implementations and will yield inf/NaN on degenerate inputs.
//
// - Uses safe, idiomatic Rust only.

use std::io::{self, Read};

fn main() {
    // Read entire stdin into a string, then split by whitespace to tokens.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading stdin fails, behave like scanf failure: do nothing and exit 0.
        return;
    }

    let mut tokens = input.split_whitespace();

    // Try to read initial integer N. If not present or not parseable, exit quietly.
    let n_token = match tokens.next() {
        Some(t) => t,
        None => return,
    };
    let n: i64 = match n_token.parse() {
        Ok(v) => v,
        Err(_) => return,
    };

    if n <= 0 {
        // Per Spec: if N <= 0, produce no output and exit normally.
        return;
    }

    // Process up to N test cases; for each, attempt to read 6 f64 values.
    for _case in 0..n {
        // Attempt to read six doubles. If any are missing or fail to parse, stop processing further cases.
        let mut vals = [0f64; 6];
        let mut ok = true;
        for i in 0..6 {
            match tokens.next() {
                Some(tok) => match tok.parse::<f64>() {
                    Ok(v) => vals[i] = v,
                    Err(_) => { ok = false; break; },
                },
                None => { ok = false; break; },
            }
        }
        if !ok {
            // Stop processing further test cases on parse/missing-token failure.
            break;
        }

        let x1 = vals[0];
        let y1 = vals[1];
        let x2 = vals[2];
        let y2 = vals[3];
        let x3 = vals[4];
        let y3 = vals[5];

        // Compute circumcenter using determinant formula.
        // d = 2 * det([x,y,1] rows) simplified form
        let d = 2.0 * (x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2));

        let s1 = x1 * x1 + y1 * y1;
        let s2 = x2 * x2 + y2 * y2;
        let s3 = x3 * x3 + y3 * y3;

        let ux_num = s1 * (y2 - y3) + s2 * (y3 - y1) + s3 * (y1 - y2);
        let uy_num = s1 * (x3 - x2) + s2 * (x1 - x3) + s3 * (x2 - x1);

        let center_x = ux_num / d;
        let center_y = uy_num / d;

        let dx = center_x - x1;
        let dy = center_y - y1;
        let radius = (dx * dx + dy * dy).sqrt();

        // Print with three digits after the decimal point.
        // Using Rust formatting "{:.3}" to mirror C's "%0.3f".
        println!("{:.3} {:.3} {:.3}", center_x, center_y, radius);
    }
}
