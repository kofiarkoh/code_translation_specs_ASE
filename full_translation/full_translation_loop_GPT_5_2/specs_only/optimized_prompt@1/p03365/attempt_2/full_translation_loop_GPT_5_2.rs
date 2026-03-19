use std::io::{self, Read};

const MOD: u64 = 1_000_000_007;

fn mod_pow(mut a: u64, mut e: u64) -> u64 {
    let mut r: u64 = 1;
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

fn mod_inv(a: u64) -> u64 {
    // MOD is prime.
    mod_pow(a, MOD - 2)
}

fn comb(n: usize, r: isize, fact: &[u64], ifact: &[u64]) -> u64 {
    // Specification: if r is out of bounds, return 0.
    if r < 0 {
        return 0;
    }
    let r = r as usize;
    if r > n {
        return 0;
    }
    fact[n] * ifact[r] % MOD * ifact[n - r] % MOD
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();
    let token = input.split_whitespace().next().unwrap_or("");

    // Specification: if input n is negative, output 0.
    let n_i64: i64 = match token.parse::<i64>() {
        Ok(v) => v,
        Err(_) => {
            println!("0");
            return;
        }
    };
    if n_i64 < 0 {
        println!("0");
        return;
    }
    let n: usize = n_i64 as usize;

    // Precompute factorials and inverse factorials up to n.
    let mut fact = vec![0u64; n + 1];
    let mut ifact = vec![0u64; n + 1];

    fact[0] = 1;
    for i in 1..=n {
        fact[i] = fact[i - 1] * (i as u64) % MOD;
    }

    ifact[n] = mod_inv(fact[n]);
    for i in (1..=n).rev() {
        ifact[i - 1] = ifact[i] * (i as u64) % MOD;
    }

    // Compute: sum_{k=0..n} C(n,k) * k!  (mod MOD)
    // This uses factorials and modular arithmetic; r-out-of-bounds behavior is
    // preserved by comb() (though k is always in range here).
    let mut ans: u64 = 0;
    for k in 0..=n {
        let c = comb(n, k as isize, &fact, &ifact);
        ans = (ans + c * fact[k]) % MOD;
    }

    println!("{}", ans);
}
