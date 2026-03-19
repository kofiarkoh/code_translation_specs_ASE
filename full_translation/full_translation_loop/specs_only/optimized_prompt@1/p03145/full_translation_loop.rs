use std::io::{self, Write};

fn main() {
    // Read three integers from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut inputs = input.split_whitespace();

    // Parse the first two integers
    let a: i32 = inputs.next().unwrap().parse().expect("Invalid input");
    let b: i32 = inputs.next().unwrap().parse().expect("Invalid input");
    // The third input is read but not used
    let _unused: i32 = inputs.next().unwrap().parse().expect("Invalid input");

    // Calculate half the product of the first two integers
    let product = a as i64 * b as i64; // Use i64 to prevent overflow
    let half_product = (product + 1) / 2; // Round to the nearest whole number

    // Output the result
    println!("{}", half_product);
}
