// Define a struct to represent an item with its costs and value
struct Item {
    cost1: usize, // Cost in resource 1
    cost2: usize, // Cost in resource 2
    value: usize, // Value gained from the item
}

// Function to compute the maximum obtainable value
fn max_value(num_items: usize, resource_limit1: usize, resource_limit2: usize, items: Vec<Item>) -> usize {
    // Create a dynamic programming table initialized to zero
    let mut dp = vec![vec![0; resource_limit2 + 1]; resource_limit1 + 1];

    // Iterate through each item
    for item in items {
        // Traverse the DP table in reverse to avoid overwriting states
        for r1 in (item.cost1..=resource_limit1).rev() {
            for r2 in (item.cost2..=resource_limit2).rev() {
                // Update the DP table if including the current item is feasible
                dp[r1][r2] = dp[r1][r2].max(dp[r1 - item.cost1][r2 - item.cost2] + item.value);
            }
        }
    }

    // The maximum value obtainable is found at the bottom-right corner of the DP table
    dp[resource_limit1][resource_limit2]
}

// Main function to demonstrate usage
fn main() {
    // Example input
    let num_items = 3;
    let resource_limit1 = 5;
    let resource_limit2 = 5;
    let items = vec![
        Item { cost1: 2, cost2: 3, value: 4 },
        Item { cost1: 1, cost2: 2, value: 2 },
        Item { cost1: 3, cost2: 2, value: 5 },
    ];

    // Calculate the maximum value
    let result = max_value(num_items, resource_limit1, resource_limit2, items);
    
    // Output the result
    println!("{}", result);
}
