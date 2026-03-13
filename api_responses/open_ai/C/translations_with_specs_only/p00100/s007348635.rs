use std::io::{self, Read};

fn main() {
    // THRESHOLD as specified
    const THRESHOLD: i64 = 1_000_000;
    // Number of tracked items: 1..4000
    const N_ITEMS: usize = 4000;

    // Read entire stdin as whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");

    // Token iterator; unwraps on parse errors / missing tokens to mirror unspecified behavior.
    let mut tokens = input.split_whitespace();
    // cumulative totals, initialized to 0
    let mut cum = [0i64; N_ITEMS];
    // reported flags
    let mut reported = [false; N_ITEMS];

    loop {
        // Read kazu (int). Per specification, kazu is an int; use i32.
        let kazu_str = match tokens.next() {
            Some(s) => s,
            None => break, // EOF unexpectedly: break (mirrors unspecified behavior; could also panic)
        };
        let kazu: i32 = kazu_str.parse().expect("failed to parse kazu");

        if kazu == 0 {
            break;
        }

        // If kazu is positive, read that many records. If negative, do not read records (per spec).
        if kazu > 0 {
            // Note: we deliberately do not guard against out-of-range `kazu` to stay faithful to specification.
            for _ in 0..(kazu as usize) {
                // Read number, v1, v2
                let number_str = tokens.next().expect("missing number token");
                let v1_str = tokens.next().expect("missing v1 token");
                let v2_str = tokens.next().expect("missing v2 token");

                let number: i32 = number_str.parse().expect("failed to parse number");
                let v1: i64 = v1_str.parse().expect("failed to parse v1");
                let v2: i64 = v2_str.parse().expect("failed to parse v2");

                // Compute product and add to cumulative total using wrapping arithmetic to emulate typical
                // two's-complement overflow behavior (C signed overflow is undefined; this choice preserves
                // observable wraparound semantics commonly relied upon).
                let prod = v1.wrapping_mul(v2);
                // Convert number (1-based) to 0-based index. This will panic on out-of-range, matching undefined behavior.
                let idx = (number - 1) as usize;
                cum[idx] = cum[idx].wrapping_add(prod);
            }
        }

        // After processing the batch, find items newly reaching threshold, in ascending order.
        let mut any = false;
        for (i, &total) in cum.iter().enumerate() {
            if !reported[i] && total >= THRESHOLD {
                println!("{}", i + 1); // print 1-based item number
                reported[i] = true;
                any = true;
            }
        }
        if !any {
            println!("NA");
        }
    }
}
