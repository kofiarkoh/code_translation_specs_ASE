// component_counter.rs
//
// Translation of the given specification to Rust.
// - Reads test cases from stdin: H W followed by H whitespace-delimited strings (rows).
// - For each test case (H,W not both zero) counts 4-connected components of equal characters.
// - During flood-fill visited cells are replaced with '+' (byte 43).
// - Stops processing when a test case with H == 0 && W == 0 is encountered.
//
// Notes on fidelity to the specification:
// - Uses a fixed-size grid of MAX x MAX (100 x 100) as in the spec.
// - Rows are taken from whitespace-delimited tokens (equivalent to scanf("%s")).
// - Assumes each row token has at least W characters (the spec declares behavior undefined otherwise).
// - Flood-fill implemented recursively to mirror the described behavior.
// - Uses bytes (u8) to model C char semantics (simple equality on single-byte characters).
//
// This file is standalone and compiles with stable Rust.

use std::io::{self, Read};

const MAX: usize = 100;
const VISITED: u8 = b'+'; // '+' char as byte

fn main() {
    // Read all stdin and split by whitespace tokens (matches scanf("%s") behavior).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");
    let mut tokens = input.split_whitespace();

    // Static grid storage as in the specification: MAX x MAX bytes.
    // Initialize with zero bytes; only the H x W submatrix is considered per test case.
    let mut grid: [[u8; MAX]; MAX] = [[0u8; MAX]; MAX];

    loop {
        // Read H and W; if tokens exhausted, stop.
        let h_token = match tokens.next() {
            Some(t) => t,
            None => break,
        };
        let w_token = match tokens.next() {
            Some(t) => t,
            None => break,
        };

        // Parse integers. The specification assumes well-formed input.
        let h: usize = match h_token.parse() {
            Ok(n) => n,
            Err(_) => break, // follow the "assume well-formed" idea; stop on parse failure
        };
        let w: usize = match w_token.parse() {
            Ok(n) => n,
            Err(_) => break,
        };

        // Termination case: both zero -> stop without output.
        if h == 0 && w == 0 {
            break;
        }

        // According to the specification, H and W are expected to be <= MAX (100).
        // The original C program would exhibit undefined behavior if they exceeded MAX.
        // Here we will enforce the MAX bound to avoid panic by safe indexing.
        // If inputs violate the specification, behavior is not defined by the spec;
        // we simply cap at MAX to keep memory accesses valid.
        //
        // NOTE: This slightly deviates from "undefined" behavior in C, but is safer.
        let h = if h > MAX { MAX } else { h };
        let w = if w > MAX { MAX } else { w };

        // Read H row tokens (whitespace-delimited). The spec expects each token to be at least W chars.
        for i in 0..h {
            let row_token = match tokens.next() {
                Some(t) => t,
                None => {
                    // Token missing: stop processing further (mirrors unspecified behavior on malformed input).
                    // We exit the program since input is incomplete.
                    return;
                }
            };
            let bytes = row_token.as_bytes();
            // Copy first W characters into the grid row.
            // The spec assumes bytes.len() >= W; if not, we copy what we can and leave remaining bytes as 0.
            for j in 0..w {
                grid[i][j] = if j < bytes.len() { bytes[j] } else { 0u8 };
            }
        }

        // Flood-fill (recursive) implementation.
        // Define a recursive closure using a named function to avoid lifetime issues.
        fn flood_fill(grid: &mut [[u8; MAX]; MAX], h: usize, w: usize, i: isize, j: isize, ch: u8) {
            // bounds check
            if i < 0 || j < 0 {
                return;
            }
            let (ui, uj) = (i as usize, j as usize);
            if ui >= h || uj >= w {
                return;
            }
            if grid[ui][uj] != ch {
                return;
            }
            // mark visited
            grid[ui][uj] = VISITED;
            // four directions
            flood_fill(grid, h, w, i - 1, j, ch);
            flood_fill(grid, h, w, i + 1, j, ch);
            flood_fill(grid, h, w, i, j - 1, ch);
            flood_fill(grid, h, w, i, j + 1, ch);
        }

        // Count components
        let mut count = 0usize;
        for i in 0..h {
            for j in 0..w {
                let cell = grid[i][j];
                if cell == VISITED {
                    continue; // already visited
                }
                // Start a flood-fill from this cell (even if cell==0 from malformed shorter input, we follow stored value)
                // But per spec, '+' cells are skipped; we've already checked that.
                count += 1;
                flood_fill(&mut grid, h, w, i as isize, j as isize, cell);
            }
        }

        // Print the count for this test case followed by newline.
        println!("{}", count);
        // By spec, the in-memory grid retains '+' marks for visited cells.
        // The next test case will overwrite the H x W submatrix when reading rows.
    }
}
