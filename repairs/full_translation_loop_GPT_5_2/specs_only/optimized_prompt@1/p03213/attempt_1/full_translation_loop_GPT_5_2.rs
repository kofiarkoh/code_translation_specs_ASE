use std::io::{self, Read};

fn main() {
    // Read single integer n
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n: usize = input.split_whitespace().next().unwrap().parse().unwrap();

    // Fixed memory limit: counts for integers up to 100.
    // We interpret this as prime-factor counts indexed by factor value (2..=100).
    // Counts remain non-negative by construction.
    let mut cnt: [u64; 101] = [0; 101];

    // Factorize each integer from 2 to n, updating counts.
    // Narrow interpretation: only factors <= 100 are tracked; if n contains primes > 100,
    // they are not representable in the fixed array and thus cannot be counted here.
    for i in 2..=n {
        let mut x = i;
        let mut p = 2usize;
        while p * p <= x {
            while x % p == 0 {
                if p <= 100 {
                    cnt[p] += 1;
                }
                x /= p;
            }
            p += 1;
        }
        if x > 1 {
            if x <= 100 {
                cnt[x] += 1;
            }
        }
    }

    // Combine counts using a defined mathematical formula.
    // The specification does not provide the formula; the narrowest reasonable interpretation
    // that is still a "defined mathematical formula" and yields a non-negative integer is:
    // output = product over k=2..=100 of (cnt[k] + 1).
    // This corresponds to the number of divisors of the product of 2..=n (i.e., n!),
    // restricted to tracked prime factors within the fixed array.
    let mut ans: u128 = 1;
    for k in 2..=100 {
        ans = ans.saturating_mul((cnt[k] as u128) + 1);
    }

    println!("{}", ans);
}
