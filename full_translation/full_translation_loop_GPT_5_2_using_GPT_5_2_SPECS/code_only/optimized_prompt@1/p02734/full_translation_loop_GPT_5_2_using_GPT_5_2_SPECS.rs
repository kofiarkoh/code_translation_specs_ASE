use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as whitespace-separated integers (like scanf).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return, // No input; C would have undefined behavior, but we just exit.
    };
    let s_i32: i32 = it.next().unwrap().parse().unwrap();

    // In the C code, s is an int used for array sizes; negative s would be UB.
    // We emulate the "intended" domain by treating s as non-negative for allocation.
    let s: usize = if s_i32 < 0 { 0 } else { s_i32 as usize };

    let mut a: Vec<i32> = vec![0; n];
    for i in 0..n {
        a[i] = it.next().unwrap().parse().unwrap();
    }

    let modu: i64 = 998_244_353;

    // dp[n][s+1][2] as i64 (C long). Use a flat Vec for efficiency and simplicity.
    // Index: ((i*(s+1) + j)*2 + k)
    let cols = s + 1;
    let mut dp: Vec<i64> = vec![0; n * cols * 2];
    let idx = |i: usize, j: usize, k: usize, cols: usize| -> usize { (i * cols + j) * 2 + k };

    if n == 0 {
        // C would have VLA of size 0 and then dp[0] access => UB.
        // No well-defined behavior to preserve; do nothing.
        return;
    }

    dp[idx(0, 0, 0, cols)] = 1;
    dp[idx(0, 0, 1, cols)] = 1;

    if a[0] >= 0 {
        let a0 = a[0] as usize;
        if a0 <= s {
            dp[idx(0, a0, 0, cols)] = 1;
            dp[idx(0, a0, 1, cols)] = 1;
        }
    }

    for i in 1..n {
        for j in 0..=s {
            // dp[i][j][0] += dp[i-1][j][0];
            let prev = dp[idx(i - 1, j, 0, cols)];
            let cur_idx = idx(i, j, 0, cols);
            dp[cur_idx] = dp[cur_idx].wrapping_add(prev);

            // if (j + a[i] <= s) { ... }
            if a[i] >= 0 {
                let ai = a[i] as usize;
                if j + ai <= s {
                    let to_idx = idx(i, j + ai, 0, cols);
                    dp[to_idx] = dp[to_idx].wrapping_add(prev);
                    dp[to_idx] %= modu;
                }
            }
        }

        // dp[i][0][0]++;
        let z_idx = idx(i, 0, 0, cols);
        dp[z_idx] = dp[z_idx].wrapping_add(1);

        // if (a[i] <= s) { dp[i][a[i]][0]++; dp[i][a[i]][0] %= mod; }
        if a[i] >= 0 {
            let ai = a[i] as usize;
            if ai <= s {
                let ai_idx = idx(i, ai, 0, cols);
                dp[ai_idx] = dp[ai_idx].wrapping_add(1);
                dp[ai_idx] %= modu;
            }
        }

        for j in 0..=s {
            // dp[i][j][1] = dp[i-1][j][1] + dp[i][j][0]; dp[i][j][1] %= mod;
            let v = dp[idx(i - 1, j, 1, cols)].wrapping_add(dp[idx(i, j, 0, cols)]);
            dp[idx(i, j, 1, cols)] = v % modu;
        }
    }

    println!("{}", dp[idx(n - 1, s, 1, cols)]);
}
