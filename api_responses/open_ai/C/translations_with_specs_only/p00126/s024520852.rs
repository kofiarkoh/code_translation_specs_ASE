use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of grids
    let num_grids: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for grid_index in 0..num_grids {
        let mut grid = [[0; 9]; 9];
        let mut valid_rows = vec![vec![false; 10]; 9];
        let mut valid_cols = vec![vec![false; 10]; 9];
        let mut valid_subgrids = vec![vec![false; 10]; 9];

        // Read the grid entries
        for i in 0..9 {
            let line = lines.next().unwrap().unwrap();
            let entries: Vec<i32> = line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            for j in 0..9 {
                grid[i][j] = entries[j];
                let num = entries[j];

                // Check for valid Sudoku numbers (1-9)
                if num >= 1 && num <= 9 {
                    let subgrid_index = (i / 3) * 3 + (j / 3);
                    if valid_rows[i][num as usize] || valid_cols[j][num as usize] || valid_subgrids[subgrid_index][num as usize] {
                        // Mark as invalid if already seen in row, column, or subgrid
                        grid[i][j] = -1; // Mark invalid with -1
                    } else {
                        // Mark as seen
                        valid_rows[i][num as usize] = true;
                        valid_cols[j][num as usize] = true;
                        valid_subgrids[subgrid_index][num as usize] = true;
                    }
                } else {
                    // Mark invalid if out of range
                    grid[i][j] = -1; // Mark invalid with -1
                }
            }
        }

        // Output the processed grid
        for i in 0..9 {
            for j in 0..9 {
                if grid[i][j] == -1 {
                    print!("* "); // Mark invalid entries with an asterisk
                } else {
                    print!("{} ", grid[i][j]); // Valid entries
                }
            }
            println!(); // New line after each row
        }
        if grid_index < num_grids - 1 {
            println!(); // Blank line between grids
        }
    }
}
