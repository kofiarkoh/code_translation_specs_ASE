use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-separated tokens (like repeated scanf calls).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut toks = input.split_whitespace();

    // Loop like: while (scanf("%d",&n) == 1) { if (n==0) break; ... }
    while let Some(n_tok) = toks.next() {
        let n_usize: usize = match n_tok.parse() {
            Ok(x) => x,
            Err(_) => break,
        };
        if n_usize == 0 {
            break;
        }

        // Read n strings into grid. The C program used grid[255][256] and scanf("%s",...).
        // We store rows as Vec<u8> (bytes) for O(1) index access.
        let mut grid: Vec<Vec<u8>> = Vec::with_capacity(n_usize);
        for _ in 0..n_usize {
            // In correct inputs there will be a token here; mimic C behavior assuming well-formed input.
            let s = toks.next().unwrap_or("");
            grid.push(s.as_bytes().to_vec());
        }

        // We use i32 for ones/max_ones to mirror C `int`.
        let mut max_ones: i32 = 0;
        let mut ones: i32;

        // Note: The original C code contains some non-intuitive index uses (e.g. grid[k][i] in places).
        // We intentionally preserve those exact access patterns to match behavior.

        // "vertical" block in C (actually iterates rows i, columns j)
        ones = 0;
        for i in 0..n_usize {
            for j in 0..n_usize {
                let ch = grid[i][j];
                if ch == b'1' {
                    ones += 1;
                } else {
                    if ones > max_ones {
                        max_ones = ones;
                    }
                    ones = 0;
                }
            }
            if ones > max_ones {
                max_ones = ones;
            }
            ones = 0;
        }

        // "horizontal" block in C (actually iterates columns j, rows i)
        ones = 0;
        for j in 0..n_usize {
            for i in 0..n_usize {
                let ch = grid[i][j];
                if ch == b'1' {
                    ones += 1;
                } else {
                    if ones > max_ones {
                        max_ones = ones;
                    }
                    ones = 0;
                }
            }
            if ones > max_ones {
                max_ones = ones;
            }
            ones = 0;
        }

        // diagonal: for (j = 0; j != n; ++j) for (i=0,k=j; k!=-1; ++i,--k) { grid[i][k] ... }
        // Here k goes down to -1, so use signed index for k.
        ones = 0;
        for j in 0..n_usize {
            let mut i_idx: usize = 0;
            let mut k: isize = j as isize;
            while k != -1 {
                let ch = grid[i_idx][k as usize];
                if ch == b'1' {
                    ones += 1;
                } else {
                    if ones > max_ones {
                        max_ones = ones;
                    }
                    ones = 0;
                }
                i_idx += 1;
                k -= 1;
            }
            if ones > max_ones {
                max_ones = ones;
            }
            ones = 0;
        }

        // for (i = 1; i != n; ++i) for (k = i, j = n-1; k != n; ++k, --j) { if (grid[k][i] == '1') ... }
        // Note: C used grid[k][i] (column fixed to i). Preserve that behavior (possibly a logical bug).
        ones = 0;
        for i_idx in 1..n_usize {
            let mut k: usize = i_idx;
            let mut _j: usize = n_usize.wrapping_sub(1); // j is decremented but not used in access; mimic updates
            while k != n_usize {
                let ch = grid[k][i_idx];
                if ch == b'1' {
                    ones += 1;
                } else {
                    if ones > max_ones {
                        max_ones = ones;
                    }
                    ones = 0;
                }
                k += 1;
                // mirror j-- (though not used for access)
                if _j == 0 {
                    _j = usize::MAX; // mimic wrap if it were to go below zero; but loop ends before this in well-formed cases
                } else {
                    _j -= 1;
                }
            }
            if ones > max_ones {
                max_ones = ones;
            }
            ones = 0;
        }

        // for (j = 0; j != n; ++j) for (i = 0, k = j; k != n; ++i, ++k) { grid[i][k] ... }
        ones = 0;
        for j in 0..n_usize {
            let mut i_idx: usize = 0;
            let mut k: usize = j;
            while k != n_usize {
                let ch = grid[i_idx][k];
                if ch == b'1' {
                    ones += 1;
                } else {
                    if ones > max_ones {
                        max_ones = ones;
                    }
                    ones = 0;
                }
                i_idx += 1;
                k += 1;
            }
            if ones > max_ones {
                max_ones = ones;
            }
            ones = 0;
        }

        // for (i = 1; i != n; ++i) for (k = i, j = 0; k != n; ++k, ++j) { if (grid[k][i] == '1') ... }
        // Again preserves grid[k][i].
        ones = 0;
        for i_idx in 1..n_usize {
            let mut k: usize = i_idx;
            let mut _j: usize = 0; // j increments but not used for access
            while k != n_usize {
                let ch = grid[k][i_idx];
                if ch == b'1' {
                    ones += 1;
                } else {
                    if ones > max_ones {
                        max_ones = ones;
                    }
                    ones = 0;
                }
                k += 1;
                _j = _j.wrapping_add(1); // mirror j++
            }
            if ones > max_ones {
                max_ones = ones;
            }
            ones = 0;
        }

        // Output like printf("%d\n", max_ones);
        println!("{}", max_ones);
    }
}
