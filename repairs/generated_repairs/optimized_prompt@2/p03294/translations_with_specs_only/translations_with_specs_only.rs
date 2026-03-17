fn main() {
    // Read the first input which indicates the number of subsequent integers
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the first input as a positive integer
    let count: usize = input.trim().parse().expect("Input must be a valid positive integer");

    // Initialize a variable to hold the sum
    let mut total_sum = 0;

    // Read the subsequent integers
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let numbers: Vec<i32> = input.trim().split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    // Ensure we have the correct number of integers
    if numbers.len() != count {
        eprintln!("Input must contain exactly {} integers", count);
        return;
    }

    for value in numbers {
        // Each value is decremented by one before adding to the total sum
        total_sum += value - 1;
    }

    // Output the total sum
    println!("{}", total_sum);
}
