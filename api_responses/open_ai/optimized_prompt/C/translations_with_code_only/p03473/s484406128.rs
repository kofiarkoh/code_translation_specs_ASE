fn main() {
    use std::io::{self, Write};

    let mut input = String::new();
    
    // Read input from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input as an integer
    let m: i32 = input.trim().parse().expect("Failed to parse integer");
    
    // Calculate the result
    let result = 48 - m;
    
    // Print the result with a newline
    println!("{}", result);
}
