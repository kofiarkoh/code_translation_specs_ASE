use std::io::{self, BufRead};

const D_SIZE_MAX: usize = 205; // Maximum size

// Internal variables
static mut SI_W: usize = 0; // Width
static mut SI_H: usize = 0; // Height
static mut SI_2_MASS: [[i32; D_SIZE_MAX]; D_SIZE_MAX] = [[0; D_SIZE_MAX]; D_SIZE_MAX]; // Grid
static mut SI_3_CNT: [[[i32; D_SIZE_MAX * 2]; D_SIZE_MAX]; D_SIZE_MAX] = [[[0; D_SIZE_MAX * 2]; D_SIZE_MAX]; D_SIZE_MAX]; // Count

// Get maximum value
fn get_max(val1: i32, val2: i32) -> i32 {
    if val1 > val2 {
        val1
    } else {
        val2
    }
}

// Move down-right
fn move_position(row: &mut isize, col: &mut isize) -> i32 {
    *row -= 1;
    *col += 1;

    if *row < 0 || *col >= (unsafe { SI_W }) as isize {
        return -1;
    }
    0
}

// Get sum of values
fn get_add(row: usize, col: usize, row2: usize, col2: usize) -> i32 {
    let mut add = unsafe { SI_2_MASS[row][col] };
    if row == row2 && col == col2 {
        return add;
    }
    add += unsafe { SI_2_MASS[row2][col2] };
    add
}

// Main execution function
fn main_logic() -> i32 {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

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

    // Initialize count
    unsafe {
        SI_3_CNT[0][0][0] = SI_2_MASS[0][0];
    }

    // Loop through moves
    for i in 0..(unsafe { SI_W } + unsafe { SI_H } - 2) {
        let (mut li_row, mut li_col) = if i < unsafe { SI_H } {
            (i as isize, 0)
        } else {
            (unsafe { SI_H as isize - 1 }, (i - (unsafe { SI_H } - 1)) as isize)
        };

        while true {
            let mut li_row2 = li_row;
            let mut li_col2 = li_col;
            let mut j = 0;

            loop {
                // Down-Down
                let li_add = get_add((li_row + 1) as usize, li_col as usize, (li_row2 + 1) as usize, li_col2 as usize);
                unsafe {
                    SI_3_CNT[(li_row + 1) as usize][li_col as usize][j] = get_max(SI_3_CNT[(li_row + 1) as usize][li_col as usize][j], SI_3_CNT[li_row as usize][li_col as usize][j] + li_add);
                }

                // Down-Right
                let li_add = get_add((li_row + 1) as usize, li_col as usize, li_row2 as usize, (li_col2 + 1) as usize);
                unsafe {
                    SI_3_CNT[(li_row + 1) as usize][li_col as usize][j + 1] = get_max(SI_3_CNT[(li_row + 1) as usize][li_col as usize][j + 1], SI_3_CNT[li_row as usize][li_col as usize][j] + li_add);
                }

                // Right-Down
                if j > 0 {
                    let li_add = get_add(li_row as usize, (li_col + 1) as usize, (li_row2 + 1) as usize, li_col2 as usize);
                    unsafe {
                        SI_3_CNT[li_row as usize][(li_col + 1) as usize][j - 1] = get_max(SI_3_CNT[li_row as usize][(li_col + 1) as usize][j - 1], SI_3_CNT[li_row as usize][li_col as usize][j] + li_add);
                    }
                }

                // Right-Right
                let li_add = get_add(li_row as usize, (li_col + 1) as usize, li_row2 as usize, (li_col2 + 1) as usize);
                unsafe {
                    SI_3_CNT[li_row as usize][(li_col + 1) as usize][j] = get_max(SI_3_CNT[li_row as usize][(li_col + 1) as usize][j], SI_3_CNT[li_row as usize][li_col as usize][j] + li_add);
                }

                // Move to next position
                if move_position(&mut li_row2, &mut li_col2) != 0 {
                    break;
                }
                j += 1;
            }

            // Move to next position
            if move_position(&mut li_row, &mut li_col) != 0 {
                break;
            }
        }
    }

    unsafe { SI_3_CNT[SI_H - 1][SI_W - 1][0] }
}

// Entry point
fn main() {
    let result = main_logic();
    println!("{}", result);
}
