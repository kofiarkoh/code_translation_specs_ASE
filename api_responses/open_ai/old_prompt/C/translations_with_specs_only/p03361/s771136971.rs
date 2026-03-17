fn main() {
    // Define the maximum dimensions for the grid
    const MAX_H: usize = 51;
    const MAX_W: usize = 60;

    // Read the height and width of the grid
    let (h, w): (usize, usize) = read_dimensions();

    // Initialize the grid
    let mut grid: [[char; MAX_W]; MAX_H] = [['.'; MAX_W]; MAX_H];

    // Read the grid input
    for i in 0..h {
        let line: String = read_line();
        for j in 0..w {
            grid[i][j] = line.chars().nth(j).unwrap();
        }
    }

    // Check for isolation of '#' characters
    let result = check_isolation(&grid, h, w);
    
    // Output the result
    println!("{}", result);
}

// Function to read dimensions from input
fn read_dimensions() -> (usize, usize) {
    // Placeholder for reading dimensions; replace with actual input reading logic
    (1, 1) // Example return value
}

// Function to read a line of input
fn read_line() -> String {
    // Placeholder for reading a line; replace with actual input reading logic
    String::from("#") // Example return value
}

// Function to check if all '#' characters are isolated
fn check_isolation(grid: &[[char; 60]; 51], h: usize, w: usize) -> &'static str {
    // Directions for checking adjacent cells (up, down, left, right)
    let directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == '#' {
                // Check all four cardinal directions
                for &(di, dj) in &directions {
                    let ni = i as isize + di;
                    let nj = j as isize + dj;

                    // Check if the new indices are within bounds
                    if ni >= 0 && ni < h as isize && nj >= 0 && nj < w as isize {
                        if grid[ni as usize][nj as usize] == '#' {
                            return "No"; // Found an adjacent '#'
                        }
                    }
                }
            }
        }
    }
    "Yes" // All '#' characters are isolated
}
