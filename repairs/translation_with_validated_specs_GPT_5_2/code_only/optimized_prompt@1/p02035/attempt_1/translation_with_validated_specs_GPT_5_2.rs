use std::io::{self, Read};

type Ll = i64;

fn pom(mut a: Ll, mut n: Ll, m: Ll) -> Ll {
    // Modular exponentiation mirroring the C loop structure.
    let mut x: Ll = 1;
    a = a.rem_euclid(m);
    while n != 0 {
        if (n & 1) != 0 {
            x = (x * a).rem_euclid(m);
        }
        a = (a * a).rem_euclid(m);
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
        fact[i + 1] = (fact[i] * (i as Ll + 1)).rem_euclid(m);
    }
    invfact[n] = invp(fact[n], m);
    for i in (1..=n).rev() {
        invfact[i - 1] = (invfact[i] * (i as Ll)).rem_euclid(m);
    }
}

fn choose(n: i32, r: i32, m: Ll, fact: &[Ll], invfact: &[Ll]) -> Ll {
    if n < 0 || r < 0 || n - r < 0 {
        return 0;
    }
    let n = n as usize;
    let r = r as usize;
    let nr = (n as i32 - r as i32) as usize;
    (((fact[n] * invfact[r]).rem_euclid(m)) * invfact[nr]).rem_euclid(m)
}

fn main() {
    // Read all input, parse as ints like scanf("%d%d", ...)
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();
    let n: i32 = it.next().unwrap().parse().unwrap();
    let m_in: i32 = it.next().unwrap().parse().unwrap();
    let m: Ll = m_in as Ll;

    // Arrays sized as in C (100010), but we only need up to 2010.
    let mut fact = vec![0 as Ll; 100010];
    let mut invfact = vec![0 as Ll; 100010];
    makefact(2010, m, &mut fact, &mut invfact);

    let mut ans: Ll = 0;
    for i in 1..(n as Ll) {
        let mut temp: Ll = 0;
        for j in 0..((n as Ll) - i) {
            // aaa = pom(pom(2,(n-i-j),m)-1,i,m) * choose(n-i,j,m) % m;
            let inner = pom(2, (n as Ll) - i - j, m) - 1;
            let aaa = (pom(inner, i, m)
                * choose((n as Ll - i) as i32, j as i32, m, &fact, &invfact))
                .rem_euclid(m);
            if (j & 1) != 0 {
                temp -= aaa;
            } else {
                temp += aaa;
            }
        }

        let term1 = (temp.rem_euclid(m) * pom(2, i * (i - 1) / 2, m)).rem_euclid(m);
        let ni = (n as Ll) - i;
        let term2 = (term1 * pom(2, ni * (ni - 1) / 2, m)).rem_euclid(m);
        let term3 = (term2 * choose(n, i as i32, m, &fact, &invfact)).rem_euclid(m);
        ans += term3;
    }

    let out = (ans.rem_euclid(m) + m).rem_euclid(m);
    println!("{}", out);
}
