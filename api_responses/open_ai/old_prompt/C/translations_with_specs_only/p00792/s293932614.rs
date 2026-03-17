// Mirror Illusion Simulation in Rust

use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of mirrors
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    // Initialize the mirror array
    // Using a fixed size of 800x800 for the grid as per the specification
    let mut mirrors = [[0; 800]; 800];

    // Read mirror configurations
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = line.split_whitespace().collect();
        let s: char = parts[0].chars().next().unwrap();
        let x: usize = parts[1].parse().unwrap();
        let y: usize = parts[2].parse().unwrap();
        
        // Store the mirror type in the array
        mirrors[x][y] = match s {
            'x' => 1, // Vertical mirror
            'y' => 2, // Horizontal mirror
            _ => unreachable!(), // Invalid input based on the specification
        };
    }

    // Initial position and direction
    let mut pos_x = 0;
    let mut pos_y = 0;
    let mut direction = 0; // 0: right, 1: down, 2: left, 3: up

    // Simulate movement until we hit the boundary
    loop {
        // Move in the current direction
        match direction {
            0 => pos_x += 1, // Move right
            1 => pos_y += 1, // Move down
            2 => pos_x -= 1, // Move left
            3 => pos_y -= 1, // Move up
            _ => unreachable!(), // Invalid direction
        }

        // Check for boundary conditions
        if pos_x >= 800 || pos_y >= 800 {
            break; // Exit if out of bounds
        }

        // Check for mirrors
        match mirrors[pos_x][pos_y] {
            1 => direction = (3 - direction) % 4, // Reflect off vertical mirror
            2 => direction = (1 - direction) % 4, // Reflect off horizontal mirror
            _ => {} // No mirror, continue in the same direction
        }
    }

    // Output the final position
    println!("{} {}", pos_x, pos_y);
}
