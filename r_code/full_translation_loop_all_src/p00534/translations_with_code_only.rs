use std::io::{self, Read};

fn main() {
    // Read entire stdin (scanf-style whitespace separated integers)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let m: usize = it.next().unwrap().parse().unwrap();

    // 1-based arrays to match C indexing
    let mut k = vec![0i32; n + 1];
    let mut t = vec![0i32; m + 1];

    for i in 1..=n {
        k[i] = it.next().unwrap().parse().unwrap();
    }
    for i in 1..=m {
        t[i] = it.next().unwrap().parse().unwrap();
    }

    // hi[m+1][n+1], initialized to 1_000_000_000 like C
    let mut hi = vec![vec![1_000_000_000i32; n + 1]; m + 1];

    // hi[i][0] = 0 for i=0..m
    for i in 0..=m {
        hi[i][0] = 0;
    }

    // DP with C-like 32-bit signed overflow behavior:
    // In C, signed overflow is UB, but typical intent is 32-bit wraparound.
    // We emulate that with wrapping arithmetic on i32.
    for i in 1..=m {
        for j in 1..=n {
            let prod = k[j].wrapping_mul(t[i]);
            let sum = prod.wrapping_add(hi[i - 1][j - 1]);
            let mut val = sum;
            if val > hi[i - 1][j] {
                val = hi[i - 1][j];
            }
            hi[i][j] = val;
        }
    }

    print!("{}\n", hi[m][n]);
}
