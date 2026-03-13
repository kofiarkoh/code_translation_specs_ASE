use std::io::{self, Read};

fn main() {
    // Read all stdin as whitespace-separated tokens.
    let mut input = String::new();
    // If reading fails, exit (mirrors unspecified behavior on malformed/EOF in the C version).
    let _ = io::stdin().read_to_string(&mut input);

    let mut toks = input.split_whitespace();

    // Process consecutive pairs of integers (x, h).
    // For each pair where x + h != 0, compute sqrt(x*x + 4*h*h) * x + x*x and print with 6 decimals.
    while let (Some(xs), Some(hs)) = (toks.next(), toks.next()) {
        // Parse tokens as 32-bit integers (C 'int' typical mapping).
        // On parse failure we stop (behavior for malformed input is unspecified in the spec).
        let x: i32 = match xs.parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        let h: i32 = match hs.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        // Termination condition: x + h == 0 (do not print in that case).
        if x + h == 0 {
            break;
        }

        // Perform arithmetic in f64 to match the real-arithmetic intent of the specification.
        // (In the original C, overflow of the intermediate integer multiplications would be undefined;
        // the spec documents that as unspecified behavior. Here we convert to f64 first to avoid
        // integer overflow and to follow the specified mathematical formula.)
        let xf = x as f64;
        let hf = h as f64;
        let value = (xf * xf + 4.0 * hf * hf).sqrt() * xf + xf * xf;

        // Print using six fractional digits to match printf("%f") default (%lf behaves the same).
        println!("{:.6}", value);
    }
}
