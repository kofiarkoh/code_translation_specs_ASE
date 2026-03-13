use std::io::{self, Read};

fn main() {
    // Read all stdin as whitespace-separated tokens.
    // The Specification expects pairs of integers on stdin; behavior on malformed input/EOF is unspecified.
    let mut input = String::new();
    // Ignore potential read errors; if read fails we'll treat it as no input (unspecified per spec).
    let _ = io::stdin().read_to_string(&mut input);

    let mut tokens = input.split_whitespace();

    // Process successive pairs of integers.
    loop {
        let tx = tokens.next();
        let th = tokens.next();

        // If we cannot obtain a full pair, the specification states behavior is unspecified.
        // We choose the narrow/benign interpretation: stop processing.
        if tx.is_none() || th.is_none() {
            break;
        }

        // Parse tokens as i32 (typical C `int` size). If parse fails, behavior is unspecified;
        // we stop processing.
        let x = match tx.unwrap().parse::<i32>() {
            Ok(v) => v,
            Err(_) => break,
        };
        let h = match th.unwrap().parse::<i32>() {
            Ok(v) => v,
            Err(_) => break,
        };

        // Termination condition: exactly x + h == 0 (as integers).
        // To avoid panics on extreme values in debug builds, evaluate sum in i64.
        if (x as i64 + h as i64) == 0 {
            break;
        }

        // Compute value = sqrt(x*x + 4*h*h) * x + x*x as real arithmetic in double precision.
        // The specification describes potential undefined behavior if integer overflow occurs;
        // to preserve the intended mathematical meaning for typical inputs we perform the
        // computation in f64 (avoiding Rust integer overflow panics).
        let xf = x as f64;
        let hf = h as f64;
        let arg = xf * xf + 4.0 * hf * hf;
        let val = arg.sqrt() * xf + xf * xf;

        // Print using six digits after the decimal point to match printf("%f") default.
        println!("{:.6}", val);
    }

    // Exit with status 0 (implicit).
}
