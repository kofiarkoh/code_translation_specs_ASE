use std::io;

fn main() {
    // Read two integers from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Split the input into two parts and parse them as integers
    let mut parts = input.split_whitespace();
    let a: i32 = parts.next().unwrap().parse().expect("Invalid input for a");
    let b: i32 = parts.next().unwrap().parse().expect("Invalid input for b");
    
    // Calculate the expression a - 2 * b
    let result = a - 2 * b;
    
    // Output the result, ensuring it is non-negative
    // If result is positive, print it; otherwise, print 0
    let output = if result > 0 { result } else { 0 };
    println!("{}", output);
}
