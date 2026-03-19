use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;
const MAXN: usize = 220_000;

fn mod_pow(mut a: i64, mut e: i64) -> i64 {
    a %= MOD;
    let mut r: i64 = 1;
    while e > 0 {
        if (e & 1) != 0 {
            r = (r * a) % MOD;
        }
        a = (a * a) % MOD;
        e >>= 1;
    }
    r
}

fn mod_inv(a: i64) -> i64 {
    // MOD is prime per specification; inverse exists under preconditions.
    mod_pow(a, MOD - 2)
}

fn comb(n: i64, k: i64, fact: &[i64], inv_fact: &[i64]) -> i64 {
    // Any invalid binomial coefficient contributes 0.
    if n < 0 || k < 0 || k > n {
        return 0;
    }
    let ni = n as usize;
    let ki = k as usize;
    let nki = (n - k) as usize;
    (((fact[ni] * inv_fact[ki]) % MOD) * inv_fact[nki]) % MOD
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let x: i64 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let y: i64 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let k: i64 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    // Precondition: k >= 0 and x + y + 2k within capacity.
    let n_total_i64 = x + y + 2 * k;
    let n_total = n_total_i64 as usize;

    // Fixed-size table per specification (dominant memory usage).
    // We only fill up to n_total, but allocate full capacity.
    let mut fact = vec![0i64; MAXN + 1];
    let mut inv_fact = vec![0i64; MAXN + 1];

    fact[0] = 1;
    for i in 1..=n_total {
        fact[i] = (fact[i - 1] * (i as i64)) % MOD;
    }

    inv_fact[n_total] = mod_inv(fact[n_total]);
    for i in (1..=n_total).rev() {
        inv_fact[i - 1] = (inv_fact[i] * (i as i64)) % MOD;
    }

    let mut ans: i64 = 0;

    for i in 0..=k {
        let a_n = x + 2 * (k - i);
        let a_k = k - i;
        let b_n = y + 2 * i;
        let b_k = i;

        // "difference of two binomial coefficients" terms:
        // narrowest reasonable interpretation: C(n, t) - C(n, t-1)
        let a = (comb(a_n, a_k, &fact, &inv_fact) - comb(a_n, a_k - 1, &fact, &inv_fact)).rem_euclid(MOD);
        let b = (comb(b_n, b_k, &fact, &inv_fact) - comb(b_n, b_k - 1, &fact, &inv_fact)).rem_euclid(MOD);

        // Third binomial coefficient: C(x + y + 2k, y + 2i)
        let c = comb(n_total_i64, y + 2 * i, &fact, &inv_fact);

        let term = (((a * b) % MOD) * c) % MOD;
        ans += term;
        ans %= MOD;
    }

    println!("{}", ans.rem_euclid(MOD));
}
