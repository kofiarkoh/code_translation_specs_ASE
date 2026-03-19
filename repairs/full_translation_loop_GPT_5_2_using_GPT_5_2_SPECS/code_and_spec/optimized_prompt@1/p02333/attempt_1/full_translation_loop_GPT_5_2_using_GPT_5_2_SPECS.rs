use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;
const MAX_M: usize = 2000;

fn mod_pow(mut a: i64, mut e: i64, m: i64) -> i64 {
    a %= m;
    let mut x: i64 = 1;
    while e > 0 {
        if (e & 1) == 1 {
            x = (x * a) % m;
        }
        a = (a * a) % m;
        e >>= 1;
    }
    x
}

fn mod_inv(a: i64, p: i64) -> i64 {
    // Fermat's little theorem; p is prime in this problem.
    mod_pow(a, p - 2, p)
}

fn make_fact(n: usize, m: i64) -> (Vec<i64>, Vec<i64>) {
    let mut fact = vec![0i64; n + 1];
    let mut invfact = vec![0i64; n + 1];
    fact[0] = 1;
    for i in 0..n {
        fact[i + 1] = fact[i] * (i as i64 + 1) % m;
    }
    invfact[n] = mod_inv(fact[n], m);
    for i in (1..=n).rev() {
        invfact[i - 1] = invfact[i] * (i as i64) % m;
    }
    (fact, invfact)
}

fn choose(n: i64, r: i64, fact: &[i64], invfact: &[i64], m: i64) -> i64 {
    // Spec invariant: invalid parameters contribute 0.
    if n < 0 || r < 0 || n - r < 0 {
        return 0;
    }
    let n = n as usize;
    let r = r as usize;
    fact[n] * invfact[r] % m * invfact[n - r] % m
}

fn main() {
    // Precompute factorials up to 2000 as required by the spec.
    let (fact, invfact) = make_fact(MAX_M, MOD);

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = it.next().unwrap().parse().unwrap();
    let m: i64 = it.next().unwrap().parse().unwrap();

    let mut s: i64 = 0;
    for i in 0..=m {
        let c = choose(m, i, &fact, &invfact, MOD);
        let p = mod_pow(i, n, MOD);
        let sign = if ((m - i) & 1) == 0 { 1i64 } else { -1i64 };
        s = (s + c * p % MOD * sign) % MOD;
    }

    // Normalize to [0, MOD-1]
    let ans = (s + MOD) % MOD;
    print!("{ans}\n");
}
