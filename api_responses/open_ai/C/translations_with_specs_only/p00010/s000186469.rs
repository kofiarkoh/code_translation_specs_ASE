// circle_from_three_points.rs
//
// Reads an integer N followed by N test cases of six doubles (x1 y1 x2 y2 x3 y3).
// For each test case prints the circumcenter (x y) and radius r with three decimals:
// "%0.3f %0.3f %0.3f\n"
//
// Implementation follows the specification: it assumes well-formed input for normal behavior.
// Degenerate cases (collinear or coincident points) are not specially handled and may
// produce `inf`/`nan` results according to IEEE floating-point semantics.
//
// Uses the standard determinant-based formula for the circumcenter:
// D  = 2*(x1*(y2-y3) + x2*(y3-y1) + x3*(y1-y2))
// Ux = ((x1^2+y1^2)*(y2-y3) + (x2^2+y2^2)*(y3-y1) + (x3^2+y3^2)*(y1-y2)) / D
// Uy = ((x1^2+y1^2)*(x3-x2) + (x2^2+y2^2)*(x1-x3) + (x3^2+y3^2)*(x2-x1)) / D
// radius = sqrt((Ux-x1)^2 + (Uy-y1)^2)
use std::io::{self, Read};

fn main() {
    // Read all stdin
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read error, behave quietly (no output). Matches "no explicit error handling" spirit.
        return;
    }

    let mut toks = input.split_whitespace();

    // Parse N as i32. If missing or unparseable, exit with no output (undefined behavior per spec).
    let n_tok = match toks.next() {
        Some(t) => t,
        None => return,
    };
    let n: i32 = match n_tok.parse() {
        Ok(v) => v,
        Err(_) => return,
    };

    // If N <= 0, perform zero iterations (consistent with typical C for-loop behavior).
    if n <= 0 {
        return;
    }

    // Helper to parse next token as f64; returns None if missing or unparseable.
    let mut parse_next = |iter: &mut std::str::SplitWhitespace| -> Option<f64> {
        iter.next()?.parse::<f64>().ok()
    };

    for _case in 0..n {
        // Parse six doubles for this test case.
        let x1 = match parse_next(&mut toks) { Some(v) => v, None => return };
        let y1 = match parse_next(&mut toks) { Some(v) => v, None => return };
        let x2 = match parse_next(&mut toks) { Some(v) => v, None => return };
        let y2 = match parse_next(&mut toks) { Some(v) => v, None => return };
        let x3 = match parse_next(&mut toks) { Some(v) => v, None => return };
        let y3 = match parse_next(&mut toks) { Some(v) => v, None => return };

        // Compute using determinant formula.
        let x1sq_y1sq = x1 * x1 + y1 * y1;
        let x2sq_y2sq = x2 * x2 + y2 * y2;
        let x3sq_y3sq = x3 * x3 + y3 * y3;

        let d = 2.0 * (x1 * (y2 - y3) + x2 * (y3 - y1) + x3 * (y1 - y2));

        let ux_num = x1sq_y1sq * (y2 - y3)
            + x2sq_y2sq * (y3 - y1)
            + x3sq_y3sq * (y1 - y2);

        let uy_num = x1sq_y1sq * (x3 - x2)
            + x2sq_y2sq * (x1 - x3)
            + x3sq_y3sq * (x2 - x1);

        // Division by zero (d == 0.0) will produce inf/nan according to IEEE semantics,
        // which matches the specification for degenerate inputs.
        let ux = ux_num / d;
        let uy = uy_num / d;

        let dx = ux - x1;
        let dy = uy - y1;
        let r = (dx * dx + dy * dy).sqrt();

        // Print with three digits after decimal point, space-separated.
        // Examples: "1.234 5.678 9.012\n"
        println!("{:.3} {:.3} {:.3}", ux, uy, r);
    }
}
