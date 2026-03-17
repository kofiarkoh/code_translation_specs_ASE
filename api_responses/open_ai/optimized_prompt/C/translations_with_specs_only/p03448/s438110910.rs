fn main() {
    // Example input values
    let a = 2; // Maximum number of 500-unit coins
    let b = 3; // Maximum number of 100-unit coins
    let c = 4; // Maximum number of 50-unit coins
    let x = 750; // Target sum

    let combinations = count_combinations(a, b, c, x);
    println!("{}", combinations);
}

// Function to count the number of valid combinations of coins
fn count_combinations(a: usize, b: usize, c: usize, x: usize) -> usize {
    let mut count = 0; // Initialize count of combinations

    // Iterate through all possible counts of 500-unit coins
    for i in 0..=a {
        // Iterate through all possible counts of 100-unit coins
        for j in 0..=b {
            // Iterate through all possible counts of 50-unit coins
            for k in 0..=c {
                // Calculate the total value of the current combination
                let total = i * 500 + j * 100 + k * 50;

                // Check if the total matches the target value
                if total == x {
                    count += 1; // Increment count for valid combination
                }
            }
        }
    }

    count // Return the total count of valid combinations
}
