use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = it.next().unwrap().parse().unwrap();
    let mut p: i64 = it.next().unwrap().parse().unwrap();

    // Specification assumes: n >= 1, p >= 1, and no overflow in intermediate products.

    let mut ans: i64 = 1;
    let mut i: i64 = 2;

    // Trial division up to sqrt(p), updating p as factors are removed.
    while i * i <= p {
        if p % i == 0 {
            let mut cnt: i64 = 0;
            while p % i == 0 {
                p /= i;
                cnt += 1;
            }
            let times = cnt / n;
            for _ in 0..times {
                ans *= i;
            }
        }
        i += 1;
    }

    // If remaining p > 1, it is prime with exponent 1 in the remaining factorization.
    // It contributes only if n == 1.
    if p > 1 && n == 1 {
        ans *= p;
    }

    println!("{}", ans);
}
