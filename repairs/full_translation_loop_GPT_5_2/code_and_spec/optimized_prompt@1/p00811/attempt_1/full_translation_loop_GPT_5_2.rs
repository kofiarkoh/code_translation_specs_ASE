use std::io::{self, Read};

fn sieve_primes(limit: usize) -> Vec<i32> {
    // Mirrors the C code's sieve up to 100000 (exclusive).
    let mut is_prime = vec![true; limit];
    if limit > 0 {
        is_prime[0] = false;
    }
    if limit > 1 {
        is_prime[1] = false;
    }

    let mut primes = Vec::new();
    for i in 2..limit {
        if is_prime[i] {
            primes.push(i as i32);
            let mut j = i + i;
            while j < limit {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    primes
}

fn main() {
    // Read all input; behavior is undefined for invalid formats per spec/C.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let primes = sieve_primes(100_000);
    let primenum = primes.len();

    let mut out = String::new();

    loop {
        let m = match it.next() {
            Some(x) => x.parse::<i64>().unwrap(),
            None => break,
        };
        let a = match it.next() {
            Some(x) => x.parse::<i64>().unwrap(),
            None => break,
        };
        let b = match it.next() {
            Some(x) => x.parse::<i64>().unwrap(),
            None => break,
        };

        if (m | a | b) == 0 {
            break;
        }

        let mut best_p: i32 = 0;
        let mut best_q: i32 = 0;
        let mut best_prod: i64 = 0;

        for i in 0..primenum {
            let pi = primes[i] as i64;

            // Binary search for the largest j in [i, primenum-1] satisfying:
            // pi*pj <= m and a*pj <= pi*b
            let mut l: i32 = i as i32;
            let mut r: i32 = (primenum as i32) - 1;

            while l <= r {
                let mid = (l + r) / 2;
                let pj = primes[mid as usize] as i64;

                if pi * pj <= m && a * pj <= pi * b {
                    l = mid + 1;
                } else {
                    r = mid - 1;
                }
            }

            // Candidate index is l-1 if l > i (i.e., at least one valid j >= i found)
            if (l as usize) > i {
                let j = (l - 1) as usize;
                let pj = primes[j] as i64;

                // Re-check conditions exactly like C (defensive, matches semantics).
                if pi * pj <= m && a * pj <= pi * b {
                    let prod = pi * pj;
                    if prod > best_prod {
                        best_prod = prod;
                        best_p = primes[i];
                        best_q = primes[j];
                    }
                }
            }
        }

        out.push_str(&format!("{} {}\n", best_p, best_q));
    }

    print!("{out}");
}
