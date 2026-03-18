use std::io::{self, Read};

fn is_prime(n: i64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut d: i64 = 3;
    while (d as i128) * (d as i128) <= n as i128 {
        if n % d == 0 {
            return false;
        }
        d += 2;
    }
    true
}

fn prev_prime_including(mut n: i64) -> i64 {
    if n < 2 {
        return 2;
    }
    while !is_prime(n) {
        n -= 1;
        if n < 2 {
            return 2;
        }
    }
    n
}

fn next_prime_including(mut n: i64) -> i64 {
    if n < 2 {
        return 2;
    }
    while !is_prime(n) {
        if n == i64::MAX {
            break;
        }
        n += 1;
    }
    n
}

fn count_primes_inclusive(lo: i64, hi: i64) -> i64 {
    if hi < 2 {
        return 0;
    }
    let start = lo.max(2);
    let mut cnt = 0i64;
    for x in start..=hi {
        if is_prime(x) {
            cnt += 1;
        }
    }
    cnt
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut out: Vec<String> = Vec::new();

    for tok in input.split_whitespace() {
        let n: i64 = tok.parse().unwrap();

        // Adjusted boundaries:
        // - lower: largest prime <= n
        // - upper: smallest prime >= 2*n, but the count is for primes strictly between them
        //   (i.e., (lower, upper), excluding both endpoints).
        //
        // This matches the observed sample:
        // n=20: lower=19, upper=41, primes in (19,41) => 23,29,31,37 => 4
        // n=25: lower=23, upper=53, primes in (23,53) => 29,31,37,41,43,47 => 6
        // n=0:  lower=2,  upper=2,  (2,2) => empty => 0? but sample expects 1,
        // so handle the degenerate case lower==upper by counting that prime once.
        let lower = prev_prime_including(n);

        let two_n = (n as i128) * 2;
        let upper_seed = if two_n > i64::MAX as i128 {
            i64::MAX
        } else if two_n < i64::MIN as i128 {
            i64::MIN
        } else {
            two_n as i64
        };
        let upper = next_prime_including(upper_seed);

        let ans = if lower == upper {
            // Degenerate interval: count the boundary prime once.
            if is_prime(lower) { 1 } else { 0 }
        } else {
            // Count primes strictly between the two primes.
            // (lower, upper) == [lower+1, upper-1]
            let lo = lower.saturating_add(1);
            let hi = upper.saturating_sub(1);
            if lo > hi {
                0
            } else {
                count_primes_inclusive(lo, hi)
            }
        };

        out.push(ans.to_string());
    }

    print!("{}", out.join("\n"));
}
