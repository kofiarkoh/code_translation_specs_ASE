use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    // Specification is incomplete/ambiguous about:
    // - what constitutes a "marked" cell,
    // - the "specific movement rules",
    // - how adjacency is defined (4-neighbor vs 8-neighbor),
    // - which cells are considered "unmarked" for counting.
    //
    // Narrowest reasonable interpretation justified by the text:
    // - Treat non-zero input cells as initially "marked".
    // - No additional marking via movement rules can be derived, so we do not
    //   invent iterative marking beyond the given grid values.
    // - Define adjacency as 4-neighbor (up/down/left/right), the most common
    //   grid adjacency in such problems.
    // - Count cells that are unmarked (value == 0) and adjacent to at least one
    //   marked cell (value != 0).
    //
    // Input format per spec:
    // width, height, then (height + 2) * (width * 2 + 4) integers.

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let width: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let height: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    let rows = height + 2;
    let cols = width * 2 + 4;

    let mut grid: Vec<Vec<i64>> = vec![vec![0; cols]; rows];
    for r in 0..rows {
        for c in 0..cols {
            if let Some(tok) = it.next() {
                grid[r][c] = tok.parse::<i64>().unwrap();
            } else {
                // If input is incomplete, spec says behavior may not be as expected.
                // We keep remaining cells as 0.
                grid[r][c] = 0;
            }
        }
    }

    // Count unmarked cells adjacent to marked cells.
    // Use a queue to avoid repeated neighbor checks? Not necessary, but we can
    // compute directly in O(rows*cols).
    let mut count: i64 = 0;

    // 4-neighborhood
    let dr = [-1i32, 1, 0, 0];
    let dc = [0i32, 0, -1, 1];

    for r in 0..rows {
        for c in 0..cols {
            if grid[r][c] != 0 {
                continue; // only count unmarked
            }
            let mut adjacent_to_marked = false;
            for k in 0..4 {
                let nr = r as i32 + dr[k];
                let nc = c as i32 + dc[k];
                if nr >= 0 && nr < rows as i32 && nc >= 0 && nc < cols as i32 {
                    if grid[nr as usize][nc as usize] != 0 {
                        adjacent_to_marked = true;
                        break;
                    }
                }
            }
            if adjacent_to_marked {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
