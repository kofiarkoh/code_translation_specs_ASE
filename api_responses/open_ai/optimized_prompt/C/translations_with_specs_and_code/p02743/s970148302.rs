use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    
    // Parse the three long double values (f64 in Rust)
    let values: Vec<&str> = input.split_whitespace().collect();
    let a: f64 = values[0].parse().unwrap();
    let b: f64 = values[1].parse().unwrap();
    let c: f64 = values[2].parse().unwrap();
    
    // Calculate d
    let d = c - a - b;
    
    // Check the condition and print the result
    if d * d > 4.0 * a * b && d > 0.0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
