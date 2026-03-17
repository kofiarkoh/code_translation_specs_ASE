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

fn invp(a: i64, p: i64) -> i64 {
    pom(a, p - 2, p)
}

struct Factorials {
    fact: Vec<i64>,
    invfact: Vec<i64>,
}

impl Factorials {
    fn new(n: usize) -> Self {
        let mut fact = vec![0; n + 1];
        let mut invfact = vec![0; n + 1];
        fact[0] = 1;

        for i in 0..n {
            fact[i + 1] = (fact[i] * (i + 1) % MOD);
        }

        invfact[n] = invp(fact[n], MOD);
        for i in (1..=n).rev() {
            invfact[i - 1] = (invfact[i] * i as i64) % MOD;
        }

        Factorials { fact, invfact }
    }

    fn choose(&self, n: i64, r: i64) -> i64 {
        if n < 0 || r < 0 || n < r {
            return 0;
        }
        (self.fact[n as usize] * self.invfact[r as usize] % MOD * self.invfact[(n - r) as usize] % MOD) % MOD
    }
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let nums: Vec<i64> = input.split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    
    let n = nums[0];
    let m = nums[1];

    if n < 0 || m < 0 || m > 2000 {
        println!("0");
        return;
    }

    let factorials = Factorials::new(2000);
    let mut s = 0;

    for i in 0..=m {
        let term = factorials.choose(m, i) * pom(i, n, MOD) % MOD;
        s = (s + term * if (m - i) % 2 == 0 { 1 } else { -1 }) % MOD;
    }

    println!("{}", (s + MOD) % MOD);
}
