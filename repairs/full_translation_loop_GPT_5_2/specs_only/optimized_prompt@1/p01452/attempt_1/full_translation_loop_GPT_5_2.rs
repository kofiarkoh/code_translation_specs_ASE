use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

fn mod_pow(mut a: i64, mut e: i64) -> i64 {
    a %= MOD;
    if a < 0 {
        a += MOD;
    }
    let mut r: i64 = 1;
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
    // MOD is treated as prime (common for combinatorics); inverse via Fermat.
    // If a == 0, this returns 0, which is a narrow, safe behavior under invalid states.
    if a % MOD == 0 {
        return 0;
    }
    mod_pow(a, MOD - 2)
}

fn main() {
    // Read three integers: n, k, m (names inferred only to structure computation).
    // If parsing fails or preconditions are violated, output 0.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();
    let mut it = input.split_whitespace();

    let a = it.next().and_then(|s| s.parse::<i64>().ok());
    let b = it.next().and_then(|s| s.parse::<i64>().ok());
    let c = it.next().and_then(|s| s.parse::<i64>().ok());

    let (n, k, m) = match (a, b, c) {
        (Some(n), Some(k), Some(m)) => (n, k, m),
        _ => {
            println!("0");
            return;
        }
    };

    // Preconditions: all non-negative, and first >= second.
    if n < 0 || k < 0 || m < 0 || n < k {
        println!("0");
        return;
    }

    // The specification describes:
    // - initializing a memoization table,
    // - computing combinations and modular inverses,
    // - updating the result based on combinatorial logic.
    //
    // The narrowest reasonable interpretation consistent with "three integers" and
    // "combinatorial value" is to compute a binomial coefficient C(n, k) modulo MOD.
    // The third input is accepted but not otherwise specified; to avoid inventing
    // extra behavior, it is not used in the computation.
    //
    // Efficient computation via factorial memoization up to n.
    let nn = match usize::try_from(n) {
        Ok(v) => v,
        Err(_) => {
            println!("0");
            return;
        }
    };
    let kk = match usize::try_from(k) {
        Ok(v) => v,
        Err(_) => {
            println!("0");
            return;
        }
    };

    // Memoization tables: factorials and inverse factorials.
    let mut fact = vec![0i64; nn + 1];
    let mut inv_fact = vec![0i64; nn + 1];

    fact[0] = 1;
    for i in 1..=nn {
        fact[i] = (fact[i - 1] * (i as i64)) % MOD;
    }

    inv_fact[nn] = mod_inv(fact[nn]);
    if inv_fact[nn] == 0 && nn != 0 {
        // Inverse doesn't exist (shouldn't happen for nn < MOD with prime MOD),
        // treat as invalid per "invalid combinations => output zero".
        println!("0");
        return;
    }
    for i in (1..=nn).rev() {
        inv_fact[i - 1] = (inv_fact[i] * (i as i64)) % MOD;
    }

    let mut res = fact[nn];
    res = (res * inv_fact[kk]) % MOD;
    res = (res * inv_fact[nn - kk]) % MOD;

    // Adjust to be non-negative.
    if res < 0 {
        res += MOD;
    }
    println!("{}", res);
}
