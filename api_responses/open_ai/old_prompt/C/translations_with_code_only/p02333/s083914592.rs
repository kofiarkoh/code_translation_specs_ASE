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

// Factorial and inverse factorial
const MAX: usize = 100010;
static mut FACT: [i64; MAX] = [0; MAX];
static mut INVFACT: [i64; MAX] = [0; MAX];

fn makefact(n: usize, m: i64) {
    unsafe {
        FACT[0] = 1;
        for i in 0..n {
            FACT[i + 1] = (FACT[i] * (i + 1) % m);
        }
        INVFACT[n] = invp(FACT[n], m);
        for i in (1..=n).rev() {
            INVFACT[i - 1] = (INVFACT[i] * i as i64) % m;
        }
    }
}

fn choose(n: i64, r: i64, m: i64) -> i64 {
    if n < 0 || r < 0 || n - r < 0 {
        return 0;
    }
    unsafe {
        (FACT[n as usize] * INVFACT[r as usize] % m * INVFACT[(n - r) as usize] % m) % m
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();
    
    makefact(2000, MOD);
    
    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace();
    let n: i64 = nums.next().unwrap().parse().unwrap();
    let m: i64 = nums.next().unwrap().parse().unwrap();
    
    let mut s = 0;
    for i in 0..=m {
        s = (s + choose(m, i, MOD) * pom(i, n, MOD) * if (m - i) % 2 == 0 { 1 } else { -1 }) % MOD;
    }
    
    println!("{}", (s + MOD) % MOD);
}
