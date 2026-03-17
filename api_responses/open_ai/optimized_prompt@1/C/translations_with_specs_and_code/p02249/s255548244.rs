// AOJ ALDS1_14_C Pattern Search
// Translated from C to Rust

use std::io::{self, BufRead};

const MOD: i64 = 1000000007;
const C1: i64 = 401;
const C2: i64 = 397;
const MAX_L: usize = 1005;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let (h, w) = read_dimensions(&reader);
    let text = read_grid(&reader, h, w);

    let (r, c) = read_dimensions(&reader);
    let pattern = read_grid(&reader, r, c);

    let a = gen_hash(&text, h, w, c, r);
    let b = gen_hash(&pattern, r, c, c, r);

    for row in 0..=h - r {
        for col in 0..=w - c {
            if a[row][col] == b[0][0] {
                println!("{} {}", row, col);
            }
        }
    }
}

fn read_dimensions<R: BufRead>(reader: &R) -> (usize, usize) {
    let mut input = String::new();
    reader.read_line(&mut input).unwrap();
    let dims: Vec<usize> = input.split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();
    (dims[0], dims[1])
}

fn read_grid<R: BufRead>(reader: &R, h: usize, w: usize) -> Vec<Vec<char>> {
    let mut grid = vec![vec![' '; w]; h];
    for i in 0..h {
        let mut input = String::new();
        reader.read_line(&mut input).unwrap();
        grid[i] = input.chars().take(w).collect();
    }
    grid
}

fn gen_hash(grid: &[Vec<char>], h: usize, w: usize, c: usize, r: usize) -> Vec<Vec<i64>> {
    let mut res = vec![vec![0; w]; h];
    let mut t = vec![vec![0; w]; h];

    let mut x = 1;
    let mut y = 1;

    for _ in 0..c {
        x = (x * C1) % MOD;
    }
    for _ in 0..r {
        y = (y * C2) % MOD;
    }

    for row in 0..h {
        let mut k = 0;
        for col in 0..c {
            k = (k * C1 + grid[row][col] as i64) % MOD;
        }
        for col in 0..=w - c {
            t[row][col] = k;
            if col + c < w {
                k = (k * C1 - grid[row][col] as i64 * x + grid[row][col + c] as i64) % MOD;
                if k < 0 {
                    k += MOD;
                }
            }
        }
    }

    for col in 0..w {
        let mut k = 0;
        for row in 0..r {
            k = (k * C2 + t[row][col]) % MOD;
        }
        for row in 0..=h - r {
            res[row][col] = k;
            if row + r < h {
                k = (k * C2 - t[row][col] * y + t[row + r][col]) % MOD;
                if k < 0 {
                    k += MOD;
                }
            }
        }
    }

    res
}
```

### Notes:
- The Rust code preserves the semantics of the original C code while adhering to the specifications provided.
- Input reading is done using `BufRead` for efficiency, and the dimensions are read in a single function.
- The grid is represented as a `Vec<Vec<char>>`, which is idiomatic in Rust.
- The hash generation function is translated to maintain the same logic as in C, ensuring that the hash values are computed correctly.
- The output format is preserved to match the expected output of the original C program.
