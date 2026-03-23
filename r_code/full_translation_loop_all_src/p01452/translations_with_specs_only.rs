use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

fn mod_norm(x: i64) -> i64 {
    let mut v = x % MOD;
    if v < 0 {
        v += MOD;
    }
    v
}

fn mod_pow(mut a: i64, mut e: i64) -> i64 {
    a = mod_norm(a);
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
    // Preconditions assert invertibility; if violated, behavior is unspecified.
    mod_pow(a, MOD - 2)
}

fn comb(n: i64, r: i64, fact: &[i64], inv_fact: &[i64]) -> i64 {
    // Any binomial coefficient with invalid parameters is treated as 0.
    if n < 0 || r < 0 || r > n {
        return 0;
    }
    let ni = n as usize;
    let ri = r as usize;
    let n_minus_r = (n - r) as usize;
    (((fact[ni] * inv_fact[ri]) % MOD) * inv_fact[n_minus_r]) % MOD
}

fn main() {
    // Input parsing: if malformed/missing, behavior is unspecified by spec.
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let x: i64 = it.next().unwrap().parse().unwrap();
    let y: i64 = it.next().unwrap().parse().unwrap();
    let k: i64 = it.next().unwrap().parse().unwrap();

    // Preconditions: k >= 0, x+y+2k >= 0, and within internal capacity.
    // Capacity is not specified; we allocate exactly up to n = x+y+2k.
    let n: i64 = x + y + 2 * k;

    let n_usize: usize = n as usize; // If n < 0, this will panic; behavior unspecified.
    let mut fact = vec![0i64; n_usize + 1];
    let mut inv_fact = vec![0i64; n_usize + 1];

    fact[0] = 1;
    for i in 1..=n_usize {
        fact[i] = (fact[i - 1] * (i as i64)) % MOD;
    }
    inv_fact[n_usize] = mod_inv(fact[n_usize]);
    for i in (1..=n_usize).rev() {
        inv_fact[i - 1] = (inv_fact[i] * (i as i64)) % MOD;
    }

    let mut sum: i64 = 0;
    // Summation index ranges over all integers i such that 0 <= i <= k.
    for i in 0..=k {
        let a_n = x + 2 * (k - i);
        let a = mod_norm(
            comb(a_n, k - i, &fact, &inv_fact) - comb(a_n, k - i - 1, &fact, &inv_fact),
        );

        let b_n = y + 2 * i;
        let b = mod_norm(comb(b_n, i, &fact, &inv_fact) - comb(b_n, i - 1, &fact, &inv_fact));

        let c = comb(x + y + 2 * k, y + 2 * i, &fact, &inv_fact);

        let term = (((a * b) % MOD) * c) % MOD;
        sum += term;
        sum %= MOD;
    }

    println!("{}", mod_norm(sum));
}
