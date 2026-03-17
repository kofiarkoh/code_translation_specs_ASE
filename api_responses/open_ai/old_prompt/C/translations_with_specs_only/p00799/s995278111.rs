fn main() {
    // Read input values for n, lim, u, and c
    let (n, lim, u, c) = read_input();

    // Check preconditions
    if n <= 0 || u <= 0 || c <= 0 {
        return; // Exit if any input is invalid
    }

    // Initialize the count of usages without recharging
    let mut usage_count = 0;

    // Simulate the battery usage over the time limit
    let mut time_passed = 0;

    while time_passed < lim {
        // Each battery can be used for 'u' time units
        usage_count += 1; // Count the usage
        time_passed += u; // Increment time by usage time

        // After usage, the battery needs to recharge for 'c' time units
        time_passed += c; // Increment time by recharge time
    }

    // Output the result
    println!("{}", usage_count);
}

// Function to read input values
fn read_input() -> (i32, i32, i32, i32) {
    // Placeholder for reading input; in a real implementation, this would read from stdin
    // For the purpose of this translation, we will return hardcoded values
    (3, 100, 10, 5) // Example values for n, lim, u, c
}
