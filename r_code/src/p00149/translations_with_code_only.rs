use std::io::{self, Read};

fn main() {
    // Corresponds to C's file-scope zero-initialized arrays `int rn[4], ln[4];`
    let mut rn = [0i32; 4];
    let mut ln = [0i32; 4];

    // Read all stdin (similar to repeated scanf calls until EOF).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read stdin");

    // Tokenize by whitespace and attempt to parse f64 values.
    // We iterate two-at-a-time to emulate reading pairs "right left".
    let mut toks = input.split_whitespace().map(|s| s.parse::<f64>());

    loop {
        // Try to get the next two f64 values. If parsing fails or we hit EOF,
        // stop processing. This matches the common/expected usage of the C code
        // where input is well-formed pairs of doubles.
        let right = match toks.next() {
            Some(Ok(v)) => v,
            _ => break,
        };
        let left = match toks.next() {
            Some(Ok(v)) => v,
            _ => break,
        };

        // The comparisons are the same as in C (double precision).
        if right >= 1.1 {
            rn[0] += 1;
        } else if right >= 0.6 {
            rn[1] += 1;
        } else if right >= 0.2 {
            rn[2] += 1;
        } else {
            rn[3] += 1;
        }

        if left >= 1.1 {
            ln[0] += 1;
        } else if left >= 0.6 {
            ln[1] += 1;
        } else if left >= 0.2 {
            ln[2] += 1;
        } else {
            ln[3] += 1;
        }
    }

    // Print results exactly as the C program: four lines "rn[i] ln[i]\n".
    for i in 0..4 {
        println!("{} {}", rn[i], ln[i]);
    }
}
