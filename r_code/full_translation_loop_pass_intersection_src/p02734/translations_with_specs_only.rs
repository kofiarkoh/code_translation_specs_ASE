use std::io::{self, Read};

const MOD: u32 = 998_244_353;

fn add_mod(a: u32, b: u32) -> u32 {
    let s = a as u64 + b as u64;
    (s % MOD as u64) as u32
}

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let target: usize = it.next().unwrap().parse().unwrap();

    let mut a = vec![0usize; n];
    for i in 0..n {
        a[i] = it.next().unwrap().parse().unwrap();
    }

    // dp[s] = number of ways over all non-empty segments ending at current position
    // to choose a subset within that segment summing to s.
    //
    // Transition when processing value x:
    // - Start a new segment at this position: subset either empty (sum 0) or includes x (sum x).
    // - Extend any existing segment ending at previous position: for each existing subset sum s,
    //   either do not take x (sum stays s) or take x (sum becomes s + x).
    //
    // This yields:
    // new_dp[s] = (start_new contributes to s) + 2*dp[s] (extend, not take / take handled separately)
    // More explicitly:
    // new_dp[s] = start_new[s] + dp[s] (extend, not take) + dp[s - x] (extend, take)
    // where start_new[0] += 1, start_new[x] += 1 (if x <= target).
    let mut dp = vec![0u32; target + 1];
    let mut ans: u32 = 0;

    for &x in &a {
        let mut ndp = vec![0u32; target + 1];

        // Start new segment [i, i] with empty subset.
        ndp[0] = add_mod(ndp[0], 1);

        // Start new segment [i, i] with subset {i}.
        if x <= target {
            ndp[x] = add_mod(ndp[x], 1);
        }

        // Extend previous segments.
        for s in 0..=target {
            // Not take x.
            ndp[s] = add_mod(ndp[s], dp[s]);
            // Take x.
            if x <= s {
                ndp[s] = add_mod(ndp[s], dp[s - x]);
            }
        }

        dp = ndp;
        ans = add_mod(ans, dp[target]);
    }

    println!("{}", ans);
}
