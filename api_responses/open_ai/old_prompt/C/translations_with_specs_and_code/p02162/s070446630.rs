use std::io::{self, Write};

fn main() -> i32 {
    let mut input = String::new();
    
    // Read input from stdin
    if io::stdin().read_line(&mut input).is_err() {
        return 1; // Return 1 if input reading fails
    }

    // Split the input into parts and parse them as integers
    let scores: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    // Check if we have exactly four scores
    if scores.len() != 4 {
        return 1; // Return 1 if fewer than four integers are provided
    }

    let (t1, t2, r1, r2) = (scores[0], scores[1], scores[2], scores[3]);

    // Determine the winner based on the rules specified
    if r1 < 0 || r2 < 0 {
        // If R1 or R2 is negative, compare T1 and T2
        if t1 < t2 {
            println!("Alice");
        } else if t1 > t2 {
            println!("Bob");
        } else {
            println!("Draw");
        }
    } else {
        // Compare R1 and R2
        if r1 > r2 {
            println!("Alice");
        } else if r1 < r2 {
            println!("Bob");
        } else {
            println!("Draw");
        }
    }

    0 // Return 0 on successful execution
}
