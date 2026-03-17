fn main() {
    // Read the total number defining the circular range and the number of queries
    let n: i32 = read_input(); // Assume read_input() reads an integer from standard input
    let k: i32 = read_input(); // Assume read_input() reads an integer from standard input

    // Ensure n is positive and k is non-negative as per the specification
    if n <= 0 || k < 0 {
        return; // Undefined behavior for invalid input; exit early
    }

    for _ in 0..k {
        let ai: i32 = read_input(); // Read first integer for the query
        let bi: i32 = read_input(); // Read second integer for the query

        // Adjust ai and bi if they exceed half of n
        let adjusted_ai = if ai > n / 2 { ai - n } else { ai };
        let adjusted_bi = if bi > n / 2 { bi - n } else { bi };

        // Compute the value based on adjusted positions
        let result = adjusted_ai + adjusted_bi;

        // Output the computed value for the query
        println!("{}", result);
    }
}

// Placeholder function to simulate reading input
fn read_input() -> i32 {
    // Implementation for reading input goes here
    // This is a stub and should be replaced with actual input handling
    0 // Placeholder return value
}
