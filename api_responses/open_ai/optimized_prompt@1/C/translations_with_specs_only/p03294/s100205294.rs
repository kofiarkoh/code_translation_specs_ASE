fn main() {
    // Read the first input which indicates the number of subsequent integers
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the first input as a positive integer
    let count: usize = input.trim().parse().expect("Input must be a positive integer");

    // Initialize a variable to hold the sum
    let mut total_sum = 0;

    // Read the subsequent integers
    for _ in 0..count {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        
        // Parse each subsequent integer
        let value: i32 = input.trim().parse().expect("Input must be a valid integer");
        
        // Each value is decremented by one before adding to the total sum
        total_sum += value - 1;
    }

    // Output the total sum
    println!("{}", total_sum);
}
