use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let n: usize = s.split_whitespace().next().unwrap().parse().unwrap();

    // Count prime exponents in n! by factoring each i in 2..=n.
    // Storage is fixed up to 100 as per the specification.
    let mut exp = [0u32; 101];

    for i in 2..=n {
        let mut x = i;
        let mut p = 2usize;
        while p * p <= x {
            while x % p == 0 {
                if p <= 100 {
                    exp[p] += 1;
                }
                x /= p;
            }
            p += 1;
        }
        if x > 1 && x <= 100 {
            exp[x] += 1;
        }
    }

    // For each prime p, the divisor-count contribution is (exp[p] + 1).
    // We need counts of primes whose (exp+1) meets certain thresholds:
    //  >= 3  (exp>=2), >= 5 (exp>=4), >= 15 (exp>=14), >= 25 (exp>=24), >= 75 (exp>=74).
    let mut c3 = 0u32;
    let mut c5 = 0u32;
    let mut c15 = 0u32;
    let mut c25 = 0u32;
    let mut c75 = 0u32;

    for p in 2..=100 {
        let e = exp[p];
        if e >= 2 {
            c3 += 1;
        }
        if e >= 4 {
            c5 += 1;
        }
        if e >= 14 {
            c15 += 1;
        }
        if e >= 24 {
            c25 += 1;
        }
        if e >= 74 {
            c75 += 1;
        }
    }

    // Count numbers with exactly 75 divisors among divisors of n!.
    // 75 factor patterns in terms of (exponent+1):
    //  (75)            -> one prime with (e+1)=75  => e>=74
    //  (25,3)          -> one prime with e>=24 and another with e>=2
    //  (15,5)          -> one prime with e>=14 and another with e>=4
    //  (5,5,3)         -> two primes with e>=4 and a third with e>=2
    //
    // Use threshold counts with careful exclusion to ensure distinct primes.
    let ans: u64 = (c75 as u64)
        + (c25 as u64) * ((c3.saturating_sub(1)) as u64)
        + (c15 as u64) * ((c5.saturating_sub(1)) as u64)
        + ((c5 as u64) * ((c5.saturating_sub(1)) as u64) / 2) * ((c3.saturating_sub(2)) as u64);

    println!("{}", ans);
}
