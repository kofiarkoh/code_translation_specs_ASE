use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next();

    // Check if we successfully read the input
    let input = match input {
        Some(Ok(line)) => line,
        _ => {
            println!("Error reading input");
            return; // Exit if input reading fails
        }
    };

    // Split the input into parts and parse them as integers
    let scores: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    // Check if we have exactly four scores
    if scores.len() != 4 {
        println!("Error: Expected exactly four integers");
        return; // Exit if fewer than four integers are provided
    }

    let (t1, t2, r1, r2) = (scores[0], scores[1], scores[2], scores[3]);

    // Check the conditions for R1 and R2
    if r1 < 0 || r2 < 0 {
        // If R1 or R2 is negative, we only compare T1 and T2
        if t1 > t2 {
            println!("Alice");
        } else if t2 > t1 {
            println!("Bob");
        } else {
            println!("Draw");
        }
    } else {
        // Normal case: both R1 and R2 are non-negative
        if r1 > r2 {
            println!("Alice");
        } else if r2 > r1 {
            println!("Bob");
        } else {
            // If R1 == R2, compare T1 and T2
            if t1 > t2 {
                println!("Alice");
            } else if t2 > t1 {
                println!("Bob");
            } else {
                println!("Draw");
            }
        }
    }
}
