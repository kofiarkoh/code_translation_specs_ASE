use std::io::{self, Read};

const MOD: u64 = 1_000_000_007;

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Preconditions say inputs are valid; undefined behavior otherwise.
    let n: usize = it.next().unwrap().parse().unwrap();
    let target: usize = it.next().unwrap().parse().unwrap();

    let mut elems: Vec<usize> = Vec::with_capacity(n);
    for _ in 0..n {
        // The list contains at least as many elements as specified by the input.
        let v: usize = it.next().unwrap().parse().unwrap();
        elems.push(v);
    }

    // Dynamic programming:
    // dp[s] = number of ways to reach sum s, considering order of selection.
    // This corresponds to counting sequences where each step chooses any element.
    let mut dp = vec![0u64; target + 1];
    dp[0] = 1;

    for s in 1..=target {
        let mut ways = 0u64;
        for &x in &elems {
            if x <= s {
                ways += dp[s - x];
                if ways >= MOD {
                    ways %= MOD;
                }
            }
        }
        dp[s] = ways % MOD;
    }

    println!("{}", dp[target] % MOD);
}
