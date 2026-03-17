use std::io::{self, BufRead};

const MAX_N: usize = 100010;

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

fn makefact(n: usize, m: i64, fact: &mut [i64; MAX_N], invfact: &mut [i64; MAX_N]) {
    fact[0] = 1;
    for i in 0..n {
        fact[i + 1] = (fact[i] * (i + 1) as i64) % m;
    }
    invfact[n] = invp(fact[n], m);
    for i in (1..=n).rev() {
        invfact[i - 1] = (invfact[i] * i as i64) % m;
    }
}

fn choose(n: i64, r: i64, m: i64, fact: &[i64; MAX_N], invfact: &[i64; MAX_N]) -> i64 {
    if n < 0 || r < 0 || n - r < 0 {
        return 0;
    }
    (fact[n as usize] * invfact[r as usize] % m * invfact[(n - r) as usize] % m) % m
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();
    
    let n: i64 = lines.next().unwrap().unwrap().parse().unwrap();
    let m: i64 = lines.next().unwrap().unwrap().parse().unwrap();

    let mut fact = [0; MAX_N];
    let mut invfact = [0; MAX_N];
    makefact(2010, m, &mut fact, &mut invfact);

    let mut ans = 0;
    for i in 1..n {
        let mut temp = 0;
        for j in 0..(n - i) {
            let aaa = (pom(pom(2, (n - i - j), m) - 1, i, m) * choose(n - i, j, m, &fact, &invfact)) % m;
            temp += if j % 2 == 1 { -aaa } else { aaa };
        }
        ans = (ans + (temp % m) * pom(2, i * (i - 1) / 2, m) % m * pom(2, (n - i) * (n - i - 1) / 2, m) % m * choose(n, i, m, &fact, &invfact) % m) % m;
    }
    
    println!("{}", (ans % m + m) % m);
}
