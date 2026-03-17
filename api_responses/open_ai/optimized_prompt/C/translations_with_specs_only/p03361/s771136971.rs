fn main() {
    use std::io::{self, BufRead};

    // Read height and width from input
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read height and width, expecting valid integers
    let height: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let width: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Initialize the grid
    let mut grid: Vec<Vec<char>> = Vec::with_capacity(height);
    
    // Read the grid from input
    for _ in 0..height {
        let line = lines.next().unwrap().unwrap();
        // Ensure each row has the correct width
        assert_eq!(line.len(), width);
        grid.push(line.chars().collect());
    }

    // Function to check if a cell is isolated
    fn is_isolated(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        for (dx, dy) in directions.iter() {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx >= 0 && nx < grid.len() as isize && ny >= 0 && ny < grid[0].len() as isize {
                if grid[nx as usize][ny as usize] == '#' {
                    return false; // Found an adjacent '#'
                }
            }
        }
        true // No adjacent '#' found
    }

    // Check for isolated '#' characters
    let mut has_isolated = false;
    for i in 0..height {
        for j in 0..width {
            if grid[i][j] == '#' && is_isolated(&grid, i, j) {
                has_isolated = true; // Found an isolated '#'
                break;
            }
        }
        if has_isolated {
            break; // No need to check further
        }
    }

    // Output the result
    if has_isolated {
        println!("No");
    } else {
        println!("Yes");
    }
}
