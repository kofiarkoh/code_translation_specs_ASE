use std::io;

fn main() {
    let (dividend, divisor) = read_input();
    let result = ceiling_division(dividend, divisor);
    println!("{}", result);
}

// Reads two integers from standard input.
fn read_input() -> (i32, i32) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let nums: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid integer"))
        .collect();
    (nums[0], nums[1])
}

// Computes the ceiling of the division of two integers.
fn ceiling_division(dividend: i32, divisor: i32) -> i32 {
    // The specification states that the divisor must not be zero.
    // We do not handle the zero case, which may lead to undefined behavior.
    // Therefore, we assume the caller guarantees that divisor != 0.
    
    // Perform the division and compute the ceiling.
    let quotient = dividend / divisor;
    let remainder = dividend % divisor;

    // If there is a remainder, we need to round up.
    if remainder != 0 {
        return quotient + 1;
    }
    quotient
}
