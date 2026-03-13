// sudoku_check.rs
// Translated from the provided C code.
// Notes:
// - The original C code had file-scope globals `num` and `flag`, but main
//   redeclared local arrays with the same names, and the function `check_box`
//   referenced the globals. That means `check_box` operates on globals while
//   main reads/writes locals. This translation preserves that behavior.
// - To model the file-scope globals without using `unsafe`, we store them in
//   Mutex-wrapped statics. This preserves single-threaded semantics while
//   allowing safe global mutation.

use std::io::{self, Read};
use std::sync::Mutex;

// Global (file-scope) arrays from the C program, zero-initialized.
// Wrapped in Mutex to allow safe mutation from functions.
static GLOBAL_NUM: Mutex<[[i32; 9]; 9]> = Mutex::new([[0; 9]; 9]);
static GLOBAL_FLAG: Mutex<[[i32; 9]; 9]> = Mutex::new([[0; 9]; 9]);

fn check_box(y: usize, x: usize) {
    // This function mirrors the C version and operates on the global arrays.
    // We assume values are in 0..=9 like the original program; out-of-range
    // values are guarded to avoid panics in Rust (the C code would be UB).
    let gn = GLOBAL_NUM.lock().unwrap();
    let mut gf = GLOBAL_FLAG.lock().unwrap();

    let mut cnt = [0i32; 10];

    for i in y..y + 3 {
        for j in x..x + 3 {
            let v = gn[i][j];
            if (0..10).contains(&v) {
                cnt[v as usize] += 1;
            } else {
                // If input falls outside 0..9, the C code would index out of
                // bounds (undefined). Here we skip counting to avoid panic.
            }
        }
    }

    for i in y..y + 3 {
        for j in x..x + 3 {
            let v = gn[i][j];
            if (0..10).contains(&v) {
                if cnt[v as usize] != 1 {
                    gf[i][j] = 1;
                }
            } else {
                // As above: skip out-of-range values.
            }
        }
    }
}

fn main() {
    // Read entire stdin and split into whitespace-separated integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut toks = input.split_whitespace().map(|s| s.parse::<i32>().unwrap());
    // Read number of test cases
    let n = match toks.next() {
        Some(v) => v,
        None => return,
    };

    for _case in 0..n {
        // Local arrays in main shadow the globals in C; we recreate that.
        let mut num = [[0i32; 9]; 9];
        let mut flag = [[0i32; 9]; 9];

        // Read and check rows
        for i in 0..9 {
            let mut cnt = [0i32; 10];
            for j in 0..9 {
                // This mirrors scanf("%d", ...)
                num[i][j] = toks.next().unwrap();
                let v = num[i][j];
                if (0..10).contains(&v) {
                    cnt[v as usize] += 1;
                } else {
                    // If out of range, skip counting; original C would be UB.
                }
            }
            for j in 0..9 {
                let v = num[i][j];
                if (0..10).contains(&v) {
                    if cnt[v as usize] != 1 {
                        flag[i][j] = 1;
                    }
                } else {
                    // skip
                }
            }
        }

        // Check columns
        for j in 0..9 {
            let mut cnt = [0i32; 10];
            for i in 0..9 {
                let v = num[i][j];
                if (0..10).contains(&v) {
                    cnt[v as usize] += 1;
                } else {
                    // skip
                }
            }
            for i in 0..9 {
                let v = num[i][j];
                if (0..10).contains(&v) {
                    if cnt[v as usize] != 1 {
                        flag[i][j] = 1;
                    }
                } else {
                    // skip
                }
            }
        }

        // Call check_box for each 3x3 box.
        // IMPORTANT: check_box operates on GLOBAL_NUM and GLOBAL_FLAG,
        // not on the local `num` and `flag` arrays. This mirrors the original C bug/behavior.
        for bi in 0..3 {
            for bj in 0..3 {
                check_box(bi * 3, bj * 3);
            }
        }

        // Print output: for each cell print either '*' or ' ' followed by the number,
        // exactly like printf("%c%d", flag[i][j] ? '*' : ' ', num[i][j]);
        for i in 0..9 {
            let mut line = String::new();
            for j in 0..9 {
                let ch = if flag[i][j] != 0 { '*' } else { ' ' };
                line.push(ch);
                line.push_str(&num[i][j].to_string());
            }
            println!("{}", line);
        }
        // Print the extra blank line (puts(""))
        println!();
    }
}
