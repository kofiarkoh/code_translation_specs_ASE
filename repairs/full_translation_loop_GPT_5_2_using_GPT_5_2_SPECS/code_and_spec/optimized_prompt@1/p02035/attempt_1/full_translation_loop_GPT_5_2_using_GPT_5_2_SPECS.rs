use std::io::{self, Read};

fn mod_pow(mut a: i64, mut e: i64, m: i64) -> i64 {
    // Computes a^e mod m, with m > 0.
    a %= m;
    if a < 0 {
        a += m;
    }
    let mut x: i64 = 1 % m;
    while e > 0 {
        if (e & 1) == 1 {
            x = (x * a) % m;
        }
        a = (a * a) % m;
        e >>= 1;
    }
    x
}

fn mod_inv(a: i64, p: i64) -> i64 {
    // SPEC precondition: p is prime and a != 0 mod p.
    mod_pow(a, p - 2, p)
}

fn make_fact(max_n: usize, m: i64) -> (Vec<i64>, Vec<i64>) {
    let mut fact = vec![0i64; max_n + 1];
    let mut invfact = vec![0i64; max_n + 1];
    fact[0] = 1 % m;
    for i in 0..max_n {
        fact[i + 1] = (fact[i] * ((i as i64) + 1)) % m;
    }
    invfact[max_n] = mod_inv(fact[max_n], m);
    for i in (1..=max_n).rev() {
        invfact[i - 1] = (invfact[i] * (i as i64)) % m;
    }
    (fact, invfact)
}

fn choose(n: i32, r: i32, m: i64, fact: &[i64], invfact: &[i64]) -> i64 {
    if n < 0 || r < 0 || n - r < 0 {
        return 0;
    }
    let n = n as usize;
    let r = r as usize;
    let nr = (n as i32 - r as i32) as usize;
    (((fact[n] * invfact[r]) % m) * invfact[nr]) % m
}

fn main() {
    // Read all input
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();
    let n: i32 = it.next().unwrap().parse().unwrap();
    let m_in: i64 = it.next().unwrap().parse().unwrap();
    let m = m_in;

    // Precompute factorials up to 2010 as in C code.
    let max_n = 2010usize;
    let (fact, invfact) = make_fact(max_n, m);

    let mut ans: i64 = 0;
    for i in 1..n {
        let mut temp: i64 = 0;
        let ni = n - i;
        for j in 0..ni {
            // aaa = ( (2^(n-i-j) - 1)^i ) * C(n-i, j)
            let exp1 = (n - i - j) as i64;
            let mut base = mod_pow(2, exp1, m) - 1;
            base %= m;
            if base < 0 {
                base += m;
            }
            let pow_part = mod_pow(base, i as i64, m);
            let comb = choose(ni, j, m, &fact, &invfact);
            let aaa = (pow_part * comb) % m;

            if (j & 1) == 1 {
                temp -= aaa;
            } else {
                temp += aaa;
            }
        }

        let temp_mod = temp % m;
        let ii = i as i64;
        let nii = (n - i) as i64;

        let p1 = mod_pow(2, ii * (ii - 1) / 2, m);
        let p2 = mod_pow(2, nii * (nii - 1) / 2, m);
        let cni = choose(n, i, m, &fact, &invfact);

        let term = (((temp_mod * p1) % m) * p2) % m;
        let term = (term * cni) % m;
        ans += term;
    }

    let out = ((ans % m) + m) % m;
    println!("{out}");
}
