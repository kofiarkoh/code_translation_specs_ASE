use std::io::{self, BufRead};

const MAX_ROWS: usize = 99;
const MAX_COLS: usize = 50;

static mut DATA: [[i32; MAX_COLS]; MAX_ROWS] = [[0; MAX_COLS]; MAX_ROWS];
static mut DP: [[i32; MAX_COLS]; MAX_ROWS] = [[0; MAX_COLS]; MAX_ROWS];

fn max(x: i32, y: i32) -> i32 {
    if x > y {
        x
    } else {
        y
    }
}

fn judge(i: usize, j: usize, sum: i32, n: usize) -> i32 {
    unsafe {
        let new_sum = sum + DATA[i][j];

        if i == n || new_sum < DP[i][j] {
            new_sum
        } else {
            DP[i][j] = new_sum;

            if i < n / 2 {
                max(judge(i + 1, j, DP[i][j], n), judge(i + 1, j + 1, DP[i][j], n))
            } else {
                max(judge(i + 1, j.wrapping_sub(1), DP[i][j], n), judge(i + 1, j, DP[i][j], n))
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    let mut i = 0;
    let mut j = 1;
    let mut ans: i32;

    for line in handle.lines() {
        let line = line.unwrap();
        let mut parts = line.split(',');

        while let Some(part) = parts.next() {
            if let Ok(value) = part.trim().parse::<i32>() {
                unsafe {
                    DATA[i][j] = value;
                    j += 1;
                }
            }
        }

        unsafe {
            DATA[i][0] = 0; // Mark the end of the row
            i += 1;
            j = 1; // Reset j for the next row
        }
    }

    let n = i;

    unsafe {
        ans = judge(0, 1, 0, n);
    }

    println!("{}", ans);
}
```

### Explanation of Key Choices:
1. **Unsafe Blocks**: The use of `unsafe` is necessary here because we are directly manipulating static mutable arrays (`DATA` and `DP`). Rust's safety guarantees prevent mutable static variables without `unsafe`.
  
2. **Wrapping Subtraction**: The use of `wrapping_sub` for `j` in the `judge` function ensures that if `j` is `0`, it wraps around to the maximum value of `usize`, mimicking potential overflow behavior in C.

3. **Input Handling**: The input is read line by line, and each line is split by commas to mimic the behavior of the original C code, ensuring that the parsing logic remains intact.

4. **Array Initialization**: The arrays are initialized statically to match the C code's structure, ensuring that the dimensions and initial values are preserved.

5. **Output**: The output format is preserved by using `println!`, which matches the C `printf` behavior for integers.
