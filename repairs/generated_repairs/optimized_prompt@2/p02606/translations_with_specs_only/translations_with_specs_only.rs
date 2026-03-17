fn count_divisible(lower: i32, upper: i32, divisor: i32) -> i32 {
    // Preconditions: lower <= upper and divisor > 0
    // If the divisor is zero, the behavior is undefined, so we will not handle that case.
    // We assume the caller adheres to the preconditions.

    // Calculate the count of integers divisible by the divisor in the range [lower, upper].
    // The formula for counting multiples of `divisor` in the range is:
    // count = floor(upper / divisor) - floor((lower - 1) / divisor)

    // Calculate the number of multiples of `divisor` up to `upper`
    let count_upper = upper / divisor;

    // Calculate the number of multiples of `divisor` up to `lower - 1`
    let count_lower = (lower - 1) / divisor;

    // The result is the difference between the two counts
    let count = count_upper - count_lower;

    // The output is guaranteed to be non-negative due to the preconditions
    count
}

fn main() {
    let lower = 10; // Example lower bound
    let upper = 50; // Example upper bound
    let divisor = 5; // Example divisor

    // Call the function and print the result
    let result = count_divisible(lower, upper, divisor);
    println!("{}", result); // Output the count followed by a newline
}
