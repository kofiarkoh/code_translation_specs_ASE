use std::io::{self, Read};

const MAX_ROWS: usize = 99;
const MAX_COLS: usize = 50;

fn main() {
    // Read entire stdin
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read error, exit printing 0 (narrow, safe fallback).
        println!("0");
        return;
    }

    // Prepare storage: grid with fixed column capacity and row lengths.
    // grid[row][col] holds an i8 (signed char semantics).
    let mut grid: Vec<[i8; MAX_COLS]> = Vec::new();
    let mut row_lens: Vec<usize> = Vec::new();

    for line in input.lines() {
        // Skip lines that are entirely whitespace (robust against trailing newlines).
        if line.trim().is_empty() {
            continue;
        }
        if grid.len() >= MAX_ROWS {
            // Cap rows at MAX_ROWS; additional rows ignored to stay within capacity.
            break;
        }

        let mut row = [0i8; MAX_COLS];
        let mut col = 0usize;
        for token in line.split(',') {
            if col >= MAX_COLS {
                // Cap columns at MAX_COLS; extra tokens are ignored.
                break;
            }
            let t = token.trim();
            if t.is_empty() {
                // Malformed/empty token: skip it (best-effort; spec leaves such cases undefined).
                continue;
            }
            // Parse as i32 then cast to i8 truncating modulo 256 to emulate C signed char behavior.
            // The spec assumes tokens fit into signed char; behavior for out-of-range is undefined,
            // so truncation is a reasonable low-level emulation.
            let parsed = t.parse::<i32>().unwrap_or(0);
            row[col] = parsed as i8;
            col += 1;
        }
        // Only consider non-empty rows (with at least one parsed column).
        // If a row had zero columns, we still record it with length 0; behavior is undefined per spec,
        // but we keep consistent internal representation.
        row_lens.push(col);
        grid.push(row);
    }

    let n = grid.len();

    // If no rows, behavior is ambiguous in the spec. We choose to print 0.
    if n == 0 {
        println!("0");
        return;
    }

    // DP table: for each cell store the maximum prefix-sum observed at that cell.
    // Initialize to i32::MIN to represent "unvisited".
    let mut dp: Vec<[i32; MAX_COLS]> = vec![[i32::MIN; MAX_COLS]; n];

    // Best result found; initialize to i32::MIN to detect if any path was found.
    let mut best = i32::MIN;

    // Midpoint rule: rows strictly before n/2 use (i+1,j) or (i+1,j+1).
    // Rows at or after n/2 use (i+1,j-1) or (i+1,j).
    let midpoint = n / 2;

    // Recursive DFS closure. We implement as a normal function via nested function.
    fn dfs(
        i: usize,
        j: usize,
        acc_before: i32,
        n: usize,
        midpoint: usize,
        grid: &Vec<[i8; MAX_COLS]>,
        row_lens: &Vec<usize>,
        dp: &mut Vec<[i32; MAX_COLS]>,
        best_ref: &mut i32,
    ) {
        // Bounds checks: ensure row i is valid and column j is within that row's length.
        if i >= n {
            // Shouldn't be called with i >= n in this implementation.
            return;
        }
        if j >= MAX_COLS {
            return;
        }
        let row_len = row_lens[i];
        if j >= row_len {
            // Accessing a column beyond parsed length is undefined per spec.
            // To stay safe and avoid panic, prune this path.
            return;
        }

        // Value at current cell, stored as i8; cast to i32 for arithmetic.
        let v = grid[i][j] as i32;
        let new_acc = acc_before + v;

        // Prune if we've already observed an equal-or-better prefix sum at (i,j).
        if dp[i][j] >= new_acc {
            return;
        }

        // Update dp entry to the new (strictly greater) prefix sum.
        dp[i][j] = new_acc;

        // If moving to next row would be beyond last row, record the accumulated sum as a candidate.
        if i + 1 == n {
            if new_acc > *best_ref {
                *best_ref = new_acc;
            }
            return;
        }

        // Determine allowed moves based on midpoint rule.
        if i < midpoint {
            // Can move to (i+1, j) or (i+1, j+1)
            let ni = i + 1;

            // Move to (ni, j)
            if j < row_lens[ni] {
                dfs(ni, j, new_acc, n, midpoint, grid, row_lens, dp, best_ref);
            } else {
                // If the target column is beyond parsed length, skip (undefined in spec).
            }

            // Move to (ni, j+1)
            let j1 = j + 1;
            if j1 < MAX_COLS && j1 < row_lens[ni] {
                dfs(ni, j1, new_acc, n, midpoint, grid, row_lens, dp, best_ref);
            }
        } else {
            // i >= midpoint: Can move to (i+1, j-1) or (i+1, j)
            let ni = i + 1;

            // Move to (ni, j-1)
            if j >= 1 {
                let jm1 = j - 1;
                if jm1 < row_lens[ni] {
                    dfs(ni, jm1, new_acc, n, midpoint, grid, row_lens, dp, best_ref);
                }
            }

            // Move to (ni, j)
            if j < row_lens[ni] {
                dfs(ni, j, new_acc, n, midpoint, grid, row_lens, dp, best_ref);
            }
        }
    }

    // Starting position is (0,1) as specified.
    // Check that column 1 exists in row 0.
    if row_lens[0] <= 1 {
        // Starting position invalid per preconditions; behavior undefined.
        // We choose to print 0 in this case.
        println!("0");
        return;
    }

    // Start DFS with acc_before = 0; the first visited cell's value will be added inside dfs.
    dfs(
        0,
        1,
        0,
        n,
        midpoint,
        &grid,
        &row_lens,
        &mut dp,
        &mut best,
    );

    // If no path was found (best still i32::MIN), print 0 as a conservative fallback.
    if best == i32::MIN {
        println!("0");
    } else {
        println!("{}", best);
    }
}
