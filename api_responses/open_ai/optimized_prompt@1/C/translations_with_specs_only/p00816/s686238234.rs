fn main() {
    let target: i32 = 10; // Example target, replace with actual input
    let input_string: &str = "abcde"; // Example input string, replace with actual input

    // Check preconditions
    if target <= 0 || input_string.is_empty() || input_string.len() > 10 {
        println!("error");
        return;
    }

    let mut max_sum = 0;
    let mut combinations = Vec::new();
    
    // Generate all possible non-empty substrings
    let n = input_string.len();
    for i in 0..n {
        for j in i + 1..=n {
            let substring = &input_string[i..j];
            let sum: i32 = substring.chars().map(|c| c as i32).sum(); // Convert chars to their integer values
            
            // Check if the sum is less than or equal to the target
            if sum <= target {
                combinations.push((sum, substring.to_string()));
                if sum > max_sum {
                    max_sum = sum;
                }
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
