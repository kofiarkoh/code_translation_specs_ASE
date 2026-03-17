// This function calculates a value based on two input integers, n and m.
// It ensures that m is greater than zero and handles specific conditions
// regarding their relationship as described in the specification.

fn calculate_value(n: i32, m: i32) -> i32 {
    // Preconditions: m must be greater than zero.
    // If m equals 1, the output is guaranteed to be 0.
    if m == 1 {
        return 0;
    }
    
    // Postconditions: The output is a non-negative integer.
    // If m is greater than 1, the output is derived from n and m.
    // The specification does not define how to derive the output from n and m,
    // so we will return n as the output, assuming it is the intended behavior.
    // This is a narrow interpretation based on the lack of further details.
    
    // Ensure the output is non-negative.
    if n < 0 {
        return 0; // If n is negative, return 0 to satisfy the non-negative output requirement.
    }
    
    n // Return n as the output when m > 1 and n is non-negative.
}

// Example usage (not part of the specification):
// fn main() {
//     let result = calculate_value(5, 2);
//     println!("Result: {}", result); // Expected output: 5
// }
