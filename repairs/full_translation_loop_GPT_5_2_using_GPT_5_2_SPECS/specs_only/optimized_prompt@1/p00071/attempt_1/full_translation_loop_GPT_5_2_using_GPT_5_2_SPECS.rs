use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    // Read entire stdin; no explicit error handling per specification.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let t: usize = it.next().unwrap().parse().unwrap();

    for case_idx in 1..=t {
        // Read 8 lines (as tokens) representing the 8x8 grid.
        // Each token must contain at least 8 characters; extra characters are ignored.
        let mut grid = [['0'; 8]; 8];
        for r in 0..8 {
            let line = it.next().unwrap();
            let mut chars = line.chars();
            for c in 0..8 {
                grid[r][c] = chars.next().unwrap();
            }
        }

        // Coordinates are 1..=8, with x as column and y as row.
        let x0: usize = it.next().unwrap().parse().unwrap();
        let y0: usize = it.next().unwrap().parse().unwrap();
        let sx = x0 - 1;
        let sy = y0 - 1;

        // Explosion process:
        // - Always clears the starting cell to '0' regardless of prior value.
        // - From any cleared cell, consider moves of 1..=3 steps in 4 cardinal directions.
        // - If the landing cell currently contains '1', clear it and continue from there.
        //
        // Use an explicit queue to avoid recursion depth concerns while preserving semantics.
        let mut q = VecDeque::new();
        grid[sy][sx] = '0';
        q.push_back((sx, sy));

        while let Some((x, y)) = q.pop_front() {
            // For each direction, try step lengths 1..=3, staying within bounds.
            // Only landing cells are tested/cleared; intermediate cells are irrelevant.
            // Right
            for d in 1..=3 {
                let nx = x + d;
                if nx >= 8 {
                    break;
                }
                if grid[y][nx] == '1' {
                    grid[y][nx] = '0';
                    q.push_back((nx, y));
                }
            }
            // Left
            for d in 1..=3 {
                if x < d {
                    break;
                }
                let nx = x - d;
                if grid[y][nx] == '1' {
                    grid[y][nx] = '0';
                    q.push_back((nx, y));
                }
            }
            // Down
            for d in 1..=3 {
                let ny = y + d;
                if ny >= 8 {
                    break;
                }
                if grid[ny][x] == '1' {
                    grid[ny][x] = '0';
                    q.push_back((x, ny));
                }
            }
            // Up
            for d in 1..=3 {
                if y < d {
                    break;
                }
                let ny = y - d;
                if grid[ny][x] == '1' {
                    grid[ny][x] = '0';
                    q.push_back((x, ny));
                }
            }
        }

        // Output format: header identifying dataset number, then 8 lines of the grid.
        // The specification does not mandate exact header text; choose a minimal, clear label.
        println!("Data Set {}", case_idx);
        for r in 0..8 {
            let line: String = grid[r].iter().collect();
            println!("{}", line);
        }
    }
}
