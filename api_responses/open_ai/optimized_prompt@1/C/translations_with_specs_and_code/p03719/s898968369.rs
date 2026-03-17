use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    
    // Read input from standard input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the three integers from the input
    let mut iter = input.split_whitespace();
    let a: i32 = iter.next().unwrap().parse().expect("Invalid input");
    let b: i32 = iter.next().unwrap().parse().expect("Invalid input");
    let c: i32 = iter.next().unwrap().parse().expect("Invalid input");

    // Check if c is within the inclusive range [a, b]
    if a <= c && c <= b {
        println!("Yes");
    } else {
        println!("No");
    }
}
