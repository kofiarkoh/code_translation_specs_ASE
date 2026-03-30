fn count_divisible_in_range(l: i32, r: i32, d: i32) -> i32 {
    // Preconditions: l <= r and d > 0 are assumed to be satisfied.
    
    // Calculate the count of integers divisible by d in the range [l, r]
    let count = (r / d) - ((l - 1) / d);
    
    // The result is guaranteed to be non-negative due to the preconditions
    count
}

fn main() {
    // Read input values for l, r, and d
    let l = 10; // Lower bound from input
    let r = 50; // Upper bound from input
    let d = 5;  // Divisor from input

    let result = count_divisible_in_range(l, r, d);
    println!("{}", result); // Output the count directly
}
