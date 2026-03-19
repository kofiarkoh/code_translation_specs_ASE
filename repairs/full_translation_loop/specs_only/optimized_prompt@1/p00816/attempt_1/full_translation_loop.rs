use std::collections::HashSet;

fn main() {
    let target: i32 = 10; // Example target, replace with actual input
    let input_string: &str = "abcde"; // Example input string, replace with actual input

    // Check preconditions
    if target <= 0 || input_string.is_empty() {
        println!("error");
        return;
    }

    let mut max_sum = 0;
    let mut valid_combinations = HashSet::new();
    let n = input_string.len();

    // Generate all possible substrings and their sums
    for i in 0..n {
        for j in i + 1..=n {
            let substring = &input_string[i..j];
            let sum: i32 = substring.chars().map(|c| c as i32).sum();

            // Check if the sum is valid
            if sum <= target {
                valid_combinations.insert((sum, substring.to_string()));
                max_sum = max_sum.max(sum);
            }
        }
    }

    // Filter combinations that yield the maximum sum
    let max_combinations: Vec<_> = valid_combinations
        .iter()
        .filter(|&&(sum, _)| sum == max_sum)
        .collect();

    // Determine output based on the number of valid combinations
    match max_combinations.len() {
        0 => println!("error"),
        1 => {
            let (sum, combination) = max_combinations[0];
            println!("{} {}", sum, combination);
        }
        _ => println!("rejected"),
    }
}
