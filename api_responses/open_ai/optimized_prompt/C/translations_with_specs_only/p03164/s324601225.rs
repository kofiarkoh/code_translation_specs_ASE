// Define the maximum number of items and maximum weight capacity
const MAX_ITEMS: usize = 100; // Arbitrary limit for the number of items
const MAX_WEIGHT: usize = 1000; // Arbitrary limit for the maximum weight

fn knapsack(num_items: usize, max_weight: usize, items: &[(usize, usize)]) -> Result<usize, &'static str> {
    // Validate input
    if num_items > MAX_ITEMS || max_weight > MAX_WEIGHT {
        return Err("Input exceeds predefined limits.");
    }
    if items.len() != num_items {
        return Err("Number of items does not match the provided item list.");
    }

    // Initialize a DP array to track maximum values for each weight
    let mut dp = vec![0; max_weight + 1];

    // Process each item
    for &(weight, value) in items {
        // Validate item weight and value
        if weight == 0 || value == 0 {
            continue; // Skip items with zero weight or value
        }
        // Update the DP array in reverse to avoid overwriting results from the same item
        for w in (weight..=max_weight).rev() {
            dp[w] = dp[w].max(dp[w - weight] + value);
        }
    }

    // Find the maximum value that can be achieved without exceeding the weight limit
    let max_value = *dp.iter().max().unwrap();

    // Check if a valid configuration exists
    if max_value == 0 {
        return Err("No valid configuration exists.");
    }

    Ok(max_value)
}

fn main() {
    // Example usage
    let num_items = 3;
    let max_weight = 50;
    let items = [(10, 60), (20, 100), (30, 120)];

    match knapsack(num_items, max_weight, &items) {
        Ok(max_value) => println!("Maximum value: {}", max_value),
        Err(err) => eprintln!("Error: {}", err),
    }
}
