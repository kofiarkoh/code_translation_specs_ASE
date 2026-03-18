use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

fn mod_pow(mut a: i64, mut n: i64, m: i64) -> i64 {
    let mut x: i64 = 1;
    a %= m;
    while n > 0 {
        if (n & 1) == 1 {
            x = (x * a) % m;
        }
        a = (a * a) % m;
        n >>= 1;
    }
    x
}

fn mod_inv(a: i64, p: i64) -> i64 {
    // p is prime (MOD), so inverse is a^(p-2) mod p
    mod_pow(a, p - 2, p)
}

fn make_fact(n: usize, m: i64) -> (Vec<i64>, Vec<i64>) {
    let mut fact = vec![0i64; n + 1];
    let mut invfact = vec![0i64; n + 1];

    fact[0] = 1;
    for i in 0..n {
        fact[i + 1] = (fact[i] * (i as i64 + 1)) % m;
    }

    invfact[n] = mod_inv(fact[n], m);
    for i in (1..=n).rev() {
        invfact[i - 1] = (invfact[i] * i as i64) % m;
    }

    (fact, invfact)
}

fn choose(n: i64, r: i64, m: i64, fact: &[i64], invfact: &[i64]) -> i64 {
    if n < 0 || r < 0 || n - r < 0 {
        return 0;
    }
    let n = n as usize;
    let r = r as usize;
    let nr = (n - r) as usize;
    (((fact[n] * invfact[r]) % m) * invfact[nr]) % m
}

fn main() {
    // Precompute factorials up to 2000 as in the C code.
    let (fact, invfact) = make_fact(2000, MOD);

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n_opt = it.next();
    let m_opt = it.next();
    if n_opt.is_none() || m_opt.is_none() {
        // Spec says no invalid-format handling; we just do nothing if missing.
        return;
    }

    let n: i64 = n_opt.unwrap().parse().unwrap();
    let m: i64 = m_opt.unwrap().parse().unwrap();

    // Spec: If n or m is negative, output 0.
    if n < 0 || m < 0 {
        print!("0\n");
        return;
    }

    // Note: The C code assumes m <= 2000 (since factorials are only built to 2000).
    // We preserve that behavior; for m > 2000 this would index out of bounds in C as well.
    let mut s: i64 = 0;
    for i in 0..=m {
        let c = choose(m, i, MOD, &fact, &invfact);
        let p = mod_pow(i % MOD, n, MOD);
        let sign = if ((m - i) % 2) == 0 { 1i64 } else { -1i64 };
        s = (s + (c * p) % MOD * sign) % MOD;
    }

    let ans = (s + MOD) % MOD;
    print!("{ans}\n");
}
