use std::io::{self, BufRead};

const MAX: usize = 100_000;

struct PrimeGenerator {
    is_prime: [bool; MAX],
    primes: Vec<usize>,
}

impl PrimeGenerator {
    fn new() -> Self {
        let mut is_prime = [true; MAX];
        is_prime[0] = false;
        is_prime[1] = false;

        let mut primes = Vec::new();
        for i in 2..MAX {
            if is_prime[i] {
                primes.push(i);
                let mut j = i * 2;
                while j < MAX {
                    is_prime[j] = false;
                    j += i;
                }
            }
        }

        PrimeGenerator { is_prime, primes }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let prime_gen = PrimeGenerator::new();
    let primenum = prime_gen.primes.len();

    loop {
        let line = lines.next();
        if line.is_none() {
            break;
        }
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
                if (prime_gen.primes[i] as i64 * prime_gen.primes[j] as i64) <= m as i64 &&
                   (a as i64 * prime_gen.primes[j] as i64) <= (prime_gen.primes[i] as i64 * b as i64) {
                    l = j + 1;
                } else {
                    r = j - 1;
                }
            }

            if l > i && (l as usize - 1 < primenum) && 
               (prime_gen.primes[i] as i64 * prime_gen.primes[l as usize - 1] as i64) <= m as i64 &&
               (a as i64 * prime_gen.primes[l as usize - 1] as i64) <= (prime_gen.primes[i] as i64 * b as i64) &&
               (prime_gen.primes[i] * prime_gen.primes[l as usize - 1] > p * q) {
                p = prime_gen.primes[i];
                q = prime_gen.primes[l as usize - 1];
            }
        }

        println!("{} {}", p, q);
    }
}
