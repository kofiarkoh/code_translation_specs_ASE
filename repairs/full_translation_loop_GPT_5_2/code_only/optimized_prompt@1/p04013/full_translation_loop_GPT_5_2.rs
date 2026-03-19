use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as i64 like scanf("%lld")
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let a: i64 = it.next().unwrap().parse().unwrap();

    // C uses VLA with indices 1..=N; keep x[0] unused.
    let n_usize = if n < 0 { 0 } else { n as usize };
    let mut x = vec![0i64; n_usize + 1];

    let mut sum: i64 = 0;
    for i in 1..=n_usize {
        let xi: i64 = it.next().unwrap().parse().unwrap();
        x[i] = xi;
        // In C, signed overflow is UB; we use wrapping to emulate typical 2's complement behavior.
        sum = sum.wrapping_add(xi);
    }

    // dp[j][k][s] for 0<=j<=N, 0<=k<=N, 0<=s<=sum
    // C allocates on stack; in Rust we allocate on heap.
    // If sum is negative, the C code's VLA size is nonsensical/UB; we clamp to 0 to keep program defined.
    let sum_usize = if sum < 0 { 0 } else { sum as usize };

    // Flattened 3D array: index = ((j*(N+1) + k)*(sum+1) + s)
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
                    let s2 = (s as i64).wrapping_sub(x[j]);
                    // In C, s-x[j] is used as an index only when s>=x[j], so s2 should be >=0.
                    // Still, keep it robust.
                    let s2u = if s2 < 0 { 0 } else { s2 as usize };
                    dp[idx(j - 1, k, s, dim_k, dim_s)]
                        .wrapping_add(dp[idx(j - 1, k - 1, s2u, dim_k, dim_s)])
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

    // C: if (sum/A < N) max = sum/A else max = N;
    // Division by zero in C is UB; here we avoid panic and mimic "no valid k" by setting max=0.
    let max_k: i64 = if a == 0 {
        0
    } else {
        let q = sum / a; // truncates toward zero like C
        if q < n { q } else { n }
    };

    if max_k >= 1 {
        for k in 1..=max_k {
            let target = k.wrapping_mul(a);
            if target >= 0 && (target as usize) <= sum_usize {
                ans = ans.wrapping_add(dp[idx(n_usize, k as usize, target as usize, dim_k, dim_s)]);
            } else {
                // In C, out-of-bounds would be UB; we treat it as contributing 0.
            }
        }
    }

    print!("{ans}\n");
}
