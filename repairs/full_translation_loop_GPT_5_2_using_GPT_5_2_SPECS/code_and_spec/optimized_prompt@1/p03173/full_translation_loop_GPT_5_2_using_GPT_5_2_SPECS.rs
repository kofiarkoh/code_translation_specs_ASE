use std::io::{self, Read};

const SUP: i64 = 1i64 << 60;

fn recursion(l: usize, r: usize, dp: &mut Vec<Vec<i64>>, sum: &Vec<i64>) -> i64 {
    // Mirrors C semantics:
    // - dp initialized to 0
    // - dp[l][r] == 0 is treated as "not computed" except when l == r
    // Spec notes this requires the assumption that true minimum costs for non-singletons are nonzero.
    if dp[l][r] != 0 || l == r {
        return dp[l][r];
    } else {
        dp[l][r] = SUP;
    }

    for k in (l + 1)..=r {
        let tmp = recursion(l, k - 1, dp, sum) + recursion(k, r, dp, sum);
        if tmp < dp[l][r] {
            dp[l][r] = tmp;
        }
    }

    dp[l][r] += sum[r] - sum[l - 1];
    dp[l][r]
}

fn main() {
    // Read all input (no explicit error handling, per spec).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    // 1-based indexing to match the C code.
    let mut a = vec![0i64; n + 1];
    for i in 1..=n {
        a[i] = it.next().unwrap().parse::<i64>().unwrap();
    }

    let mut sum = vec![0i64; n + 1];
    for i in 1..=n {
        sum[i] = sum[i - 1] + a[i];
    }

    let mut dp = vec![vec![0i64; n + 1]; n + 1];

    let ans = if n == 0 {
        // Not in spec (precondition says 1..=400), but avoid panic on recursion(1,0).
        0i64
    } else {
        recursion(1, n, &mut dp, &sum)
    };

    print!("{ans}\n");
}
