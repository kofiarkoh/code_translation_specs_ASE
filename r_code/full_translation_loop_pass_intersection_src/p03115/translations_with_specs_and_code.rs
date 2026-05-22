use std::io::{self, Read};

const CVAL: i64 = 200_000_000_000_000_000;

fn ncr_bit(n: i64, r: i64) -> i64 {
    // SPEC predicate: 1 iff every bit set in r is also set in n (i.e., (n|r)==n),
    // with standard domain guards.
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

fn find_col(a: i64, b: i64, xs: &[i64], ys: &[i64]) -> i64 {
    let mut r: i64 = 0;
    for (&x, &y) in xs.iter().zip(ys.iter()) {
        let da = a - x;
        let db = b - y;
        if da < 0 || db < 0 {
            continue;
        }
        let p = da + db;
        let q = db;
        r ^= ncr_bit(p, q);
    }
    r
}

fn main() {
    // Read all input (no explicit error handling per spec).
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    // Spec precondition: n <= 131072; we rely on it (no explicit checks).
    let mut xs = Vec::with_capacity(n);
    let mut ys = Vec::with_capacity(n);
    for _ in 0..n {
        let x: i64 = it.next().unwrap().parse().unwrap();
        let y: i64 = it.next().unwrap().parse().unwrap();
        xs.push(x);
        ys.push(y);
    }

    // Greedy bitwise maximization of R in [0, 2^61-1], from bit 60 down to 0.
    let mut r: i64 = 0;
    for k in (0..=60).rev() {
        let w: i64 = 1_i64 << k;
        let cand_r = r + w;
        let a = CVAL - cand_r;
        let b = cand_r;
        if find_col(a, b, &xs, &ys) == 1 {
            r = cand_r;
        }
    }

    let ans = CVAL - r;
    print!("{ans}\n");
}
