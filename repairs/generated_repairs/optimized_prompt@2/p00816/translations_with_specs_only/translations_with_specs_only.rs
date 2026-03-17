fn main() {
    let target: i32 = 50; // Example target, replace with actual input
    let input_string: &str = "1234\n2222\n13579\n0"; // Example input string, replace with actual input

    // Check preconditions
    if target <= 0 || input_string.is_empty() {
        println!("error");
        return;
    }

    let mut combinations = Vec::new();
    
    // Generate all possible non-empty substrings
    let numbers: Vec<i32> = input_string.lines()
        .filter_map(|line| line.parse::<i32>().ok())
        .collect();

    for &num in &numbers {
        if num <= target {
            combinations.push(num);
        }
    }

    // Determine output based on combinations
    if combinations.is_empty() {
        println!("error"); // No valid combinations found
    } else {
        let max_sum = *combinations.iter().max().unwrap();
        let valid_combinations: Vec<_> = combinations
            .iter()
            .filter(|&&sum| sum == max_sum)
            .collect();

        match valid_combinations.len() {
            1 => {
                let sum = valid_combinations[0];
                println!("{}", sum); // Output max sum
            }
            _ => {
                print!("rejected"); // Multiple combinations yield the same max sum
                for &sum in valid_combinations {
                    print!(" {}", sum);
                }
                println!();
            }
        }
    }
}
