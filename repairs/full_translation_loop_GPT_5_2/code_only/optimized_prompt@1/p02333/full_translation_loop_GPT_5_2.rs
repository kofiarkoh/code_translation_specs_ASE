use std::io::{self, Read};

type Ll = i64;

const MOD: Ll = 1_000_000_007;

fn pom(mut a: Ll, mut n: Ll, m: Ll) -> Ll {
    // Modular exponentiation, matching the C loop structure.
    let mut x: Ll = 1;
    a %= m;
    while n != 0 {
        if (n & 1) != 0 {
            x = (x * a) % m;
        }
        a = (a * a) % m;
        n /= 2;
    }
    x
}

fn invp(a: Ll, p: Ll) -> Ll {
    // Fermat inverse as in C: a^(p-2) mod p
    pom(a, p - 2, p)
}

fn makefact(n: usize, m: Ll, fact: &mut [Ll], invfact: &mut [Ll]) {
    fact[0] = 1;
    for i in 0..n {
        fact[i + 1] = (fact[i] * (i as Ll + 1)) % m;
    }
    invfact[n] = invp(fact[n], m);
    for i in (1..=n).rev() {
        invfact[i - 1] = (invfact[i] * (i as Ll)) % m;
    }
}

fn choose(n: i32, r: i32, m: Ll, fact: &[Ll], invfact: &[Ll]) -> Ll {
    if n < 0 || r < 0 || n - r < 0 {
        0
    } else {
        let n = n as usize;
        let r = r as usize;
        let nr = (n as i32 - r as i32) as usize;
        (((fact[n] * invfact[r]) % m) * invfact[nr]) % m
    }
}

fn main() {
    // Precompute factorials up to 2000 as in C.
    let mut fact = vec![0i64; 2001];
    let mut invfact = vec![0i64; 2001];
    makefact(2000, MOD, &mut fact, &mut invfact);

    // Read input exactly like scanf("%d%d", &n, &m)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let n: i32 = it.next().unwrap().parse().unwrap();
    let m: i32 = it.next().unwrap().parse().unwrap();

    let mut s: Ll = 0;
    for i in 0..=m {
        let c = choose(m, i, MOD, &fact, &invfact);
        let p = pom(i as Ll, n as Ll, MOD);
        let sign: Ll = if ((m - i) % 2) == 0 { 1 } else { -1 };
        s = (s + c * p % MOD * sign) % MOD;
    }

    // Match C: printf("%lld\n", (s+MOD)%MOD);
    let ans = (s + MOD) % MOD;
    print!("{ans}\n");
}
