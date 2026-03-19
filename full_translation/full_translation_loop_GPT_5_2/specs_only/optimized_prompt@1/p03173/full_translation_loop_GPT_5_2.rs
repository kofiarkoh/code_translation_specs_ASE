use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(t) => t.parse().unwrap(),
        None => return,
    };

    let mut a: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        if let Some(t) = it.next() {
            a.push(t.parse::<i64>().unwrap());
        } else {
            // Specification requires exactly n integers; if missing, do nothing further.
            return;
        }
    }

    // Prefix sums for O(1) range sum queries.
    let mut pref = vec![0i64; n + 1];
    for i in 0..n {
        pref[i + 1] = pref[i].wrapping_add(a[i]);
    }
    let range_sum = |l: usize, r: usize, pref: &Vec<i64>| -> i64 {
        // sum of a[l..=r]
        pref[r + 1].wrapping_sub(pref[l])
    };

    // DP matrix: dp[l][r] = minimum cost to merge subarray a[l..=r].
    // Invariant: once computed, stored and reused (memoization).
    let mut dp: Vec<Vec<Option<i64>>> = vec![vec![None; n]; n];

    fn solve(
        l: usize,
        r: usize,
        pref: &Vec<i64>,
        dp: &mut Vec<Vec<Option<i64>>>,
    ) -> i64 {
        if l >= r {
            return 0;
        }
        if let Some(v) = dp[l][r] {
            return v;
        }

        let total = pref[r + 1].wrapping_sub(pref[l]);
        let mut best: Option<i64> = None;

        for k in l..r {
            let left = solve(l, k, pref, dp);
            let right = solve(k + 1, r, pref, dp);
            let cost = left.wrapping_add(right).wrapping_add(total);
            best = Some(match best {
                None => cost,
                Some(b) => if cost < b { cost } else { b },
            });
        }

        let ans = best.unwrap_or(0);
        dp[l][r] = Some(ans);
        ans
    }

    let ans = if n == 0 { 0 } else { solve(0, n - 1, &pref, &mut dp) };
    println!("{}", ans);
}
