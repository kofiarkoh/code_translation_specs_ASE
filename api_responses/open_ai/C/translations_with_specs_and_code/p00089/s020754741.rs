// translator.rs
// Translated from the provided C program according to the SPEC.
// Preserves semantics: fixed-size storage, start position (0,1), movement rules,
// dp pruning logic, and stdout formatting (single integer + newline).

use std::io::{self, Read};
use std::process::exit;

const MAX_R: usize = 99;
const MAX_C: usize = 50;

fn max_i32(a: i32, b: i32) -> i32 {
    if a > b { a } else { b }
}

// Recursive judge function mirroring the C logic.
// - data: &[ [i8; MAX_C]; MAX_R ]
// - dp: &mut [[i32; MAX_C]; MAX_R ]
// - n: number of rows parsed
// Behavior: sum is the incoming prefix sum; function adds data[i][j],
// if i == n OR sum < dp[i][j] returns sum, else updates dp[i][j] = sum
// and recurses according to midpoint rules.
fn judge(
    i: usize,
    j: usize,
    sum: i32,
    n: usize,
    data: &[[i8; MAX_C]; MAX_R],
    dp: &mut [[i32; MAX_C]; MAX_R],
) -> i32 {
    // Add cell value (i32 promotion)
    let sum = sum + (data[i][j] as i32);

    // If reached beyond last logical row (i == n) or pruning condition
    if i == n || sum < dp[i][j] {
        return sum;
    } else {
        dp[i][j] = sum;
        // midpoint decision: rows strictly before midpoint use down/right,
        // rows at or after midpoint use down/left.
        if i < n / 2 {
            // move to (i+1, j) or (i+1, j+1)
            let a = judge(i + 1, j, dp[i][j], n, data, dp);
            let b = judge(i + 1, j + 1, dp[i][j], n, data, dp);
            max_i32(a, b)
        } else {
            // move to (i+1, j-1) or (i+1, j)
            // We assume inputs satisfy the preconditions so j >= 1 when j-1 is used.
            let a = judge(i + 1, j.saturating_sub(1), dp[i][j], n, data, dp);
            let b = judge(i + 1, j, dp[i][j], n, data, dp);
            max_i32(a, b)
        }
    }
}

// Parse stdin according to the SPEC:
// - rows separated by '\n'
// - numbers separated by ','
// - tokens are integers; start column index is 1 (col 0 reserved)
// Returns (data array, n = number of rows parsed).
fn parse_input() -> ([[i8; MAX_C]; MAX_R], usize) {
    let mut buf = String::new();
    if io::stdin().read_to_string(&mut buf).is_err() {
        // On read error, exit (mirrors undefined behavior in C; but we choose to exit).
        exit(1);
    }

    let mut data = [[0i8; MAX_C]; MAX_R];

    let bytes = buf.as_bytes();
    let mut pos = 0usize;
    let len = bytes.len();

    let mut row: usize = 0;
    let mut col: usize = 1; // start at column 1 per SPEC/C code
    let mut any_number = false;

    while pos < len {
        // Skip spaces and tabs and carriage returns but treat '\n' specially (as row sep).
        // However, digits or +/- may appear immediately.
        // Skip other whitespace that are not '\n'.
        while pos < len {
            let c = bytes[pos];
            if c == b' ' || c == b'\t' || c == b'\r' {
                pos += 1;
                continue;
            }
            break;
        }
        if pos >= len {
            break;
        }

        // If we encounter a newline without a number (empty line), treat as new row.
        if bytes[pos] == b'\n' {
            // Move to next row; mimic C behavior of resetting column to 1.
            row = if row + 1 <= MAX_R { row + 1 } else { row }; // Guard though spec assumes valid.
            col = 1;
            pos += 1;
            continue;
        }

        // Parse optional sign
        let mut sign = 1i32;
        if bytes[pos] == b'+' {
            pos += 1;
        } else if bytes[pos] == b'-' {
            sign = -1;
            pos += 1;
        }

        // Parse digits
        if pos >= len || !bytes[pos].is_ascii_digit() {
            // If malformed, skip this char (mirrors permissive behavior in C to some degree).
            pos += 1;
            continue;
        }

        let mut val: i32 = 0;
        while pos < len && bytes[pos].is_ascii_digit() {
            val = val * 10 + (bytes[pos] - b'0') as i32;
            pos += 1;
        }
        val *= sign;

        // Store into data[row][col] if within bounds.
        if row < MAX_R && col < MAX_C {
            data[row][col] = val as i8; // per SPEC values fit within signed char
            any_number = true;
        } else {
            // Out-of-bounds per SPEC is undefined; we choose to ignore extra values.
            // (Alternatively could exit or panic, but spec treats out-of-range as undefined.)
        }

        // After number, expect delimiter: ',' or '\n' or EOF. Skip spaces between number and delimiter.
        while pos < len && (bytes[pos] == b' ' || bytes[pos] == b'\t' || bytes[pos] == b'\r') {
            pos += 1;
        }

        if pos >= len {
            // EOF after number: treat as end of row (increment row count)
            row = if row + 1 <= MAX_R { row + 1 } else { row };
            col = 1;
            break;
        }

        let sep = bytes[pos];
        if sep == b',' {
            // move to next column in same row
            col = col.saturating_add(1);
            pos += 1;
        } else if sep == b'\n' {
            // move to next row
            row = if row + 1 <= MAX_R { row + 1 } else { row };
            col = 1;
            pos += 1;
        } else {
            // Unexpected character: skip it and continue parsing.
            pos += 1;
        }
    }

    // If no numbers were read, row remains 0 and n = 0.
    // If we read numbers and last row wasn't terminated by newline, ensure we count that row.
    let n = if any_number {
        // If last action didn't increment row for the last filled row (i.e., we filled at current row),
        // we need to ensure n is number of rows filled. The parsing logic increments row when it sees newline
        // or EOF after a number. If the last number was stored at row R and row wasn't incremented,
        // then row currently is R. We need to compute n accordingly: if the last stored column was at col>1 or we stored any number at row,
        // we want n = row + 1 if the last stored was not followed by a newline that already incremented row.
        // Our parsing increments row on newline and on EOF after a number; so if any_number true and row==0 and we stored to row 0,
        // but didn't increment row, we should set n = row + 1.
        // To be robust, determine n by scanning whether row index points to the next row or current row:
        // We will compute n as the highest row index with any non-zero column or col==1 with zero maybe; simpler: track rows by counting row increments and initial row presence.
        // For simplicity and correctness with spec: if any_number true and row == 0 && (we filled at least one value), set n = 1.
        // More generally, if row == 0 (we never advanced), n = 1. Else n = row.
        if row == 0 {
            1
        } else {
            row
        }
    } else {
        0
    };

    (data, n)
}

fn main() {
    let (data, n) = parse_input();

    // dp initialized to zeros
    let mut dp = [[0i32; MAX_C]; MAX_R];

    // The SPEC defines starting at (0,1).
    // Ensure indices are within bounds; if not, behavior is undefined in SPEC. Here we handle gracefully:
    if n > MAX_R || 1 >= MAX_C {
        // Out of bounds situation: per SPEC behavior is undefined. We'll exit with non-zero.
        exit(1);
    }

    // The judge function reads data[i][j] even when i == n; data is zero-initialized so this matches C globals.
    let ans = judge(0, 1, 0, n, &data, &mut dp);

    // Print the single integer followed by newline exactly.
    println!("{}", ans);
}
