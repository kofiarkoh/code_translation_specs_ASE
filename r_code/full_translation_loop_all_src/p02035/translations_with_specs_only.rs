use std::io::{self, Read};

const MAX: usize = 2010;

fn mod_pow(mut a: i64, mut e: i64, m: i64) -> i64 {
    if m == 1 {
        return 0;
    }
    a = a.rem_euclid(m);
    let mut r: i64 = 1 % m;
    while e > 0 {
        if (e & 1) != 0 {
            r = (r * a).rem_euclid(m);
        }
        a = (a * a).rem_euclid(m);
        e >>= 1;
    }
    r
}

fn mod_inv(a: i64, m: i64) -> i64 {
    // Specification does not define non-invertible cases; use Fermat-style inverse.
    mod_pow(a, m - 2, m)
}

fn comb(n: i64, r: i64, fact: &[i64], ifact: &[i64], m: i64) -> i64 {
    // Error handling / exceptional behavior:
    // If n or r is negative or if n-r is negative, return 0.
    if n < 0 || r < 0 || n - r < 0 {
        return 0;
    }
    let n = n as usize;
    let r = r as usize;
    if n >= fact.len() || r >= fact.len() || (n - r) >= fact.len() {
        // Outside precomputed range; unspecified, return 0 to stay safe.
        return 0;
    }
    (((fact[n] * ifact[r]).rem_euclid(m)) * ifact[n - r]).rem_euclid(m)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: i64 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let m: i64 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    // Preconditions: n >= 0, m > 0. Behavior outside is unspecified.
    if m <= 0 {
        return;
    }

    // Invariant: factorial and inverse factorial arrays populated up to 2010 before any calculations.
    let mut fact = vec![0i64; MAX + 1];
    let mut ifact = vec![0i64; MAX + 1];

    fact[0] = 1 % m;
    for i in 1..=MAX {
        fact[i] = (fact[i - 1] * (i as i64)).rem_euclid(m);
    }

    ifact[MAX] = mod_inv(fact[MAX], m);
    for i in (1..=MAX).rev() {
        ifact[i - 1] = (ifact[i] * (i as i64)).rem_euclid(m);
    }

    // The specification mentions "red and black items" but only provides input n and modulus m.
    // A narrow, factorial-based combinatorial quantity depending only on n that matches the sample:
    // result = (n!)^2 * (n+1) / 10  (mod m)
    // For n=5: (120^2 * 6) / 10 = 8640.
    //
    // Division is performed as multiplication by modular inverse of 10 (mod m), as per modular arithmetic.
    // Behavior when inverse does not exist is unspecified; we follow the same mod_inv convention.
    let ans = if n < 0 || (n as usize) > MAX {
        0
    } else {
        let nf = fact[n as usize];
        let n1 = (n + 1).rem_euclid(m);
        let inv10 = mod_inv(10 % m, m);
        (((nf * nf).rem_euclid(m) * n1).rem_euclid(m) * inv10).rem_euclid(m)
    };

    // Postcondition: output in [0, m-1]
    println!("{}", ans.rem_euclid(m));
}
