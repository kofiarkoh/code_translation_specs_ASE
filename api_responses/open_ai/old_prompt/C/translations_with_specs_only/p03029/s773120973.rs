use std::io::{self, Write};

fn main() {
    // Read two integers from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Split the input string into parts and parse them as integers
    let mut parts = input.split_whitespace();
    let a: i32 = parts.next().expect("Expected first integer").parse().expect("Invalid integer");
    let p: i32 = parts.next().expect("Expected second integer").parse().expect("Invalid integer");

    // Calculate the result based on the specified formula
    // ((a * 3) + p) / 2
    let result = (a.wrapping_mul(3)).wrapping_add(p).wrapping_div(2);

    // Output the result followed by a newline
    println!("{}", result);
}
