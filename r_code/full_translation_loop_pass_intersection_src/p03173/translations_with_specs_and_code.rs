use std::io::{self, Read};

const SUP: i64 = 1i64 << 60;

fn recursion(l: usize, r: usize, dp: &mut Vec<Vec<i64>>, prefix: &[i64]) -> i64 {
    // Base case: single item costs 0 to "merge" (no merges).
    if l == r {
        return 0;
    }
    // Memoized: dp[l][r] == 0 means "not computed yet" for l < r.
    // This matches the C code's behavior; note that true minimal costs for l<r
    // are strictly positive when values are non-negative, but the spec does not
    // guarantee non-negativity. We still preserve the C program's semantics.
    if dp[l][r] != 0 {
        return dp[l][r];
    }

    let mut best = SUP;
    for k in (l + 1)..=r {
        let tmp = recursion(l, k - 1, dp, prefix) + recursion(k, r, dp, prefix);
        if tmp < best {
            best = tmp;
        }
    }

    let interval_sum = prefix[r] - prefix[l - 1];
    let ans = best + interval_sum;
    dp[l][r] = ans;
    ans
}

fn main() {
    // Read all input (scanf-like behavior; unspecified on malformed input).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    // 1-based arrays to match the C code and the spec.
    let mut a = vec![0i64; n + 1];
    for i in 1..=n {
        a[i] = it.next().unwrap().parse::<i32>().unwrap() as i64;
    }

    let mut prefix = vec![0i64; n + 1];
    for i in 1..=n {
        prefix[i] = prefix[i - 1] + a[i];
    }

    // dp dimensions: (n+1) x (n+1), indices 1..=n used.
    let mut dp = vec![vec![0i64; n + 1]; n + 1];

    let result = if n <= 1 { 0 } else { recursion(1, n, &mut dp, &prefix) };
    print!("{result}\n");
}
