use std::io::{self, Read};

fn main() {
    // Read entire stdin; blank lines are ignored by tokenization.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let x_opt = it.next();
        if x_opt.is_none() {
            break; // unspecified by spec; just stop if no more tokens
        }
        let x: usize = x_opt.unwrap().parse().unwrap();
        let y: usize = it.next().unwrap().parse().unwrap();

        if x == 0 && y == 0 {
            break;
        }

        // course[row][col], but input is provided column-by-column:
        // for each column j, we read X integers for rows 0..X-1.
        let mut course = vec![vec![0i32; y]; x];
        for j in 0..y {
            for i in 0..x {
                course[i][j] = it.next().unwrap().parse().unwrap();
            }
        }

        // pattern[row][col], with two conceptual end columns: y and y+1
        let mut pattern = vec![vec![0i64; y + 2]; x];

        // Initialize start positions: any row in first column with value 0 gets 1.
        for i in 0..x {
            if course[i][0] == 0 {
                pattern[i][0] = 1;
            }
        }

        // Transitions for columns 0..y-2 (since we look at j+1).
        if y >= 2 {
            for j in 0..(y - 1) {
                for i in 0..x {
                    let cur = pattern[i][j];
                    if cur == 0 {
                        // Not required for correctness, but avoids extra work.
                        continue;
                    }
                    // k in {i-1, i, i+1} within bounds
                    for dk in [-1isize, 0, 1] {
                        let k_isize = i as isize + dk;
                        if k_isize < 0 || k_isize >= x as isize {
                            continue;
                        }
                        let k = k_isize as usize;
                        match course[k][j + 1] {
                            0 => {
                                pattern[k][j + 1] += cur;
                            }
                            1 => {
                                // Overwrite to 0, matching C semantics.
                                pattern[k][j + 1] = 0;
                            }
                            _ => {
                                // Only possible when staying in the same row.
                                if k == i {
                                    let mut m: usize = 0;
                                    while j + 3 + m < y {
                                        if course[k][j + 3 + m] == 2 {
                                            m += 2;
                                        } else {
                                            break;
                                        }
                                    }
                                    // Target column may be within [0..y+1] (two conceptual end columns).
                                    let target = j + 3 + m;
                                    if target < y + 2 {
                                        pattern[k][target] += cur;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        // Sum over last real column (y-1) and the two conceptual end columns (y, y+1).
        let mut sum: i64 = 0;
        for i in 0..x {
            // Note: y is non-zero for non-terminating scenarios per spec's well-formedness.
            for col in (y - 1)..=(y + 1) {
                sum += pattern[i][col];
            }
        }

        out.push_str(&format!("{sum}\n"));
    }

    print!("{out}");
}
