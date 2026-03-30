fn main() {
    use std::io::{self, BufRead};

    // Read the dimensions of the grid
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    let first_line = lines.next().unwrap().unwrap();
    let dimensions: Vec<usize> = first_line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let h = dimensions[0]; // Height of the grid
    let w = dimensions[1]; // Width of the grid

    // Initialize the grid
    let mut grid: Vec<Vec<char>> = Vec::with_capacity(h);
    for _ in 0..h {
        let line = lines.next().unwrap().unwrap();
        grid.push(line.chars().collect());
    }

    // Directions for adjacent cells (up, down, left, right)
    let directions: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    // Function to check if a position is within the grid bounds
    fn in_bounds(x: isize, y: isize, h: usize, w: usize) -> bool {
        x >= 0 && x < h as isize && y >= 0 && y < w as isize
    }

    // Check each cell in the grid
    let mut all_adjacent = true;
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] == '#' {
                // Check for adjacent '#' characters
                let mut has_adjacent = false;
                for &(dx, dy) in &directions {
                    let new_x = i as isize + dx;
                    let new_y = j as isize + dy;
                    if in_bounds(new_x, new_y, h, w) && grid[new_x as usize][new_y as usize] == '#' {
                        has_adjacent = true;
                        break;
                    }
                }
                if !has_adjacent {
                    all_adjacent = false;
                    break;
                }
            }
        }
        if !all_adjacent {
            break;
        }
    }

    // Output the result based on the adjacency check
    if all_adjacent {
        println!("Yes");
    } else {
        println!("No");
    }
}
