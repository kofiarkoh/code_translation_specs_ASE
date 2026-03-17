// This program calculates the maximum value among the sum, difference, and product of two integers.

fn max_of_operations(a: i32, b: i32) -> i32 {
    // Initialize max_value to a value less than any possible computed result.
    // The minimum value of i32 is -2^31, which is less than any possible sum, difference, or product of two i32 values.
    let mut max_value = i32::MIN;

    // Calculate the sum, difference, and product
    let sum = a + b;
    let difference = a - b;
    let product = a * b;

    // Update max_value with the maximum of the computed values
    max_value = max_value.max(sum);
    max_value = max_value.max(difference);
    max_value = max_value.max(product);

    // Return the maximum value among the sum, difference, and product
    max_value
}

// Example usage
fn main() {
    let a = 5;
    let b = 3;
    let result = max_of_operations(a, b);
    println!("The maximum value is: {}", result);
}
