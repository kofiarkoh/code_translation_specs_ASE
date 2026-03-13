// translated_farm.rs
//
// Translation of the provided C program to Rust following the specification.
// Behavior: For each test case (H W followed by H whitespace-delimited strings),
// counts 4-connected components of equal characters, marking visited cells as '+'
// in the in-memory grid, and prints the component count per test case.
// Terminates when a test case "0 0" is encountered.
//
// Notes:
// - Assumes well-formed input as per specification: each of the H row tokens has
//   at least W characters and 0 <= H,W <= MAX (100).
// - Uses an explicit stack (iterative DFS) instead of recursion to avoid potential
//   deep recursion and stack overflow while preserving DFS semantics.
// - No unsafe code is used.

use std::io::{self, Read};

const MAX: usize = 100;

fn main() {
    // Read entire stdin and split on whitespace to emulate scanf("%d %d" and "%s")
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("failed to read stdin");
    let mut tokens = input.split_whitespace();

    loop {
        // Read H and W
        let h_tok = match tokens.next() {
            Some(t) => t,
            None => break, // EOF or incomplete input
        };
        let w_tok = match tokens.next() {
            Some(t) => t,
            None => break,
        };

        let h: i32 = match h_tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };
        let w: i32 = match w_tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        if h == 0 && w == 0 {
            break; // termination case, produce no output
        }

        // According to the spec, H and W are non-negative and <= MAX.
        // We follow the spec's assumption (undefined input otherwise).
        let H = h as usize;
        let W = w as usize;

        // Defensive check: if H or W exceed MAX, behavior is outside spec; here we
        // choose to clamp to MAX to avoid panics, but this should not be needed
        // for well-formed inputs. (Alternatively, could panic/assert.)
        if H > MAX || W > MAX {
            // Panic to reflect that input violates preconditions.
            panic!("H or W exceeds MAX ({})", MAX);
        }

        // Read H row tokens
        let mut grid: Vec<Vec<char>> = Vec::with_capacity(H);
        for _ in 0..H {
            let row_tok = match tokens.next() {
                Some(t) => t,
                None => {
                    // Incomplete input; stop processing.
                    return;
                }
            };
            // As per spec, each row token must have at least W characters.
            if row_tok.chars().count() < W {
                // Input violates preconditions; behavior undefined per spec.
                // Abort with panic to avoid silent incorrect behavior.
                panic!("Row string has fewer than W characters");
            }
            // Take the first W characters for the row.
            let row: Vec<char> = row_tok.chars().take(W).collect();
            grid.push(row);
        }

        // Component counting
        let mut cnt = 0usize;

        // Directions for 4-neighborhood: right, down, left, up
        let neighbors = [(0_i32, 1_i32), (1, 0), (0, -1), (-1, 0)];

        for i in 0..H {
            for j in 0..W {
                if grid[i][j] == '+' {
                    continue;
                }
                // Start a DFS flood-fill from (i, j) for character target
                let target = grid[i][j];
                cnt += 1;

                // Iterative stack of (row, col) as usize
                let mut stack: Vec<(usize, usize)> = Vec::new();
                stack.push((i, j));

                while let Some((r, c)) = stack.pop() {
                    // Check current cell matches target (it may have been changed)
                    if grid[r][c] != target {
                        continue;
                    }
                    // Mark visited
                    grid[r][c] = '+';

                    // Push neighbors
                    for &(dr, dc) in &neighbors {
                        let nr_i32 = r as i32 + dr;
                        let nc_i32 = c as i32 + dc;
                        if nr_i32 >= 0 && nr_i32 < H as i32 && nc_i32 >= 0 && nc_i32 < W as i32 {
                            let nr = nr_i32 as usize;
                            let nc = nc_i32 as usize;
                            if grid[nr][nc] == target {
                                // Will be processed (and marked) later
                                stack.push((nr, nc));
                            }
                        }
                    }
                }
            }
        }

        // Print the count for this test case
        println!("{}", cnt);
    }
}
