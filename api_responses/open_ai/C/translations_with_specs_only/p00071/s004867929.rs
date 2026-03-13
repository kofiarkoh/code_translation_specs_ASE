use std::io::{self, Read};

fn main() {
    // Read entire stdin into a string and split on whitespace.
    // The specification guarantees well-formed input (so we keep parsing simple).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut toks = input.split_whitespace();

    // Parse number of datasets n
    let n: usize = toks
        .next()
        .expect("expected number of datasets")
        .parse()
        .expect("invalid integer for n");

    for i in 0..n {
        // Read 8 grid lines, each an 8-character string of '0'/'1'
        let mut grid: [[u8; 8]; 8] = [[0u8; 8]; 8];
        for r in 0..8 {
            let line = toks
                .next()
                .expect("expected grid line")
                .as_bytes()
                .to_vec();
            // Specification demands lines are exactly length 8 composed of '0'/'1'.
            // We assume this and index accordingly.
            debug_assert!(
                line.len() == 8,
                "grid line length not 8 as specification requires"
            );
            for c in 0..8 {
                grid[r][c] = match line[c] {
                    b'0' => 0,
                    b'1' => 1,
                    other => {
                        // Input is assumed valid per specification; treat unexpected as error.
                        panic!("unexpected character in grid: {}", other);
                    }
                };
            }
        }

        // Read x and y: first integer is column (1-based), second is row (1-based).
        let x_int: isize = toks
            .next()
            .expect("expected x")
            .parse()
            .expect("invalid integer for x");
        let y_int: isize = toks
            .next()
            .expect("expected y")
            .parse()
            .expect("invalid integer for y");

        // Convert to 0-based indices: row = y-1, col = x-1
        let start_row = (y_int - 1) as isize;
        let start_col = (x_int - 1) as isize;

        // Per specification, coordinates are expected to be in 1..8. We assume valid input.
        // Seed the recursion/propagation from the start cell. Even if it is already 0,
        // we still "mark" it as cleared (leave as 0) and inspect neighbours.
        // Use an explicit stack to avoid deep recursion.
        let mut stack: Vec<(usize, usize)> = Vec::new();

        // Convert start indices to usize for stack/pushing. This will panic if indices out of range,
        // which mirrors the unspecified behavior for invalid inputs described in the specification.
        let sr = start_row as usize;
        let sc = start_col as usize;

        // Set the starting cell to 0 (no-op if already 0) and push to stack.
        grid[sr][sc] = 0;
        stack.push((sr, sc));

        // Directions: right, left, down, up
        let directions: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        while let Some((r_usize, c_usize)) = stack.pop() {
            // Use isize for arithmetic to handle negative checks easily.
            let r = r_usize as isize;
            let c = c_usize as isize;

            for &(dr, dc) in &directions {
                for d in 1..=3 {
                    let nr = r + dr * d;
                    let nc = c + dc * d;
                    if nr < 0 || nr >= 8 || nc < 0 || nc >= 8 {
                        continue;
                    }
                    let nr_us = nr as usize;
                    let nc_us = nc as usize;
                    // If the cell has value 1, clear it (set to 0) and continue recursion from it.
                    if grid[nr_us][nc_us] == 1 {
                        grid[nr_us][nc_us] = 0;
                        stack.push((nr_us, nc_us));
                    }
                    // If the cell is 0, per specification we do NOT recurse from it,
                    // but we still continue checking further distances (propagation can hop over zeros).
                }
            }
        }

        // Output for this dataset
        println!("Data {}:", i + 1);
        for r in 0..8 {
            let mut row_str = String::with_capacity(8);
            for c in 0..8 {
                // Convert 0/1 to '0'/'1'
                let ch = if grid[r][c] == 0 { '0' } else { '1' };
                row_str.push(ch);
            }
            println!("{}", row_str);
        }
    }
}
