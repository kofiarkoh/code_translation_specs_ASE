// AOJ ALDS1_14_C Pattern Search
// 2018.5.13 bal4u

use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;
const C1: i64 = 401;
const C2: i64 = 397;

const MAX_L: usize = 1005;
type Matrix<T> = [[T; MAX_L]; MAX_L];

fn gen_hash(res: &mut Matrix<i64>, s: &Matrix<u8>, h: usize, w: usize, r: usize, c: usize) {
    let mut t = [[0; MAX_L]; MAX_L];
    let mut x = 1;
    let mut y = 1;

    for _ in 0..c {
        x = (x * C1) % MOD;
    }
    for _ in 0..r {
        y = (y * C2) % MOD;
    }

    for r in 0..h {
        let mut k = 0;
        for cc in 0..c {
            k = (k * C1 + s[r][cc] as i64) % MOD;
        }
        for cc in 0..=w - c {
            t[r][cc] = k;
            if cc + c < w {
                k = (k * C1 - s[r][cc] as i64 * x + s[r][cc + c] as i64) % MOD;
            }
            if k < 0 {
                k += MOD;
            }
        }
    }

    for cc in 0..w {
        let mut k = 0;
        for rr in 0..r {
            k = (k * C2 + t[rr][cc]) % MOD;
        }
        for rr in 0..=h - r {
            res[rr][cc] = k;
            if rr + r < h {
                k = (k * C2 - t[rr][cc] * y + t[rr + r][cc]) % MOD;
            }
            if k < 0 {
                k += MOD;
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_to_string(&mut input).unwrap();

    let mut lines = input.lines();
    let h: usize = lines.next().unwrap().parse().unwrap();
    let w: usize = lines.next().unwrap().parse().unwrap();

    let mut text: Matrix<u8> = [[0; MAX_L]; MAX_L];
    for r in 0..h {
        let line = lines.next().unwrap();
        for (c, ch) in line.bytes().enumerate() {
            text[r][c] = ch;
        }
    }

    let r: usize = lines.next().unwrap().parse().unwrap();
    let c: usize = lines.next().unwrap().parse().unwrap();

    let mut pat: Matrix<u8> = [[0; MAX_L]; MAX_L];
    for rr in 0..r {
        let line = lines.next().unwrap();
        for (cc, ch) in line.bytes().enumerate() {
            pat[rr][cc] = ch;
        }
    }

    let mut a: Matrix<i64> = [[0; MAX_L]; MAX_L];
    let mut b: Matrix<i64> = [[0; MAX_L]; MAX_L];

    gen_hash(&mut a, &text, h, w, r, c);
    gen_hash(&mut b, &pat, r, c, r, c);

    for rr in 0..=h - r {
        for cc in 0..=w - c {
            if a[rr][cc] == b[0][0] {
                println!("{} {}", rr, cc);
            }
        }
    }
}
