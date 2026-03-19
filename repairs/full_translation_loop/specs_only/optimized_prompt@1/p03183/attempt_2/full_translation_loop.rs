use std::cmp;

struct Item {
    weight: usize,
    size: usize,
    value: usize,
}

fn max_value(items: Vec<Item>, max_size: usize) -> usize {
    let n = items.len();
    // Create a DP table to store the maximum value for each size limit
    let mut dp = vec![0; max_size + 1];

    // Iterate over each item
    for item in items {
        // Traverse the dp array backwards to avoid overwriting values we still need to use
        for size in (item.size..=max_size).rev() {
            // Update the dp table if including the current item yields a better value
            dp[size] = cmp::max(dp[size], dp[size - item.size] + item.value);
        }
    }

    // The maximum obtainable value is found at dp[max_size]
    dp[max_size]
}

fn main() {
    // Read input from standard input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let num_items: usize = input.trim().parse().unwrap();

    let mut items = Vec::new();
    for _ in 0..num_items {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let parts: Vec<usize> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        items.push(Item {
            weight: parts[0],
            size: parts[1],
            value: parts[2],
        });
    }

    // Define a maximum size limit (this should be set according to the problem requirements)
    let max_size = 50; // Example size limit, adjust as needed
    let result = max_value(items, max_size);
    println!("{}", result); // Output the maximum obtainable value
}
