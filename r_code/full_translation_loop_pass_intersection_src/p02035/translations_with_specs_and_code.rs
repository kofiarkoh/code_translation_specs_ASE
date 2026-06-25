use std::io::{self, Read};

fn mod_pow(mut a: i64, mut e: i64, m: i64) -> i64 {
    // Mirrors C behavior: all arithmetic is done modulo m.
    // Assumes m > 0 as in typical modular arithmetic usage.
    a %= m;
    if a < 0 {
        a += m;
    }
    let mut x: i64 = 1 % m;
    while e > 0 {
        if (e & 1) == 1 {
            x = (x * a) % m;
        }
        a = (a * a) % m;
        e >>= 1;
    }
    x
}

fn make_fact(n: usize, m: i64) -> (Vec<i64>, Vec<i64>) {
    let mut fact = vec![0i64; n + 1];
    let mut invfact = vec![0i64; n + 1];
    fact[0] = 1 % m;
    for i in 0..n {
        fact[i + 1] = (fact[i] * (i as i64 + 1)) % m;
    }
    // Fermat-style inversion as specified: x^(m-2) mod m
    invfact[n] = mod_pow(fact[n], m - 2, m);
    for i in (1..=n).rev() {
        invfact[i - 1] = (invfact[i] * (i as i64)) % m;
    }
    (fact, invfact)
}

fn choose(n: i32, r: i32, m: i64, fact: &[i64], invfact: &[i64]) -> i64 {
    if n < 0 || r < 0 || n - r < 0 {
        return 0;
    }
    let n = n as usize;
    let r = r as usize;
    let nr = (n - r) as usize;
    (((fact[n] * invfact[r]) % m) * invfact[nr]) % m
}

fn main() {
    // Read all input
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();
    let n: i32 = it.next().unwrap().parse().unwrap();
    let m_in: i32 = it.next().unwrap().parse().unwrap();
    let m: i64 = m_in as i64;

    // Precompute factorials up to 2010 inclusive, as per spec/C code.
    let (fact, invfact) = make_fact(2010usize, m);

    let mut ans: i64 = 0;
    for i in 1..n {
        let mut temp: i64 = 0;
        let ni = n - i;
        for j in 0..ni {
            // aaa = (2^(n-i-j) - 1)^i * C(n-i, j)
            let pow2 = mod_pow(2, (ni - j) as i64, m);
            let base = (pow2 - 1).rem_euclid(m);
            let term = (mod_pow(base, i as i64, m) * choose(ni, j, m, &fact, &invfact)) % m;
            if (j & 1) == 1 {
                temp -= term;
            } else {
                temp += term;
            }
        }

        let ii = i as i64;
        let nii = (n - i) as i64;
        let a = mod_pow(2, ii * (ii - 1) / 2, m);
        let b = mod_pow(2, nii * (nii - 1) / 2, m);
        let c = choose(n, i, m, &fact, &invfact);

        let add = (((temp % m) * a) % m * b) % m * c % m;
        ans += add;
    }

    let out = (ans % m + m) % m;
    println!("{out}");
}
