use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read height and width
    let first_line = lines.next().unwrap().unwrap();
    let dimensions: Vec<usize> = first_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let height = dimensions[0];
    let width = dimensions[1];

    // Handle the case where both height and width are zero
    if height == 0 && width == 0 {
        return;
    }

    // Read the grid
    let mut grid: Vec<Vec<char>> = Vec::new();
    for _ in 0..height {
        let line = lines.next().unwrap().unwrap();
        grid.push(line.chars().collect());
    }

    // Data structure to track visits
    let mut visit_count = vec![vec![0; width]; height];
    let mut x = 0;
    let mut y = 0;

    // Directions mapping
    let directions: std::collections::HashMap<char, (isize, isize)> = [
        ('.', (0, 0)), // Stay in place
        ('U', (-1, 0)), // Up
        ('D', (1, 0)), // Down
        ('L', (0, -1)), // Left
        ('R', (0, 1)), // Right
    ].iter().cloned().collect();

    // Process the grid
    loop {
        // Check if current position is out of bounds
        if x < 0 || x >= height as isize || y < 0 || y >= width as isize {
            break;
        }

        // Get the current tile
        let tile = grid[x as usize][y as usize];

        // Increment visit count
        visit_count[x as usize][y as usize] += 1;

        // Print coordinates if the tile is a dot
        if tile == '.' {
            println!("({}, {})", x, y);
        }

        // Check for loop
        if visit_count[x as usize][y as usize] > 1 {
            println!("LOOP");
            break;
        }

        // Move in the direction specified by the tile
        if let Some(&(dx, dy)) = directions.get(&tile) {
            x += dx;
            y += dy;
        } else {
            // If the tile is not a valid direction, we stop processing
            break;
        }
    }
}
