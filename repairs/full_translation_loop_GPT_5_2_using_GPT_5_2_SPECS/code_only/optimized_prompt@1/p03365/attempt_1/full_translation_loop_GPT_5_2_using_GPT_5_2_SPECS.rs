use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

fn mod_pow(mut a: i64, mut n: i64, m: i64) -> i64 {
    // Mirrors the C function `pom` (binary exponentiation, all operations mod m).
    a %= m;
    let mut x: i64 = 1;
    while n != 0 {
        if (n & 1) != 0 {
            x = (x * a) % m;
        }
        a = (a * a) % m;
        n /= 2;
    }
    x
}

fn choose(n: i64, r: i64, fact: &[i64], invfact: &[i64]) -> i64 {
    if n < 0 || r < 0 || r > n {
        return 0;
    }
    let ni = n as usize;
    let ri = r as usize;
    let nri = (n - r) as usize;
    (((fact[ni] * invfact[ri]) % MOD) * invfact[nri]) % MOD
}

fn main() {
    // Arrays sized to match the C globals.
    let mut fact = vec![0i64; 1_000_010];
    let mut invfact = vec![0i64; 1_000_010];
    let mut temp = vec![0i64; 1_000_010];

    fact[0] = 1;
    for i in 1..1_000_005usize {
        fact[i] = (fact[i - 1] * (i as i64)) % MOD;
    }

    invfact[1_000_000] = mod_pow(fact[1_000_000], MOD - 2, MOD);
    for i in (0..=999_999usize).rev() {
        invfact[i] = (invfact[i + 1] * ((i as i64) + 1)) % MOD;
    }

    // Read n
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n: i64 = input.split_whitespace().next().unwrap().parse().unwrap();

    // The original C code assumes n <= 1_000_000 (due to precomputed factorials and temp size).
    // We preserve behavior for valid inputs; for invalid larger n, Rust would panic on bounds.
    for k in 0..n {
        let val = choose(k - 1, n - 1 - k, &fact, &invfact);
        let fk = fact[k as usize];
        let fn1k = fact[(n - 1 - k) as usize];
        temp[k as usize] = (((val * fk) % MOD) * fn1k) % MOD;
    }

    let mut ans: i64 = 0;
    for k in 1..n {
        let tk = temp[k as usize];
        let tkm1 = temp[(k - 1) as usize];
        // (temp[k]-temp[k-1]+MOD) in C, then multiplied by k and accumulated mod MOD.
        let diff = (tk - tkm1 + MOD) % MOD;
        ans = (ans + (k % MOD) * diff) % MOD;
    }

    print!("{ans}\n");
}
