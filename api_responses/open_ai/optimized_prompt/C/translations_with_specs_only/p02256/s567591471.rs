fn gcd(a: u64, b: u64) -> u64 {
    // Calculate the GCD using the Euclidean algorithm
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    // Define the range limits
    const MIN_VALUE: u64 = 1;
    const MAX_VALUE: u64 = 1_000_000_000;

    // Simulate user input (for example purposes, replace with actual input handling)
    let input_a: u64 = 100; // Example input, replace with actual user input
    let input_b: u64 = 200; // Example input, replace with actual user input

    // Check if inputs are within the specified range
    if input_a < MIN_VALUE || input_a > MAX_VALUE || input_b < MIN_VALUE || input_b > MAX_VALUE {
        // Inputs are invalid, return failure status
        println!("Failure: Inputs are out of range.");
        return;
    }

    // Calculate and print the GCD
    let result = gcd(input_a, input_b);
    println!("GCD: {}", result);
}
