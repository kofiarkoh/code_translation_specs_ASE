// knapsack.rs
//
// Translates the provided Specification into Rust.
// Implements a 0/1 knapsack per input instance, maximizing total value (coin)
// with a weight capacity W, and among selections achieving the maximum value
// chooses the minimal total weight. Input format is sequential tokens:
//   W
//   N
//   N tokens of the form "value,weight" (exactly a comma between two integers)
// A W value of 0 terminates the program (no output for that sentinel).
//
// The original specification implied internal arrays sized up to 1000×1000
// (indices up to 999). To remain faithful, this implementation enforces the
// same bounds: 0 <= N <= 999 and 0 <= W <= 999. If inputs violate these
// constraints, the program will panic (mirroring the undefined/unsafe behavior
// described in the specification).

use std::io::{self, Read};

fn parse_i32_token(tok: &str, ctx: &str) -> i32 {
    tok.parse::<i32>()
        .unwrap_or_else(|_| panic!("Failed to parse integer for {}: '{}'", ctx, tok))
}

fn main() {
    // Read whole stdin
    let mut input = String::new();
    io::stdin()
        .read_to_string(&mut input)
        .expect("Failed to read stdin");

    // Tokenize by whitespace. Specification expects tokens in sequence.
    let mut toks = input.split_whitespace();

    let mut case_no: i32 = 1;

    loop {
        let w_tok = match toks.next() {
            Some(t) => t,
            None => break, // EOF reached unexpectedly; stop processing.
        };

        let W = parse_i32_token(w_tok, "W");

        // Termination sentinel: W == 0 -> normal termination, no output for sentinel.
        if W == 0 {
            break;
        }

        // Validate W according to inferred internal limits.
        if W < 0 || W > 999 {
            panic!("W out of supported range [0,999]: {}", W);
        }
        let W_usize = W as usize;

        // Read N
        let n_tok = toks
            .next()
            .expect("Expected N token after W but found EOF");
        let N = parse_i32_token(n_tok, "N");
        if N < 0 || N > 999 {
            panic!("N out of supported range [0,999]: {}", N);
        }
        let N_usize = N as usize;

        // Read N items in "value,weight" form.
        let mut values: Vec<i32> = Vec::with_capacity(N_usize);
        let mut weights: Vec<usize> = Vec::with_capacity(N_usize);

        for idx in 0..N_usize {
            let item_tok = toks
                .next()
                .unwrap_or_else(|| panic!("Expected item {} but found EOF", idx + 1));
            // Expect exactly one comma separating two integers, as per "%d,%d".
            let mut parts = item_tok.splitn(2, ',');
            let v_str = parts
                .next()
                .expect("splitn always returns at least one part");
            let w_str = parts
                .next()
                .unwrap_or_else(|| panic!("Item token missing comma-separated second integer: '{}'", item_tok));
            let val = parse_i32_token(v_str, &format!("item {} value", idx + 1));
            let wt = parse_i32_token(w_str, &format!("item {} weight", idx + 1));
            if wt < 0 {
                panic!("Negative weight encountered for item {}: {}", idx + 1, wt);
            }
            let wt_usize = wt as usize;
            if wt_usize > 999 {
                // This could cause out-of-bounds in DP since W <= 999; treat as unsupported.
                // The specification treats out-of-range behavior as undefined; here we panic explicitly.
                panic!("Item weight out of supported range [0,999]: {}", wt_usize);
            }
            values.push(val);
            weights.push(wt_usize);
        }

        // DP table: (N+1) x (W+1), storing maximal coin value achievable.
        // Use i32 for values as specified by input types.
        // dp[i][w] = maximum value using first i items with total weight <= w.
        // Initialize to 0 for the base case (i=0).
        let mut dp: Vec<Vec<i32>> = vec![vec![0i32; W_usize + 1]; N_usize + 1];

        for i in 1..=N_usize {
            let val = values[i - 1];
            let wt = weights[i - 1];
            for cap in 0..=W_usize {
                // Without current item
                let without = dp[i - 1][cap];
                // With current item if weight allows
                let with = if cap >= wt {
                    // cap - wt is valid usize
                    let candidate = dp[i - 1][cap - wt].saturating_add(val);
                    candidate
                } else {
                    // cannot include item
                    i32::MIN // so max picks 'without'
                };
                dp[i][cap] = if with > without { with } else { without };
            }
        }

        // Determine maximum value achievable with weight <= W.
        let last_row = &dp[N_usize];
        let mut max_value = last_row[0];
        for &v in last_row.iter().take(W_usize + 1) {
            if v > max_value {
                max_value = v;
            }
        }

        // Among weights achieving max_value, find minimal weight (smallest cap).
        // The minimal weight is the smallest w in [0..=W] such that dp[N][w] == max_value.
        let mut min_weight_for_max: Option<usize> = None;
        for w in 0..=W_usize {
            if last_row[w] == max_value {
                min_weight_for_max = Some(w);
                break;
            }
        }
        // As per specification, empty selection is valid so at least dp[..][0] exists.
        let min_weight = min_weight_for_max
            .expect("Invariant violated: at least empty selection should be feasible");

        // Output exactly:
        // Case X:
        // M
        // Wmin
        println!("Case {}:", case_no);
        println!("{}", max_value);
        println!("{}", min_weight);

        case_no += 1;
    }

    // Normal termination (exit status 0)
}
