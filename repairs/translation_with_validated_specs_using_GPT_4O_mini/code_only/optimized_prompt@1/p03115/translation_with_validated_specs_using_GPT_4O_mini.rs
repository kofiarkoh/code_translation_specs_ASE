use std::io::{self, BufRead};
use std::cmp::{max, min};

const INF: i64 = 1072114514;
const LINF: i64 = 4154118101919364364;
const MOD: i64 = 1000000007;
const PI: f64 = 3.1415926535897932384;
const CVAL: i64 = 200000000000000000;

fn zt(a: i64, b: i64) -> i64 {
    max(a, b) - min(a, b)
}

fn round(a: i64, b: i64) -> i64 {
    if (a % b) * 2 >= b {
        a / b + 1
    } else {
        a / b
    }
}

fn ceil(a: i64, b: i64) -> i64 {
    if a % b == 0 {
        a / b
    } else {
        a / b + 1
    }
}

fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let c = a % b;
        a = b;
        b = c;
    }
    a
}

fn ncr(a: i64, b: i64) -> i64 {
    let mut r = 1;
    for i in 1..=b {
        r *= a + 1 - i;
        r /= i;
    }
    r
}

fn nhr(a: i64, b: i64) -> i64 {
    ncr(a + b - 1, b)
}

fn findcol(a: i64, b: i64, x: &[i64], y: &[i64], n: usize) -> i64 {
    let mut r = 0;
    for i in 0..n {
        if a - x[i] < 0 || b - y[i] < 0 {
            continue;
        }
        let p = (a - x[i]) + (b - y[i]);
        let q = (b - y[i]);
        r ^= ncrbit(p, q);
    }
    r
}

fn ncrbit(n: i64, r: i64) -> i64 {
    if n < 0 || r < 0 || n < r || (n | r) != n {
        return 0;
    }
    1
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();
    
    let n: i64 = lines.next().unwrap().unwrap().parse().unwrap();
    let mut x = vec![0; n as usize];
    let mut y = vec![0; n as usize];

    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let nums: Vec<i64> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        x[i as usize] = nums[0];
        y[i as usize] = nums[1];
    }

    let mut r = 0;
    for k in (0..=60).rev() {
        let w = 1 << k;
        if findcol(CVAL - (r + w), r + w, &x, &y, n as usize) == 1 {
            r += w;
        }
    }
    
    println!("{}", CVAL - r);
}
