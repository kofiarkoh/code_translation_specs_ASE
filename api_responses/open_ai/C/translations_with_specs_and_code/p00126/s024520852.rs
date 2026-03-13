// sudoku_flag.rs
//
// Translated from the given C program according to the provided specification.
// Preserves the observable behavior described in the SPEC, including the
// implementation-level mismatch where the 3x3 box checking operates on
// different storage than the per-grid data used for rows/columns (so box
// duplicates do not affect the printed flags).
//
// Note: To reproduce the C program's separate-global behavior for check_box,
// this translation uses module-level mutable statics for the globals that
// check_box reads/writes. Access to these statics requires unsafe blocks.
// The unsafe is minimized and confined to the check_box function, matching
// the original program's semantics (the globals are zero-initialized and
// are distinct from the per-grid local arrays used in main).
//
// Preconditions (as in SPEC): input must contain an integer n followed by
// n*81 integers. Cell values are expected to be in 0..9; out-of-range values
// will cause a runtime panic due to indexing.

use std::io::{self, Read};

static mut NUM_G: [[i32; 9]; 9] = [[0; 9]; 9];
static mut FLAG_G: [[i32; 9]; 9] = [[0; 9]; 9];

fn check_box(y: usize, x: usize) {
    // This function intentionally operates on the module-level globals
    // NUM_G and FLAG_G to mirror the C program's bug where the box check
    // uses different storage than the per-grid arrays used in main.
    //
    // Accessing static mut globals requires unsafe.
    let mut cnt: [i32; 10] = [0; 10];

    unsafe {
        for i in y..(y + 3) {
            for j in x..(x + 3) {
                let v = NUM_G[i][j];
                // Assumes v is in 0..=9 per SPEC; otherwise this will panic.
                cnt[v as usize] += 1;
            }
        }
        for i in y..(y + 3) {
            for j in x..(x + 3) {
                let v = NUM_G[i][j];
                if cnt[v as usize] != 1 {
                    FLAG_G[i][j] = 1;
                }
            }
        }
    }
}

fn main() {
    // Read all stdin, split by whitespace, parse into i32 tokens.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On I/O error, exit normally (spec indicates program returns 0).
        return;
    }
    let mut tokens = input.split_whitespace();

    // Parse n; if missing or unparsable, exit (SPEC: behavior unspecified on malformed input).
    let n_val = match tokens.next() {
        Some(s) => match s.parse::<i32>() {
            Ok(v) => v,
            Err(_) => return,
        },
        None => return,
    };

    // In C the loop is: scanf("%d",&n); while (n-- > 0) { ... }
    // That executes body exactly n times if n>0, otherwise zero times (including negative n).
    let mut n = n_val;
    while n > 0 {
        // Per-grid local arrays (shadowing the C globals).
        let mut num: [[i32; 9]; 9] = [[0; 9]; 9];
        let mut flag: [[i32; 9]; 9] = [[0; 9]; 9];

        // Read rows: for each row, read 9 ints, count duplicates within row.
        for i in 0..9 {
            let mut cnt: [i32; 10] = [0; 10];
            for j in 0..9 {
                // Read next token; if missing/unparsable, treat as 0 and continue.
                // The C code used scanf without checking return; here we choose to
                // return early on parse failure to avoid panics, but this diverges
                // only in malformed-input cases (SPEC allows unspecified behavior).
                let v = match tokens.next() {
                    Some(s) => match s.parse::<i32>() {
                        Ok(x) => x,
                        Err(_) => return,
                    },
                    None => return,
                };
                num[i][j] = v;
                // Assumes v in 0..=9 per SPEC.
                cnt[v as usize] += 1;
            }
            for j in 0..9 {
                let v = num[i][j];
                if cnt[v as usize] != 1 {
                    flag[i][j] = 1;
                }
            }
        }

        // Column checks: count duplicates per column and set flags in local flag[][].
        for j in 0..9 {
            let mut cnt: [i32; 10] = [0; 10];
            for i in 0..9 {
                let v = num[i][j];
                cnt[v as usize] += 1;
            }
            for i in 0..9 {
                let v = num[i][j];
                if cnt[v as usize] != 1 {
                    flag[i][j] = 1;
                }
            }
        }

        // Box checks: call check_box on each 3x3 box. check_box operates on separate
        // globals and therefore does not affect the local `flag` used for printing,
        // reproducing the bug described in SPEC.
        for bi in 0..3 {
            for bj in 0..3 {
                check_box(bi * 3, bj * 3);
            }
        }

        // Print the grid: for each cell print either '*' or ' ' then the integer.
        for i in 0..9 {
            for j in 0..9 {
                let marker = if flag[i][j] != 0 { '*' } else { ' ' };
                // Print without separators, matching printf("%c%d", ...)
                print!("{}{}", marker, num[i][j]);
            }
            println!();
        }
        // Blank line after each grid
        println!();

        n -= 1;
    }

    // Program exits with code 0 implicitly.
}
