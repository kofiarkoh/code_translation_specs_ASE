use std::io::{self, Read};

fn main() {
    // Read entire stdin as text and split by whitespace, mirroring scanf("%f") tokenization.
    let mut input = String::new();
    // On read error, mimic typical C program that would likely terminate; unwrap is acceptable per spec.
    io::stdin().read_to_string(&mut input).unwrap();

    for token in input.split_whitespace() {
        // Specification treats each input as a C float (single-precision).
        // We parse into f32 to preserve that semantics.
        let v: f32 = match token.parse() {
            Ok(x) => x,
            // The spec assumes tokens are parseable as floats. If not, behavior is unspecified.
            // Choosing the narrowest reasonable behavior: stop processing further tokens.
            // (This keeps the implementation simple and consistent with the assumption.)
            Err(_) => break,
        };

        // Compute y = 4.9 * (v / 9.8)^2 using f32 operations.
        let ratio = v / 9.8_f32;
        let y = 4.9_f32 * ratio * ratio;

        // Compute k = ceil(y / 5.0) + 1
        let k_f = (y / 5.0_f32).ceil() + 1.0_f32;

        // Convert to signed integer. This uses Rust's `as` cast to i32,
        // which is the straightforward translation of the C cast in the spec.
        let k = k_f as i32;

        // Print one line per input value as specified.
        println!("{}", k);
    }

    // Normal termination -> exit code 0 (implicit).
}
