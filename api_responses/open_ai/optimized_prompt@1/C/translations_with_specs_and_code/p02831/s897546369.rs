use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let mut inputs = input.split_whitespace();
    let a: i64 = inputs.next().expect("Expected first input").parse().expect("Invalid input for a");
    let b: i64 = inputs.next().expect("Expected second input").parse().expect("Invalid input for b");

    // Preconditions check
    if a < 0 {
        eprintln!("Error: The first input must be a non-negative integer.");
        return;
    }
    if b <= 0 {
        eprintln!("Error: The second input must be a positive integer.");
        return;
    }

    // Find the smallest multiple of `a` that is also a multiple of `b`
    for i in 1..=b + 1 {
        let multiple = a * i;
        if multiple % b == 0 {
            println!("{}", multiple);
            break;
        }
    }
}
