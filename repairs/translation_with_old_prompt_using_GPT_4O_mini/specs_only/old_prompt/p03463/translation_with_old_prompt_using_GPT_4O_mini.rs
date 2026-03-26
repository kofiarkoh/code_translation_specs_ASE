use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    
    // Parse the three integers from the input
    let numbers: Vec<i32> = input.split_whitespace()
        .map(|s| s.parse().unwrap()) // Assuming valid input as per specification
        .collect();
    
    // Unpack the integers
    let _n = numbers[0]; // N is not used in decision-making
    let a = numbers[1]; // A's score
    let b = numbers[2]; // B's score
    
    // Determine the winner based on the parity of the difference (B - A)
    if (b - a) % 2 == 0 {
        println!("Alice");
    } else {
        println!("Borys");
    }
    
    // Return 0 upon successful execution
    std::process::exit(0);
}
