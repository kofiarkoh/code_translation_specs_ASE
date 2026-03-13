use std::io::{self, BufRead};

fn main() {
    // 10x10 grid initialized to zero
    let mut grid = [[0i32; 10]; 10];

    let stdin = io::stdin();
    // Read line-by-line. The specification describes C-like behavior where input
    // lines are expected to match "%d,%d,%d" and malformed input is not handled.
    // We therefore stop processing further lines if a line does not parse as three integers.
    for line_res in stdin.lock().lines() {
        let line = match line_res {
            Ok(l) => l,
            Err(_) => break, // I/O error: stop processing (narrow interpretation)
        };
        let s = line.trim();
        if s.is_empty() {
            // Treat empty line as malformed / termination (consistent with narrow interpretation).
            break;
        }

        // Split by comma and expect exactly three tokens.
        let parts: Vec<&str> = s.split(',').collect();
        if parts.len() != 3 {
            break;
        }

        // Parse the three integers. If any fail, stop processing further input.
        let px = parts[0].trim().parse::<i32>();
        let py = parts[1].trim().parse::<i32>();
        let ps = parts[2].trim().parse::<i32>();
        let (x, y, sval) = match (px, py, ps) {
            (Ok(a), Ok(b), Ok(c)) => (a, b, c),
            _ => break,
        };

        // Helper to increment a cell if it's within 0..10 (both row and col).
        let mut incr_if_in_bounds = |nx: i32, ny: i32| {
            if (0..10).contains(&nx) && (0..10).contains(&ny) {
                // safe to convert to usize after bounds check
                grid[nx as usize][ny as usize] += 1;
            }
        };

        // Apply effects according to s, with fall-through semantics:
        // s >= 1 => orthogonal neighbors at distance 1
        // s >= 2 => diagonal neighbors (x±1,y±1)
        // s >= 3 => orthogonal neighbors at distance 2
        if sval == 1 || sval == 2 || sval == 3 {
            // s >= 1 effects
            if sval >= 1 {
                incr_if_in_bounds(x - 1, y);
                incr_if_in_bounds(x + 1, y);
                incr_if_in_bounds(x, y - 1);
                incr_if_in_bounds(x, y + 1);
            }
            // s >= 2 effects (diagonals)
            if sval >= 2 {
                incr_if_in_bounds(x - 1, y - 1);
                incr_if_in_bounds(x - 1, y + 1);
                incr_if_in_bounds(x + 1, y - 1);
                incr_if_in_bounds(x + 1, y + 1);
            }
            // s >= 3 effects (orthogonal at distance 2)
            if sval >= 3 {
                incr_if_in_bounds(x - 2, y);
                incr_if_in_bounds(x + 2, y);
                incr_if_in_bounds(x, y - 2);
                incr_if_in_bounds(x, y + 2);
            }
        } else {
            // s not in {1,2,3} => no-op per specification
        }
    }

    // After processing all droplets, compute outputs.
    let mut zero_count: usize = 0;
    let mut max_val: i32 = 0;
    for row in &grid {
        for &cell in row {
            if cell == 0 {
                zero_count += 1;
            }
            if cell > max_val {
                max_val = cell;
            }
        }
    }

    // Print two lines: number of zero cells, then maximum cell value.
    println!("{}", zero_count);
    println!("{}", max_val);
}
