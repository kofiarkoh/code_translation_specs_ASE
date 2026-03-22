use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as i64 (matches C's long long on typical platforms).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let a: i64 = it.next().unwrap().parse().unwrap();

    // C uses VLA: long long x[N+1]; with indices 1..=N.
    // We'll store x[0]=0 unused to preserve indexing.
    let n_usize = if n < 0 { 0 } else { n as usize };
    let mut x = vec![0i64; n_usize + 1];

    let mut sum: i64 = 0;
    for i in 1..=n_usize {
        let xi: i64 = it.next().unwrap().parse().unwrap();
        x[i] = xi;
        // In C, signed overflow is UB; we assume inputs avoid it.
        sum += xi;
    }

    // dp[j][k][s] where:
    // j in 0..=N, k in 0..=N, s in 0..=sum
    // Use a flat Vec for memory; indices are usize.
    // Note: if sum is negative, the C code's VLA size is invalid/UB.
    // We emulate typical intended constraints by treating negative sum as 0-sized s-dimension.
    let sum_usize = if sum < 0 { 0 } else { sum as usize };
    let dim_j = n_usize + 1;
    let dim_k = n_usize + 1;
    let dim_s = sum_usize + 1;

    let total = dim_j
        .saturating_mul(dim_k)
        .saturating_mul(dim_s);
    let mut dp = vec![0i64; total];

    let idx = |j: usize, k: usize, s: usize, dim_k: usize, dim_s: usize| -> usize {
        (j * dim_k + k) * dim_s + s
    };

    for j in 0..=n_usize {
        for k in 0..=n_usize {
            for s in 0..=sum_usize {
                let val = if j >= 1 && (s as i64) < x[j] {
                    dp[idx(j - 1, k, s, dim_k, dim_s)]
                } else if j >= 1 && k >= 1 && (s as i64) >= x[j] {
                    let s2 = (s as i64) - x[j];
                    // In C, s2 is non-negative here due to condition.
                    let s2u = s2 as usize;
                    dp[idx(j - 1, k, s, dim_k, dim_s)]
                        + dp[idx(j - 1, k - 1, s2u, dim_k, dim_s)]
                } else if j == 0 && k == 0 && s == 0 {
                    1
                } else {
                    0
                };
                let p = idx(j, k, s, dim_k, dim_s);
                dp[p] = val;
            }
        }
    }

    let mut ans: i64 = 0;

    // C: if(sum/A < N) max = sum/A else max = N;
    // This uses C's signed division truncating toward zero.
    // Also note: division by zero would be UB in C; we mirror by panicking.
    let max_k: i64 = {
        let q = sum / a;
        if q < n { q } else { n }
    };

    // for k=1..=max: ans += dp[N][k][k*A];
    // If k*A is out of [0..=sum], C would index out of bounds (UB).
    // We assume inputs keep it in range; otherwise we avoid panic by skipping.
    for k in 1..=max_k {
        if k < 0 {
            continue;
        }
        let ku = k as usize;
        let s = k * a;
        if s < 0 || s > sum {
            continue;
        }
        let su = s as usize;
        ans += dp[idx(n_usize, ku, su, dim_k, dim_s)];
    }

    print!("{ans}\n");
}
