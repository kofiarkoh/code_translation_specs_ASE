fn cth_common_divisor(a: i32, b: i32, c: i32) -> i32 {
    // Preconditions: a and b must be positive, c must be a positive integer.
    if a <= 0 || b <= 0 || c <= 0 {
        panic!("Inputs must be positive integers.");
    }

    let mut sum = 0; // Counts the number of common divisors found
    let mut l = 0;   // Holds the value of the last found common divisor

    // Iterate from the smaller of a and b down to 1
    let limit = std::cmp::min(a, b);
    for i in (1..=limit).rev() {
        if a % i == 0 && b % i == 0 {
            sum += 1; // Found a common divisor
            l = i;    // Update the last found common divisor
        }
        // If we have found c common divisors, we can stop early
        if sum == c {
            return l; // Return the c-th common divisor
        }
    }

    // If we exit the loop without finding the c-th common divisor,
    // we return an uninitialized value (undefined behavior in Rust).
    // In Rust, we can return 0 to indicate that it does not exist,
    // but this is not strictly "undefined" as per the specification.
    // However, we will return 0 to indicate the absence of a valid c-th common divisor.
    0
}
