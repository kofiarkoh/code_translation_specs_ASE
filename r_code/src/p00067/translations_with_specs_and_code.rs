// islands.rs
//
// Translated from the provided C program per the SPEC.
// Notes on divergence from the C source:
// - The original C code performed recursive flood-fill without explicit bounds checks,
//   which can cause undefined behavior. The SPEC describes the intended safe behavior
//   (count 4-connected components inside a 12x12 grid). This Rust translation follows
//   the SPEC and uses explicit bounds checks and an iterative stack-based flood-fill
//   to avoid recursion/UB.
// - Input reading uses whitespace-separated tokens to match scanf("%s", ...).
//
// Behavior:
// - Repeatedly consumes 12 tokens (rows) from stdin. Each token must have at least
//   12 characters (as per SPEC). For each 12x12 grid the program prints the number
//   of 4-connected components of '1' (one line per test case).
//
// The program intentionally treats each character's numeric value as (byte - b'0'),
// and a cell is considered part of an island only if that value equals exactly 1,
// matching the SPEC description.

use std::io::{self, Read};

fn main() -> io::Result<()> {
    // Read entire stdin and split on whitespace, matching scanf("%s", ...) behavior.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let tokens: Vec<&str> = input.split_whitespace().collect();

    let mut idx = 0;
    while idx + 12 <= tokens.len() {
        // Prepare grid; store numeric value (byte - b'0') as i32 to mirror C behavior.
        let mut grid = [[0i32; 12]; 12];

        // Validate and fill grid. SPEC requires each token (row) to have at least 12 chars.
        let mut ok = true;
        for r in 0..12 {
            let row_bytes = tokens[idx + r].as_bytes();
            if row_bytes.len() < 12 {
                // Input does not satisfy SPEC expectation; stop processing further.
                ok = false;
                break;
            }
            for c in 0..12 {
                grid[r][c] = (row_bytes[c] as i32) - ('0' as i32);
            }
        }
        if !ok {
            break;
        }

        // Count 4-connected components of cells equal to 1.
        let mut count = 0;
        for r in 0..12 {
            for c in 0..12 {
                if grid[r][c] == 1 {
                    count += 1;
                    // Iterative flood-fill to clear the component (set visited to 0).
                    let mut stack = Vec::with_capacity(144);
                    stack.push((r, c));
                    while let Some((x, y)) = stack.pop() {
                        if grid[x][y] != 1 {
                            continue;
                        }
                        grid[x][y] = 0;
                        if x > 0 {
                            stack.push((x - 1, y));
                        }
                        if x + 1 < 12 {
                            stack.push((x + 1, y));
                        }
                        if y > 0 {
                            stack.push((x, y - 1));
                        }
                        if y + 1 < 12 {
                            stack.push((x, y + 1));
                        }
                    }
                }
            }
        }

        println!("{}", count);
        idx += 12;
    }

    Ok(())
}
