use std::io::{self, BufRead};

#[derive(Debug)]
struct Item {
    weight: i32,
    size: i32,
    value: i32,
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of items
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut items: Vec<Item> = Vec::with_capacity(n);

    // Read each item's weight, size, and value
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        items.push(Item {
            weight: parts[0],
            size: parts[1],
            value: parts[2],
        });
    }

    // Sort items based on the sum of size and weight
    items.sort_by_key(|item| item.size + item.weight);

    const MAX_SIZE: usize = 20000;
    let mut dp = vec![0; MAX_SIZE + 1];

    // Dynamic programming to compute the maximum value
    for item in items {
        for j in (0..=MAX_SIZE - item.size).rev() {
            let k = j + item.weight as usize;
            dp[k] = dp[k].max(dp[j] + item.value);
        }
    }

    // Find the maximum value obtainable
    let max_value = dp.iter().copied().max().unwrap_or(0);
    println!("{}", max_value);
}
