fn main() {
    use std::io::{self, BufRead};

    // Read the input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read height and width
    let mut dimensions = String::new();
    reader.read_line(&mut dimensions).unwrap();
    let mut dims_iter = dimensions.split_whitespace();
    let height: usize = dims_iter.next().unwrap().parse().unwrap();
    let width: usize = dims_iter.next().unwrap().parse().unwrap();

    // Initialize the grid
    let mut grid: Vec<Vec<char>> = Vec::with_capacity(height);
    for _ in 0..height {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        grid.push(line.chars().collect());
    }

    // Function to check if a '#' character is adjacent to another '#'
    fn is_adjacent(grid: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
        let directions = [
            (1, 0),  // down
            (0, 1),  // right
            (-1, 0), // up
            (0, -1), // left
        ];
        for (dx, dy) in directions.iter() {
            let new_x = x as isize + dx;
            let new_y = y as isize + dy;
            if new_x >= 0 && new_x < grid.len() as isize && new_y >= 0 && new_y < grid[0].len() as isize {
                if grid[new_x as usize][new_y as usize] == '#' {
                    return true;
                }
            }
        }
        false
    }

    // Check the adjacency condition for all '#' characters
    let mut all_adjacent = true;
    for i in 0..height {
        for j in 0..width {
            if grid[i][j] == '#' && !is_adjacent(&grid, i, j) {
                all_adjacent = false;
                break;
            }
        }
        if !all_adjacent {
            break;
        }
    }

    // Output the result based on the adjacency condition
    if all_adjacent {
        println!("Yes");
    } else {
        println!("No");
    }
}
