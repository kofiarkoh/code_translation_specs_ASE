const MOD: i64 = 1_000_000_007;

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

fn choose(n: usize, r: usize, fact: &[i64], invfact: &[i64]) -> i64 {
    if n < r {
        return 0;
    }
    (fact[n] * invfact[r] % MOD * invfact[n - r] % MOD) % MOD
}

fn main() {
    let mut fact = vec![0; 1_000_010];
    let mut invfact = vec![0; 1_000_010];
    let mut temp = vec![0; 1_000_010];

    fact[0] = 1;
    for i in 1..1_000_010 {
        fact[i] = (fact[i - 1] * i as i64) % MOD;
    }

    invfact[1_000_000] = pom(fact[1_000_000], MOD - 2, MOD);
    for i in (0..999_999).rev() {
        invfact[i] = (invfact[i + 1] * (i + 1) as i64) % MOD;
    }

    let mut n = String::new();
    std::io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: usize = n.trim().parse().expect("Please enter a valid number");

    for k in 0..n {
        temp[k] = (choose(k, n - 1 - k, &fact, &invfact) * fact[k] % MOD * fact[(n - 1 - k)] % MOD) % MOD;
    }

    let mut ans = 0;
    for k in 1..n {
        ans = (ans + k as i64 * (temp[k] - temp[k - 1] + MOD) % MOD) % MOD;
    }

    println!("{}", ans);
}
