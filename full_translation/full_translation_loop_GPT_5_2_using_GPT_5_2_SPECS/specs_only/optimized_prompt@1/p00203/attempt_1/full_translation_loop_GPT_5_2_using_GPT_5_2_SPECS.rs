use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    loop {
        let rows: usize = match it.next() {
            Some(t) => t.parse().unwrap(),
            None => break,
        };
        let cols: usize = it.next().unwrap().parse().unwrap();

        if rows == 0 && cols == 0 {
            break;
        }

        // Grid is provided column-by-column: for each column, one integer per row.
        // Store as grid[row][col].
        let mut grid = vec![vec![0i32; cols]; rows];
        for c in 0..cols {
            for r in 0..rows {
                grid[r][c] = it.next().unwrap().parse().unwrap();
            }
        }

        // Path counts per row and per column position, including two conceptual end columns.
        // Columns: 0..cols-1 are real, cols and cols+1 are conceptual end columns.
        let mut dp = vec![vec![0i64; cols + 2]; rows];

        // Start positions: any row in first column with value 0 contributes one initial path.
        if cols > 0 {
            for r in 0..rows {
                if grid[r][0] == 0 {
                    dp[r][0] = 1;
                }
            }
        }

        // Process transitions from each real column.
        for c in 0..cols {
            for r in 0..rows {
                let count = dp[r][c];
                if count == 0 {
                    continue;
                }

                // Transitions are attempted to column c+1 in rows r-1, r, r+1 (within bounds).
                // If c is the last real column, c+1 is the first conceptual end column.
                let next_col = c + 1;

                for dr in [-1i32, 0, 1] {
                    let rr_i = r as i32 + dr;
                    if rr_i < 0 || rr_i >= rows as i32 {
                        continue;
                    }
                    let rr = rr_i as usize;

                    if next_col < cols {
                        let v = grid[rr][next_col];
                        if v == 0 {
                            dp[rr][next_col] = dp[rr][next_col].wrapping_add(count);
                        } else if v == 1 {
                            dp[rr][next_col] = 0;
                        } else {
                            // Non-0/1: transition only possible when staying in same row.
                            if rr != r {
                                continue;
                            }

                            // Skipping forward starting from c+3, repeatedly advancing by 2
                            // for each consecutive cell with value 2 encountered at those checked positions.
                            // Stop at first checked position that is not 2 or when the check would exceed
                            // the last column. The resulting target column may be within the grid or in
                            // one of the two conceptual end columns.
                            let mut target = c + 3;
                            while target < cols && grid[r][target] == 2 {
                                target += 2;
                            }

                            if target < cols + 2 {
                                dp[r][target] = dp[r][target].wrapping_add(count);
                            }
                        }
                    } else {
                        // Destination is in conceptual end region (no cell-type rules apply there).
                        if next_col < cols + 2 {
                            dp[rr][next_col] = dp[rr][next_col].wrapping_add(count);
                        }
                    }
                }
            }
        }

        // Output is the sum over all rows of counts that end in:
        // - the last column, or
        // - one of two conceptual end columns immediately after the last column.
        let mut total: i64 = 0;
        if cols > 0 {
            let last = cols - 1;
            for r in 0..rows {
                total = total.wrapping_add(dp[r][last]);
                total = total.wrapping_add(dp[r][cols]);
                total = total.wrapping_add(dp[r][cols + 1]);
            }
        } else {
            // With zero columns, there is no first column to start from; thus no paths.
            // (Narrowest reasonable interpretation consistent with start-position rule.)
            total = 0;
        }

        println!("{}", total);
    }
}
