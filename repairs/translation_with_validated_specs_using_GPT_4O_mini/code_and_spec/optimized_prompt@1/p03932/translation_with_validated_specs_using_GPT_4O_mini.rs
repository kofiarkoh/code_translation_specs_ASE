use std::io::{self, BufRead};

const D_SIZE_MAX: usize = 205; // Maximum size

// Struct to hold the grid and counts
struct Grid {
    mass: [[i32; D_SIZE_MAX]; D_SIZE_MAX], // Grid
    count: [[[i32; D_SIZE_MAX * 2]; D_SIZE_MAX]; D_SIZE_MAX], // Count
    width: usize, // Width
    height: usize, // Height
}

impl Grid {
    fn new() -> Self {
        Self {
            mass: [[0; D_SIZE_MAX]; D_SIZE_MAX],
            count: [[[0; D_SIZE_MAX * 2]; D_SIZE_MAX]; D_SIZE_MAX],
            width: 0,
            height: 0,
        }
    }

    // Get maximum value
    fn get_max(&self, val1: i32, val2: i32) -> i32 {
        if val1 > val2 {
            val1
        } else {
            val2
        }
    }

    // Move down and right
    fn move_position(&self, row: &mut isize, col: &mut isize) -> bool {
        *row -= 1;
        *col += 1;

        if *row < 0 || *col >= self.width as isize {
            return false;
        }
        true
    }

    // Get the value to add
    fn get_add(&self, row: usize, col: usize, row2: usize, col2: usize) -> i32 {
        let mut add = self.mass[row][col];
        if row == row2 && col == col2 {
            return add;
        }
        add += self.mass[row2][col2];
        add
    }

    // Main logic to compute the maximum sum
    fn compute_max_sum(&mut self) -> i32 {
        // Initialize counts
        self.count[0][0][0] = self.mass[0][0];

        // Loop through movement counts
        for i in 0..(self.width + self.height - 2) {
            let (mut li_row, mut li_col) = if i < self.height {
                (i as isize, 0)
            } else {
                (self.height as isize - 1, (i - (self.height - 1)) as isize)
            };

            loop {
                let mut li_row2 = li_row;
                let mut li_col2 = li_col;
                let mut j = 0;

                loop {
                    // Down-Down
                    if (li_row + 1) < self.height as isize {
                        let li_add = self.get_add((li_row + 1) as usize, li_col as usize, (li_row2 + 1) as usize, li_col2 as usize);
                        self.count[(li_row + 1) as usize][li_col as usize][j] = self.get_max(self.count[(li_row + 1) as usize][li_col as usize][j], self.count[li_row as usize][li_col as usize][j] + li_add);
                    }

                    // Down-Right
                    if (li_row + 1) < self.height as isize && (li_col2 + 1) < self.width as isize {
                        let li_add = self.get_add((li_row + 1) as usize, li_col as usize, li_row2 as usize, (li_col2 + 1) as usize);
                        self.count[(li_row + 1) as usize][li_col as usize][j + 1] = self.get_max(self.count[(li_row + 1) as usize][li_col as usize][j + 1], self.count[li_row as usize][li_col as usize][j] + li_add);
                    }

                    // Right-Down
                    if j > 0 && (li_col + 1) < self.width as isize {
                        let li_add = self.get_add(li_row as usize, (li_col + 1) as usize, (li_row2 + 1) as usize, li_col2 as usize);
                        self.count[li_row as usize][(li_col + 1) as usize][j - 1] = self.get_max(self.count[li_row as usize][(li_col + 1) as usize][j - 1], self.count[li_row as usize][li_col as usize][j] + li_add);
                    }

                    // Right-Right
                    if (li_col + 1) < self.width as isize {
                        let li_add = self.get_add(li_row as usize, (li_col + 1) as usize, li_row2 as usize, (li_col2 + 1) as usize);
                        self.count[li_row as usize][(li_col + 1) as usize][j] = self.get_max(self.count[li_row as usize][(li_col + 1) as usize][j], self.count[li_row as usize][li_col as usize][j] + li_add);
                    }

                    // Move to next position
                    if !self.move_position(&mut li_row2, &mut li_col2) {
                        break;
                    }
                    j += 1;
                }

                // Move to next position
                if !self.move_position(&mut li_row, &mut li_col) {
                    break;
                }
            }
        }

        self.count[self.height - 1][self.width - 1][0]
    }
}

// Entry point
fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let mut grid = Grid::new();

    // Read dimensions
    let first_line = lines.next().unwrap().unwrap();
    let dimensions: Vec<usize> = first_line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    grid.height = dimensions[0];
    grid.width = dimensions[1];

    // Read grid values
    for i in 0..grid.height {
        let line = lines.next().unwrap().unwrap();
        let values: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        for j in 0..grid.width {
            grid.mass[i][j] = values[j];
        }
    }

    // Compute and print the result
    let result = grid.compute_max_sum();
    println!("{}", result);
}
``` 

### Changes Made:
1. **Bounds Checking**: Added checks to ensure that we do not access out-of-bounds indices in the `compute_max_sum` method, which could lead to stack overflow.
2. **Loop Control**: Ensured that the loops do not run indefinitely by checking the conditions before accessing the grid. 

These changes should help prevent stack overflow errors during execution.
