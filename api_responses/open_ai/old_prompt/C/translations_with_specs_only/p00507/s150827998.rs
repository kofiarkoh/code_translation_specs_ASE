fn main() {
    // Example input; in a real scenario, this would be read from standard input.
    let n = 5; // Number of integers
    let a = vec![3, 1, 4, 2, 5]; // Input array of integers

    // Ensure N is greater than 1 for valid combinations
    if n < 2 {
        panic!("N must be greater than 1");
    }

    // Use a HashSet to track unique combinations
    let mut combinations = std::collections::HashSet::new();

    // Generate combinations of two distinct integers
    for i in 0..n {
        for j in 0..n {
            if i != j {
                // Concatenate integers as strings and parse back to integer
                let combined = format!("{}{}", a[i], a[j]);
                if let Ok(value) = combined.parse::<i32>() {
                    combinations.insert(value);
                }
            }
        }
    }

    // Collect unique combinations and sort them
    let mut unique_combinations: Vec<i32> = combinations.into_iter().collect();
    unique_combinations.sort_unstable();

    // Ensure we have at least three unique combinations
    if unique_combinations.len() < 3 {
        panic!("Not enough unique combinations found");
    }

    // Output the third smallest unique combination
    println!("{}", unique_combinations[2]);
}
