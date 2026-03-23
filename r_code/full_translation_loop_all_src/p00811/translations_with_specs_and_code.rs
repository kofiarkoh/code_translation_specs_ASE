use std::io::{self, Read};

fn generate_primes_under_100000() -> Vec<i32> {
    const N: usize = 100_000;
    let mut is_prime = vec![true; N];
    if N > 0 {
        is_prime[0] = false;
    }
    if N > 1 {
        is_prime[1] = false;
    }

    let mut primes = Vec::new();
    for i in 2..N {
        if is_prime[i] {
            primes.push(i as i32);
            let mut j = i + i;
            while j < N {
                is_prime[j] = false;
                j += i;
            }
        }
    }
    primes
}

fn main() {
    // Read entire stdin; stop if we can't read full triples.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let primes = generate_primes_under_100000();
    let primenum = primes.len();

    let mut out = String::new();

    loop {
        let m_opt = it.next();
        let a_opt = it.next();
        let b_opt = it.next();
        if m_opt.is_none() || a_opt.is_none() || b_opt.is_none() {
            break; // input ended or incomplete record
        }

        let m: i64 = m_opt.unwrap().parse::<i64>().unwrap();
        let a: i64 = a_opt.unwrap().parse::<i64>().unwrap();
        let b: i64 = b_opt.unwrap().parse::<i64>().unwrap();

        if ((m as i64) | (a as i64) | (b as i64)) == 0 {
            break; // terminating triple
        }

        let mut best_p: i32 = 0;
        let mut best_q: i32 = 0;
        let mut best_prod: i64 = 0;

        for i in 0..primenum {
            let pi = primes[i] as i64;

            // Binary search for the largest j in [i, primenum-1] satisfying both constraints.
            let mut l: isize = i as isize;
            let mut r: isize = (primenum as isize) - 1;
            while l <= r {
                let mid = (l + r) / 2;
                let pj = primes[mid as usize] as i64;

                if pi * pj <= m && a * pj <= pi * b {
                    l = mid + 1;
                } else {
                    r = mid - 1;
                }
            }

            // Candidate index is l-1; must be >= i.
            if l > i as isize {
                let j = (l - 1) as usize;
                let pj = primes[j] as i64;

                // Re-check constraints (mirrors C code) and update only on strictly larger product.
                let prod = pi * pj;
                if prod <= m && a * pj <= pi * b && prod > best_prod {
                    best_prod = prod;
                    best_p = primes[i];
                    best_q = primes[j];
                }
            }
        }

        out.push_str(&format!("{} {}\n", best_p, best_q));
    }

    print!("{out}");
}
