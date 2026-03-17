use std::io;

fn main() {
    // Read two integers from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Split the input string into parts and parse them as integers
    let mut iter = input.split_whitespace();
    let a: i32 = iter.next().expect("Expected first integer").parse().expect("Invalid integer");
    let b: i32 = iter.next().expect("Expected second integer").parse().expect("Invalid integer");

    // Calculate sum, difference, and product
    let sum = a + b;
    let difference = a - b;
    let product = a * b;

    // Determine the maximum value among sum, difference, and product
    let max = sum.max(difference).max(product);

    // Print the maximum value
    println!("{}", max);
}
