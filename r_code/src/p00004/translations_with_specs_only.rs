use std::io::{self, Read};

fn main() {
    // Read entire stdin as whitespace-separated tokens.
    // The specification expects the program to read sequences of six C ints (whitespace separated)
    // until EOF. We choose the narrowest reasonable behavior: stop processing when fewer than
    // six integers remain or a token fails to parse as an integer.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading stdin fails, exit normally (no output), matching "stop on EOF" spirit.
        return;
    }

    let mut tokens = input.split_whitespace();

    // Process sets of six integers until EOF (i.e., until we can't obtain six integers).
    loop {
        // Attempt to read six tokens and parse them as i32.
        let a_tok = match tokens.next() {
            Some(t) => t,
            None => break, // EOF: stop normally
        };
        let b_tok = match tokens.next() {
            Some(t) => t,
            None => break,
        };
        let c_tok = match tokens.next() {
            Some(t) => t,
            None => break,
        };
        let d_tok = match tokens.next() {
            Some(t) => t,
            None => break,
        };
        let e_tok = match tokens.next() {
            Some(t) => t,
            None => break,
        };
        let f_tok = match tokens.next() {
            Some(t) => t,
            None => break,
        };

        // Parse as i32. If parsing fails (non-integer token), behavior in the specification is
        // underspecified; we choose to stop processing further input.
        let a = match a_tok.parse::<i32>() { Ok(v) => v, Err(_) => break };
        let b = match b_tok.parse::<i32>() { Ok(v) => v, Err(_) => break };
        let c = match c_tok.parse::<i32>() { Ok(v) => v, Err(_) => break };
        let d = match d_tok.parse::<i32>() { Ok(v) => v, Err(_) => break };
        let e = match e_tok.parse::<i32>() { Ok(v) => v, Err(_) => break };
        let f = match f_tok.parse::<i32>() { Ok(v) => v, Err(_) => break };

        // Perform integer arithmetic using wrapping operations to emulate typical C int
        // two's-complement wraparound behavior on overflow.
        let num_x = c.wrapping_mul(e).wrapping_sub(b.wrapping_mul(f));
        let num_y = a.wrapping_mul(f).wrapping_sub(c.wrapping_mul(d));
        let den = a.wrapping_mul(e).wrapping_sub(b.wrapping_mul(d));

        // Convert to floating point for division. This follows the specification: floating-point
        // division may produce +Inf, -Inf, or NaN if denominator is zero.
        let x = (num_x as f64) / (den as f64);
        let y = (num_y as f64) / (den as f64);

        // Apply the small near-zero correction:
        // If value is > -0.0005 and <= 0, coerce to +0.0 before printing.
        let x = if x <= 0.0 && x > -0.0005 { 0.0 } else { x };
        let y = if y <= 0.0 && y > -0.0005 { 0.0 } else { y };

        // Print with three digits after the decimal point, separated by a space, followed by newline.
        // This uses Rust's formatting with fixed precision to emulate "%.3f".
        println!("{:.3} {:.3}", x, y);
    }
}
