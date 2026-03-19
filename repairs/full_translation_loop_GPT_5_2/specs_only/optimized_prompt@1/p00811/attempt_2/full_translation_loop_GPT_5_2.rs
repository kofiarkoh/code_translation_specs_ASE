use std::io::{self, Read};

fn sieve_primes(limit: usize) -> Vec<u32> {
    if limit < 2 {
        return Vec::new();
    }
    let mut is_prime = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    let mut p = 2usize;
    while p * p <= limit {
        if is_prime[p] {
            let mut k = p * p;
            while k <= limit {
                is_prime[k] = false;
                k += p;
            }
        }
        p += 1;
    }

    let mut primes = Vec::new();
    for i in 2..=limit {
        if is_prime[i] {
            primes.push(i as u32);
        }
    }
    primes
}

fn main() {
    // The original problem this spec corresponds to is typically solved by precomputing primes
    // and then searching for the maximum p*q <= m under the ratio constraint.
    // The sample indicates m up to at least 100000; we keep a generous prime table.
    const PRIME_LIMIT: usize = 1_299_709; // 100,000th prime
    let primes = sieve_primes(PRIME_LIMIT);

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let Some(ms) = it.next() else { break };
        let m: i64 = ms.parse().unwrap();
        let a: i64 = it.next().unwrap().parse().unwrap();
        let b: i64 = it.next().unwrap().parse().unwrap();

        if m == 0 && a == 0 && b == 0 {
            break;
        }

        // We must maximize p*q subject to:
        //   p, q prime
        //   p*q <= m
        //   a*q <= p*b  <=>  p/q >= a/b
        //
        // The failing attempt bounded q by (p*b)/a, which is incorrect because the inequality
        // constrains p relative to q, not q relative to p in that direction.
        //
        // Standard approach:
        // For each q, choose the largest prime p such that:
        //   p <= m/q
        //   p >= ceil(a*q / b)  (when b > 0)
        // and maximize p*q.
        //
        // The spec does not define negative/zero a,b behavior; we implement the narrowest
        // reasonable interpretation for typical valid inputs (positive a,b,m), while still
        // keeping comparisons in i128 to avoid overflow.
        let mut best_p: i64 = 0;
        let mut best_q: i64 = 0;
        let mut best_prod: i64 = 0;

        if m >= 4 && b != 0 {
            for &q_u in &primes {
                let q = q_u as i64;
                if q > m {
                    break;
                }

                let p_max = m / q;
                if p_max < 2 {
                    break;
                }

                // Compute lower bound for p from a*q <= p*b.
                // For b > 0: p >= ceil(a*q / b).
                // For b < 0: inequality flips; we avoid inventing behavior and just check directly later.
                let p_min = if b > 0 {
                    let num = (a as i128) * (q as i128);
                    let den = b as i128;
                    // ceil_div for possibly negative num: (num + den - 1)/den only works for num>=0.
                    // Given typical constraints a,b>0, this is fine; otherwise fall back to floor division
                    // and rely on direct check.
                    if num >= 0 {
                        ((num + den - 1) / den) as i64
                    } else {
                        (num / den) as i64
                    }
                } else {
                    i64::MIN / 4
                };

                if p_max < p_min {
                    continue;
                }

                // Find the largest prime p <= p_max.
                let mut idx = match primes.binary_search(&(p_max as u32)) {
                    Ok(i) => i,
                    Err(i) => {
                        if i == 0 {
                            continue;
                        }
                        i - 1
                    }
                };

                // Walk down until p >= p_min and satisfies the inequality exactly.
                loop {
                    let p = primes[idx] as i64;
                    if p < p_min {
                        break;
                    }

                    let prod = (p as i128) * (q as i128);
                    if prod <= m as i128 && (a as i128) * (q as i128) <= (p as i128) * (b as i128) {
                        let prod_i64 = prod as i64;
                        if prod_i64 > best_prod {
                            best_prod = prod_i64;
                            best_p = p;
                            best_q = q;
                        }
                        break; // for this q, this is the largest p; smaller p won't improve product
                    }

                    if idx == 0 {
                        break;
                    }
                    idx -= 1;
                }
            }
        }

        out.push_str(&format!("{} {}\n", best_p, best_q));
    }

    print!("{out}");
}
