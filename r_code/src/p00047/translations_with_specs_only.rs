use std::io::{self, BufRead};

fn main() {
    // Initial positions of the balls
    let mut positions = ['A', 'B', 'C'];
    
    // Read from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        // Split the line by commas and process each pair
        for pair in line.split(',') {
            let trimmed = pair.trim();
            if trimmed.len() == 2 {
                let (first, second) = (trimmed.chars().nth(0).unwrap(), trimmed.chars().nth(1).unwrap());
                // Perform the swap if both characters are valid positions
                if let (Some(pos1), Some(pos2)) = (positions.iter().position(|&p| p == first), positions.iter().position(|&p| p == second)) {
                    positions.swap(pos1, pos2);
                }
            }
        }
    }

    // Output the final position of the ball that started in position A
    let final_position = positions[0]; // The ball that started in position A is now at positions[0]
    println!("{}", final_position);
}
