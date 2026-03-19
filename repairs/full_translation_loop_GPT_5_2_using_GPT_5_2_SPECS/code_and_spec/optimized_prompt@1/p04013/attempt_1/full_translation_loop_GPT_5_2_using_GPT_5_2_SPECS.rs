use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let a: i64 = it.next().unwrap().parse().unwrap();

    // x is 1-indexed to match the C code.
    let mut x: Vec<i64> = vec![0; n + 1];
    let mut sum: i64 = 0;
    for i in 1..=n {
        let v: i64 = it.next().unwrap().parse().unwrap();
        x[i] = v;
        sum += v;
    }

    // SPEC preconditions: sum is non-negative; a is non-zero.
    // The C code uses sum as an array bound; if sum < 0, behavior is undefined.
    // We follow the spec and assume sum >= 0.
    let sum_usize: usize = sum as usize;

    // dp[j][k][s] = number of ways to choose k elements from first j items with sum s.
    // Allocate on heap (Vec) rather than stack; semantics are the same.
    let mut dp = vec![vec![vec![0i64; sum_usize + 1]; n + 1]; n + 1];

    for j in 0..=n {
        for k in 0..=n {
            for s in 0..=sum_usize {
                dp[j][k][s] = if j >= 1 && (s as i64) < x[j] {
                    dp[j - 1][k][s]
                } else if j >= 1 && k >= 1 && (s as i64) >= x[j] {
                    let prev = dp[j - 1][k][s];
                    let idx = (s as i64 - x[j]) as usize;
                    prev + dp[j - 1][k - 1][idx]
                } else if j == 0 && k == 0 && s == 0 {
                    1
                } else {
                    0
                };
            }
        }
    }

    let mut ans: i64 = 0;

    // Match C's computation of max:
    // max = min(N, sum / A) using integer division.
    // Note: if A <= 0, C's behavior is still defined for division, but the spec only
    // requires A != 0; we preserve C's behavior.
    let max_k: i64 = {
        let div = sum / a;
        if div < n as i64 { div } else { n as i64 }
    };

    // In C, if max is negative, the loop doesn't execute. Preserve that.
    if max_k >= 1 {
        for k in 1..=max_k as usize {
            let target = (k as i64) * a;
            // In the C code, dp is indexed by k*A; if out of bounds, behavior is undefined.
            // Under the spec's assumptions, this should be within 0..=sum.
            if target >= 0 && (target as usize) <= sum_usize {
                ans += dp[n][k][target as usize];
            }
        }
    }

    print!("{ans}\n");
}
