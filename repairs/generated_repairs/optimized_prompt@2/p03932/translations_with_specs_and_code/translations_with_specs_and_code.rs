use std::io::{self, BufRead};

const D_SIZE_MAX: usize = 205; // Maximum size

// Global variables
static mut SI2MASS: [[i32; D_SIZE_MAX]; D_SIZE_MAX] = [[0; D_SIZE_MAX]; D_SIZE_MAX]; // Grid
static mut SI3CNT: [[[i32; D_SIZE_MAX * 2]; D_SIZE_MAX]; D_SIZE_MAX] = [[[0; D_SIZE_MAX * 2]; D_SIZE_MAX]; D_SIZE_MAX]; // Count
static SIW: std::sync::Mutex<usize> = std::sync::Mutex::new(0); // Width
static SIH: std::sync::Mutex<usize> = std::sync::Mutex::new(0); // Height

// Get maximum value
fn get_max(val1: i32, val2: i32) -> i32 {
    if val1 > val2 {
        val1
    } else {
        val2
    }
}

// Move down and right
fn move_position(row: &mut isize, col: &mut isize) -> bool {
    *row -= 1;
    *col += 1;

    let width = *SIW.lock().unwrap();
    if *row < 0 || *col >= width as isize {
        return false;
    }
    true
}

// Get the sum of values at the given positions
fn get_add(row: usize, col: usize, row2: usize, col2: usize) -> i32 {
    let mut add = unsafe { SI2MASS[row][col] };
    if row == row2 && col == col2 {
        return add;
    }
    add += unsafe { SI2MASS[row2][col2] };
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
    
    {
        let mut width = SIW.lock().unwrap();
        let mut height = SIH.lock().unwrap();
        *height = dimensions[0];
        *width = dimensions[1];
    }

    // Read grid values
    for i in 0..*SIH.lock().unwrap() {
        let line = lines.next().unwrap().unwrap();
        let values: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        for j in 0..*SIW.lock().unwrap() {
            unsafe {
                SI2MASS[i][j] = values[j];
            }
        }
    }

    // Initialize counts
    unsafe {
        SI3CNT[0][0][0] = SI2MASS[0][0];
    }

    // Loop through moves
    for i in 0..(*SIW.lock().unwrap() + *SIH.lock().unwrap() - 2) {
        let (mut li_row, mut li_col) = if i < *SIH.lock().unwrap() {
            (i as isize, 0)
        } else {
            (*SIH.lock().unwrap() as isize - 1, (i - (*SIH.lock().unwrap() - 1)) as isize)
        };

        loop {
            let mut li_row2 = li_row;
            let mut li_col2 = li_col;
            let mut j = 0;

            loop {
                // Down-Down
                let li_add = get_add((li_row + 1) as usize, li_col as usize, (li_row2 + 1) as usize, li_col2 as usize);
                unsafe {
                    SI3CNT[(li_row + 1) as usize][li_col as usize][j] = get_max(SI3CNT[(li_row + 1) as usize][li_col as usize][j], SI3CNT[li_row as usize][li_col as usize][j] + li_add);
                }

                // Down-Right
                let li_add = get_add((li_row + 1) as usize, li_col as usize, li_row2 as usize, (li_col2 + 1) as usize);
                unsafe {
                    SI3CNT[(li_row + 1) as usize][li_col as usize][j + 1] = get_max(SI3CNT[(li_row + 1) as usize][li_col as usize][j + 1], SI3CNT[li_row as usize][li_col as usize][j] + li_add);
                }

                // Right-Down
                if j > 0 {
                    let li_add = get_add(li_row as usize, (li_col + 1) as usize, (li_row2 + 1) as usize, li_col2 as usize);
                    unsafe {
                        SI3CNT[li_row as usize][(li_col + 1) as usize][j - 1] = get_max(SI3CNT[li_row as usize][(li_col + 1) as usize][j - 1], SI3CNT[li_row as usize][li_col as usize][j] + li_add);
                    }
                }

                // Right-Right
                let li_add = get_add(li_row as usize, (li_col + 1) as usize, li_row2 as usize, (li_col2 + 1) as usize);
                unsafe {
                    SI3CNT[li_row as usize][(li_col + 1) as usize][j] = get_max(SI3CNT[li_row as usize][(li_col + 1) as usize][j], SI3CNT[li_row as usize][li_col as usize][j] + li_add);
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

    unsafe { SI3CNT[*SIH.lock().unwrap() - 1][*SIW.lock().unwrap() - 1][0] }
}

// Main function
fn main() {
    let result = main_logic();
    println!("{}", result);
}
