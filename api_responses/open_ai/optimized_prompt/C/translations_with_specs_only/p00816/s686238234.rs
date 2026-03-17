fn main() {
    // Read target sum and input string
    let target: i32 = 15; // Example target, replace with actual input
    let input_string: &str = "abcde"; // Example input, replace with actual input

    // Preconditions check
    if target <= 0 {
        println!("error");
        return;
    }
    if input_string.is_empty() || input_string.len() > 10 {
        println!("error");
        return;
    }

    let mut max_sum = 0;
    let mut combinations = Vec::new();

    // Generate all possible substrings and calculate their sums
    let n = input_string.len();
    for i in 0..n {
        for j in i + 1..=n {
            let substring = &input_string[i..j];
            let sum: i32 = substring.chars().map(|c| c as i32).sum();

            // Check if the sum is valid
            if sum <= target {
                if sum > max_sum {
                    max_sum = sum;
                    combinations.clear();
                    combinations.push(substring.to_string());
                } else if sum == max_sum {
                    combinations.push(substring.to_string());
                }
            }
        }
    }

    // Determine output based on combinations found
    match combinations.len() {
        0 => println!("error"), // No valid combinations found
        1 => println!("{} {}", max_sum, combinations[0]), // Exactly one valid combination
        _ => println!("rejected"), // Multiple combinations yield the same max sum
    }
}
