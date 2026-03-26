use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;

fn pom(mut a: i64, mut n: i64, m: i64) -> i64 {
    let mut x = 1;
    a %= m;
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

fn makefact(n: usize, m: i64, fact: &mut [i64], invfact: &mut [i64]) {
    fact[0] = 1;
    for i in 0..n {
        fact[i + 1] = fact[i] * (i + 1) as i64 % m; // Cast (i + 1) to i64
    }
    invfact[n] = invp(fact[n], m);
    for i in (1..=n).rev() {
        invfact[i - 1] = invfact[i] * i as i64 % m; // Cast i to i64
    }
}

fn choose(n: i64, r: i64, m: i64, fact: &[i64], invfact: &[i64]) -> i64 {
    if n < 0 || r < 0 || n - r < 0 {
        return 0;
    }
    (fact[n as usize] * invfact[r as usize] % m * invfact[(n - r) as usize] % m) % m
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();
    
    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace();
    let n: i64 = nums.next().unwrap().parse().unwrap();
    let m: i64 = nums.next().unwrap().parse().unwrap();

    let mut fact = vec![0; 100010];
    let mut invfact = vec![0; 100010];
    
    makefact(2000, MOD, &mut fact, &mut invfact);
    
    let mut s = 0;
    for i in 0..=m {
        let choose_val = choose(m, i, MOD, &fact, &invfact);
        let pom_val = pom(i, n, MOD);
        s = (s + choose_val * pom_val % MOD * if (m - i) % 2 == 0 { 1 } else { -1 }) % MOD;
    }
    
    println!("{}", (s + MOD) % MOD);
}
