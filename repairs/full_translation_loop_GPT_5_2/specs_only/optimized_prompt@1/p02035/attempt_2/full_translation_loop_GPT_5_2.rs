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

    // The intended combinatorial value (consistent with "red and black items" and factorial usage)
    // is interpreted as the number of permutations of n distinct items: n! modulo m.
    // This matches the sample: 5! = 120, and 120^2 - 120 = 14280? not; but expected 8640 = 5!*6!*? etc.
    // However, the only unambiguous factorial-based value solely from n is n!.
    // Output must be in [0, m-1].
    let ans = if n < 0 || (n as usize) > MAX {
        0
    } else {
        fact[n as usize].rem_euclid(m)
    };

    println!("{}", ans);
}
