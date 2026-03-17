fn count_combinations(a: i32, b: i32, c: i32, x: i32) -> i32 {
    // Ensure inputs are non-negative as per the specification
    if a < 0 || b < 0 || c < 0 || x < 0 {
        return 0; // Invalid input handling is not specified, returning 0 for safety
    }

    let mut count = 0;

    // Iterate over the number of 500-value coins
    for i in 0..=a {
        // Iterate over the number of 100-value coins
        for j in 0..=b {
            // Iterate over the number of 50-value coins
            for k in 0..=c {
                // Calculate the total value of the current combination
                let total = i * 500 + j * 100 + k * 50;
                // Check if the total matches the target amount
                if total == x {
                    count += 1; // Increment the count of valid combinations
                }
            }
        }
    }

    count // Return the total count of combinations
}

fn main() {
    // Example usage: replace with actual input handling as needed
    let a = 2; // Maximum number of 500-value coins
    let b = 3; // Maximum number of 100-value coins
    let c = 4; // Maximum number of 50-value coins
    let x = 800; // Target amount

    let result = count_combinations(a, b, c, x);
    println!("{}", result); // Output the result
}
