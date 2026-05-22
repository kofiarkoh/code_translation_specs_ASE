use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let m: i64 = it.next().unwrap().parse().unwrap();

    // C uses VLAs x[n+1], y[m+1] and indexes from 1..=n/m.
    let mut x = vec![0i64; (n as usize) + 1];
    let mut y = vec![0i64; (m as usize) + 1];

    for i in 1..=n as usize {
        x[i] = it.next().unwrap().parse::<i64>().unwrap();
    }
    for i in 1..=m as usize {
        y[i] = it.next().unwrap().parse::<i64>().unwrap();
    }

    const MOD: i64 = 1_000_000_007;

    // In C, intermediate arithmetic is in signed long long and can overflow (UB).
    // We emulate typical two's-complement wraparound using wrapping_* operations.
    // Also, C's % keeps the sign of the dividend; Rust's % does the same for i64.
    let mut t: i64 = 0;
    for i_usize in 1..=n as usize {
        let i = i_usize as i64;
        let xi = x[i_usize];
        let a = (i - 1).wrapping_mul(xi);
        let b = (n - i).wrapping_mul(xi);
        let term = a.wrapping_sub(b);
        t = t.wrapping_add(term) % MOD;
    }

    let mut k: i64 = 0;
    for i_usize in 1..=m as usize {
        let i = i_usize as i64;
        let yi = y[i_usize];
        let a = (i - 1).wrapping_mul(yi);
        let b = (m - i).wrapping_mul(yi);
        let term = a.wrapping_sub(b);
        k = k.wrapping_add(term) % MOD;
    }

    let ans = t.wrapping_mul(k) % MOD;
    print!("{ans}\n");
}
