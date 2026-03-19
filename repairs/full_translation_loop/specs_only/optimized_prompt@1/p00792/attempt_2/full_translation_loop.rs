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
    
    // Read the number of mirrors
    let mut lines = reader.lines();
    let first_line = lines.next().unwrap().unwrap();
    let num_mirrors: i32 = first_line.trim().parse().unwrap();

    // Handle negative input case
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

    // Initial position of the entity (assuming starting at (0, 0))
    let mut pos_x = 0;
    let mut pos_y = 0;
    let mut direction_x = 1; // Moving right
    let mut direction_y = 1; // Moving down

    // Simulate movement and reflections
    loop {
        // Move the entity
        pos_x += direction_x;
        pos_y += direction_y;

        // Check for bounds (assuming some arbitrary bounds, e.g., 250)
        if pos_x < 0 || pos_x > 250 || pos_y < 0 || pos_y > 250 {
            break; // Exit if out of bounds
        }

        // Check for mirror interactions
        let mut hit_mirror = false;
        for mirror in &mirrors {
            if (mirror.orientation == 'x' && pos_y == mirror.y && pos_x == mirror.x) ||
               (mirror.orientation != 'x' && pos_x == mirror.x && pos_y == mirror.y) {
                // Reflect the direction based on the mirror orientation
                if mirror.orientation == 'x' {
                    direction_y *= -1; // Reflect vertically
                } else {
                    direction_x *= -1; // Reflect horizontally
                }
                hit_mirror = true;
                break; // Exit the loop after hitting a mirror
            }
        }

        // If no mirror was hit, we can continue moving
        if !hit_mirror {
            continue;
        }
    }

    // Output the final coordinates
    println!("{} {}", pos_x, pos_y);
}
