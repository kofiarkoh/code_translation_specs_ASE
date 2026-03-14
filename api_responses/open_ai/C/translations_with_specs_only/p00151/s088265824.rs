use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let n: usize = match line.trim().parse() {
            Ok(num) if num > 0 => num,
            Ok(_) => continue, // Ignore non-positive integers
            Err(_) => continue, // Ignore invalid integers
        };

        let mut grid = Vec::with_capacity(n);
        for _ in 0..n {
            let mut row = String::new();
            reader.read_line(&mut row).unwrap();
            grid.push(row.trim().chars().collect::<Vec<_>>());
        }

        let max_length = find_max_consecutive_ones(&grid);
        println!("{}", max_length);
    }
}

fn find_max_consecutive_ones(grid: &[Vec<char>]) -> usize {
    let n = grid.len();
    let mut max_length = 0;

    // Check horizontal and vertical
    for i in 0..n {
        let mut horizontal_count = 0;
        let mut vertical_count = 0;

        for j in 0..n {
            // Horizontal
            if grid[i][j] == '1' {
                horizontal_count += 1;
                max_length = max_length.max(horizontal_count);
            } else {
                horizontal_count = 0;
            }

            // Vertical
            if grid[j][i] == '1' {
                vertical_count += 1;
                max_length = max_length.max(vertical_count);
            } else {
                vertical_count = 0;
            }
        }
    }

    // Check diagonals
    for d in 0..(2 * n - 1) {
        let mut diag1_count = 0; // Top-left to bottom-right
        let mut diag2_count = 0; // Top-right to bottom-left

        for i in 0..n {
            let j1 = d - i; // For diag1
            let j2 = i - d + (n - 1); // For diag2

            if j1 >= 0 && j1 < n && grid[i][j1] == '1' {
                diag1_count += 1;
                max_length = max_length.max(diag1_count);
            } else {
                diag1_count = 0;
            }

            if j2 >= 0 && j2 < n && grid[i][j2] == '1' {
                diag2_count += 1;
                max_length = max_length.max(diag2_count);
            } else {
                diag2_count = 0;
            }
        }
    }

    max_length
}
