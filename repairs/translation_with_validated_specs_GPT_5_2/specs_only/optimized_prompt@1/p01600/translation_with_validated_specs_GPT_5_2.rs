use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return, // unspecified behavior; choose to do nothing if no input
    };

    let mut x: Vec<i32> = Vec::with_capacity(n);
    let mut y: Vec<i32> = Vec::with_capacity(n);

    for _ in 0..n {
        let xi: i32 = it.next().unwrap().parse().unwrap();
        let yi: i32 = it.next().unwrap().parse().unwrap();
        x.push(xi);
        y.push(yi);
    }

    if n <= 1 {
        println!("0");
        return;
    }

    // dp[i][k] = minimum cost to build a tree over contiguous interval [i..k]
    // Memory proportional to n^2, time O(n^3).
    let mut dp = vec![vec![0i32; n]; n];

    // Initialize dp[i][i] = 0 already.
    // Compute increasing interval lengths.
    for len in 2..=n {
        for i in 0..=n - len {
            let k = i + len - 1;
            let mut best: i32 = i32::MAX;

            for j in i..k {
                // Merge cost for combining [i..j] and [j+1..k]:
                // |x_i - x_{j+1}| + |y_j - y_k|
                let merge_cost = (x[i] - x[j + 1]).abs() + (y[j] - y[k]).abs();

                // Overflow behavior is unspecified in the spec; use i32 arithmetic.
                let cand = dp[i][j].wrapping_add(dp[j + 1][k]).wrapping_add(merge_cost);

                if cand < best {
                    best = cand;
                }
            }

            dp[i][k] = best;
        }
    }

    println!("{}", dp[0][n - 1]);
}
