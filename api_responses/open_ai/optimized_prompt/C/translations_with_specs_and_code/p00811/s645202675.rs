use std::io::{self, BufRead};

const MAX_PRIMES: usize = 100_000;

fn is_prime_sieve() -> Vec<usize> {
    let mut is_prime = vec![true; MAX_PRIMES];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut primes = Vec::new();

    for i in 2..MAX_PRIMES {
        if is_prime[i] {
            primes.push(i);
            let mut multiple = i * 2;
            while multiple < MAX_PRIMES {
                is_prime[multiple] = false;
                multiple += i;
            }
        }
    }
    primes
}

fn main() {
    let primes = is_prime_sieve();
    let primenum = primes.len();
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut inputs = line.split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok());
        
        let m = inputs.next().unwrap_or(0);
        let a = inputs.next().unwrap_or(0);
        let b = inputs.next().unwrap_or(0);

        if m == 0 && a == 0 && b == 0 {
            break;
        }

        let (mut p, mut q) = (0, 0);

        for i in 0..primenum {
            let mut l = i + 1; // Start l from i + 1 to ensure l > i
            let mut r = primenum - 1;

            while l <= r {
                let j = (l + r) / 2;
                if (primes[i] as i64 * primes[j] as i64) <= (m as i64) &&
                   (a as i64 * primes[j] as i64) <= (primes[i] as i64 * b as i64) {
                    l = j + 1;
                } else {
                    r = j - 1;
                }
            }

            if l > i && (primes[i] as i64 * primes[l - 1] as i64) <= (m as i64) &&
               (a as i64 * primes[l - 1] as i64) <= (primes[i] as i64 * b as i64) &&
               (primes[i] * primes[l - 1] > p * q) {
                p = primes[i];
                q = primes[l - 1];
            }
        }

        println!("{} {}", p, q);
    }
}
