use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;
const MAX: usize = 1_000_000;

fn mod_pow(mut a: i64, mut e: i64) -> i64 {
    a %= MOD;
    let mut x: i64 = 1;
    while e > 0 {
        if (e & 1) == 1 {
            x = (x * a) % MOD;
        }
        a = (a * a) % MOD;
        e >>= 1;
    }
    x
}

fn choose(n: i64, r: i64, fact: &[i64], invfact: &[i64]) -> i64 {
    if n < 0 || r < 0 || r > n {
        return 0;
    }
    let ni = n as usize;
    let ri = r as usize;
    let n_minus_r = (n - r) as usize;
    (((fact[ni] * invfact[ri]) % MOD) * invfact[n_minus_r]) % MOD
}

fn main() {
    // Precompute factorials and inverse factorials up to 1_000_000 (inclusive),
    // matching the C code's fixed-table approach.
    let mut fact = vec![0i64; MAX + 1];
    let mut invfact = vec![0i64; MAX + 1];

    fact[0] = 1;
    for i in 1..=MAX {
        fact[i] = (fact[i - 1] * (i as i64)) % MOD;
    }
    invfact[MAX] = mod_pow(fact[MAX], MOD - 2);
    for i in (0..MAX).rev() {
        invfact[i] = (invfact[i + 1] * ((i + 1) as i64)) % MOD;
    }

    // Read input n
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n: i64 = input.split_whitespace().next().unwrap().parse().unwrap();

    // temp[k] for k in [0, n-1]
    // Note: Spec precondition requires 0 <= n <= 1_000_000.
    let n_usize = n as usize;
    let mut temp = vec![0i64; n_usize];

    for k in 0..n_usize {
        let k_i64 = k as i64;
        let val = choose(k_i64 - 1, (n - 1) - k_i64, &fact, &invfact);
        let val = (val * fact[k]) % MOD;
        let val = (val * fact[(n - 1 - k_i64) as usize]) % MOD;
        temp[k] = val;
    }

    let mut ans: i64 = 0;
    for k in 1..n_usize {
        let diff = (temp[k] - temp[k - 1] + MOD) % MOD;
        ans = (ans + (k as i64) * diff) % MOD;
    }

    println!("{}", ans);
}
