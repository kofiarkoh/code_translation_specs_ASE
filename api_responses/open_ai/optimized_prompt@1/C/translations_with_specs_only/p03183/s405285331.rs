// Define a struct to represent an item with weight, size, and value.
struct Item {
    weight: usize,
    size: usize,
    value: usize,
}

// Function to compute the maximum obtainable value from a set of items.
fn max_value(items: &[Item], max_size: usize) -> usize {
    // Create a vector to store the maximum value for each size limit.
    let mut dp = vec![0; max_size + 1];

    // Iterate over each item.
    for item in items {
        // Iterate backwards to avoid overwriting values we still need to use.
        for size in (item.size..=max_size).rev() {
            // Update the maximum value for the current size limit.
            dp[size] = dp[size].max(dp[size - item.size] + item.value);
        }
    }

    // The maximum obtainable value is found at dp[max_size].
    dp[max_size]
}

// Main function to demonstrate the usage of max_value.
fn main() {
    // Example input: number of items and their properties.
    let items = vec![
        Item { weight: 1, size: 2, value: 3 },
        Item { weight: 2, size: 3, value: 4 },
        Item { weight: 3, size: 4, value: 5 },
    ];
    let max_size = 5;

    // Compute the maximum value obtainable.
    let result = max_value(&items, max_size);

    // Output the result.
    println!("{}", result);
}
