use std::io::{self, Read};

fn main() {
    // Read entire stdin (matches typical competitive-programming style; no extra output).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let h: usize = it.next().unwrap().parse().unwrap();
    let w: usize = it.next().unwrap().parse().unwrap();

    // SPEC: fixed maximum size (H<=51, W<=60) is assumed; no explicit error handling.
    // Store as bytes for O(1) indexing.
    let mut grid: Vec<Vec<u8>> = Vec::with_capacity(h);
    for _ in 0..h {
        let row = it.next().unwrap().as_bytes().to_vec();
        grid.push(row);
    }

    // Check: every '#' must have at least one adjacent '#' (up/down/left/right).
    for i in 0..h {
        for j in 0..w {
            if grid[i][j] != b'#' {
                continue;
            }

            let mut ok = false;
            if i > 0 && grid[i - 1][j] == b'#' {
                ok = true;
            }
            if i + 1 < h && grid[i + 1][j] == b'#' {
                ok = true;
            }
            if j > 0 && grid[i][j - 1] == b'#' {
                ok = true;
            }
            if j + 1 < w && grid[i][j + 1] == b'#' {
                ok = true;
            }

            if !ok {
                // Output formatting: exactly "No" (no newline), matching the C code.
                print!("No");
                return;
            }
        }
    }

    // Output formatting: exactly "Yes" (no newline), matching the C code.
    print!("Yes");
}
