use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;
const MAX: usize = 100010;

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
            fact[i + 1] = (fact[i] * (i + 1) as i64) % MOD;
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
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace();
    let n: i64 = nums.next().unwrap().parse().unwrap();
    let m: i64 = nums.next().unwrap().parse().unwrap();

    let factorials = Factorials::new(2000);
    let mut s = 0;

    for i in 0..=m {
        let sign = if (m - i) % 2 == 0 { 1 } else { -1 };
        s = (s + factorials.choose(m, i) * pom(i, n, MOD) * sign) % MOD;
    }

    println!("{}", (s + MOD) % MOD);
}
