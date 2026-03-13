use std::io::{self, Read};

fn main() {
    // Read entire stdin into a string, then split on whitespace to get tokens.
    // This mirrors the C program's repeated scanf behavior: we consume
    // whitespace-separated integer tokens until EOF.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On I/O error, exit (no explicit non-zero code per spec).
        return;
    }

    let mut nums: Vec<i64> = Vec::new();
    // Parse tokens as signed integers. On first non-integer token we stop
    // processing further tokens (narrow interpretation of "not handled").
    for tok in input.split_whitespace() {
        match tok.parse::<i64>() {
            Ok(v) => nums.push(v),
            Err(_) => break,
        }
    }

    // Process complete records of six integers each.
    let mut i = 0usize;
    while i + 5 < nums.len() {
        // According to the specification, inputs are integers (C 'int').
        // We use i64 to avoid accidental overflow in Rust builds;
        // the spec assumes intermediate integer arithmetic does not overflow.
        let a = nums[i];
        let b = nums[i + 1];
        let c = nums[i + 2];
        let d = nums[i + 3];
        let e = nums[i + 4];
        let f = nums[i + 5];

        // Compute determinant and numerators as integers, then convert to f64.
        let det = a * e - b * d;
        let num_x = c * e - b * f;
        let num_y = a * f - c * d;

        // Convert to floating point for division. This will produce infinities/NaN
        // if det == 0, matching the specification that division by zero is not handled.
        let mut x = (num_x as f64) / (det as f64);
        let mut y = (num_y as f64) / (det as f64);

        // Tiny negative-to-zero normalization: if value in (-0.0005, 0], set to 0.
        if x <= 0.0 && x > -0.0005 {
            x = 0.0;
        }
        if y <= 0.0 && y > -0.0005 {
            y = 0.0;
        }

        // Print with exactly three digits after the decimal point, separated by single space.
        // Each record on its own line.
        println!("{:.3} {:.3}", x, y);

        i += 6;
    }
}
