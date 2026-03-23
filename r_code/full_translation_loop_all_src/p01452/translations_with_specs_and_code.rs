use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;
const MEMO_CAP: usize = 220_001;

fn mod_norm(x: i64) -> i64 {
    let r = x % MOD;
    if r < 0 { r + MOD } else { r }
}

fn mod_pow(mut a: i64, mut e: i64) -> i64 {
    a = mod_norm(a);
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
    // MOD is prime; under the spec preconditions, a is coprime to MOD.
    mod_pow(a, MOD - 2)
}

fn init_table(memo: &mut [i64], n: usize) {
    // Mirrors the C behavior: memo[1]=1; for i=2..=n memo[i]=i*memo[i-1] mod MOD.
    // Then caller sets memo[0]=1.
    if n >= 1 {
        memo[1] = 1;
    }
    for i in 2..=n {
        memo[i] = ((i as i64) * memo[i - 1]) % MOD;
    }
}

fn mod_cmb(memo: &[i64], n: i64, r: i64) -> i64 {
    // Any invalid parameters yield 0, per spec.
    if n < 0 || r < 0 || n < r {
        return 0;
    }
    let n_us = n as usize;
    let r_us = r as usize;
    let nr_us = (n - r) as usize;

    let a1 = memo[n_us];
    let a2 = memo[r_us];
    let a3 = memo[nr_us];
    let denom = (a2 * a3) % MOD;
    (a1 * mod_inv(denom)) % MOD
}

fn main() {
    // Read exactly three integers; malformed input => unspecified (as in spec).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let x: i64 = it.next().unwrap().parse().unwrap();
    let y: i64 = it.next().unwrap().parse().unwrap();
    let k: i64 = it.next().unwrap().parse().unwrap();

    // Precompute factorials up to x+y+2k (inclusive). Preconditions ensure nonnegative and within capacity.
    let nmax_i64 = x + y + 2 * k;
    let nmax = nmax_i64 as usize;

    // Fixed-size table, matching the C code's bounded internal capacity.
    let mut memo = vec![0i64; MEMO_CAP];
    init_table(&mut memo, nmax);
    memo[0] = 1;

    let mut ans: i64 = 0;
    for i in 0..=k {
        let x_n = x + 2 * (k - i);
        let x_r = k - i;
        let xgr = mod_norm(mod_cmb(&memo, x_n, x_r) - mod_cmb(&memo, x_n, x_r - 1));

        let y_n = y + 2 * i;
        let y_r = i;
        let ygr = mod_norm(mod_cmb(&memo, y_n, y_r) - mod_cmb(&memo, y_n, y_r - 1));

        let c = mod_cmb(&memo, x + y + 2 * k, y + 2 * i);
        ans = (ans + ((xgr * ygr) % MOD) * c) % MOD;
    }

    // Print canonical residue in [0, MOD-1] with newline, matching C I/O.
    println!("{}", mod_norm(ans));
}
