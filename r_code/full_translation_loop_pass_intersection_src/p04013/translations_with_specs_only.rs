use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return, // Missing input => unspecified; exit quietly.
    };
    let a: i64 = it.next().unwrap().parse().unwrap(); // target (precondition: non-zero)

    let mut xs: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        xs.push(it.next().unwrap().parse().unwrap());
    }

    // Precondition: sum of all input elements is non-negative.
    // If violated, behavior is undefined per spec; we do not add explicit handling.
    let total_sum_i64: i64 = xs.iter().copied().sum();
    let total_sum: usize = total_sum_i64 as usize;

    // dp[k][s] = number of ways to choose exactly k elements with sum exactly s
    // from the processed prefix. This matches the invariant in the specification.
    //
    // Note: The spec mentions stack storage; Rust's Vec allocates on the heap.
    // The observable behavior is the same; failure modes (OOM vs stack overflow)
    // are unspecified/undefined anyway per spec.
    let mut dp: Vec<Vec<u128>> = vec![vec![0u128; total_sum + 1]; n + 1];
    dp[0][0] = 1;

    for &x in &xs {
        // The computation assumes subset sums are representable over a non-negative range.
        // We therefore only update for non-negative x; negative x would imply undefined behavior.
        let xi = x as usize;

        // Iterate k descending to avoid reusing the same element multiple times.
        for k in (1..=n).rev() {
            // Iterate s descending to preserve 0/1 choice semantics.
            for s in (xi..=total_sum).rev() {
                let add = dp[k - 1][s - xi];
                dp[k][s] += add;
            }
        }
    }

    // Sum over all non-empty subset sizes k >= 1 of dp[k][k * a], when in range.
    // Only count sizes where k*a is within achievable non-negative sum range.
    let mut ans: u128 = 0;
    for k in 1..=n {
        let target_sum_i64 = (k as i64) * a;
        if target_sum_i64 < 0 {
            continue;
        }
        let target_sum = target_sum_i64 as usize;
        if target_sum <= total_sum {
            ans += dp[k][target_sum];
        }
    }

    // Output as a single integer. The spec assumes it fits in the output type.
    println!("{}", ans);
}
