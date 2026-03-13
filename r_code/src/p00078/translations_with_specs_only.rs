use std::io::{self, Read};

fn main() {
    // Read all stdin and split into whitespace-separated tokens, akin to repeated scanf("%d", &n)
    let mut input = String::new();
    let _ = io::stdin().read_to_string(&mut input);

    // Parse tokens as i32. If parsing of a particular token fails, we skip it.
    // The Specification assumes tokens are valid ints; we follow that assumption.
    let mut tokens = input
        .split_whitespace()
        .filter_map(|s| s.parse::<i32>().ok())
        .into_iter();

    // Fixed backing store: 16x16 as specified.
    // We use i32 to match C int semantics for the small ranges used.
    let mut buf: [[i32; 16]; 16] = [[0; 16]; 16];

    while let Some(n) = tokens.next() {
        // Terminate on zero as per specification.
        if n == 0 {
            break;
        }

        // Treat non-zero n as the logical size. The spec expects 1 <= n <= 16 for safe execution.
        // We convert to usize for indexing; per spec behavior for other values is unspecified.
        let n_usize = n as usize;

        // Clear only the logical n x n portion of the fixed buffer.
        // This mirrors allocating a fresh n x n logical matrix backed by the fixed storage.
        for r in 0..n_usize {
            for c in 0..n_usize {
                // This will panic if n_usize > 16 (mirroring out-of-bounds behavior in C,
                // although Rust panics instead of undefined behavior).
                buf[r][c] = 0;
            }
        }

        // Placement algorithm: the conventional "Siamese" method for odd n,
        // which is the reasonable deterministic algorithm intended by the Specification.
        // Start at row = 0, col = n/2.
        let mut row = 0usize;
        let mut col = n_usize / 2;

        // Fill values 1..=n^2
        let max_k = n_usize.checked_mul(n_usize).unwrap_or(0);
        for k in 1..=max_k {
            // Place k at (row, col)
            buf[row][col] = k as i32;

            // Compute candidate next position: up one row, right one column (with wrap)
            let next_row = if row == 0 { n_usize - 1 } else { row - 1 };
            let next_col = (col + 1) % n_usize;

            // If the candidate cell is already occupied, move down one from current position instead.
            if buf[next_row][next_col] != 0 {
                row = (row + 1) % n_usize;
                // col stays the same
            } else {
                row = next_row;
                col = next_col;
            }
        }

        // Print the logical n x n matrix, each entry in a width-4 field ("%4d"), rows on separate lines.
        for r in 0..n_usize {
            for c in 0..n_usize {
                // As in the C specification, no extra delimiters beyond the fixed-width fields.
                print!("{:4}", buf[r][c]);
            }
            println!();
        }
    }

    // Exit with status 0 implicitly by returning from main.
}
