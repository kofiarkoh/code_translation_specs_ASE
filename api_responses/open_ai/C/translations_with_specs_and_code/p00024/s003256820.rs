use std::io::{self, Read};

fn main() {
    // Read all stdin into a string and iterate whitespace-separated tokens.
    // The SPEC assumes inputs are parseable floats; on a parse error we stop processing.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read error, mimic C program behavior which would terminate on EOF/read error.
        return;
    }

    for token in input.split_whitespace() {
        // Parse token as f32 (C's float).
        let v: f32 = match token.parse() {
            Ok(x) => x,
            // If token isn't a valid float, the original C program with scanf("%f", &v)
            // would fail to make progress; the SPEC does not require handling this case.
            // Here we choose to stop processing further tokens.
            Err(_) => break,
        };

        // Compute using f32 arithmetic to match C's float behavior.
        let t = v / 9.8_f32;
        let y = t * t * 4.9_f32;

        // According to the SPEC: compute ceil(y/5.0) + 1, then convert to integer and print.
        let k_float = (y / 5.0_f32).ceil() + 1.0_f32;

        // Cast to i32 and print one integer per line.
        // Note: behavior for NaN/Inf/overflow mirrors that the SPEC says is not explicitly handled.
        let k_int = k_float as i32;
        println!("{}", k_int);
    }
}
