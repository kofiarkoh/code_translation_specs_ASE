fn main() {
    // Read input values for T, A, and B
    let t1: i64 = read_input(); // First time factor
    let t2: i64 = read_input(); // Second time factor
    let a1: i64 = read_input(); // First parameter of set A
    let a2: i64 = read_input(); // Second parameter of set A
    let b1: i64 = read_input(); // First parameter of set B
    let b2: i64 = read_input(); // Second parameter of set B

    // Check if sets A and B are equivalent
    if (a1 == b1 && a2 == b2) || (a1 == b2 && a2 == b1) {
        // If A and B are equivalent, output "infinity"
        println!("infinity");
    } else {
        // Calculate the output based on the differences
        let result = calculate_difference(a1, a2, b1, b2);
        println!("{}", result);
    }
}

// Function to read input values (stub for demonstration)
fn read_input() -> i64 {
    // In a real implementation, this would read from standard input
    // For now, we return a placeholder value
    0 // Placeholder value; replace with actual input reading logic
}

// Function to calculate the output based on differences between A and B
fn calculate_difference(a1: i64, a2: i64, b1: i64, b2: i64) -> i64 {
    // Example calculation: return the sum of differences
    // This is a placeholder; the actual calculation should be defined based on the specification
    (a1 - b1).abs() + (a2 - b2).abs() // Using absolute differences as a simple example
}
