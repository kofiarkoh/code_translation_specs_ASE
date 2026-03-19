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
        None => return, // behavior not inferable
    };
    let y: i64 = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return, // behavior not inferable
    };

    let sum = x + y; // spec requires no overflow for defined behavior
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

    let a_us = a as usize;
    let b_us = b as usize;
    let k = a_us + b_us;

    // Precompute only as far as needed (but never beyond MAX, per spec's table validity).
    // If k > MAX, behavior is not inferable from the spec; we do not add extra handling.
    let limit = k;

    let mut fact = vec![0i64; limit + 1];
    let mut finv = vec![0i64; limit + 1];

    fact[0] = 1;
    for i in 1..=limit {
        fact[i] = (fact[i - 1] * (i as i64)) % MOD;
    }

    finv[limit] = mod_pow(fact[limit], MOD - 2);
    for i in (1..=limit).rev() {
        finv[i - 1] = (finv[i] * (i as i64)) % MOD;
    }

    let ans = (((fact[k] * finv[a_us]) % MOD) * finv[b_us]) % MOD;
    println!("{}", ans);
}
