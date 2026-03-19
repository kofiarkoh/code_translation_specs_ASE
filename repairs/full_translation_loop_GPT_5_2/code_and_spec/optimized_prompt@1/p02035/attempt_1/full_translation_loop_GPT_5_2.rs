use std::io::{self, Read};

fn mod_pow(mut a: i64, mut e: i64, m: i64) -> i64 {
    // Computes a^e mod m for e >= 0, m > 0.
    a %= m;
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

fn make_fact(n: usize, m: i64, fact: &mut [i64], invfact: &mut [i64]) {
    // Matches the C code: uses Fermat inverse via a^(m-2) mod m.
    // NOTE: This requires m to be prime for correctness; the SPEC does not state this,
    // but the provided C code relies on it.
    fact[0] = 1 % m;
    for i in 0..n {
        fact[i + 1] = (fact[i] * (i as i64 + 1)) % m;
    }
    invfact[n] = mod_pow(fact[n], m - 2, m);
    for i in (1..=n).rev() {
        invfact[i - 1] = (invfact[i] * (i as i64)) % m;
    }
}

fn choose(n: i32, r: i32, m: i64, fact: &[i64], invfact: &[i64]) -> i64 {
    // Per SPEC: if n or r negative or n-r negative, return 0.
    if n < 0 || r < 0 || n - r < 0 {
        return 0;
    }
    let n = n as usize;
    let r = r as usize;
    let nr = (n as i32 - r as i32) as usize;
    (((fact[n] * invfact[r]) % m) * invfact[nr]) % m
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: i32 = it.next().unwrap().parse().unwrap();
    let m_i32: i32 = it.next().unwrap().parse().unwrap();
    let m: i64 = m_i32 as i64;

    // Factorials up to 2010 as in the C code / SPEC invariant.
    let maxn: usize = 2010;
    let mut fact = vec![0i64; maxn + 1];
    let mut invfact = vec![0i64; maxn + 1];
    make_fact(maxn, m, &mut fact, &mut invfact);

    let mut ans: i64 = 0;
    for i in 1..n {
        let mut temp: i64 = 0;
        for j in 0..(n - i) {
            // aaa = ( (2^(n-i-j) - 1)^i ) * C(n-i, j) mod m
            let pow2 = mod_pow(2, (n - i - j) as i64, m);
            let base = (pow2 - 1).rem_euclid(m);
            let part = mod_pow(base, i as i64, m);
            let comb = choose(n - i, j, m, &fact, &invfact);
            let aaa = (part * comb) % m;

            if (j & 1) == 1 {
                temp -= aaa;
            } else {
                temp += aaa;
            }
        }

        let temp_mod = temp.rem_euclid(m);
        let p1 = mod_pow(2, (i as i64) * (i as i64 - 1) / 2, m);
        let ni = (n - i) as i64;
        let p2 = mod_pow(2, ni * (ni - 1) / 2, m);
        let comb_ni = choose(n, i, m, &fact, &invfact);

        let add = (((temp_mod * p1) % m) * p2) % m;
        let add = (add * comb_ni) % m;
        ans = (ans + add) % m;
    }

    // Output in [0, m-1]
    let out = ans.rem_euclid(m);
    println!("{out}");
}
