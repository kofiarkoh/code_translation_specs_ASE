use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;
const MAX: usize = 1_000_000;

fn mod_pow(mut a: i64, mut e: i64) -> i64 {
    let mut r: i64 = 1;
    a %= MOD;
    while e > 0 {
        if (e & 1) == 1 {
            r = (r * a) % MOD;
        }
        a = (a * a) % MOD;
        e >>= 1;
    }
    r
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let x: i64 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return, // behavior not inferable; keep minimal
    };
    let y: i64 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return, // behavior not inferable; keep minimal
    };

    // Reachability checks per specification.
    let sum = x + y; // spec requires no overflow; we assume defined inputs satisfy it
    if sum % 3 != 0 {
        println!("0");
        return;
    }
    let n = sum / 3;
    let a = x - n;
    let b = y - n;
    if a < 0 || b < 0 {
        println!("0");
        return;
    }

    // From here, spec says computation uses precomputed tables valid up to 1_000_000.
    // If indices exceed bounds, behavior is not inferable; we do not add extra handling.
    let a_us = a as usize;
    let b_us = b as usize;
    let k = a_us + b_us;

    // Precompute factorials and inverse factorials up to MAX.
    // Memory: three arrays of size 1_000_001 of 64-bit integers (as per spec).
    let mut fact = vec![0i64; MAX + 1];
    let mut inv = vec![0i64; MAX + 1];
    let mut finv = vec![0i64; MAX + 1];

    fact[0] = 1;
    for i in 1..=MAX {
        fact[i] = (fact[i - 1] * (i as i64)) % MOD;
    }

    // inv[MAX] = fact[MAX]^(MOD-2) mod MOD, then derive inv[i-1] = inv[i]*i mod MOD
    inv[MAX] = mod_pow(fact[MAX], MOD - 2);
    for i in (1..=MAX).rev() {
        inv[i - 1] = (inv[i] * (i as i64)) % MOD;
    }

    for i in 0..=MAX {
        finv[i] = inv[i];
    }

    // Compute C(k, a) mod MOD.
    let ans = (((fact[k] * finv[a_us]) % MOD) * finv[b_us]) % MOD;
    println!("{}", ans);
}
