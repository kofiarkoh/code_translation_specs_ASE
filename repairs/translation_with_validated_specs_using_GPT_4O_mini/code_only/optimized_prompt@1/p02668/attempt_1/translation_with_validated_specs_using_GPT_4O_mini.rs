#![allow(non_snake_case)]

use std::cmp::{min, max};
use std::io::{self, Read};
use std::str;

const BIG: u64 = 2000000007;
const MOD: u64 = 1000000007;
const N_MAX: usize = 1048576;

#[derive(Clone, Copy)]
struct hwll {
    a: i64,
    b: i64,
}

#[derive(Clone, Copy)]
struct hwllc {
    a: i64,
    b: i64,
    c: i64,
}

#[derive(Clone, Copy)]
struct linell {
    a: hwll,
    b: hwll,
}

fn gcd(x: u64, y: u64) -> u64 {
    if y == 0 {
        x
    } else {
        gcd(y, x % y)
    }
}

fn bitpow(a: u64, mut x: u64, modulo: u64) -> u64 {
    let mut result = 1;
    while x > 0 {
        if x & 1 == 1 {
            result = (result * a) % modulo;
        }
        x /= 2;
        a = (a * a) % modulo;
    }
    result
}

fn divide(a: u64, b: u64, modulo: u64) -> u64 {
    (a * bitpow(b, modulo - 2, modulo)) % modulo
}

fn f_init(n: i64, m: u64, frac: &mut [u64], invf: &mut [u64]) {
    frac[0] = 1;
    for i in 1..=n {
        frac[i as usize] = (frac[(i - 1) as usize] * i as u64) % m;
    }

    invf[n as usize] = divide(1, frac[n as usize], m);
    for i in (0..n).rev() {
        invf[i as usize] = (invf[(i + 1) as usize] * (i + 1) as u64) % m;
    }
}

fn ncr(n: i64, r: i64, m: u64, frac: &[u64], invf: &[u64]) -> u64 {
    if n < 0 || r < 0 || n < r {
        return 0;
    }
    frac[n as usize] * (invf[r as usize] * invf[(n - r) as usize] % m) % m
}

fn solve(n: i64, m: i64) -> i64 {
    let mut dp = vec![vec![0; 5050]; 5050];
    let mut dq = vec![vec![0; 5050]; 5050];
    let mut pow2 = vec![0; N_MAX];
    let mut inv2 = vec![0; N_MAX];

    let inv = (MOD + 1) / 2;

    pow2[0] = 1;
    inv2[0] = 1;
    for i in 1..=n + m {
        pow2[i as usize] = (pow2[(i - 1) as usize] * 2) % MOD;
        inv2[i as usize] = (inv2[(i - 1) as usize] * inv) % MOD;
    }

    let mut result = 0;

    for i in 0..=n {
        for j in 1..=m {
            if i == 0 {
                dp[i as usize][j as usize] = 1;
            } else {
                dp[i as usize][j as usize] = ((j + 1) * dp[(i - 1) as usize][j as usize] + dq[(i - 1) as usize][j as usize - 1] * pow2[(j - 1) as usize]) % MOD;
            }

            dq[i as usize][j as usize] = (dq[i as usize][j as usize - 1] + (dp[i as usize][j as usize] * j % MOD) * inv2[j as usize]) % MOD;
        }
    }
    result = dp[n as usize][m as usize];

    println!("{}", result);
    0
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: i64 = iter.next().unwrap().parse().unwrap();
    let m: i64 = iter.next().unwrap().parse().unwrap();

    let frac = vec![0; N_MAX * 10];
    let invf = vec![0; N_MAX * 10];

    f_init(n * 5 + 5, MOD, &mut frac.clone(), &mut invf.clone());
    solve(n, m);
}
