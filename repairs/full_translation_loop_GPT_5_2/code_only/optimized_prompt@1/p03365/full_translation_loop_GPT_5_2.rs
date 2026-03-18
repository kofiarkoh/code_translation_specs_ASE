use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

fn mod_pow(mut a: i64, mut n: i64, m: i64) -> i64 {
    // Mirrors the C function `pom` (binary exponentiation, all ops mod m).
    let mut x: i64 = 1;
    a %= m;
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
    let n_ri = (n - r) as usize;
    (((fact[ni] * invfact[ri]) % MOD) * invfact[n_ri]) % MOD
}

fn main() {
    // Read exactly one integer like scanf("%lld",&n)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n: i64 = input.split_whitespace().next().unwrap().parse().unwrap();

    // Arrays sized to match the C globals (1000010), though we only need up to 1_000_000.
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

    // temp[k] for k in [0, n)
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
        // (temp[k]-temp[k-1]+MOD) in C, then multiplied by k and added to ans, all mod MOD.
        let diff = (tk - tkm1 + MOD) % MOD;
        ans = (ans + (k % MOD) * diff) % MOD;
    }

    print!("{ans}\n");
}
