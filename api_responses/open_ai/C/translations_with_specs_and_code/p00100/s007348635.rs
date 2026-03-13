use std::io::{self, Read};

fn main() {
    // Read entire stdin as whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // There are exactly 4000 tracked items (indices 0..3999 represent items 1..4000).
    let mut cumulative = [0i64; 4000];
    let mut reported = [false; 4000];

    // Helper to parse next token or exit if missing.
    let mut next_token = || -> Option<&str> { it.next() };

    // Read batches: each batch begins with an integer kazu. kazu == 0 terminates.
    loop {
        let tok = match next_token() {
            Some(t) => t,
            None => return, // EOF encountered; behave by exiting (input is assumed well-formed per spec)
        };
        let kazu: i32 = tok.parse().expect("expected integer for kazu");

        if kazu == 0 {
            break;
        }

        // If kazu is positive, read that many records. If kazu is negative, the C code's for-loop would skip,
        // so we mirror that behavior by doing nothing when kazu <= 0.
        if kazu > 0 {
            for _ in 0..kazu {
                // Read item number
                let num_tok = next_token().expect("unexpected EOF while reading number");
                let number: i32 = num_tok.parse().expect("expected integer for number");

                // Per specification, number is expected in 1..4000; behavior undefined otherwise.
                // We check and panic to fail-fast if violated (consistent with assuming well-formed input).
                if number < 1 || number > 4000 {
                    panic!("number out of range: {}", number);
                }
                let idx = (number as usize) - 1;

                // Read v1 and v2 as 64-bit signed integers
                let v1_tok = next_token().expect("unexpected EOF while reading v1");
                let v2_tok = next_token().expect("unexpected EOF while reading v2");
                let v1: i64 = v1_tok.parse().expect("expected 64-bit integer for v1");
                let v2: i64 = v2_tok.parse().expect("expected 64-bit integer for v2");

                // In C signed overflow is undefined; to avoid panics in Rust (and to have a deterministic behavior
                // similar to typical two's-complement wrapping), perform wrapping arithmetic.
                let prod = v1.wrapping_mul(v2);
                cumulative[idx] = cumulative[idx].wrapping_add(prod);
            }
        }

        // After processing the batch, output items that newly reach or exceed threshold and haven't been reported.
        let mut any = false;
        for i in 0..4000 {
            if cumulative[i] >= 1_000_000 && !reported[i] {
                println!("{}", i + 1);
                reported[i] = true;
                any = true;
            }
        }
        if !any {
            println!("NA");
        }
    }
}
