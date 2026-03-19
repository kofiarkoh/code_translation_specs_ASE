use std::cmp::min;
use std::io::{self, Read};

fn main() {
    // Read entire stdin as whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(t) => t.parse().unwrap(),
        None => return, // Behavior unspecified; choose to do nothing if no input.
    };

    let mut xs: Vec<i32> = Vec::with_capacity(n);
    let mut ys: Vec<i32> = Vec::with_capacity(n);

    for _ in 0..n {
        let x: i32 = it.next().unwrap().parse().unwrap();
        let y: i32 = it.next().unwrap().parse().unwrap();
        xs.push(x);
        ys.push(y);
    }

    // DP over contiguous subranges [i..=k].
    // dp[i][k] = minimum cost for subrange.
    // Base: dp[i][i] = 0.
    // Transition:
    // dp[i][k] = min over j in [i..k-1] of
    //   dp[i][j] + dp[j+1][k] +
    //   abs(x[i] - x[j+1]) + abs(y[j] - y[k])
    //
    // Use i32 throughout to match the specified 32-bit signed result range.
    // Overflow behavior is unspecified by the spec; we use normal i32 arithmetic.
    let mut dp: Vec<Vec<i32>> = vec![vec![0; n]; n];

    // Length from 2 to n
    for len in 2..=n {
        for i in 0..=n - len {
            let k = i + len - 1;
            let mut best: i32 = i32::MAX;

            for j in i..k {
                let merge_cost = (xs[i] - xs[j + 1]).abs() + (ys[j] - ys[k]).abs();
                let cand = dp[i][j] + dp[j + 1][k] + merge_cost;
                best = min(best, cand);
            }

            dp[i][k] = best;
        }
    }

    let ans = dp[0][n - 1];
    print!("{ans}");
}
