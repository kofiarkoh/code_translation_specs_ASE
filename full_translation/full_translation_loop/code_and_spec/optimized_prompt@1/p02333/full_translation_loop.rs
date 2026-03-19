use std::io::{self, BufRead};

const MOD: i64 = 1_000_000_007;

fn pom(a: i64, n: i64, m: i64) -> i64 {
    let mut x = 1;
    let mut a = a % m;
    let mut n = n;
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

fn makefact(n: usize, m: i64) -> (Vec<i64>, Vec<i64>) {
    let mut fact = vec![0; n + 1];
    let mut invfact = vec![0; n + 1];
    fact[0] = 1;
    for i in 0..n {
        fact[i + 1] = (fact[i] * (i + 1) as i64) % m;
    }
    invfact[n] = invp(fact[n], m);
    for i in (1..=n).rev() {
        invfact[i - 1] = (invfact[i] * i as i64) % m;
    }
    (fact, invfact)
}

fn choose(n: i64, r: i64, m: i64) -> i64 {
    if n < 0 || r < 0 || n - r < 0 {
        return 0;
    }
    let (fact, invfact) = makefact(n as usize, m);
    (fact[n as usize] * invfact[r as usize] % m * invfact[(n - r) as usize] % m) % m
}

fn main() {
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    let mut nums = input.split_whitespace().map(|s| s.parse::<i64>().unwrap());
    let n = nums.next().unwrap();
    let m = nums.next().unwrap();

    // Handle precondition: if n or m is negative, output 0
    if n < 0 || m < 0 {
        println!("0");
        return;
    }

    let mut s = 0;
    for i in 0..=m {
        s = (s + choose(m, i, MOD) * pom(i, n, MOD) * if (m - i) % 2 == 0 { 1 } else { -1 }) % MOD;
    }
    println!("{}", (s + MOD) % MOD);
}
