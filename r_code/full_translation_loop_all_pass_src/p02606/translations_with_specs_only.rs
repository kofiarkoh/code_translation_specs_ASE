fn count_divisible(lower: i32, upper: i32, divisor: i32) -> i32 {
    // Preconditions check: lower <= upper and divisor > 0
    // The specification states that the behavior is undefined for divisor = 0,
    // so we assume the caller ensures that divisor is always positive.

    // Calculate the number of multiples of `divisor` in the range [lower, upper].
    if divisor <= 0 {
        // Undefined behavior according to the specification; we return 0 as a safe fallback.
        return 0;
    }

    // Calculate the number of multiples of `divisor` up to `upper`
    let count_upper = upper / divisor;

    // Calculate the number of multiples of `divisor` up to `lower - 1`
    let count_lower = (lower - 1) / divisor;

    // The count of divisible integers in the range [lower, upper]
    count_upper - count_lower
}

fn main() {
    // Example input
    let lower = 10;
    let upper = 50;
    let divisor = 5;

    // Call the function and print the result
    let result = count_divisible(lower, upper, divisor);
    println!("{}", result); // Output the count followed by a newline
}
