use std::io;

const MOD: i64 = 1_000_000_007;
const MAX_N: usize = 1_000_001;

fn pom(a: i64, mut n: i64, m: i64) -> i64 {
    let mut x = 1;
    let mut a = a % m;
    while n > 0 {
        if n % 2 == 1 {
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
    (fact[n as usize] * invfact[r as usize] % MOD * invfact[(n - r) as usize] % MOD) % MOD
}

fn main() {
    let mut fact = vec![0; MAX_N];
    let mut invfact = vec![0; MAX_N];
    let mut temp = vec![0; MAX_N];

    fact[0] = 1;
    for i in 1..MAX_N {
        fact[i] = (fact[i - 1] * i as i64) % MOD;
    }

    invfact[MAX_N - 1] = pom(fact[MAX_N - 1], MOD - 2, MOD);
    for i in (0..MAX_N - 1).rev() {
        invfact[i] = (invfact[i + 1] * (i + 1) as i64) % MOD;
    }

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i64 = input.trim().parse().expect("Input not a valid integer");

    for k in 0..n {
        temp[k as usize] = (choose(k - 1, n - 1 - k, &fact, &invfact) * fact[k as usize] % MOD * fact[(n - 1 - k) as usize] % MOD) % MOD;
    }

    let mut ans = 0;
    for k in 1..n {
        ans = (ans + k * (temp[k as usize] - temp[(k - 1) as usize] + MOD) % MOD) % MOD;
    }

    println!("{}", ans);
}
