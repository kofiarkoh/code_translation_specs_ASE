use std::io::{self, BufRead};

const CVAL: i64 = 200_000_000_000_000_000;

fn ncr_bit(n: i64, r: i64) -> i64 {
    if n < 0 || r < 0 || n < r || (n | r) != n {
        return 0;
    }
    1
}

fn find_col(n: usize, x: &[i64], y: &[i64], a: i64, b: i64) -> i64 {
    let mut r = 0;
    for i in 0..n {
        if a - x[i] < 0 || b - y[i] < 0 {
            continue;
        }
        let p = (a - x[i]) + (b - y[i]);
        let q = (b - y[i]);
        r ^= ncr_bit(p, q);
    }
    r
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    if n == 0 {
        println!("{}", CVAL);
        return;
    }

    let mut x = vec![0; n];
    let mut y = vec![0; n];

    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let coords: Vec<i64> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        x[i] = coords[0];
        y[i] = coords[1];
    }

    let mut r = 0;
    for k in (0..=60).rev() {
        let w = 1 << k;
        if find_col(n, &x, &y, CVAL - (r + w), r + w) == 1 {
            r += w;
        }
    }

    println!("{}", CVAL - r);
}
