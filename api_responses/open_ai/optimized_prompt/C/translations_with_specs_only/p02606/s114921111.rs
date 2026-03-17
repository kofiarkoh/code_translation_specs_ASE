// This program calculates the count of integers divisible by a specified divisor
// within a given inclusive range defined by lower and upper bounds.

fn count_divisible(lower: i32, upper: i32, divisor: i32) -> i32 {
    // Preconditions: lower <= upper and divisor > 0
    // We assume these conditions are met as per the specification.
    
    // Initialize the count of divisible integers
    let mut count = 0;

    // Iterate through the range from lower to upper (inclusive)
    for number in lower..=upper {
        // Check if the current number is divisible by the divisor
        if number % divisor == 0 {
            count += 1; // Increment the count if divisible
        }
    }

    // Postcondition: The output integer is non-negative
    count // Return the count of divisible integers
}

fn main() {
    // Example usage of the count_divisible function
    let lower = 1; // Example lower bound
    let upper = 10; // Example upper bound
    let divisor = 2; // Example divisor

    // Call the function and print the result
    let result = count_divisible(lower, upper, divisor);
    println!("{}", result); // Output the count followed by a newline
}
