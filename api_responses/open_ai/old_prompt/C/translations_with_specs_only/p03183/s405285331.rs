use std::io::{self, BufRead};

#[derive(Debug)]
struct Item {
    weight: i32,
    size: i32,
    value: i32,
}

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of items
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    // Edge case: if n is zero, print 0 and return
    if n == 0 {
        println!("0");
        return;
    }

    // Read items
    let mut items: Vec<Item> = Vec::with_capacity(n);
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

    // Sort items based on a combined metric of size and weight
    items.sort_by(|a, b| {
        let metric_a = a.size as f64 / a.weight as f64;
        let metric_b = b.size as f64 / b.weight as f64;
        metric_a.partial_cmp(&metric_b).unwrap()
    });

    // Define the maximum size limit
    const MAX_SIZE: usize = 20000;
    let mut dp = vec![0; MAX_SIZE + 1];

    // Dynamic programming to compute the maximum value
    for item in items {
        for j in (item.size as usize..=MAX_SIZE).rev() {
            dp[j] = dp[j].max(dp[j - item.size as usize] + item.value);
        }
    }

    // The maximum value is the maximum in the dp array
    let max_value = *dp.iter().max().unwrap();
    println!("{}", max_value);
}
