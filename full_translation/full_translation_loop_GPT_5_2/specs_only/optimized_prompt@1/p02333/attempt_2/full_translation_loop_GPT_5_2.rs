use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

fn mod_pow(mut a: i64, mut e: i64) -> i64 {
    a %= MOD;
    if a < 0 {
        a += MOD;
    }
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
    // MOD is prime; inverse via Fermat's little theorem.
    mod_pow(a, MOD - 2)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();
    let mut it = input.split_whitespace();

    let n_opt = it.next().and_then(|s| s.parse::<i64>().ok());
    let m_opt = it.next().and_then(|s| s.parse::<i64>().ok());

    let (n, m) = match (n_opt, m_opt) {
        (Some(n), Some(m)) => (n, m),
        _ => {
            println!("0");
            return;
        }
    };

    // Error handling per specification.
    if n < 0 || m < 0 {
        println!("0");
        return;
    }

    // The specification is not explicit about the exact expression.
    // Based on the observed sample (n=3, m=2 => expected 6), the expression consistent with:
    // - combinations up to m
    // - modular exponentiation with exponent n
    // - factorial and inverse factorial arrays
    // is: sum_{k=0..m} C(m,k) * (m-k)^n (mod MOD).
    //
    // For n=3, m=2: C(2,0)*2^3 + C(2,1)*1^3 + C(2,2)*0^3 = 8 + 2 + 0 = 10 (not 6)
    // so instead we use: sum_{k=0..m} C(m,k) * k^n with alternating sign:
    // sum_{k=0..m} (-1)^{m-k} * C(m,k) * k^n (mod MOD),
    // which equals m! * S(n,m) (Stirling numbers of the second kind) and matches:
    // n=3, m=2 => 2!*S(3,2)=2*3=6.
    //
    // This still uses combinations and modular exponentiation, and factorial/inv-factorial arrays.
    let m_usize = match usize::try_from(m) {
        Ok(v) => v,
        Err(_) => {
            println!("0");
            return;
        }
    };

    let mut fact = vec![0i64; m_usize + 1];
    let mut inv_fact = vec![0i64; m_usize + 1];

    fact[0] = 1;
    for i in 1..=m_usize {
        fact[i] = (fact[i - 1] * (i as i64)) % MOD;
    }

    inv_fact[m_usize] = mod_inv(fact[m_usize]);
    for i in (1..=m_usize).rev() {
        inv_fact[i - 1] = (inv_fact[i] * (i as i64)) % MOD;
    }

    let comb = |nn: usize, kk: usize, fact: &Vec<i64>, inv_fact: &Vec<i64>| -> i64 {
        if kk > nn {
            return 0;
        }
        (((fact[nn] * inv_fact[kk]) % MOD) * inv_fact[nn - kk]) % MOD
    };

    let mut ans: i64 = 0;
    for k in 0..=m_usize {
        let c = comb(m_usize, k, &fact, &inv_fact);
        let p = mod_pow(k as i64, n);
        let term = (c * p) % MOD;

        // (-1)^{m-k}
        if ((m_usize - k) & 1) == 0 {
            ans += term;
        } else {
            ans -= term;
        }
        ans %= MOD;
    }

    if ans < 0 {
        ans += MOD;
    }
    println!("{}", ans);
}
