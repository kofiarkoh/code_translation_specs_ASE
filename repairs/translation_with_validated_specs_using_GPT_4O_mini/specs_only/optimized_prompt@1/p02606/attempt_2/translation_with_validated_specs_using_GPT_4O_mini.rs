fn count_divisible_in_range(l: i32, r: i32, d: i32) -> i32 {
    // Preconditions: l <= r and d > 0 are assumed to be satisfied.
    
    // Calculate the count of integers divisible by d in the range [l, r]
    let count = (r / d) - ((l - 1) / d);
    
    // The result is guaranteed to be non-negative due to the preconditions
    count
}

fn main() {
    // Example usage of the function
    let l = 1; // Lower bound
    let r = 10; // Upper bound
    let d = 2; // Divisor

    let result = count_divisible_in_range(l, r, d);
    println!("Count of integers divisible by {} in range [{}..{}]: {}", d, l, r, result);
}
