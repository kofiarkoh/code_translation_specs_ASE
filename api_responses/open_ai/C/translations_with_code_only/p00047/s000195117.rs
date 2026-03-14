use std::io::{self, BufRead};

fn main() {
    let mut ball = [1, 0, 0]; // Initialize the ball array
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut chars = line.chars();
        
        // Read characters a and b, expecting a format like "A,B"
        if let (Some(a), Some(b)) = (chars.next(), chars.next()) {
            // Skip the comma
            chars.next(); 

            let x = match a {
                'A' => 0,
                'B' => 1,
                'C' => 2,
                _ => continue, // Ignore invalid input
            };

            let y = match b {
                'A' => 0,
                'B' => 1,
                'C' => 2,
                _ => continue, // Ignore invalid input
            };

            // Swap the balls
            ball.swap(x, y);
        }
    }

    // Output the result based on the final state of the ball array
    if ball[0] == 1 {
        println!("A");
    } else if ball[1] == 1 {
        println!("B");
    } else {
        println!("C");
    }
}
