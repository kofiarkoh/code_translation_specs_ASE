// Define a struct to represent a transaction with its resource cost and value.
struct Transaction {
    resource_cost: (i32, i32), // Tuple representing the cost in two resources
    value: i32,                // Value obtained from the transaction
}

// Function to compute the maximum value obtainable through transactions
fn max_value(transactions: Vec<Transaction>, resource_limits: (i32, i32)) -> i32 {
    // Initialize a 2D vector to store the maximum values for each resource combination
    let mut dp = vec![vec![0; (resource_limits.1 + 1) as usize]; (resource_limits.0 + 1) as usize];

    // Iterate over each transaction
    for transaction in transactions {
        let (cost1, cost2) = transaction.resource_cost;
        let value = transaction.value;

        // Update the dp table in reverse order to avoid overwriting results from the same transaction
        for r1 in (cost1..=resource_limits.0).rev() {
            for r2 in (cost2..=resource_limits.1).rev() {
                // Update the maximum value for the current resource limits
                dp[r1 as usize][r2 as usize] = dp[r1 as usize][r2 as usize].max(dp[(r1 - cost1) as usize][(r2 - cost2) as usize] + value);
            }
        }
    }

    // The maximum value obtainable is found at the full resource limits
    dp[resource_limits.0 as usize][resource_limits.1 as usize]
}

// Main function to demonstrate the usage of max_value
fn main() {
    // Example usage (this part can be modified or removed as needed)
    let transactions = vec![
        Transaction { resource_cost: (1, 2), value: 3 },
        Transaction { resource_cost: (2, 1), value: 4 },
        Transaction { resource_cost: (3, 2), value: 5 },
    ];
    let resource_limits = (5, 5);
    
    // Calculate the maximum value obtainable
    let result = max_value(transactions, resource_limits);
    
    // Print the result
    println!("Maximum value obtainable: {}", result);
}
