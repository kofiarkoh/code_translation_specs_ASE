use std::io::{self, Read};

fn main() {
    // Read entire stdin as whitespace-separated tokens.
    let mut input = String::new();
    // If reading fails, propagate via panic (matches C behavior of aborting on unexpected I/O error).
    io::stdin().read_to_string(&mut input).unwrap();

    let mut tokens = input.split_whitespace();

    // Process repeatedly groups of six integers until EOF (i.e., fewer than six tokens).
    loop {
        // Helper to obtain the next token parsed as i32; if missing or malformed, stop processing.
        let next_i32 = |tokens: &mut std::str::SplitWhitespace| -> Option<i32> {
            match tokens.next() {
                Some(tok) => match tok.parse::<i32>() {
                    Ok(v) => Some(v),
                    // On parse error, behavior is unspecified in the original C program;
                    // here we stop processing further input (treat as EOF/termination).
                    Err(_) => None,
                },
                None => None,
            }
        };

        let a = match next_i32(&mut tokens) { Some(v) => v, None => break };
        let b = match next_i32(&mut tokens) { Some(v) => v, None => break };
        let c = match next_i32(&mut tokens) { Some(v) => v, None => break };
        let d = match next_i32(&mut tokens) { Some(v) => v, None => break };
        let e = match next_i32(&mut tokens) { Some(v) => v, None => break };
        let f = match next_i32(&mut tokens) { Some(v) => v, None => break };

        // Use f64 for the floating calculations.
        let a_f = a as f64;
        let b_f = b as f64;
        let c_f = c as f64;
        let d_f = d as f64;
        let e_f = e as f64;
        let f_f = f as f64;

        // Determinant D = a*e - b*d
        let dnm = a_f * e_f - b_f * d_f;

        // According to the specification:
        // x = (c*e - b*f) / D
        // y = (a*f - c*d) / D
        // Note: if D == 0.0, IEEE semantics will produce Inf/NaN, which we preserve.
        let mut x = (c_f * e_f - b_f * f_f) / dnm;
        let mut y = (a_f * f_f - c_f * d_f) / dnm;

        // Apply small near-zero correction: if (-0.0005 < value && value <= 0) then set to 0
        if -0.0005 < x && x <= 0.0 {
            x = 0.0;
        }
        if -0.0005 < y && y <= 0.0 {
            y = 0.0;
        }

        // Print with three digits after the decimal point, separated by a space, followed by newline.
        // This matches the "%.3f" style formatting from the C program/spec.
        println!("{:.3} {:.3}", x, y);
    }
}
