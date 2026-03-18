use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    // Read input from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Split the input string into parts and parse them as integers
    let mut iter = input.split_whitespace();
    let a: i32 = iter.next().unwrap().parse().expect("Failed to parse a");
    let b: i32 = iter.next().unwrap().parse().expect("Failed to parse b");
    let c: i32 = iter.next().unwrap().parse().expect("Failed to parse c");

    // Calculate the answer as a floating-point number
    let ans = 0.5 * (a as f64 * b as f64);
    
    // Print the result formatted as a floating-point number with no decimal places
    // Using `println!` with formatting to match the C code output
    print!("{:.0}", ans);
    io::stdout().flush().expect("Failed to flush stdout");
}
