use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as whitespace-separated tokens (like scanf).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = it.next().unwrap().parse().unwrap();
    let m: i64 = it.next().unwrap().parse().unwrap();

    // C uses VLAs and 1-based indexing; emulate with Vec sized n+1/m+1.
    let mut x = vec![0i64; (n as usize) + 1];
    let mut y = vec![0i64; (m as usize) + 1];

    for i in 1..=n as usize {
        x[i] = it.next().unwrap().parse().unwrap();
    }
    for i in 1..=m as usize {
        y[i] = it.next().unwrap().parse().unwrap();
    }

    const MOD: i64 = 1_000_000_007;

    // In C, intermediate arithmetic is in signed long long and can overflow (UB).
    // We emulate typical two's-complement wraparound using wrapping_* operations.
    let mut t: i64 = 0;
    for i in 1..=n {
        let xi = x[i as usize];
        let a = (i - 1).wrapping_mul(xi);
        let b = (n - i).wrapping_mul(xi);
        let term = a.wrapping_sub(b);
        t = t.wrapping_add(term).wrapping_rem(MOD);
    }

    let mut k: i64 = 0;
    for i in 1..=m {
        let yi = y[i as usize];
        let a = (i - 1).wrapping_mul(yi);
        let b = (m - i).wrapping_mul(yi);
        let term = a.wrapping_sub(b);
        k = k.wrapping_add(term).wrapping_rem(MOD);
    }

    let ans = t.wrapping_mul(k).wrapping_rem(MOD);
    print!("{ans}\n");
}
