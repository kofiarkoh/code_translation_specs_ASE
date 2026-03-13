use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated tokens, matching C's scanf behavior.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut toks = input.split_whitespace();

    // `first` tracks whether we've printed any dataset yet, to insert a single blank
    // line before each dataset except the first (matches the C program & spec).
    let mut first = true;

    // Loop over datasets. The SPEC defines termination when n == 0.
    loop {
        // Read dataset size `n`. If missing, stop (SPEC assumes well-formed input).
        let n_token = match toks.next() {
            Some(t) => t,
            None => break,
        };
        let n: i32 = match n_token.parse() {
            Ok(v) => v,
            Err(_) => break, // malformed input: SPEC treats such cases as undefined; we stop.
        };

        if n == 0 {
            break;
        }

        // Print a single blank line between consecutive dataset outputs (no leading blank line).
        if !first {
            println!();
        }

        // In C, for negative n the for-loop `for(i=0;i<n;i++)` does not execute.
        // To match that behavior, treat negative n as 0 iterations (but still count as a dataset).
        let n_usize = if n > 0 { n as usize } else { 0usize };

        // Read n records: name (string) and three integers a, b, c.
        // Compute score = 3*a + c for each record.
        let mut records: Vec<(String, i32)> = Vec::with_capacity(n_usize);
        for _ in 0..n_usize {
            let name = toks.next().expect("missing name token").to_string();
            let a: i32 = toks.next().expect("missing a token").parse().expect("expected int");
            let _b: i32 = toks.next().expect("missing b token").parse().expect("expected int");
            let c: i32 = toks.next().expect("missing c token").parse().expect("expected int");
            let score = a.saturating_mul(3).saturating_add(c);
            records.push((name, score));
        }

        // Sort records in non-increasing (descending) order of score.
        // Use a stable sort so that records with equal score preserve input order.
        records.sort_by(|x, y| y.1.cmp(&x.1));

        // Print each record as "name,score" on its own line.
        for (name, score) in records {
            println!("{},{}", name, score);
        }

        // Mark that we've processed at least one dataset (even if n was negative and printed nothing).
        first = false;
    }
}
