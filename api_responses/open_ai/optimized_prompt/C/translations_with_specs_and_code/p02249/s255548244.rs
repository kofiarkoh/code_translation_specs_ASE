// AOJ ALDS1_14_C Pattern Search
// Translated from C to Rust

use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;
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

    let mut a = [[0; MAX_L]; MAX_L];
    let mut b = [[0; MAX_L]; MAX_L];

    gen_hash(&mut a, &text, h, w, r, c);
    gen_hash(&mut b, &pattern, r, c, h, w);

    for row in 0..=h - r {
        for col in 0..=w - c {
            if a[row][col] == b[0][0] {
                println!("{} {}", row, col);
            }
        }
    }
}

fn read_dimensions<R: BufRead>(reader: &R) -> (usize, usize) {
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let mut iter = line.split_whitespace();
    let h = iter.next().unwrap().parse::<usize>().unwrap();
    let w = iter.next().unwrap().parse::<usize>().unwrap();
    (h, w)
}

fn read_grid<R: BufRead>(reader: &R, rows: usize, cols: usize) -> Vec<Vec<char>> {
    let mut grid = vec![vec![' '; cols]; rows];
    for r in 0..rows {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        grid[r] = line.chars().take(cols).collect();
    }
    grid
}

fn gen_hash(res: &mut [[i64; MAX_L]; MAX_L], s: &Vec<Vec<char>>, h: usize, w: usize, r: usize, c: usize) {
    let mut t = [[0; MAX_L]; MAX_L];
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
            k = (k * C1 + s[row][col] as i64) % MOD;
        }
        for col in 0..=w - c {
            t[row][col] = k;
            if col + c < w {
                k = (k * C1 - s[row][col] as i64 * x + s[row][col + c] as i64) % MOD;
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
}
