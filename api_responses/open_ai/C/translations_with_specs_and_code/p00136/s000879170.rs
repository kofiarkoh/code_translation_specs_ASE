use std::io::{self, Read};

fn main() {
    // Read all input from stdin and split on whitespace tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");

    let mut tokens = input.split_whitespace();

    // Spec precondition: the first token is a parsable integer n.
    // We follow the spec and assume this parse succeeds for well-formed inputs.
    // If it fails, unwrap() will cause a panic (undefined behavior per spec is allowed).
    let n_token = match tokens.next() {
        Some(t) => t,
        None => {
            // No tokens at all: behave like n can't be read (spec allows undefined behavior).
            // We choose to print six empty buckets (same as n <= 0) and exit normally.
            for i in 1..=6 {
                println!("{}:", i);
            }
            return;
        }
    };
    let n: i32 = n_token.parse().expect("first token must be an integer per spec");

    // Use an array of length 7 and use indices 1..=6 as in the C program / spec.
    let mut cnt: [usize; 7] = [0; 7];

    // If n <= 0, process zero values (matches C for-loop behavior).
    let target: usize = if n > 0 { n as usize } else { 0 };

    // Process tokens in order: we count a value as "processed" when it successfully parses as f64,
    // even if it doesn't match any bucket (e.g., NaN). This matches the spec notes about NaN.
    let mut processed: usize = 0;
    while processed < target {
        match tokens.next() {
            Some(tok) => {
                match tok.parse::<f64>() {
                    Ok(v) => {
                        // Successfully read a double: classify it into buckets per spec.
                        // Note: comparisons with NaN will be false, so NaN will not increment any bucket.
                        if v < 165.0 {
                            cnt[1] += 1;
                        } else if v < 170.0 {
                            cnt[2] += 1;
                        } else if v < 175.0 {
                            cnt[3] += 1;
                        } else if v < 180.0 {
                            cnt[4] += 1;
                        } else if v < 185.0 {
                            cnt[5] += 1;
                        } else if v >= 185.0 {
                            cnt[6] += 1;
                        }
                        processed += 1;
                    }
                    Err(_) => {
                        // Token could not be parsed as a double: per spec, such tokens do not
                        // affect counters. Continue to next token without incrementing `processed`.
                        continue;
                    }
                }
            }
            None => break, // No more tokens available.
        }
    }

    // Print six lines: "i:" followed immediately by cnt[i] asterisks, then newline.
    for i in 1..=6 {
        // Using repeat to produce the required number of asterisks.
        let stars = "*".repeat(cnt[i]);
        println!("{}:{}", i, stars);
    }
}
