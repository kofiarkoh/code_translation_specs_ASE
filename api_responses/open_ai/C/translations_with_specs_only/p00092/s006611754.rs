use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let size: usize = line.trim().parse().unwrap();

        // Termination condition
        if size == 0 {
            break;
        }

        let mut grid = Vec::new();
        for _ in 0..size {
            let line = reader.lines().next().unwrap().unwrap();
            grid.push(line.chars().collect::<Vec<_>>());
        }

        let max_length = find_max_consecutive_non_obstacles(&grid);
        println!("{}", max_length);
    }
}

fn find_max_consecutive_non_obstacles(grid: &[Vec<char>]) -> usize {
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let mut max_length = 0;

    for col in 0..num_cols {
        let mut current_length = 0;

        for row in 0..num_rows {
            if grid[row][col] != '*' {
                current_length += 1;
                max_length = max_length.max(current_length);
            } else {
                current_length = 0; // Reset on obstacle
            }
        }
    }

    max_length
}
