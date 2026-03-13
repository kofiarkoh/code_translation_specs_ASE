// islands.rs
//
// Translated from the provided Specification.
// Reads successive test cases from stdin. Each test case consists of 12
// contiguous non-whitespace tokens (rows), each expected to have at least
// 12 characters. For each test case, prints the number of 4-connected
// components of '1' characters (orthogonal connectivity).
//
// Implementation notes (kept close to the Spec):
// - Input is consumed as whitespace-separated tokens (emulating scanf("%s", ...)).
// - For each token (row) only the first 12 characters are used.
// - Characters are converted by subtracting '0' (implemented as wrapping subtraction
//   on the byte value); only cells with value exactly 1 are considered island cells.
// - A recursive flood-fill clears visited '1' cells to 0 so they are not counted again.
// - If the input does not contain a whole number of complete 12-row test cases
//   (i.e., remaining tokens < 12), those trailing tokens are ignored (loop stops).
//   This choice avoids panics while staying narrowly faithful to the Spec's expectation
//   that input is organized into full 12-line test cases.
// - The flood-fill performs bounds checks before accessing the grid to avoid actual
//   undefined behavior; this preserves intended behavior for valid inputs while
//   avoiding unsafe memory access in Rust.
//
// This program returns 0 on normal termination (default for Rust main when it completes).

use std::io::{self, Read};

fn main() {
    // Read all stdin into a string
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read error, exit early (no output). This mirrors that the Spec does not
        // define special error handling.
        return;
    }

    // Split on whitespace to emulate scanf("%s", ...)
    let tokens: Vec<&str> = input.split_whitespace().collect();

    // Process complete groups of 12 tokens (rows) per test case.
    let mut idx = 0usize;
    while idx + 12 <= tokens.len() {
        // Prepare 12x12 grid of u8 values
        let mut grid: [[u8; 12]; 12] = [[0u8; 12]; 12];
        let mut valid = true;

        for r in 0..12 {
            let row = tokens[idx + r];
            let bytes = row.as_bytes();
            // If a token/row is shorter than 12 characters, we stop processing further test cases.
            // The Spec considers such situations as potentially undefined; here we choose to stop.
            if bytes.len() < 12 {
                valid = false;
                break;
            }
            for c in 0..12 {
                // Convert by subtracting '0' (wrapping_sub to emulate simple subtraction on bytes)
                grid[r][c] = bytes[c].wrapping_sub(b'0');
            }
        }

        if !valid {
            break;
        }

        // Flood-fill function: recursive, clears visited '1's to 0.
        fn flood(grid: &mut [[u8; 12]; 12], i: isize, j: isize) {
            // Bounds checks to avoid Rust panics / UB.
            if i < 0 || i >= 12 || j < 0 || j >= 12 {
                return;
            }
            let (ui, uj) = (i as usize, j as usize);
            if grid[ui][uj] != 1 {
                return;
            }
            grid[ui][uj] = 0;
            // Recurse in 4 orthogonal directions
            flood(grid, i - 1, j);
            flood(grid, i + 1, j);
            flood(grid, i, j - 1);
            flood(grid, i, j + 1);
        }

        // Count islands
        let mut count = 0u32;
        for r in 0..12 {
            for c in 0..12 {
                if grid[r][c] == 1 {
                    count += 1;
                    flood(&mut grid, r as isize, c as isize);
                }
            }
        }

        // Print result for this test case
        println!("{}", count);

        idx += 12;
    }

    // Normal termination: return code 0 implicitly.
}
