use std::io::{self, BufRead};

fn main() {
    // Initialize the ball positions: 1 in position A, 0 in B and C
    let mut ball = [1, 0, 0];

    // Read from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.split(',').collect();

        // Ensure we have exactly two parts for a valid swap
        if parts.len() != 2 {
            continue; // Skip malformed input
        }

        let a = parts[0].chars().next().unwrap();
        let b = parts[1].chars().next().unwrap();

        // Determine the indices for the positions
        let x = match a {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            _ => continue, // Skip invalid characters
        };

        let y = match b {
            'A' => 0,
            'B' => 1,
            'C' => 2,
            _ => continue, // Skip invalid characters
        };

        // Swap the balls at positions x and y
        ball.swap(x, y);
    }

    // Determine the final position of the ball that started in position A
    if ball[0] == 1 {
        println!("A");
    } else if ball[1] == 1 {
        println!("B");
    } else {
        println!("C");
    }
}
