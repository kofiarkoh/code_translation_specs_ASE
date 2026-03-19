use std::io::{self, BufRead};

const D_SIZE_MAX: usize = 205; // Maximum size

// Global variables
static mut SI_W: usize = 0; // Width
static mut SI_H: usize = 0; // Height
static mut SI_2_MASS: [[i32; D_SIZE_MAX]; D_SIZE_MAX] = [[0; D_SIZE_MAX]; D_SIZE_MAX]; // Grid
static mut SI_3_CNT: [[[i32; D_SIZE_MAX * 2]; D_SIZE_MAX]; D_SIZE_MAX] = [[[0; D_SIZE_MAX * 2]; D_SIZE_MAX]; D_SIZE_MAX]; // Count

// Get maximum of two values
fn get_max(val1: i32, val2: i32) -> i32 {
    if val1 > val2 {
        val1
    } else {
        val2
    }
}

// Move right and down
fn move_position(row: &mut usize, col: &mut usize) -> bool {
    *row = row.saturating_sub(1);
    *col += 1;

    *row < 0 || *col >= unsafe { SI_W }
}

// Get the sum of values at the given positions
fn get_add(row: usize, col: usize, row2: usize, col2: usize) -> i32 {
    let mut add = unsafe { SI_2_MASS[row][col] };
    if row == row2 && col == col2 {
        return add;
    }
    add += unsafe { SI_2_MASS[row2][col2] };
    add
}

// Main processing function
fn main_processing() -> i32 {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read dimensions
    let first_line = lines.next().unwrap().unwrap();
    let dimensions: Vec<usize> = first_line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    unsafe {
        SI_H = dimensions[0];
        SI_W = dimensions[1];
    }

    // Read grid values
    for i in 0..unsafe { SI_H } {
        let line = lines.next().unwrap().unwrap();
        let values: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        for j in 0..unsafe { SI_W } {
            unsafe {
                SI_2_MASS[i][j] = values[j];
            }
        }
    }

    // Initialize counts
    unsafe {
        SI_3_CNT[0][0][0] = SI_2_MASS[0][0];
    }

    // Loop through moves
    for i in 0..(unsafe { SI_W } + unsafe { SI_H } - 2) {
        let (mut li_row, mut li_col) = if i < unsafe { SI_H } {
            (i, 0)
        } else {
            (unsafe { SI_H - 1 }, i - (unsafe { SI_H - 1 }))
        };

        loop {
            let mut li_row2 = li_row;
            let mut li_col2 = li_col;
            let mut j = 0;

            loop {
                // Down, Down
                let li_add = get_add(li_row + 1, li_col, li_row2 + 1, li_col2);
                unsafe {
                    SI_3_CNT[li_row + 1][li_col][j] = get_max(SI_3_CNT[li_row + 1][li_col][j], SI_3_CNT[li_row][li_col][j] + li_add);
                }

                // Down, Right
                let li_add = get_add(li_row + 1, li_col, li_row2, li_col2 + 1);
                unsafe {
                    SI_3_CNT[li_row + 1][li_col][j + 1] = get_max(SI_3_CNT[li_row + 1][li_col][j + 1], SI_3_CNT[li_row][li_col][j] + li_add);
                }

                // Right, Down
                if j > 0 {
                    let li_add = get_add(li_row, li_col + 1, li_row2 + 1, li_col2);
                    unsafe {
                        SI_3_CNT[li_row][li_col + 1][j - 1] = get_max(SI_3_CNT[li_row][li_col + 1][j - 1], SI_3_CNT[li_row][li_col][j] + li_add);
                    }
                }

                // Right, Right
                let li_add = get_add(li_row, li_col + 1, li_row2, li_col2 + 1);
                unsafe {
                    SI_3_CNT[li_row][li_col + 1][j] = get_max(SI_3_CNT[li_row][li_col + 1][j], SI_3_CNT[li_row][li_col][j] + li_add);
                }

                // Move second entity
                if !move_position(&mut li_row2, &mut li_col2) {
                    break;
                }
                j += 1;
            }

            // Move first entity
            if !move_position(&mut li_row, &mut li_col) {
                break;
            }
        }
    }

    unsafe { SI_3_CNT[unsafe { SI_H - 1 }][unsafe { SI_W - 1 }][0] }
}

// Main function
fn main() {
    let result = main_processing();
    println!("{}", result);
}
