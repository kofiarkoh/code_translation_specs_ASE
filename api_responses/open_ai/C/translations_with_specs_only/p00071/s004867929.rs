fn main() {
    use std::io::{self, BufRead};

    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of test cases
    let test_cases: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..test_cases {
        // Read the 8x8 grid
        let mut grid: [[char; 8]; 8] = [['0'; 8]; 8];
        for i in 0..8 {
            let line = lines.next().unwrap().unwrap();
            grid[i] = line.chars().collect::<Vec<char>>().try_into().unwrap();
        }

        // Read the coordinates for the explosion
        let coords: Vec<usize> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        let (start_x, start_y) = (coords[0], coords[1]);

        // Perform the explosion
        explode(&mut grid, start_x, start_y);

        // Print the modified grid
        for row in grid.iter() {
            println!("{}", row.iter().collect::<String>());
        }
    }
}

// Function to perform the explosion
fn explode(grid: &mut [[char; 8]; 8], x: usize, y: usize) {
    if grid[x][y] == '1' {
        grid[x][y] = '0'; // Set the current position to '0'
        
        // Explore the four possible directions (up, down, left, right)
        if x > 0 { explode(grid, x - 1, y); } // Up
        if x < 7 { explode(grid, x + 1, y); } // Down
        if y > 0 { explode(grid, x, y - 1); } // Left
        if y < 7 { explode(grid, x, y + 1); } // Right
    }
}
