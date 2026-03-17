fn compute_combinations(n: usize, sequence: &str) -> i32 {
    // Define the modulo constant as specified
    const MODULO: i32 = 10007;

    // Preconditions: Check if N is non-negative and the sequence length matches N
    assert!(n > 0, "N must be positive");
    assert_eq!(sequence.len(), n, "Sequence length must match N");

    // Initialize a schedule array to maintain counts of valid key combinations
    let mut schedule = vec![0; 3]; // For 'J', 'O', 'I'
    
    // Process each character in the sequence
    for ch in sequence.chars() {
        match ch {
            'J' => schedule[0] += 1, // Increment count for 'J'
            'O' => schedule[1] += 1, // Increment count for 'O'
            'I' => schedule[2] += 1, // Increment count for 'I'
            _ => panic!("Invalid character in sequence"), // Handle unexpected characters
        }
    }

    // Calculate the total combinations based on the counts in the schedule
    let total_combinations = (schedule[0] * 504 + schedule[1] * 6 + schedule[2]) % MODULO;

    // Postcondition: Ensure the output is non-negative and less than 10007
    assert!(total_combinations >= 0 && total_combinations < MODULO, "Output must be within valid range");

    total_combinations
}

fn main() {
    // Example usage
    let n = 5;
    let sequence = "JOIJO";
    let result = compute_combinations(n, sequence);
    println!("{}", result); // Output the result
}
