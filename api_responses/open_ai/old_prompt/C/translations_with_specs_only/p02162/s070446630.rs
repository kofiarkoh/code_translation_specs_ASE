use std::io::{self, BufRead};

fn main() -> i32 {
    // Read input from standard input
    let stdin = io::stdin();
    let input = stdin.lock().lines().next();

    // Check if we received input
    let input = match input {
        Some(Ok(line)) => line,
        _ => return 1, // Return 1 if input reading fails
    };

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

    // Ensure R1 and R2 are non-negative as per the specification
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
        // Normal case: compare total scores
        let alice_total = t1 + r1;
        let bob_total = t2 + r2;

        if alice_total > bob_total {
            println!("Alice");
        } else if bob_total > alice_total {
            println!("Bob");
        } else {
            println!("Draw");
        }
    }

    0 // Return 0 on successful execution
}
