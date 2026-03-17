fn main() {
    let target: i32 = 50; // Example target, replace with actual input
    let input_string: &str = "1234\n2222\n13579\n0"; // Example input string, replace with actual input

    // Check preconditions
    if target <= 0 || input_string.is_empty() {
        println!("error");
        return;
    }

    let mut max_sum = 0;
    let mut combinations = Vec::new();
    
    // Generate all possible non-empty substrings
    let lines: Vec<&str> = input_string.lines().collect();
    for line in lines {
        let num: i32 = line.parse().unwrap_or(0);
        if num == 0 { break; } // Stop processing on zero input

        let sum = num; // Use the number directly as the sum
        if sum <= target {
            combinations.push((sum, line.to_string()));
            if sum > max_sum {
                max_sum = sum;
            }
        }
    }

    // Filter combinations that match the maximum sum
    let valid_combinations: Vec<_> = combinations
        .into_iter()
        .filter(|(sum, _)| *sum == max_sum)
        .collect();

    // Determine output based on valid combinations
    match valid_combinations.len() {
        0 => println!("error"), // No valid combinations found
        1 => {
            let (sum, chars) = &valid_combinations[0];
            println!("{} {}", sum, chars); // Output max sum and corresponding characters
        }
        _ => println!("rejected"), // Multiple combinations yield the same max sum
    }
}
