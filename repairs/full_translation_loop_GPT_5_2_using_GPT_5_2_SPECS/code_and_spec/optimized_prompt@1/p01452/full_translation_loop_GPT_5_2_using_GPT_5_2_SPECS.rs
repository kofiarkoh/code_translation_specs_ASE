use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;
const MAX_MEMO: usize = 220_001;

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

fn mod_inv(a: i64) -> i64 {
    // MOD is prime per spec assumptions; inverse exists for nonzero a mod MOD.
    mod_pow(a, MOD - 2)
}

fn mod_cmb(n: i64, r: i64, fact: &[i64]) -> i64 {
    if n < 0 || r < 0 || n < r {
        return 0;
    }
    let ni = n as usize;
    let ri = r as usize;
    let n_ri = (n - r) as usize;

    let a1 = fact[ni];
    let a2 = fact[ri];
    let a3 = fact[n_ri];
    let denom = (a2 * a3) % MOD;
    (a1 * mod_inv(denom)) % MOD
}

fn main() {
    // Read exactly three integers; behavior on malformed input is unspecified by spec.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let x: i64 = it.next().unwrap().parse().unwrap();
    let y: i64 = it.next().unwrap().parse().unwrap();
    let k: i64 = it.next().unwrap().parse().unwrap();

    let nmax_i64 = x + y + 2 * k;
    let nmax = nmax_i64 as usize;

    // Spec precondition: x + y + 2k must not exceed 220000.
    // We still allocate fixed-size table as in C (220001).
    let mut fact = vec![0i64; MAX_MEMO];
    fact[0] = 1;
    if nmax >= 1 {
        fact[1] = 1;
    }
    for i in 2..=nmax {
        fact[i] = ((i as i64) * fact[i - 1]) % MOD;
    }

    let mut ans: i64 = 0;
    for i in 0..=k {
        let x_n = x + 2 * (k - i);
        let x_r = k - i;
        let xgr = (mod_cmb(x_n, x_r, &fact) - mod_cmb(x_n, x_r - 1, &fact) + MOD) % MOD;

        let y_n = y + 2 * i;
        let y_r = i;
        let ygr = (mod_cmb(y_n, y_r, &fact) - mod_cmb(y_n, y_r - 1, &fact) + MOD) % MOD;

        let comb = mod_cmb(x + y + 2 * k, y + 2 * i, &fact);
        ans = (ans + ((xgr * ygr) % MOD) * comb) % MOD;
    }

    // Match C behavior: print as non-negative modulo MOD with newline.
    let out = (ans + MOD) % MOD;
    println!("{}", out);
}
