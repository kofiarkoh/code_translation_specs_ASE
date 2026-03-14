use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let test_cases: usize = input.trim().parse().expect("Invalid input");
    
    for _ in 0..test_cases {
        let mut size_input = String::new();
        io::stdin().read_line(&mut size_input).expect("Failed to read line");
        
        let size: usize = size_input.trim().parse().expect("Invalid input");
        
        let spiral = generate_spiral(size);
        print_spiral(&spiral);
        
        // Print a blank line between test cases, but not after the last one
        if _ < test_cases - 1 {
            println!();
        }
    }
}

fn generate_spiral(size: usize) -> Vec<Vec<char>> {
    let mut grid = vec![vec![' '; size]; size];
    let (mut x, mut y) = (0, 0);
    let mut direction = 0; // 0: right, 1: down, 2: left, 3: up
    let total_cells = size * size;

    for i in 0..total_cells {
        grid[y][x] = '*'; // Fill the current position with the spiral character

        // Calculate the next position
        let (next_x, next_y) = match direction {
            0 => (x + 1, y), // right
            1 => (x, y + 1), // down
            2 => (x.wrapping_sub(1), y), // left (using wrapping to handle bounds)
            3 => (x, y.wrapping_sub(1)), // up (using wrapping to handle bounds)
            _ => unreachable!(),
        };

        // Check if the next position is valid
        if next_x >= size || next_y >= size || grid[next_y][next_x] == '*' {
            // Change direction clockwise
            direction = (direction + 1) % 4;
            // Recalculate the next position based on the new direction
            let (new_x, new_y) = match direction {
                0 => (x + 1, y), // right
                1 => (x, y + 1), // down
                2 => (x.wrapping_sub(1), y), // left
                3 => (x, y.wrapping_sub(1)), // up
                _ => unreachable!(),
            };
            x = new_x;
            y = new_y;
        } else {
            x = next_x;
            y = next_y;
        }
    }

    grid
}

fn print_spiral(grid: &[Vec<char>]) {
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}
