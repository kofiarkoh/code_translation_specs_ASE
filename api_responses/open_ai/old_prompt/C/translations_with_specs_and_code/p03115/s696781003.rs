use std::io::{self, BufRead};

const CVAL: i64 = 200_000_000_000_000_000;

fn ncr_bit(n: i64, r: i64) -> i64 {
    if n < 0 || r < 0 || n < r || (n | r) != n {
        return 0;
    }
    1
}

fn find_col(n: i64, x: &[i64], y: &[i64], a: i64, b: i64) -> i64 {
    let mut r = 0;
    for i in 0..n {
        if a - x[i as usize] < 0 || b - y[i as usize] < 0 {
            continue;
        }
        let p = (a - x[i as usize]) + (b - y[i as usize]);
        let q = (b - y[i as usize]);
        r ^= ncr_bit(p, q);
    }
    r
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let n: i64 = lines.next().unwrap().unwrap().parse().unwrap();
    let mut x = vec![0; n as usize];
    let mut y = vec![0; n as usize];

    for i in 0..n {
        let coords: Vec<i64> = lines.next().unwrap().unwrap()
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        x[i as usize] = coords[0];
        y[i as usize] = coords[1];
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
