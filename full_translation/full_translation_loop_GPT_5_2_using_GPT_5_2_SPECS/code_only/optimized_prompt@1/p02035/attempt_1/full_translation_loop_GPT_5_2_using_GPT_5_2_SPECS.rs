use std::io::{self, Read};

type Ll = i64;

fn pom(mut a: Ll, mut n: Ll, m: i32) -> Ll {
    // Modular exponentiation mirroring the C code's behavior.
    // Note: C uses signed long long; we keep i64 and use % with possible negatives.
    let mm = m as Ll;
    a %= mm;
    let mut x: Ll = 1;
    while n != 0 {
        if (n & 1) != 0 {
            x = (x * a) % mm;
        }
        a = (a * a) % mm;
        n /= 2;
    }
    x
}

fn invp(a: Ll, p: i32) -> Ll {
    // Fermat inverse as in C: a^(p-2) mod p
    pom(a, (p as Ll) - 2, p)
}

fn makefact(n: usize, m: i32, fact: &mut [Ll], invfact: &mut [Ll]) {
    let mm = m as Ll;
    fact[0] = 1;
    for i in 0..n {
        fact[i + 1] = (fact[i] * ((i as Ll) + 1)) % mm;
    }
    invfact[n] = invp(fact[n], m);
    for i in (1..=n).rev() {
        invfact[i - 1] = (invfact[i] * (i as Ll)) % mm;
    }
}

fn choose(n: i32, r: i32, m: i32, fact: &[Ll], invfact: &[Ll]) -> Ll {
    if n < 0 || r < 0 || n - r < 0 {
        return 0;
    }
    let nn = n as usize;
    let rr = r as usize;
    let mm = m as Ll;
    (((fact[nn] * invfact[rr]) % mm) * invfact[nn - rr]) % mm
}

fn main() {
    // Read all input, parse two ints.
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();
    let n: i32 = it.next().unwrap().parse().unwrap();
    let m: i32 = it.next().unwrap().parse().unwrap();

    // C allocates 100010, but only uses up to 2010.
    let maxn: usize = 2010;
    let mut fact = vec![0 as Ll; maxn + 1];
    let mut invfact = vec![0 as Ll; maxn + 1];
    makefact(maxn, m, &mut fact, &mut invfact);

    let mut ans: Ll = 0;
    for i in 1..(n as Ll) {
        let mut temp: Ll = 0;
        for j in 0..((n as Ll) - i) {
            // aaa = pom(pom(2,(n-i-j),m)-1,i,m) * choose(n-i,j,m) % m;
            let inner = pom(2, (n as Ll) - i - j, m) - 1;
            let aaa = (pom(inner, i, m)
                * choose((n as i32) - (i as i32), j as i32, m, &fact, &invfact))
                % (m as Ll);
            if (j % 2) != 0 {
                temp -= aaa;
            } else {
                temp += aaa;
            }
        }

        let mm = m as Ll;
        let part1 = pom(2, i * (i - 1) / 2, m);
        let ni = (n as Ll) - i;
        let part2 = pom(2, ni * (ni - 1) / 2, m);
        let comb = choose(n, i as i32, m, &fact, &invfact);

        // ans += temp%m * part1 %m * part2 %m * comb %m;
        ans += (((((temp % mm) * part1) % mm) * part2) % mm * comb) % mm;
    }

    let mm = m as Ll;
    let out = ((ans % mm) + mm) % mm;
    print!("{}\n", out);
}
