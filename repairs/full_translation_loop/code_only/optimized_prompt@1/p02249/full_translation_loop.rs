// AOJ ALDS1_14_C Pattern Search
// 2018.5.13 bal4u

use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;
const C1: i64 = 401;
const C2: i64 = 397;
const MAX_L: usize = 1005;

fn gc() -> char {
    let mut buffer = [0; 1];
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    handle.read(&mut buffer).unwrap();
    buffer[0] as char
}

fn in_number() -> i64 {
    let mut n = 0;
    let mut c = gc();
    while c >= '0' {
        n = 10 * n + (c as i64 - '0' as i64);
        c = gc();
    }
    n
}

fn gen_hash(res: &mut [[i64; MAX_L]; MAX_L], s: &[[char; MAX_L]; MAX_L], h: usize, w: usize, r: usize, c: usize) {
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
        for j in 0..c {
            k = (k * C1 + s[r][j] as i64) % MOD;
        }
        for j in 0..=w - c {
            t[r][j] = k;
            if j + c < w {
                k = (k * C1 - s[r][j] as i64 * x + s[r][j + c] as i64) % MOD;
                if k < 0 {
                    k += MOD;
                }
            }
        }
    }

    for j in 0..w {
        let mut k = 0;
        for i in 0..r {
            k = (k * C2 + t[i][j]) % MOD;
        }
        for i in 0..=h - r {
            res[i][j] = k;
            if i + r < h {
                k = (k * C2 - t[i][j] * y + t[i + r][j]) % MOD;
                if k < 0 {
                    k += MOD;
                }
            }
        }
    }
}

fn main() {
    let h = in_number() as usize;
    let w = in_number() as usize;
    let mut text = [['\0'; MAX_L]; MAX_L];
    let mut pat = [['\0'; MAX_L]; MAX_L];
    let mut a = [[0; MAX_L]; MAX_L];
    let mut b = [[0; MAX_L]; MAX_L];

    for r in 0..h {
        let mut p = 0;
        while {
            let ch = gc();
            ch > ' '
        } {
            let ch = gc();
            text[r][p] = ch;
            p += 1;
        }
        text[r][p] = '\0'; // Null-terminate for consistency
    }

    let r = in_number() as usize;
    let c = in_number() as usize;

    for i in 0..r {
        let mut p = 0;
        while {
            let ch = gc();
            ch > ' '
        } {
            let ch = gc();
            pat[i][p] = ch;
            p += 1;
        }
        pat[i][p] = '\0'; // Null-terminate for consistency
    }

    gen_hash(&mut a, &text, h, w, r, c);
    gen_hash(&mut b, &pat, r, c, r, c);

    for i in 0..=h - r {
        for j in 0..=w - c {
            if a[i][j] == b[0][0] {
                println!("{} {}", i, j);
            }
        }
    }
}
