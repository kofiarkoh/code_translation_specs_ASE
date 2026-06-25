use std::io::{self, Read};

const MOD: u64 = 1_000_000_007;
const MAXN: usize = 1_000_000;

fn mod_pow(mut a: u64, mut e: u64) -> u64 {
    let mut r: u64 = 1;
    while e > 0 {
        if (e & 1) == 1 {
            r = (r * a) % MOD;
        }
        a = (a * a) % MOD;
        e >>= 1;
    }
    r
}

fn main() {
    // Input parsing: behavior is undefined by spec if parsing fails; we keep it simple.
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let n_i64: i64 = s.split_whitespace().next().unwrap().parse().unwrap();
    let n: usize = n_i64 as usize;

    // Precompute factorials and inverse factorials up to MAXN.
    // Spec requires 0 <= n <= 1_000_000; otherwise behavior is undefined.
    let mut fact = vec![0u64; MAXN + 1];
    let mut invfact = vec![0u64; MAXN + 1];

    fact[0] = 1;
    for i in 1..=MAXN {
        fact[i] = (fact[i - 1] * (i as u64)) % MOD;
    }

    invfact[MAXN] = mod_pow(fact[MAXN], MOD - 2);
    for i in (1..=MAXN).rev() {
        invfact[i - 1] = (invfact[i] * (i as u64)) % MOD;
    }

    // Compute T(k) for k = 0..n-1, then ans = sum_{k=1}^{n-1} k*(T(k)-T(k-1)) mod MOD.
    // Binomial convention: C(a,b)=0 if a<0 or b<0 or b>a.
    let mut prev_t: u64 = 0;
    let mut ans: u64 = 0;

    for k in 0..n {
        let t = if k == 0 {
            // a = k-1 = -1 => binomial is 0 by definition.
            0
        } else {
            // a = k-1 >= 0
            let a = k - 1;
            // b = n-1-k; if n==0 then n-1 underflows in usize, but spec allows n=0.
            // We avoid underflow by checking the inequality in signed space.
            let b_i64 = (n as i64) - 1 - (k as i64);
            if b_i64 < 0 {
                0
            } else {
                let b = b_i64 as usize;
                if b > a {
                    0
                } else {
                    // T(k) = C(a,b) * k! * (n-1-k)! mod MOD
                    // C(a,b) = fact[a] * invfact[b] * invfact[a-b] mod MOD
                    let comb = (((fact[a] * invfact[b]) % MOD) * invfact[a - b]) % MOD;

                    // (n-1-k)! is only defined for n-1-k >= 0; ensured by b_i64 >= 0.
                    let nm1k = (n - 1) - k;
                    let t = (((comb * fact[k]) % MOD) * fact[nm1k]) % MOD;
                    t
                }
            }
        };

        if k >= 1 {
            let diff = (t + MOD - prev_t) % MOD;
            ans = (ans + ((k as u64) % MOD) * diff) % MOD;
        }
        prev_t = t;
    }

    // Output canonical residue in [0, MOD-1].
    println!("{}", ans % MOD);
}
