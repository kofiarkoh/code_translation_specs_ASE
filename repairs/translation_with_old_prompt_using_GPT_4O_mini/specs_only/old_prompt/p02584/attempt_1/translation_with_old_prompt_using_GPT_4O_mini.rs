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
    let decrements = k.min(abs_x / d as i64) as u64;

    // Apply the decrements to the absolute value
    abs_x -= decrements * d as i64;

    // If there are remaining operations, we can apply one more decrement
    // This is because if k is odd, we can effectively "flip" the sign of the remaining value
    if (k - decrements) % 2 == 1 {
        abs_x = abs_x.abs() - d as i64; // Apply one more decrement
    }

    // Ensure the result is non-negative
    let result = abs_x.abs();

    // Print the result
    println!("{}", result);
}
