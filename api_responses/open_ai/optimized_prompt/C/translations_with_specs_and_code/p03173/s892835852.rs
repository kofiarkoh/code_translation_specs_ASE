use std::io::{self, BufRead};

const SUP: i64 = 1 << 60;

fn recursion(l: usize, r: usize, dp: &mut Vec<Vec<i64>>, sum: &[i64]) -> i64 {
    if dp[l][r] != 0 || l == r {
        return dp[l][r];
    } else {
        dp[l][r] = SUP;
    }

    for k in l + 1..=r {
        let tmp = recursion(l, k - 1, dp, sum) + recursion(k, r, dp, sum);
        if tmp < dp[l][r] {
            dp[l][r] = tmp;
        }
    }
    dp[l][r] += sum[r] - sum[l - 1];
    dp[l][r]
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of elements
    let n: usize = lines.next().unwrap().unwrap().parse().expect("Invalid input");
    assert!(n > 0, "The number of elements must be a positive integer.");

    let mut a = vec![0; n + 1];
    let mut sum = vec![0; n + 1];
    let mut dp = vec![vec![0; n + 1]; n + 1];

    // Read the sequence of integers
    for i in 1..=n {
        a[i] = lines.next().unwrap().unwrap().parse().expect("Invalid input");
    }

    // Compute the cumulative sum
    for i in 1..=n {
        sum[i] = sum[i - 1] + a[i];
    }

    // Calculate the minimum cost of merging
    let result = recursion(1, n, &mut dp, &sum);
    println!("{}", result);
}
