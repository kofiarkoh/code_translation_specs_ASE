use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return, // No explicit error handling specified.
    };

    // Supported bound is 1..=400; behavior is undefined beyond that per spec.
    let mut w: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        let v: i64 = it.next().unwrap().parse().unwrap();
        w.push(v);
    }

    if n <= 1 {
        // Postcondition: for length 1, output 0.
        println!("0");
        return;
    }

    // Prefix sums: prefix[i] = sum of w[0..i)
    let mut prefix = vec![0i64; n + 1];
    for i in 0..n {
        prefix[i + 1] = prefix[i] + w[i];
    }

    // Memoization table for minimal merge cost of interval [l, r] inclusive.
    // The specification mentions a "0 means not computed" memoization hazard.
    // We emulate that behavior by using 0 as the sentinel for "not computed".
    // This relies on the precondition that any non-singleton interval's true
    // minimum cost is nonzero; otherwise behavior is not reliably inferable.
    let mut memo = vec![vec![0i64; n]; n];

    fn sum(prefix: &[i64], l: usize, r: usize) -> i64 {
        // Sum of w[l..=r]
        prefix[r + 1] - prefix[l]
    }

    fn solve(
        l: usize,
        r: usize,
        prefix: &[i64],
        memo: &mut [Vec<i64>],
    ) -> i64 {
        if l >= r {
            return 0;
        }
        if memo[l][r] != 0 {
            return memo[l][r];
        }

        let total = sum(prefix, l, r);
        let mut best: i64 = i64::MAX;

        for k in l..r {
            let left = solve(l, k, prefix, memo);
            let right = solve(k + 1, r, prefix, memo);
            let cand = left + right + total;
            if cand < best {
                best = cand;
            }
        }

        memo[l][r] = best;
        best
    }

    let ans = solve(0, n - 1, &prefix, &mut memo);
    println!("{ans}");
}
