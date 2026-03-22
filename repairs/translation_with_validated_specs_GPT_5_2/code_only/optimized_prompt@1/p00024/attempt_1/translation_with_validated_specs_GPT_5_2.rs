use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse whitespace-separated tokens like scanf("%f", ...)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    for tok in input.split_whitespace() {
        // C's scanf("%f") reads a float; if parsing fails, it would stop.
        // Here we stop on the first unparsable token to mimic that behavior.
        let v: f32 = match tok.parse() {
            Ok(x) => x,
            Err(_) => break,
        };

        // Compute in f32 to match C 'float' arithmetic as closely as possible.
        // C cast to int truncates toward zero; Rust as-cast from float to i32
        // saturates on out-of-range/NaN, which differs from C UB. We emulate
        // truncation toward zero and clamp to i32 range for defined behavior.
        let expr: f32 = (4.9_f32 * v * v / 9.8_f32 / 9.8_f32) + 5.0_f32;
        let mut n_i64: i64 = if expr.is_nan() {
            0
        } else {
            // truncation toward zero
            let t = expr.trunc();
            if t > i32::MAX as f32 {
                i32::MAX as i64
            } else if t < i32::MIN as f32 {
                i32::MIN as i64
            } else {
                t as i64
            }
        };

        n_i64 /= 5;
        n_i64 += 1;

        // C prints "%d\n" (int). We print as i64 but formatting matches.
        println!("{}", n_i64);
    }
}
