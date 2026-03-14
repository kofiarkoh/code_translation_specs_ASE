use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    
    // Read input from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input as an integer
    let mut i: i32 = input.trim().parse().expect("Failed to parse input");
    let mut j: i32 = 1;

    // Calculate the factorial using a loop
    while i > 0 {
        j *= i;
        i -= 1;
    }

    // Print the result
    println!("{}", j);
}
