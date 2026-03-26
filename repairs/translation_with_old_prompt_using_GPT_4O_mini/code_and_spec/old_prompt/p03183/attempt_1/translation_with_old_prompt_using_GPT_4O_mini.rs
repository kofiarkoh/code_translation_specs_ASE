use std::cmp::max;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Item {
    w: i32,
    s: i32,
    v: i32,
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of items
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut items = Vec::with_capacity(n);

    // Read each item's weight, size, and value
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        items.push(Item { w: parts[0], s: parts[1], v: parts[2] });
    }

    // Sort items based on the combined metric of size and weight
    items.sort_by_key(|item| item.s + item.w);

    const M: usize = 20000;
    let mut dp = vec![0; M + 1];

    // Dynamic programming to compute the maximum value
    for item in items {
        for j in (0..=M - item.s).rev() {
            let k = j + item.w as usize;
            dp[k] = max(dp[k], dp[j] + item.v);
        }
    }

    // Find the maximum value achievable
    let ans = dp.iter().cloned().max().unwrap_or(0);
    println!("{}", ans);
}
