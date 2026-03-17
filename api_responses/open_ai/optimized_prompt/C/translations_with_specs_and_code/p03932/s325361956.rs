use std::io::{self, BufRead};

const D_SIZE_MAX: usize = 205; // Maximum size

// Global variables
static mut SI3_CNT: [[[i32; D_SIZE_MAX * 2]; D_SIZE_MAX]; D_SIZE_MAX] = [[[0; D_SIZE_MAX * 2]; D_SIZE_MAX]; D_SIZE_MAX];
static mut SI2_MASS: [[i32; D_SIZE_MAX]; D_SIZE_MAX] = [[0; D_SIZE_MAX]; D_SIZE_MAX];

fn get_max(val1: i32, val2: i32) -> i32 {
    if val1 > val2 {
        val1
    } else {
        val2
    }
}

fn get_add(row: usize, col: usize, row2: usize, col2: usize) -> i32 {
    let mut add = unsafe { SI2_MASS[row][col] };
    if row == row2 && col == col2 {
        return add;
    }
    add += unsafe { SI2_MASS[row2][col2] };
    add
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read dimensions
    let first_line = lines.next().unwrap().unwrap();
    let dimensions: Vec<usize> = first_line.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let (si_h, si_w) = (dimensions[0], dimensions[1]);

    // Read grid values
    for i in 0..si_h {
        let line = lines.next().unwrap().unwrap();
        let values: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        for j in 0..si_w {
            unsafe {
                SI2_MASS[i][j] = values[j];
            }
        }
    }

    // Initialize count
    unsafe {
        SI3_CNT[0][0][0] = SI2_MASS[0][0];
    }

    // Loop through movement counts
    for i in 0..(si_w + si_h - 2) {
        let (mut li_row, mut li_col) = if i < si_h {
            (i, 0)
        } else {
            (si_h - 1, i - (si_h - 1))
        };

        loop {
            let mut li_row2 = li_row;
            let mut li_col2 = li_col;
            let mut j = 0;

            loop {
                // Down-Down
                let li_add = get_add(li_row + 1, li_col, li_row2 + 1, li_col2);
                unsafe {
                    SI3_CNT[li_row + 1][li_col][j] = get_max(SI3_CNT[li_row + 1][li_col][j], SI3_CNT[li_row][li_col][j] + li_add);
                }

                // Down-Right
                let li_add = get_add(li_row + 1, li_col, li_row2, li_col2 + 1);
                unsafe {
                    SI3_CNT[li_row + 1][li_col][j + 1] = get_max(SI3_CNT[li_row + 1][li_col][j + 1], SI3_CNT[li_row][li_col][j] + li_add);
                }

                // Right-Down
                if j > 0 {
                    let li_add = get_add(li_row, li_col + 1, li_row2 + 1, li_col2);
                    unsafe {
                        SI3_CNT[li_row][li_col + 1][j - 1] = get_max(SI3_CNT[li_row][li_col + 1][j - 1], SI3_CNT[li_row][li_col][j] + li_add);
                    }
                }

                // Right-Right
                let li_add = get_add(li_row, li_col + 1, li_row2, li_col2 + 1);
                unsafe {
                    SI3_CNT[li_row][li_col + 1][j] = get_max(SI3_CNT[li_row][li_col + 1][j], SI3_CNT[li_row][li_col][j] + li_add);
                }

                // Move to next position
                li_row2 += 1;
                li_col2 += 1;
                if li_row2 >= si_h || li_col2 >= si_w {
                    break;
                }
                j += 1;
            }

            // Move to next position
            li_row += 1;
            li_col += 1;
            if li_row >= si_h || li_col >= si_w {
                break;
            }
        }
    }

    // Output the result
    unsafe {
        println!("{}", SI3_CNT[si_h - 1][si_w - 1][0]);
    }
}
```

### Notes:
1. **Global Variables**: The use of `unsafe` is necessary for accessing the static arrays, as Rust does not allow mutable static variables without `unsafe`.
2. **Input Handling**: The input is read from standard input, as specified. The dimensions and grid values are parsed accordingly.
3. **Loop Logic**: The logic for moving through the grid and calculating the maximum sums is preserved from the C code.
4. **Output**: The output format is preserved to match the specification exactly.
