use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

fn main() {
    // Read entire stdin, parse two integers (no explicit error handling per spec).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: i32 = it.next().unwrap().parse().unwrap();
    let m: i32 = it.next().unwrap().parse().unwrap();

    // Fixed-size DP table as in C (bounds assumed valid by preconditions).
    let mut dp = vec![vec![0i64; 302]; 302];
    dp[1][1] = 1;

    // Semantics-preserving translation of the in-place reverse-iteration DP.
    for k in 1..=m {
        for i in (1..=k).rev() {
            for j in (1..=i).rev() {
                let tmp = dp[i as usize][j as usize] % MOD;
                dp[i as usize][j as usize] = 0;

                // Note: This matches the C code exactly, including use of signed arithmetic
                // for (n - i). The spec states overflow/invalid inputs are undefined.
                let ni = (n - i) as i64;
                let jj = j as i64;
                let ij = (i - j) as i64;

                dp[(i + 1) as usize][j as usize] += tmp * ni;
                dp[i as usize][i as usize] += tmp * jj;
                dp[i as usize][j as usize] += tmp * ij;
            }
        }
    }

    let ans = dp[n as usize][n as usize] % MOD;
    println!("{ans}");
}
