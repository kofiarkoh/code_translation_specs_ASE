use std::io::{self, Write};

fn main() {
    // Read target value and number of items
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut inputs = input.split_whitespace();
    
    let target: usize = inputs.next().unwrap().parse().expect("Invalid target value");
    let num_items: usize = inputs.next().unwrap().parse().expect("Invalid number of items");

    // Preconditions check
    if target < 0 || num_items < 0 || num_items > 100 {
        eprintln!("Input values must be non-negative and number of items must not exceed 100.");
        std::process::exit(1);
    }

    let mut items = Vec::new();

    // Read item values, energy costs, and recovery values
    for _ in 0..num_items {
        let mut item_input = String::new();
        io::stdin().read_line(&mut item_input).expect("Failed to read line");
        let mut item_values = item_input.split_whitespace();
        
        let value: usize = item_values.next().unwrap().parse().expect("Invalid item value");
        let energy_cost: usize = item_values.next().unwrap().parse().expect("Invalid energy cost");
        let recovery_value: usize = item_values.next().unwrap().parse().expect("Invalid recovery value");

        // Each item must have non-negative values
        if value < 0 || energy_cost < 0 || recovery_value < 0 {
            eprintln!("Item attributes must be non-negative.");
            std::process::exit(1);
        }

        items.push((value, energy_cost, recovery_value));
    }

    // Call the function to compute the minimum number of items
    match compute_min_items(target, &items) {
        Some(min_items) => println!("{}", min_items),
        None => println!("Target value cannot be reached."),
    }
}

// Function to compute the minimum number of items required to reach the target value
fn compute_min_items(target: usize, items: &[(usize, usize, usize)]) -> Option<usize> {
    let mut dp = vec![None; target + 1]; // Memoization structure
    dp[0] = Some(0); // Base case: 0 items needed to reach target 0

    for i in 1..=target {
        for &(value, energy_cost, recovery_value) in items {
            if i >= value {
                if let Some(prev) = dp[i - value] {
                    let new_count = prev + 1;
                    dp[i] = match dp[i] {
                        Some(existing) => Some(existing.min(new_count)),
                        None => Some(new_count),
                    };
                }
            }
        }
    }

    dp[target] // Return the minimum number of items or None if not achievable
}
