use std::io::{self, Write};

fn main() {
    // Read three long double values from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Split the input into parts and parse them as f64 (Rust's closest to long double)
    let mut values = input.split_whitespace()
        .map(|s| s.parse::<f64>().expect("Failed to parse input as f64"));
    
    let a = values.next().unwrap();
    let b = values.next().unwrap();
    let c = values.next().unwrap();
    
    // Calculate the difference
    let d = c - a - b;
    
    // Check the condition and print the result
    if d * d > 4.0 * a * b && d > 0.0 {
        println!("Yes");
    } else {
        println!("No");
    }
    
    // Return 0 upon successful execution
    std::process::exit(0);
}
