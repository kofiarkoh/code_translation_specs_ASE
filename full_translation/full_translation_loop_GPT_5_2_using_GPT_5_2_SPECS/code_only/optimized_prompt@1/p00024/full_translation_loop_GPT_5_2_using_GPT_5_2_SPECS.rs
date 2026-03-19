use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse whitespace-separated tokens like scanf("%f", ...)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut out = String::new();

    for tok in input.split_whitespace() {
        // In C, scanf("%f", &v) reads a float; if parsing fails, it would stop.
        // Here we stop on the first non-float token to emulate that behavior.
        let v: f32 = match tok.parse() {
            Ok(x) => x,
            Err(_) => break,
        };

        // Compute in float (C uses float for v; constants are double but result assigned to int).
        // Use f64 for intermediate to better match C's usual float->double promotions in expressions.
        let vf = v as f64;
        let expr = (4.9f64 * vf * vf / 9.8f64 / 9.8f64) + 5.0f64;

        // C cast to int truncates toward zero; Rust as i32 does the same for finite values.
        // For NaN/inf/out-of-range, C behavior is undefined; Rust saturates/has defined behavior.
        // We keep Rust's behavior (no unsafe) since C provides no defined semantics there.
        let mut n: i32 = (expr as i32) / 5;
        n = n + 1;

        out.push_str(&format!("{n}\n"));
    }

    print!("{out}");
}
