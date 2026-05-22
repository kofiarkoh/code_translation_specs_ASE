use std::io::{self, Read};

const M: i64 = 998_244_353;
const CAP: usize = 90310;

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // SPEC: at least one integer must be present; otherwise unspecified.
    // We mirror C behavior by assuming it exists.
    let _first: i64 = it.next().unwrap().parse().unwrap();

    // Fixed-size DP arrays as in C (bounded capacity).
    let mut d = vec![0i64; CAP];
    let mut e = vec![0i64; CAP];

    // Initialization occurs if the first integer is successfully read.
    d[0] = 1;
    e[0] = 1;

    let mut s: usize = 0; // running sum of subsequent integers
    let mut r: i64 = 1;   // baseline, becomes 3^n mod M

    // Process remaining integers until EOF.
    for tok in it {
        let a_i: usize = tok.parse::<i64>().unwrap() as usize; // SPEC assumes nonnegative
        // Decreasing-index DP update: for t from s down to 0.
        for t in (0..=s).rev() {
            let idx = t + a_i; // SPEC assumes within capacity; no checks (as in C).
            d[idx] = (d[idx] + (d[t] * 2) % M) % M;
            e[idx] = (e[idx] + e[t]) % M;
        }
        s += a_i;
        r = (r * 3) % M;
    }

    // Finalization:
    // Subtract 3 * D[t] for t = 0..floor(s/2).
    for t in 0..=s / 2 {
        r = (r - (d[t] * 3) % M) % M;
    }

    // If s is even, additionally subtract 3 * E[s/2].
    if s % 2 == 0 {
        r = (r - (e[s / 2] * 3) % M) % M;
    }

    // Normalize to [0, M-1] and print.
    r %= M;
    if r < 0 {
        r += M;
    }
    print!("{}", r);
}
