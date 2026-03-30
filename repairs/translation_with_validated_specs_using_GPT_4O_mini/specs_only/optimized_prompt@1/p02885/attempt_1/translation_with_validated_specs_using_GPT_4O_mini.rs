use std::io;

fn main() {
    // Read two integers from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut numbers = input.split_whitespace();

    // Parse the first integer
    let a: i32 = numbers.next().unwrap().parse().expect("Invalid input");
    // Parse the second integer
    let b: i32 = numbers.next().unwrap().parse().expect("Invalid input");

    // Compute the result of the operation a - 2b
    let result = a - 2 * b;

    // Output the result, ensuring it is non-negative
    let output = if result > 0 { result } else { 0 };
    println!("{}", output);
}
