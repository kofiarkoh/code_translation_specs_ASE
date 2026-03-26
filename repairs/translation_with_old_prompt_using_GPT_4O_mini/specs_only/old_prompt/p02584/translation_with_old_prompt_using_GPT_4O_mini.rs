use std::io;

fn main() {
    // Read input values from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut values = input.split_whitespace();

    // Parse the three input values
    let x: i64 = values.next().unwrap().parse().expect("Invalid input for x");
    let k: u64 = values.next().unwrap().parse().expect("Invalid input for k");
    let d: u64 = values.next().unwrap().parse().expect("Invalid input for d");

    // Calculate the absolute value of x
    let mut abs_x = x.abs();

    // Determine how many decrements can be applied
    // We need to convert abs_x to u64 for the division
    let decrements = (abs_x / d as i64).max(0).min(k as i64) as u64;

    // Apply the decrements to the absolute value
    abs_x -= decrements as i64 * d as i64;

    // If there are remaining operations, we can apply one more decrement
    // This is because if (k - decrements) is odd, we can effectively "flip" the sign of the remaining value
    if (k as i64 - decrements as i64) % 2 == 1 {
        abs_x -= d as i64; // Apply one more decrement
    }

    // Ensure the result is non-negative
    let result = abs_x.abs();

    // Print the result
    println!("{}", result);
}
