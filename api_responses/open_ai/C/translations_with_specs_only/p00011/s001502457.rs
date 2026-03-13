use std::io::{self, Read};

fn main() {
    // Read entire stdin. The Specification assumes well-formed input and no validation.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");

    // Tokens are expected to be whitespace-separated; each swap is a single token like "a,b".
    let mut toks = input.split_whitespace();

    // Parse w and n. On malformed input, behavior is allowed to be undefined per the spec,
    // so unwrap/expect is acceptable to mirror that.
    let w_tok = toks.next().expect("expected w");
    let w: usize = w_tok.parse().expect("failed to parse w");

    let n_tok = toks.next().expect("expected n");
    let n: usize = n_tok.parse().expect("failed to parse n");

    // Use a 1-based array of length w+1; index 0 is unused. Values are integers 1..=w.
    let mut l = vec![0i32; w + 1];
    for i in 1..=w {
        l[i] = i as i32;
    }

    // For each of the n operations, read one token like "a,b", split on comma, parse and swap.
    for _ in 0..n {
        let pair_tok = toks.next().expect("expected pair token");
        let mut parts = pair_tok.splitn(2, ',');
        let a_str = parts.next().expect("expected first index");
        let b_str = parts.next().expect("expected second index");
        let a: usize = a_str.parse().expect("failed to parse a");
        let b: usize = b_str.parse().expect("failed to parse b");

        // Perform swap. If a or b are out of bounds, this will panic (undefined behavior as per spec).
        l.swap(a, b);
    }

    // Print positions 1..=w, one per line.
    for i in 1..=w {
        println!("{}", l[i]);
    }
}
