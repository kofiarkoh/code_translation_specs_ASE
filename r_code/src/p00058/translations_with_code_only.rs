use std::io::{self, Read};

fn main() {
    // Read entire stdin to a string, split on whitespace, parse f64 tokens.
    // This mirrors the C program's scanf("%lf ...") behavior for well-formed numeric input.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Collect only successfully parsed f64 values. If a token fails to parse,
    // we stop collecting (similar to scanf stopping on a matching failure).
    let mut nums: Vec<f64> = Vec::new();
    for tok in input.split_whitespace() {
        match tok.parse::<f64>() {
            Ok(v) => nums.push(v),
            Err(_) => break,
        }
    }

    // Process in groups of 8 numbers. The original C code used scanf in a loop
    // testing for EOF; it effectively processed complete 8-tuples from input.
    // For any trailing incomplete group we stop (avoids undefined behavior from
    // partially-initialized variables in the original C).
    for chunk in nums.chunks(8) {
        if chunk.len() < 8 {
            break;
        }
        // chunk is &[f64] with length >= 8
        let a0 = chunk[0];
        let a1 = chunk[1];
        let a2 = chunk[2];
        let a3 = chunk[3];
        let a4 = chunk[4];
        let a5 = chunk[5];
        let a6 = chunk[6];
        let a7 = chunk[7];

        // Preserve the exact floating-point comparison used in C.
        let expr = (a1 - a3) * (a5 - a7) + (a0 - a2) * (a4 - a6);
        if expr == 0.0 {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
