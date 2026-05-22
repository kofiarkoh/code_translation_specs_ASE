use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse whitespace-separated tokens as f32 (matches C `float`).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    for tok in input.split_whitespace() {
        // Per spec, invalid tokens are outside the contract; we simply stop if parsing fails.
        let v: f32 = match tok.parse() {
            Ok(x) => x,
            Err(_) => break,
        };

        // x = 4.9 * v^2 / 9.8^2
        let x: f32 = (4.9_f32 * v * v) / 9.8_f32 / 9.8_f32;

        // k = truncate(x + 5) toward zero, then m = truncate(k / 5) (integer division).
        // This matches the C expression: n = (int)(x + 5) / 5; n = n + 1;
        let k: i32 = (x + 5.0_f32) as i32;
        let m: i32 = k / 5;
        let n: i32 = m + 1;

        println!("{n}");
    }
}
