use std::io::{self, BufRead};

#[derive(Debug)]
struct Mirror {
    orientation: char,
    x: i32,
    y: i32,
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of mirrors
    let first_line = lines.next().unwrap().unwrap();
    let num_mirrors: i32 = first_line.trim().parse().unwrap();

    // Handle negative integer case
    if num_mirrors < 0 {
        return; // Terminate without processing
    }

    let mut mirrors: Vec<Mirror> = Vec::new();

    // Read mirror definitions
    for _ in 0..num_mirrors {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let orientation = parts[0].chars().next().unwrap();
        let x: i32 = parts[1].parse().unwrap();
        let y: i32 = parts[2].parse().unwrap();
        mirrors.push(Mirror { orientation, x, y });
    }

    // Initial position of the entity
    let mut pos_x = 0;
    let mut pos_y = 0;
    let mut direction_x = 1; // Moving right
    let mut direction_y = 0; // Not moving vertically

    // Simulate movement and reflections
    loop {
        // Move the entity
        pos_x += direction_x;
        pos_y += direction_y;

        // Check for reflections with mirrors
        let mut reflected = false;
        for mirror in &mirrors {
            if pos_x == mirror.x && pos_y == mirror.y {
                // Reflect based on mirror orientation
                if mirror.orientation == 'x' {
                    direction_x = -direction_x; // Reflect horizontally
                } else {
                    direction_y = -direction_y; // Reflect vertically
                }
                reflected = true;
                break; // Exit the loop after reflecting
            }
        }

        // Check for termination condition (e.g., out of bounds)
        if pos_x < 0 || pos_y < 0 {
            break; // Terminate if out of bounds
        }

        // If not reflected, check for bounds
        if !reflected && (pos_x > 1000 || pos_y > 1000) { // Assuming a boundary limit
            break; // Terminate if out of bounds
        }

        // Prevent infinite loop by adding a timeout condition
        if pos_x == 0 && pos_y == 0 && !reflected {
            break; // Break if back to the starting point without reflection
        }
    }

    // Output the final coordinates
    println!("{} {}", pos_x, pos_y);
}
