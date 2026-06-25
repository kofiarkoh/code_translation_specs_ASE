use std::io::{self, Read};

fn is_prime(x: usize) -> bool {
    if x < 2 {
        return false;
    }
    if x == 2 {
        return true;
    }
    if x % 2 == 0 {
        return false;
    }
    let mut d = 3usize;
    while d * d <= x {
        if x % d == 0 {
            return false;
        }
        d += 2;
    }
    true
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n: i32 = input.split_whitespace().next().unwrap().parse().unwrap();

    if n < 2 {
        println!("0");
        return;
    }

    // Fixed-size storage implied by the specification (supports up to ~100).
    // For non-primes, values are not used to define the result.
    let mut exp = vec![0i32; 101];

    // Compute prime exponents in n! by factoring each i in 2..=n.
    for i in 2..=(n as usize) {
        let mut x = i;
        let mut p = 2usize;
        while p * p <= x {
            while x % p == 0 {
                if p < exp.len() {
                    exp[p] += 1;
                }
                x /= p;
            }
            p += if p == 2 { 1 } else { 2 };
        }
        if x > 1 && x < exp.len() {
            exp[x] += 1;
        }
    }

    // Count primes with exponent >= thresholds.
    let mut c2 = 0i32;
    let mut c4 = 0i32;
    let mut c14 = 0i32;
    let mut c24 = 0i32;
    let mut c74 = 0i32;

    for k in 2..exp.len() {
        if !is_prime(k) {
            continue;
        }
        let e = exp[k];
        if e >= 2 {
            c2 += 1;
        }
        if e >= 4 {
            c4 += 1;
        }
        if e >= 14 {
            c14 += 1;
        }
        if e >= 24 {
            c24 += 1;
        }
        if e >= 74 {
            c74 += 1;
        }
    }

    // Number of divisors with exactly 75 divisors:
    // 75 = 75
    // 75 = 25 * 3
    // 75 = 15 * 5
    // 75 = 5 * 5 * 3
    //
    // In terms of (exponent+1) factors:
    // (75) -> exponent 74
    // (25,3) -> exponents (24,2)
    // (15,5) -> exponents (14,4)
    // (5,5,3) -> exponents (4,4,2)
    let mut ans: i64 = 0;

    // One prime with exponent >= 74
    ans += c74 as i64;

    // One prime with exponent >= 24 and a different prime with exponent >= 2
    if c24 > 0 && c2 > 1 {
        ans += (c24 as i64) * ((c2 - 1) as i64);
    }

    // One prime with exponent >= 14 and a different prime with exponent >= 4
    if c14 > 0 && c4 > 1 {
        ans += (c14 as i64) * ((c4 - 1) as i64);
    }

    // Two distinct primes with exponent >= 4 and a third distinct prime with exponent >= 2
    if c4 > 1 && c2 > 2 {
        // Choose 2 primes from c4, then choose 1 from remaining primes with exp>=2.
        ans += ((c4 as i64) * ((c4 - 1) as i64) / 2) * ((c2 - 2) as i64);
    }

    println!("{}", ans);
}
