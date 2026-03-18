use std::io::{self, Read};

const CVAL: i64 = 200_000_000_000_000_000;

fn ncr_bit(n: i64, r: i64) -> i64 {
    // nCr mod 2 via Lucas theorem bit condition:
    // returns 1 iff r is a submask of n (and 0 otherwise), with basic guards.
    if n < 0 || r < 0 {
        return 0;
    }
    if n < r {
        return 0;
    }
    if (n | r) != n {
        return 0;
    }
    1
}

fn findcol(a: i64, b: i64, xs: &[i64], ys: &[i64]) -> i64 {
    let mut r: i64 = 0;
    for i in 0..xs.len() {
        let dx = a - xs[i];
        let dy = b - ys[i];
        if dx < 0 || dy < 0 {
            continue;
        }
        let p = dx + dy;
        let q = dy;
        r ^= ncr_bit(p, q);
    }
    r
}

fn main() {
    // The spec assumes valid input (non-negative n and coordinates).
    // We preserve the C program's behavior: no explicit error handling.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse::<i64>().unwrap() as usize,
        None => return,
    };

    let mut xs = vec![0i64; n];
    let mut ys = vec![0i64; n];
    for i in 0..n {
        xs[i] = it.next().unwrap().parse::<i64>().unwrap();
        ys[i] = it.next().unwrap().parse::<i64>().unwrap();
    }

    let mut r: i64 = 0;
    for k in (0..=60).rev() {
        let w: i64 = 1i64 << k;
        if findcol(CVAL - (r + w), r + w, &xs, &ys) == 1 {
            r += w;
        }
    }

    println!("{}", CVAL - r);
}
